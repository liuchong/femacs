// femacs: port of nt/addpm.c @ fd7afced403.
//
// C 19-30's banner, carried as this unit's header prose:
//
//   Program: addpm	(adds emacs to the Windows program manager)
//
//   Usage:
//   	argv[1] = install path for emacs
//
//   argv[2] used to be an optional argument for setting the icon.
//   But now Emacs has a professional looking icon of its own.
//   If users really want to change it, they can go into the settings of
//   the shortcut that is created and do it there.
//
// C 1 (the banner's first line) is where this file named the reference program, and PORTING.md
// 0.1-19's seventh half (status 3.503) makes a program's statement of its *own* name this
// repository's own; every other word above is C's.  C 32-36, why the shell API and the DDE
// fallback are both here:
//
//   Use parts of shell API that were introduced by the merge of IE4
//   into the desktop shell.  If Windows 95 or NT4 users do not have IE4
//   installed, then the DDE fallback for creating icons the Windows 3.1
//   progman way will be used instead, but that is prone to lockups
//   caused by other applications not servicing their message queues.
//
// The file is a P-86 unit end to end: the frozen configuration compiles no `nt/` object at all
// (`docs/baseline/frozen-build-sources.txt` records none; `emacs/nt/Makefile.in` builds it for the
// Windows port only), so `extract-abi.py`'s checklist is empty and the **build-level file gate** is
// `feature = "windowsnt"` - on every item and on the include graph (P-86 ⑧).  The whole text is
// carried rather than summarised, and `docs/ports/addpm.md` holds the C line ranges, the readings
// and every deviation.  No Windows SDK, CRT or MinGW-w64 exists on this machine, so the API surface
// this unit names is declared at the C `#include` lines that would have declared it (P-88 ①) and
// every value and record in it is **UNVERIFIED** (P-88 ②: no number from memory) - the ledger's
// owed list names what measures it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens, unused_labels, clippy::all)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");                    // C 39: `#include <config.h>`

/* C 38: `#define DEFER_MS_W32_H` - the marker `nt/inc/ms-w32.h` reads to defer itself; no `w32.h`
   is in this unit's graph, and the name is the row `docs/ports/excluded-macros.txt` carries for it
   (`femacs/src/w32.rs` 83 is the const that replaces it elsewhere).  It has no site here.  */

/* C 41-43: `<stdlib.h>`, `<stdio.h>`, `<malloc.h>`.  The first two are mirrors; the third has
   none, and it contributes only `alloca`, which is P-05's stack pool below.  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");                    // C 41: `#include <stdlib.h>` (exit below)
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");                     // C 42: `#include <stdio.h>` (sprintf, fprintf, stderr)
// C 43: `#include <malloc.h>` - no mirror; the only name this unit takes from it is `alloca`, which
// C 137 calls.  That is PORTING.md P-05's stack pool: `sa_stack_alloc` (`femacs/src/alloc.rs` 1039,
// the `nt/runemacs.rs` 305-311 reading and the one deliberate class of port-only exports).
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn sa_stack_alloc (size: usize) -> *mut c_void;
}

/* C's `<string.h>` names: the reference does not include `<string.h>`, and reaches `strlen`,
   `strrchr`, `strcat` and `stricmp` through the SDK headers it does include (there is no SDK here),
   so the unit declares the four at their use site (P-88 ①, the `nt/runemacs.rs` 289-297 shape for
   the two `<string.h>` does not carry there).  `stricmp` is the MSVC CRT's case-insensitive
   `strcmp`.  */
/// C: <stdlib.h>: `void exit (int status) _Noreturn;`
/// C: <string.h>: `size_t strlen (const char *s);`
/// C: <string.h>: `char *strrchr (const char *s, int c);`
/// C: <string.h>: `char *strcat (char *dest, const char *src);`
/// C: <string.h> (MSVC): `int stricmp (const char *s1, const char *s2);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
  pub fn strlen (s: *const c_char) -> usize;
  pub fn strrchr (s: *const c_char, c: c_int) -> *mut c_char;
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn stricmp (s1: *const c_char, s2: *const c_char) -> c_int;
}

/* ----------------------------------------------------------------------------------------------
   C 45-53: the SDK request macros and the two `#include`s that follow them.
   ---------------------------------------------------------------------------------------------- */

/* C 45-51: C's own note that MinGW64 barfs if `_WIN32_IE` is defined to anything below 0x0500,
   then `#ifndef MINGW_W64` / `# ifdef _WIN32_IE` / `#  undef _WIN32_IE` / `# endif` /
   `#define _WIN32_IE 0x0400` / `#endif`.  Both arms are carried; the frozen side is
   `not (feature = "mingw_w64")`.  The `#ifdef _WIN32_IE`/`#undef` pair inside it is a
   preprocessor operation on a macro this unit does not own (a build-environment or
   earlier-header value) - it has no Rust name, and this unit declares the surface it uses itself
   (Deviation 3).  */
#[cfg (all (feature = "windowsnt", not (feature = "mingw_w64")))]
pub const _WIN32_IE: c_uint = 0x0400;                    // placeholder, not <sdkddkver.h>'s value

/* C 52-53: C's own note that the C object macros are requested for COM interfaces, then
   `#define COBJMACROS 1` - the macro that makes the SDK's headers emit the
   `IShellLinkA_SetPath (p, a)` spellings this unit calls; the port spells each of them as a
   `macro_rules!` of the same name, below (the `femacs/src/w32fns.rs` 92-95 reading of the same
   macro).  */
#[cfg (feature = "windowsnt")]
pub const COBJMACROS: c_int = 1;

/* ----------------------------------------------------------------------------------------------
   C 55: `#include <windows.h>` - the SDK surface this unit uses (P-88 ①: no mirror carries
   `<windows.h>`, so the unit declares what it names).  C 56's `<shlobj.h>` and C 57's `<ddeml.h>`
   are declared in their own sections below.  Every constant value here is a **placeholder** and
   every record layout is unmeasured: there is no Windows SDK, CRT or MinGW-w64 on this machine, so
   the section is UNVERIFIED (P-88 ②; the ledger's Owed 2 names the host that would measure it).
   Where a landed unit already measured a value, this one cites that file and line instead.
   ---------------------------------------------------------------------------------------------- */

/// C: `typedef void *HANDLE, *HWND, *HMODULE, *LPVOID;` (the `nt/runemacs.rs` 60-70 reading)
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HWND = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
/// C: `typedef char CHAR, *LPSTR, *PSTR; typedef const char *LPCSTR, *PCSTR; typedef BYTE *LPBYTE;`
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPBYTE = *mut u8;
/// C: `typedef unsigned long DWORD;`   // measured (`src/w32.rs` 230, `src/w32proc.rs` 257)
#[cfg (feature = "windowsnt")]
pub type DWORD = u32;
/// C: `typedef unsigned int UINT;` (the `nt/runemacs.rs` 78 reading)
#[cfg (feature = "windowsnt")]
pub type UINT = c_uint;
/// C: `typedef int BOOL;`   // measured (`src/w32.rs` 236)
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
/// C: `typedef long LONG;` (32-bit on the Windows ABI) and `typedef LONG LSTATUS;`
#[cfg (feature = "windowsnt")]
pub type LONG = i32;
#[cfg (feature = "windowsnt")]
pub type LSTATUS = LONG;
/// C: `typedef ULONG_PTR DWORD_PTR;` (a pointer-sized unsigned integer on a 64-bit target)
#[cfg (feature = "windowsnt")]
pub type DWORD_PTR = usize;
/// C: `typedef DWORD *LPDWORD;`
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;
/// C: `typedef LONG_PTR HRESULT;` (the `nt/runemacs.rs` 82-84 reading)
#[cfg (feature = "windowsnt")]
pub type HRESULT = isize;
/// C: `typedef HANDLE HKEY;` and `typedef HKEY *PHKEY;` (winreg.h)
#[cfg (feature = "windowsnt")]
pub type HKEY = HANDLE;
#[cfg (feature = "windowsnt")]
pub type PHKEY = *mut HKEY;
/// C: `typedef DWORD REGSAM;` (winreg.h) and `typedef ACCESS_MASK REGSAM;`
#[cfg (feature = "windowsnt")]
pub type REGSAM = DWORD;
/// C: `typedef struct _SECURITY_ATTRIBUTES { ... } SECURITY_ATTRIBUTES;` - only ever passed as
/// the NULL pointer here, so the parameter is C's pointer type spelled `*mut c_void` (P-55).
#[cfg (feature = "windowsnt")]
pub type LPSECURITY_ATTRIBUTES = *mut c_void;

/// C: `#define MAX_PATH 260`   // measured (`src/w32.rs` 316, clang against the frozen table)
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;
/// C: `#define TRUE 1` / `#define FALSE 0`   // measured (`src/w32.rs` 310/313)
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;

/// C: `#define HKEY_LOCAL_MACHINE ((HKEY) (ULONG_PTR) (LONG) 0x80000002)`
#[cfg (feature = "windowsnt")]
pub const HKEY_LOCAL_MACHINE: HKEY = 0x8000_0002isize as HKEY;   // placeholder, not winreg.h's
/// C: `#define HKEY_CURRENT_USER ((HKEY) (ULONG_PTR) (LONG) 0x80000001)`
#[cfg (feature = "windowsnt")]
pub const HKEY_CURRENT_USER: HKEY = 0x8000_0001isize as HKEY;    // placeholder, not winreg.h's
/// C: `#define REG_OPTION_NON_VOLATILE 0x00000000L`
#[cfg (feature = "windowsnt")]
pub const REG_OPTION_NON_VOLATILE: DWORD = 0x0000_0000;   // placeholder, not winreg.h's value
/// C: `#define KEY_WRITE (STANDARD_RIGHTS_WRITE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY)`
#[cfg (feature = "windowsnt")]
pub const KEY_WRITE: REGSAM = 0x0002_0006;                 // placeholder, not winreg.h's value
/// C: `#define KEY_QUERY_VALUE 0x0001`
#[cfg (feature = "windowsnt")]
pub const KEY_QUERY_VALUE: REGSAM = 0x0000_0001;           // placeholder, not winreg.h's value
/// C: `#define ERROR_SUCCESS 0L`
#[cfg (feature = "windowsnt")]
pub const ERROR_SUCCESS: LSTATUS = 0;                      // placeholder, not winerror.h's value
/// C: `#define ERROR_ALREADY_EXISTS 183L`
#[cfg (feature = "windowsnt")]
pub const ERROR_ALREADY_EXISTS: DWORD = 183;               // placeholder, not winerror.h's value
/// C: `#define REG_EXPAND_SZ 2`
#[cfg (feature = "windowsnt")]
pub const REG_EXPAND_SZ: DWORD = 2;                        // placeholder, not winreg.h's value
/// C: `#define MB_OKCANCEL 0x00000001L`
#[cfg (feature = "windowsnt")]
pub const MB_OKCANCEL: UINT = 0x0000_0001;                 // placeholder, not winuser.h's value
/// C: `#define MB_ICONQUESTION 0x00000020L`
#[cfg (feature = "windowsnt")]
pub const MB_ICONQUESTION: UINT = 0x0000_0020;             // placeholder, not winuser.h's value
/// C: `#define IDOK 1`
#[cfg (feature = "windowsnt")]
pub const IDOK: c_int = 1;                                 // placeholder, not winuser.h's value
/// C: `#define CLSCTX_INPROC_SERVER 0x1`
#[cfg (feature = "windowsnt")]
pub const CLSCTX_INPROC_SERVER: DWORD = 0x1;               // placeholder, not wtypesbase.h's value
/// C: `#define CP_ACP 0` and `#define CP_WINANSI 1004` (winnls.h; `CP_WINANSI` is ddeml.h's
/// spelling of the same code page)
#[cfg (feature = "windowsnt")]
pub const CP_ACP: UINT = 0;                                // placeholder, not winnls.h's value
#[cfg (feature = "windowsnt")]
pub const CP_WINANSI: c_int = 1004;                        // placeholder, not winnls.h's value

/// C: `WINADVAPI LONG WINAPI RegCreateKeyExA (HKEY hKey, LPCSTR lpSubKey, DWORD Reserved,
/// C:   LPSTR lpClass, DWORD dwOptions, REGSAM samDesired, const LPSECURITY_ATTRIBUTES
/// C:   lpSecurityAttributes, PHKEY phkResult, LPDWORD lpdwDisposition);`
/// C: `WINADVAPI LONG WINAPI RegSetValueExA (HKEY hKey, LPCSTR lpValueName, DWORD Reserved,
/// C:   DWORD dwType, const BYTE *lpData, DWORD cbData);`
/// C: `WINADVAPI LONG WINAPI RegOpenKeyExA (HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions,
/// C:   REGSAM samDesired, PHKEY phkResult);`
/// C: `WINADVAPI LONG WINAPI RegQueryValueExA (HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved,
/// C:   LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);`
/// C: `WINADVAPI LONG WINAPI RegCloseKey (HKEY hKey);`
/// C: `WINBASEAPI DWORD WINAPI GetModuleFileNameA (HMODULE hModule, LPSTR lpFilename, DWORD nSize);`
/// C: `WINBASEAPI BOOL WINAPI CreateDirectoryA (LPCSTR lpPathName,
/// C:   LPSECURITY_ATTRIBUTES lpSecurityAttributes);`
/// C: `WINBASEAPI DWORD WINAPI GetLastError (void);`
/// C: `WINBASEAPI int WINAPI lstrlenA (LPCSTR lpString);`
/// C: `WINUSERAPI int WINAPI MessageBoxA (HWND hWnd, LPCSTR lpText, LPCSTR lpCaption, UINT uType);`
/// C: `WINBASEAPI int WINAPI MultiByteToWideChar (UINT CodePage, DWORD dwFlags,
/// C:   LPCCH lpMultiByteStr, int cbMultiByte, LPWSTR lpWideCharStr, int cchWideChar);`
/// C: `WINOLEAPI CoInitialize (LPVOID pvReserved);`
/// C: `WINOLEAPI CoCreateInstance (REFCLSID rclsid, LPUNKNOWN pUnkOuter, DWORD dwClsContext,
/// C:   REFIID riid, LPVOID *ppv);`
/// C: `WINOLEAPI (CoUninitialize) (void);`
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn RegCreateKeyEx (hKey: HKEY, lpSubKey: LPCSTR, Reserved: DWORD, lpClass: LPSTR,
                         dwOptions: DWORD, samDesired: REGSAM,
                         lpSecurityAttributes: LPSECURITY_ATTRIBUTES, phkResult: PHKEY,
                         lpdwDisposition: LPDWORD) -> LSTATUS;
  pub fn RegSetValueEx (hKey: HKEY, lpValueName: LPCSTR, Reserved: DWORD, dwType: DWORD,
                        lpData: LPBYTE, cbData: DWORD) -> LSTATUS;
  pub fn RegOpenKeyEx (hKey: HKEY, lpSubKey: LPCSTR, ulOptions: DWORD, samDesired: REGSAM,
                       phkResult: PHKEY) -> LSTATUS;
  pub fn RegQueryValueEx (hKey: HKEY, lpValueName: LPCSTR, lpReserved: LPDWORD, lpType: LPDWORD,
                          lpData: LPBYTE, lpcbData: LPDWORD) -> LSTATUS;
  pub fn RegCloseKey (hKey: HKEY) -> LSTATUS;
  pub fn GetModuleFileName (hModule: HMODULE, lpFilename: LPSTR, nSize: DWORD) -> DWORD;
  pub fn CreateDirectory (lpPathName: LPCSTR,
                          lpSecurityAttributes: LPSECURITY_ATTRIBUTES) -> BOOL;
  pub fn GetLastError () -> DWORD;
  pub fn lstrlen (lpString: LPCSTR) -> c_int;
  pub fn MessageBox (hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
  pub fn MultiByteToWideChar (CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCSTR,
                              cbMultiByte: c_int, lpWideCharStr: *mut u16,
                              cchWideChar: c_int) -> c_int;
  pub fn CoInitialize (pvReserved: LPVOID) -> HRESULT;
  pub fn CoCreateInstance (rclsid: *const CLSID, pUnkOuter: LPVOID, dwClsContext: DWORD,
                           riid: *const IID, ppv: *mut LPVOID) -> HRESULT;
  pub fn CoUninitialize ();
}

/// C: `#define SUCCEEDED(hr) (((HRESULT) (hr)) >= 0)` (winerror.h) - C's literal fold.  It sits in
/// expression position at every use, so the port spells it as C's call syntax names it, an inline
/// function (the `femacs/src/callproc.rs` 262-265 reading of `<sys/wait.h>`'s `W*` family, which is
/// this same class of external-header macro).  `FAILED` is not named by this file.
#[cfg (feature = "windowsnt")]
#[inline(always)] fn SUCCEEDED (hr: HRESULT) -> bool { hr >= 0 }

/* ----------------------------------------------------------------------------------------------
   C 56: `#include <shlobj.h>` (with `#include <shobjidl.h>`, `<objidl.h>`, `<ole2.h>` behind it) -
   the COM side of the shell: `IShellLinkA`, `IPersistFile`, the two interface identifiers, the
   shell-link class identifier and the special-folder call.  The `COBJMACROS` spelling of each
   interface method is what C calls (`IShellLinkA_SetPath (p, a)`), so each is a `macro_rules!` of
   the same name whose body is the `(p)->lpVtbl->Method (p, …)` expansion the header defines (the
   `w32fns.rs` 92-95 reading of the same macro family).  The vtable member *order* is the
   interface's definition; no SDK is installed here, so the order and the record layout are
   declared from the interface and marked UNVERIFIED (P-88 ②, Owed 2).
   ---------------------------------------------------------------------------------------------- */

/// C: `typedef struct _GUID { unsigned long Data1; unsigned short Data2; unsigned short Data3;
/// C:   unsigned char Data4[8]; } GUID;` and `typedef GUID CLSID, IID;`
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID
{
  pub Data1: u32,
  pub Data2: u16,
  pub Data3: u16,
  pub Data4: [u8; 8],
}
#[cfg (feature = "windowsnt")]
pub type CLSID = GUID;
#[cfg (feature = "windowsnt")]
pub type IID = GUID;
/// C: `typedef unsigned short OLECHAR; typedef OLECHAR *LPOLESTR; typedef const OLECHAR *LPCOLESTR;`
#[cfg (feature = "windowsnt")]
pub type LPCOLESTR = *const u16;

/// C: `EXTERN_C const CLSID CLSID_ShellLink;` (shlguid.h/initguid.h: the shell-link coclass) and
/// the two `IID`s `IShellLinkA_QueryInterface` is asked for.
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub static CLSID_ShellLink: CLSID;
  pub static IID_IShellLinkA: IID;
  pub static IID_IPersistFile: IID;
}

/// C: `typedef struct IShellLinkA IShellLinkA;` with `THIS_ IShellLinkA *` in each method - the
/// instance is the vtable pointer (`#ifdef COBJMACROS`'s `(p)->lpVtbl->…`) .
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IShellLinkA
{
  pub lpVtbl: *mut IShellLinkAVtbl,
}

/// C: `IShellLinkAVtbl` (shobjidl.h/shobjidl_core.h): `IUnknown`'s three, `IPersist`'s
/// `GetClassID`, then `IShellLinkA`'s own in declaration order.  Only `QueryInterface`,
/// `Release`, `SetPath` and `SetDescription` are called by this file; the members between them are
/// carried because the offsets of the called ones are decided by them.  **The order is
/// UNVERIFIED** (no SDK on this machine).
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IShellLinkAVtbl
{
  pub QueryInterface: unsafe extern "system" fn (This: *mut IShellLinkA, riid: *const IID,
                                                ppvObject: *mut LPVOID) -> HRESULT,
  pub AddRef: unsafe extern "system" fn (This: *mut IShellLinkA) -> u32,
  pub Release: unsafe extern "system" fn (This: *mut IShellLinkA) -> u32,
  pub GetClassID: unsafe extern "system" fn (This: *mut IShellLinkA, pclsid: *mut CLSID)
                                             -> HRESULT,
  pub GetPath: unsafe extern "system" fn (This: *mut IShellLinkA, pszFile: LPSTR,
                                          cch: c_int, pfd: *mut c_void, fFlags: DWORD) -> HRESULT,
  pub SetPath: unsafe extern "system" fn (This: *mut IShellLinkA, pszFile: LPCSTR) -> HRESULT,
  pub GetWorkingDirectory: unsafe extern "system" fn (This: *mut IShellLinkA, pszDir: LPSTR,
                                                      cch: c_int) -> HRESULT,
  pub SetWorkingDirectory: unsafe extern "system" fn (This: *mut IShellLinkA, pszDir: LPCSTR)
                                                      -> HRESULT,
  pub GetArguments: unsafe extern "system" fn (This: *mut IShellLinkA, pszArgs: LPSTR, cch: c_int)
                                               -> HRESULT,
  pub SetArguments: unsafe extern "system" fn (This: *mut IShellLinkA, pszArgs: LPCSTR) -> HRESULT,
  pub GetDescription: unsafe extern "system" fn (This: *mut IShellLinkA, pszName: LPSTR,
                                                 cch: c_int) -> HRESULT,
  pub SetDescription: unsafe extern "system" fn (This: *mut IShellLinkA, pszName: LPCSTR)
                                                 -> HRESULT,
  pub GetHotkey: unsafe extern "system" fn (This: *mut IShellLinkA, pwHotkey: *mut u16) -> HRESULT,
  pub SetHotkey: unsafe extern "system" fn (This: *mut IShellLinkA, wHotkey: u16) -> HRESULT,
  pub GetShowCmd: unsafe extern "system" fn (This: *mut IShellLinkA, piShowCmd: *mut c_int)
                                             -> HRESULT,
  pub SetShowCmd: unsafe extern "system" fn (This: *mut IShellLinkA, iShowCmd: c_int) -> HRESULT,
  pub GetIconLocation: unsafe extern "system" fn (This: *mut IShellLinkA, pszIconPath: LPSTR,
                                                  cch: c_int, piIcon: *mut c_int) -> HRESULT,
  pub SetIconLocation: unsafe extern "system" fn (This: *mut IShellLinkA, pszIconPath: LPCSTR,
                                                  iIcon: c_int) -> HRESULT,
  pub SetRelativePath: unsafe extern "system" fn (This: *mut IShellLinkA, pszPathRel: LPCSTR,
                                                  dwReserved: DWORD) -> HRESULT,
  pub Resolve: unsafe extern "system" fn (This: *mut IShellLinkA, hwnd: HWND, fFlags: DWORD)
                                          -> HRESULT,
  pub SetIDList: unsafe extern "system" fn (This: *mut IShellLinkA, pidl: *const c_void)
                                            -> HRESULT,
  pub GetIDList: unsafe extern "system" fn (This: *mut IShellLinkA, ppidl: *mut *mut c_void)
                                            -> HRESULT,
  pub GetFlags: unsafe extern "system" fn (This: *mut IShellLinkA, pfFlags: *mut DWORD) -> HRESULT,
  pub SetFlags: unsafe extern "system" fn (This: *mut IShellLinkA, fFlags: DWORD) -> HRESULT,
}

/// C: `typedef struct IPersistFile IPersistFile;` - the second interface `main` reaches through
/// `IShellLinkA_QueryInterface`.
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IPersistFile
{
  pub lpVtbl: *mut IPersistFileVtbl,
}

/// C: `IPersistFileVtbl`: `IUnknown`'s three, `IPersist`'s `GetClassID`, then `IPersistFile`'s
/// own.  `Save` and `Release` are the two this file calls.  **The order is UNVERIFIED.**
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IPersistFileVtbl
{
  pub QueryInterface: unsafe extern "system" fn (This: *mut IPersistFile, riid: *const IID,
                                                ppvObject: *mut LPVOID) -> HRESULT,
  pub AddRef: unsafe extern "system" fn (This: *mut IPersistFile) -> u32,
  pub Release: unsafe extern "system" fn (This: *mut IPersistFile) -> u32,
  pub GetClassID: unsafe extern "system" fn (This: *mut IPersistFile, pClassID: *mut CLSID)
                                             -> HRESULT,
  pub IsDirty: unsafe extern "system" fn (This: *mut IPersistFile) -> HRESULT,
  pub Load: unsafe extern "system" fn (This: *mut IPersistFile, pszFileName: LPCOLESTR,
                                       dwMode: DWORD) -> HRESULT,
  pub Save: unsafe extern "system" fn (This: *mut IPersistFile, pszFileName: LPCOLESTR,
                                       fRemember: BOOL) -> HRESULT,
  pub SaveCompleted: unsafe extern "system" fn (This: *mut IPersistFile, pszFileName: LPCOLESTR)
                                                -> HRESULT,
  pub GetCurFile: unsafe extern "system" fn (This: *mut IPersistFile, ppszFileName: *mut LPCOLESTR)
                                             -> HRESULT,
}

/// C: `SHSTDAPI_(BOOL) SHGetSpecialFolderPathA (HWND hwnd, LPSTR pszPath, int csidl, BOOL fCreate);`
/// C: `#define CSIDL_PROGRAMS 0x0002` and `#define CSIDL_COMMON_PROGRAMS 0x0017` (shlobj_core.h)
/// C: `#define SHGetSpecialFolderPath SHGetSpecialFolderPathA` under `#ifndef _UNICODE`
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn SHGetSpecialFolderPath (hwnd: HWND, pszPath: LPSTR, csidl: c_int, fCreate: BOOL) -> BOOL;
}
#[cfg (feature = "windowsnt")]
pub const CSIDL_PROGRAMS: c_int = 0x0002;                  // placeholder, not shlobj_core.h's value
#[cfg (feature = "windowsnt")]
pub const CSIDL_COMMON_PROGRAMS: c_int = 0x0017;           // placeholder, not shlobj_core.h's value

/// C: `#define IShellLinkA_QueryInterface(p, a, b) ((p)->lpVtbl->QueryInterface (p, a, b))`
/// C: `#define IShellLinkA_Release(p) ((p)->lpVtbl->Release (p))`
/// C: `#define IShellLinkA_SetPath(p, a) ((p)->lpVtbl->SetPath (p, a))`
/// C: `#define IShellLinkA_SetDescription(p, a) ((p)->lpVtbl->SetDescription (p, a))`
/// Each parameter is used as often as C's own expansion uses it (PORTING.md 2.1).
#[cfg (feature = "windowsnt")]
macro_rules! IShellLinkA_QueryInterface {
  ($p:expr, $a:expr, $b:expr) => { ((*(*$p).lpVtbl).QueryInterface) ($p, $a, $b) };
}
#[cfg (feature = "windowsnt")]
macro_rules! IShellLinkA_Release {
  ($p:expr) => { ((*(*$p).lpVtbl).Release) ($p) };
}
#[cfg (feature = "windowsnt")]
macro_rules! IShellLinkA_SetPath {
  ($p:expr, $a:expr) => { ((*(*$p).lpVtbl).SetPath) ($p, $a) };
}
#[cfg (feature = "windowsnt")]
macro_rules! IShellLinkA_SetDescription {
  ($p:expr, $a:expr) => { ((*(*$p).lpVtbl).SetDescription) ($p, $a) };
}
/// C: `#define IPersistFile_Save(This, a, b) ((This)->lpVtbl->Save (This, a, b))`
/// C: `#define IPersistFile_Release(This) ((This)->lpVtbl->Release (This))`
#[cfg (feature = "windowsnt")]
macro_rules! IPersistFile_Save {
  ($this:expr, $a:expr, $b:expr) => { ((*(*$this).lpVtbl).Save) ($this, $a, $b) };
}
#[cfg (feature = "windowsnt")]
macro_rules! IPersistFile_Release {
  ($this:expr) => { ((*(*$this).lpVtbl).Release) ($this) };
}

/* ----------------------------------------------------------------------------------------------
   C 57: `#include <ddeml.h>` - the DDE fallback's surface.  `<ddeml.h>`'s three handles are
   `DECLARE_HANDLE` pointer types, so each is `*mut c_void` here (the `nt/runemacs.rs` 60-70
   reading of `HANDLE`); every value is a **placeholder** and the section is UNVERIFIED (P-88 ②).
   ---------------------------------------------------------------------------------------------- */

/// C: `DECLARE_HANDLE(HSZ); DECLARE_HANDLE(HDDEDATA); typedef HANDLE HCONV;`
#[cfg (feature = "windowsnt")]
pub type HSZ = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HCONV = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HDDEDATA = *mut c_void;
/// C: `typedef HDDEDATA (CALLBACK *PFNCALLBACK) (UINT, UINT, HCONV, HSZ, HSZ, HDDEDATA,
/// C:   DWORD_PTR, DWORD_PTR);` - a callback pointer is nullable, so the typedef is an `Option`
/// (the `nt/runemacs.rs` 84-97 `FARPROC` reading).
#[cfg (feature = "windowsnt")]
pub type PFNCALLBACK
  = Option<unsafe extern "system" fn (UINT, UINT, HCONV, HSZ, HSZ, HDDEDATA, DWORD_PTR, DWORD_PTR)
                                      -> HDDEDATA>;
/// C: `#define CF_TEXT 1` (winuser.h: the clipboard formats this DDE transaction carries)
#[cfg (feature = "windowsnt")]
pub const CF_TEXT: UINT = 1;                               // placeholder, not winuser.h's value
/// C: `#define XTYP_EXECUTE (0x0040 | XTYPF_NOBLOCK)`
#[cfg (feature = "windowsnt")]
pub const XTYP_EXECUTE: UINT = 0x4050;                     // placeholder, not ddeml.h's value
/// C: `#define APPCMD_CLIENTONLY 0x00000010L`
#[cfg (feature = "windowsnt")]
pub const APPCMD_CLIENTONLY: DWORD = 0x0000_0010;          // placeholder, not ddeml.h's value

/// C: `UINT DdeInitializeA (LPDWORD pidInst, PFNCALLBACK pfnCallback, DWORD afCmd, DWORD ulRes);`
/// C: `HSZ DdeCreateStringHandleA (DWORD idInst, LPCSTR psz, int iCodePage);`
/// C: `HCONV DdeConnect (DWORD idInst, HSZ hszService, HSZ hszTopic, PCONVCONTEXT pCC);`
/// C: `HDDEDATA DdeClientTransaction (LPBYTE pData, DWORD cbData, HCONV hConv, HSZ hsz1,
/// C:   UINT wType, UINT wFmt, DWORD dwTimeout, LPDWORD pdwResult);`
/// C: `BOOL DdeDisconnect (HCONV hConv);`
/// C: `BOOL DdeFreeStringHandle (DWORD idInst, HSZ hsz);`
/// C: `BOOL DdeUninitialize (DWORD idInst);`
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn DdeInitialize (pidInst: LPDWORD, pfnCallback: PFNCALLBACK, afCmd: DWORD, ulRes: DWORD)
                        -> UINT;
  pub fn DdeCreateStringHandle (idInst: DWORD, psz: LPCSTR, iCodePage: c_int) -> HSZ;
  pub fn DdeConnect (idInst: DWORD, hszService: HSZ, hszTopic: HSZ, pCC: *mut c_void) -> HCONV;
  pub fn DdeClientTransaction (pData: LPBYTE, cbData: DWORD, hConv: HCONV, hsz1: HSZ, wType: UINT,
                               wFmt: UINT, dwTimeout: DWORD, pdwResult: LPDWORD) -> HDDEDATA;
  pub fn DdeDisconnect (hConv: HCONV) -> BOOL;
  pub fn DdeFreeStringHandle (idInst: DWORD, hsz: HSZ) -> BOOL;
  pub fn DdeUninitialize (idInst: DWORD) -> BOOL;
}

/* ----------------------------------------------------------------------------------------------
   C 59-61: `#ifndef OLD_PATHS` / `#include "../src/epaths.h"` / `#endif` - a header the build
   generates (`src/epaths.in` is its template), so no mirror exists and the port keeps the values
   the frozen build measured (`build/emacs-ref/src/epaths.h`, the `femacs/src/lread.rs` 69-80 and
   `femacs/src/callproc.rs` 271-277 shape).  Both arms are carried: the frozen side is
   `not (feature = "old_paths")`.
   ---------------------------------------------------------------------------------------------- */
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
const PATH_LOADSEARCH: *const c_char =
  c"/usr/local/share/emacs/32.0.50/lisp".as_ptr ();
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
const PATH_SITELOADSEARCH: *const c_char =
  c"/usr/local/share/emacs/32.0.50/site-lisp:/usr/local/share/emacs/site-lisp".as_ptr ();
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
const PATH_EXEC: *const c_char =
  c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0".as_ptr ();
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
const PATH_DATA: *const c_char = c"/usr/local/share/emacs/32.0.50/etc".as_ptr ();
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
const PATH_DOC: *const c_char = c"/usr/local/share/emacs/32.0.50/etc".as_ptr ();

/* C 63-64: `HDDEDATA CALLBACK DdeCallback (UINT, UINT, HCONV, HSZ, HSZ, HDDEDATA, DWORD_PTR,
   DWORD_PTR);` - C's forward declaration of the definition at C 66-72.  PORTING.md 0.1-12: Rust
   has no declaration that a later item satisfies, so the declaration is dropped and the definition
   stays at C's own line (Deviation 1).  */

// C 66-72: `DdeCallback` - C's `CALLBACK` is the `__stdcall` convention, which the port spells
// `extern "system"` (PORTING.md §5.1), and the definition is non-`static`, so it keeps its symbol.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DdeCallback (uType: UINT, uFmt: UINT, hconv: HCONV, hsz1: HSZ,
                                           hsz2: HSZ, hdata: HDDEDATA, dwData1: DWORD_PTR,
                                           dwData2: DWORD_PTR) -> HDDEDATA
{
  /* C 71 */
  return std::ptr::null_mut () as HDDEDATA;
}

/* C 74-76: `#define DdeCommand(str) DdeClientTransaction ((LPBYTE)str, strlen (str)+1,
   conversation, (HSZ)NULL, CF_TEXT, XTYP_EXECUTE, 30000, NULL)` - C's macro reaches
   `conversation` from the scope of its only caller, which a `macro_rules!` cannot do (macro
   hygiene: a macro body cannot name the caller's locals), so that local is an explicit second
   parameter and each call site passes it (PORTING.md 2.1/P-22's `marker.c` `CONSIDER` reading,
   where the same limitation moved the caller's locals into a helper's parameters; Deviation 4).
   The macro body's own tokens are all C's, `30000` included.  */
#[cfg (feature = "windowsnt")]
macro_rules! DdeCommand {
  ($str:expr, $conversation:expr) => {
    DdeClientTransaction ($str as LPBYTE, (strlen ($str) + 1) as DWORD, $conversation,
                          std::ptr::null_mut () as HSZ, CF_TEXT, XTYP_EXECUTE, 30000,
                          std::ptr::null_mut ())
  };
}

// C 78: `#define REG_ROOT` - the registry key the reference program's own settings live under;
// an external path, not a statement of this program's name, so C's bytes are carried (allowlisted).
#[cfg (feature = "windowsnt")]
pub const REG_ROOT: &std::ffi::CStr = c"SOFTWARE\\GNU\\Emacs";
// C 79-80: `#define REG_APP_PATH "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\emacs.exe"`
#[cfg (feature = "windowsnt")]
pub const REG_APP_PATH: &std::ffi::CStr
  = c"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\emacs.exe";

/* ----------------------------------------------------------------------------------------------
   C 82-112: `static struct entry { const char *name; const char *value; } env_vars[] = { … };`
   Both `#ifdef OLD_PATHS` arms are carried (P-86 ①).  The array is C's mutable file-scope object
   with internal linkage, so it is a non-`pub` `static mut` (PORTING.md 0.2) and its two
   definitions sit under complementary gates; the loop in `add_registry` counts its elements with
   C's own `sizeof` fold, which needs the length written out.
   ---------------------------------------------------------------------------------------------- */

// C 82-87: the record type is C's `struct entry`, declared inside the object's declaration.
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct entry
{
  pub name: *const c_char,
  pub value: *const c_char,
}

// C 87-99: `#ifdef OLD_PATHS` arm.
#[cfg (all (feature = "windowsnt", feature = "old_paths"))]
static mut env_vars: [entry; 7] =
[
  // C 90-99
  entry { name: c"emacs_dir".as_ptr (), value: std::ptr::null () },
  entry { name: c"EMACSLOADPATH".as_ptr (),
    value: c"%emacs_dir%/site-lisp;%emacs_dir%/../site-lisp;%emacs_dir%/lisp".as_ptr () },
  entry { name: c"SHELL".as_ptr (), value: c"%emacs_dir%/bin/cmdproxy.exe".as_ptr () },
  entry { name: c"EMACSDATA".as_ptr (), value: c"%emacs_dir%/etc".as_ptr () },
  entry { name: c"EMACSPATH".as_ptr (), value: c"%emacs_dir%/bin".as_ptr () },
  /* C 95-97: C's own note that INFOPATH is no longer set because Info-default-directory-list is
     then ignored, and the commented-out `{"INFOPATH", "%emacs_dir%/info"},` entry.  */
  entry { name: c"EMACSDOC".as_ptr (), value: c"%emacs_dir%/etc".as_ptr () },
  entry { name: c"TERM".as_ptr (), value: c"cmd".as_ptr () },
];

// C 100-111: `#else  /* !OLD_PATHS */` arm - the frozen one.  C's `PATH_SITELOADSEARCH ";"
// PATH_LOADSEARCH` is an adjacent-literal fold of the two `<epaths.h>` values declared above; Rust
// has no such join, so the entry spells the folded value out and names the two pieces (Deviation 5).
#[cfg (all (feature = "windowsnt", not (feature = "old_paths")))]
static mut env_vars: [entry; 7] =
[
  // C 101-110
  entry { name: c"emacs_dir".as_ptr (), value: std::ptr::null () },
  entry { name: c"EMACSLOADPATH".as_ptr (),
    value: c"/usr/local/share/emacs/32.0.50/site-lisp:/usr/local/share/emacs/site-lisp;/usr/local/share/emacs/32.0.50/lisp".as_ptr () },
  entry { name: c"SHELL".as_ptr (), value: c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0/cmdproxy.exe".as_ptr () },
  entry { name: c"EMACSDATA".as_ptr (), value: c"/usr/local/share/emacs/32.0.50/etc".as_ptr () },
  entry { name: c"EMACSPATH".as_ptr (), value: c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0".as_ptr () },
  /* C 106-108: the same INFOPATH note and commented-out entry as C 95-97.  */
  entry { name: c"EMACSDOC".as_ptr (), value: c"/usr/local/share/emacs/32.0.50/etc".as_ptr () },
  entry { name: c"TERM".as_ptr (), value: c"cmd".as_ptr () },
];

/* ----------------------------------------------------------------------------------------------
   C 114-170: `static void add_registry (const char *path)`
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "windowsnt")]
unsafe fn add_registry (path: *const c_char)
{
  let mut hrootkey: HKEY = std::ptr::null_mut ();          /* C 117 */
  let mut i: c_int;                                        /* C 118 */

  /* C 120-128: Record the location of Emacs to the App Paths key if we have
     sufficient permissions to do so.  This helps Windows find emacs quickly
     if the user types emacs.exe in the "Run Program" dialog without having
     emacs on their path.  Some applications also use the same registry key
     to discover the installation directory for programs they are looking for.
     Multiple installations cannot be handled by this method, but it does not
     affect the general operation of other installations of Emacs, and we
     are blindly overwriting the Start Menu entries already.  */

  // C 129-131
  if RegCreateKeyEx (HKEY_LOCAL_MACHINE, REG_APP_PATH.as_ptr (), 0, std::ptr::null_mut (),
                     REG_OPTION_NON_VOLATILE, KEY_WRITE, std::ptr::null_mut (),
                     &raw mut hrootkey, std::ptr::null_mut ()) == ERROR_SUCCESS
    {
      let mut len: c_int;                                  /* C 133 */
      let mut emacs_path: *mut c_char;                     /* C 134 */

      /* C 136: `len = strlen (path) + 15;` with C's own trailing note that the 15 is
         `\bin\emacs.exe` plus its terminator.  */
      len = strlen (path) as c_int + 15;
      /* C 137: `emacs_path = (char *) alloca (len);` - P-05's stack pool.  */
      emacs_path = sa_stack_alloc (len as usize) as *mut c_char;
      // C 138
      sprintf (emacs_path, c"%s\\bin\\emacs.exe".as_ptr (), path);

      // C 140-141
      RegSetValueEx (hrootkey, std::ptr::null (), 0, REG_EXPAND_SZ, emacs_path as LPBYTE,
                     len as DWORD);
      RegCloseKey (hrootkey);
    }

  /* C 144-147: Previous versions relied on registry settings, but we do not need
     them any more.  If registry settings are installed from a previous
     version, replace them to ensure they are the current settings.
     Otherwise, do nothing.  */

  /* C 149-150: Check both the current user and the local machine to see if we
     have any resources.  */

  // C 152-156
  if RegOpenKeyEx (HKEY_LOCAL_MACHINE, REG_ROOT.as_ptr (), 0, KEY_WRITE | KEY_QUERY_VALUE,
                   &raw mut hrootkey) != ERROR_SUCCESS
      && RegOpenKeyEx (HKEY_CURRENT_USER, REG_ROOT.as_ptr (), 0, KEY_WRITE | KEY_QUERY_VALUE,
                       &raw mut hrootkey) != ERROR_SUCCESS
    {
      return;
    }

  /* C 158: `for (i = 0; i < (sizeof (env_vars) / sizeof (env_vars[0])); i++)` - C's two-`sizeof`
     element count, written out because the array's length is part of its Rust type.  */
  i = 0;
  while i < (core::mem::size_of::<[entry; 7]> () / core::mem::size_of::<entry> ()) as c_int
    {
      let value: *const c_char;                            /* C 160 */

      // C 160: `const char * value = env_vars[i].value ? env_vars[i].value : path;`
      value = if ! env_vars[i as usize].value.is_null () { env_vars[i as usize].value } else { path };

      /* C 162: Replace only those settings that already exist.  */
      // C 163-166
      if RegQueryValueEx (hrootkey, env_vars[i as usize].name, std::ptr::null_mut (),
                          std::ptr::null_mut (), std::ptr::null_mut (),
                          std::ptr::null_mut ()) == ERROR_SUCCESS
        {
          RegSetValueEx (hrootkey, env_vars[i as usize].name, 0, REG_EXPAND_SZ,
                         value as LPBYTE, lstrlen (value) as DWORD + 1);
        }
      i += 1;
    }

  RegCloseKey (hrootkey);                                  /* C 169 */
}

/* ----------------------------------------------------------------------------------------------
   C 172-347: `int main (int argc, char *argv[])`
   ---------------------------------------------------------------------------------------------- */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut start_folder: [c_char; MAX_PATH as usize + 1] = [0; MAX_PATH as usize + 1]; /* C 175 */
  let mut shortcuts_created: c_int = 0;                    /* C 176 */
  let mut com_available: c_int = 1;                        /* C 177 */
  let mut modname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];               /* C 178 */
  let mut prog_name: *const c_char;                        /* C 179 */
  let mut emacs_path: *const c_char;                       /* C 180 */
  /* C 181's `char *p`: PORTING.md 1.5 - C assigns it before its first read on every path (C 202,
     210), so the port zero-initialises the binding rather than leaving it unset (Deviation 6).  */
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut quiet: c_int = 0;                                /* C 182 */
  let mut result: HRESULT;                                 /* C 183 */
  /* C 184's `IShellLinkA *shortcut` is written by `CoCreateInstance` at C 245-247; PORTING.md 1.5:
     C leaves it uninitialised and reads it only where `com_available` says the call ran, so the port
     zero-initialises it.  The same holds for C 181's `char *p`, C 262/293's `IPersistFile *lnk` and
     C 324-325's `HCONV conversation`/`HSZ progman` - one `## Deviations` entry, Deviation 6.
     C 193-194's `--argc; ++argv;` need `mut` bindings for the parameters, which Rust gives through
     the two shadowing `let`s below (Deviation 7).  */
  let mut shortcut: *mut IShellLinkA = std::ptr::null_mut ();
  let mut argc = argc;
  let mut argv = argv;
  /* C 271/C 302's `wchar_t unicode_path[MAX_PATH]`: the SDK's 16-bit code unit, declared where C
     first names it (`nt/runemacs.rs` 524-527 declares the same type the same way; no mirror
     carries `<wchar.h>`).  */
  type wchar_t = u16;

  /* C 186: If no args specified, use our location to set emacs_path.  */

  // C 188-190
  if argc > 1
      && (*(*argv.offset (1)).offset (0) == b'/' as c_char
          || *(*argv.offset (1)).offset (0) == b'-' as c_char)
      && *(*argv.offset (1)).offset (1) == b'q' as c_char
    {
      // C 192-194
      quiet = 1;
      argc -= 1;
      argv = argv.offset (1);
    }

  // C 197-198
  if argc > 1
    {
      emacs_path = *argv.offset (1) as *const c_char;
    }
  else
    {
      /* C 201-207: `GetModuleFileName`'s count is 0 on failure, so C's `!value` is `== 0`; the
         second operand is C's `(p = strrchr (…)) == NULL`, kept inside the condition as a block
         expression so the short-circuit order is C's (the assignment runs only when the first test
         failed - Deviation 11).  */
      if GetModuleFileName (std::ptr::null_mut (), modname.as_mut_ptr (), MAX_PATH as DWORD) == 0
          || { p = strrchr (modname.as_ptr (), b'\\' as c_int); p.is_null () }
        {
          // C 204-205
          fprintf (stderr, c"fatal error".as_ptr ());
          exit (1);
        }
      *p = 0;                                                /* C 207 */

      /* C 209: Set emacs_path to emacs_dir if we are in "%emacs_dir%\bin".  */
      /* C 210-219: C's `(p = strrchr (…)) && stricmp (…) == 0` again - the assignment stays in the
         condition (Deviation 11).  */
      if {
           p = strrchr (modname.as_ptr (), b'\\' as c_int);
           ! p.is_null () && stricmp (p, c"\\bin".as_ptr ()) == 0
         }
        {
          *p = 0;
          emacs_path = modname.as_ptr ();
        }
      else
        {
          fprintf (stderr, c"usage: addpm emacs_path\n".as_ptr ());
          exit (1);
        }

      /* C 221: Tell user what we are going to do.  */
      // C 222-236
      if quiet == 0
        {
          let result: c_int;                                 /* C 224 */
          /* C 226-227: `const char install_msg[] = "Install Emacs at %s?\n";` and
             `char msg[ MAX_PATH + sizeof (install_msg) ];` - the port keeps C's bytes without the
             terminator and adds one back where C's `sizeof` counts it (PORTING.md 2.1).  */
          const install_msg: &[u8] = b"Install Emacs at %s?\n";
          let mut msg: [c_char; MAX_PATH as usize + install_msg.len () + 1]
            = [0; MAX_PATH as usize + install_msg.len () + 1];
          sprintf (msg.as_mut_ptr (), install_msg.as_ptr () as *const c_char, emacs_path); /* C 228 */
          result = MessageBox (std::ptr::null_mut (), msg.as_ptr (), c"Install Emacs".as_ptr (),
                               MB_OKCANCEL | MB_ICONQUESTION);                          /* C 229-230 */
          if result != IDOK                                                             /* C 231 */
            {
              fprintf (stderr, c"Install canceled\n".as_ptr ());                        /* C 233 */
              exit (1);                                                                 /* C 234 */
            }
        }
    }

  add_registry (emacs_path);                               /* C 239 */
  prog_name = c"runemacs.exe".as_ptr ();                   /* C 240 */

  /* C 242: Try to install globally.  */

  // C 244-250
  if ! SUCCEEDED (CoInitialize (std::ptr::null_mut ()))
      || ! SUCCEEDED (CoCreateInstance (&raw const CLSID_ShellLink, std::ptr::null_mut (),
                                        CLSCTX_INPROC_SERVER, &raw const IID_IShellLinkA,
                                        &raw mut shortcut as *mut LPVOID))
    {
      com_available = 0;
    }

  // C 252-253
  if com_available != 0
      && SHGetSpecialFolderPath (std::ptr::null_mut (), start_folder.as_mut_ptr (),
                                 CSIDL_COMMON_PROGRAMS, 0) != 0
    {
      // C 255
      if strlen (start_folder.as_ptr ()) < (MAX_PATH - 20) as usize
        {
          strcat (start_folder.as_mut_ptr (), c"\\Gnu Emacs".as_ptr ());   /* C 257 */
          // C 258-259
          if CreateDirectory (start_folder.as_ptr (), std::ptr::null_mut ()) != 0
              || GetLastError () == ERROR_ALREADY_EXISTS
            {
              let mut full_emacs_path: [c_char; MAX_PATH as usize + 1 + 5 + 12]
                = [0; MAX_PATH as usize + 1 + 5 + 12];                     /* C 261 */
              /* C 262's `IPersistFile *lnk` is written by the `QueryInterface` call at C 267-268
                 and read only after it succeeded; zero-initialised for the same reason as
                 `shortcut` (Deviation 6).  */
              let mut lnk: *mut IPersistFile = std::ptr::null_mut ();
              strcat (start_folder.as_mut_ptr (), c"\\Emacs.lnk".as_ptr ());               /* C 263 */
              sprintf (full_emacs_path.as_mut_ptr (), c"%s\\bin\\%s".as_ptr (), emacs_path,
                       prog_name);                                                          /* C 264 */
              IShellLinkA_SetPath! (shortcut, full_emacs_path.as_ptr ());                   /* C 265 */
              /* C 266: C hands the shell the *reference* program's name as the shortcut's
                 description; that is this program's statement of its own name, so the port passes
                 its own (PORTING.md 0.1-19's seventh half, status 3.503).  The reference's own
                 literal stays in `nt/addpm.c` 266 - and in the same line of the second block at
                 C 297.  */
              IShellLinkA_SetDescription! (shortcut, c"femacs".as_ptr ());                  /* C 266 */
              result = IShellLinkA_QueryInterface! (shortcut, &raw const IID_IPersistFile,
                                                    &raw mut lnk as *mut LPVOID);           /* C 267-268 */
              if SUCCEEDED (result)                                                         /* C 269 */
                {
                  let mut unicode_path: [wchar_t; MAX_PATH as usize]
                    = [0; MAX_PATH as usize];                                               /* C 271 */
                  // C 272-273
                  MultiByteToWideChar (CP_ACP, 0, start_folder.as_ptr (), -1,
                                       unicode_path.as_mut_ptr (), MAX_PATH);
                  // C 274-275
                  if SUCCEEDED (IPersistFile_Save! (lnk, unicode_path.as_ptr (), TRUE))
                    {
                      shortcuts_created = 1;
                    }
                  IPersistFile_Release! (lnk);                                              /* C 276 */
                }
            }
        }
    }

  // C 282-283
  if shortcuts_created == 0 && com_available != 0
      && SHGetSpecialFolderPath (std::ptr::null_mut (), start_folder.as_mut_ptr (),
                                 CSIDL_PROGRAMS, 0) != 0
    {
      /* C 285: C's own note that there must be enough room for the Start Menu
         folder name C 257/288 append and the shortcut file name C 294 adds;
         both are external names, registered in the allowlist.  */
      // C 286
      if strlen (start_folder.as_ptr ()) < (MAX_PATH - 20) as usize
        {
          strcat (start_folder.as_mut_ptr (), c"\\Gnu Emacs".as_ptr ());   /* C 288 */
          // C 289-290
          if CreateDirectory (start_folder.as_ptr (), std::ptr::null_mut ()) != 0
              || GetLastError () == ERROR_ALREADY_EXISTS
            {
              let mut full_emacs_path: [c_char; MAX_PATH as usize + 1 + 5 + 12]
                = [0; MAX_PATH as usize + 1 + 5 + 12];                     /* C 292 */
              let mut lnk: *mut IPersistFile = std::ptr::null_mut ();      /* C 293 */
              strcat (start_folder.as_mut_ptr (), c"\\Emacs.lnk".as_ptr ());               /* C 294 */
              sprintf (full_emacs_path.as_mut_ptr (), c"%s\\bin\\%s".as_ptr (), emacs_path,
                       prog_name);                                                          /* C 295 */
              IShellLinkA_SetPath! (shortcut, full_emacs_path.as_ptr ());                   /* C 296 */
              IShellLinkA_SetDescription! (shortcut, c"femacs".as_ptr ());                  /* C 297 */
              result = IShellLinkA_QueryInterface! (shortcut, &raw const IID_IPersistFile,
                                                    &raw mut lnk as *mut LPVOID);           /* C 298-299 */
              if SUCCEEDED (result)                                                         /* C 300 */
                {
                  let mut unicode_path: [wchar_t; MAX_PATH as usize]
                    = [0; MAX_PATH as usize];                                               /* C 302 */
                  // C 303-304
                  MultiByteToWideChar (CP_ACP, 0, start_folder.as_ptr (), -1,
                                       unicode_path.as_mut_ptr (), MAX_PATH);
                  // C 305-306
                  if SUCCEEDED (IPersistFile_Save! (lnk, unicode_path.as_ptr (), TRUE))
                    {
                      shortcuts_created = 1;
                    }
                  IPersistFile_Release! (lnk);                                              /* C 307 */
                }
            }
        }
    }

  if com_available != 0                                   /* C 314 */
    {
      IShellLinkA_Release! (shortcut);                     /* C 315 */
    }

  /* C 317: Need to call uninitialize, even if ComInitialize failed.  */
  CoUninitialize ();                                       /* C 318 */

  /* C 320: Fallback on old DDE method if the above failed.  */
  // C 321-344
  if shortcuts_created == 0
    {
      let mut dde: DWORD = 0;                              /* C 323 */
      /* C 324-325's `HCONV conversation` and `HSZ progman` are assigned before their first read
         (C 329-330); zero-initialised for the same reason as `shortcut` (Deviation 6).  */
      let mut conversation: HCONV = std::ptr::null_mut ();
      let mut progman: HSZ = std::ptr::null_mut ();
      let mut add_item: [c_char; MAX_PATH as usize * 2 + 100]
        = [0; MAX_PATH as usize * 2 + 100];                /* C 326 */

      /* C 328: `DdeInitialize (&dde, (PFNCALLBACK) DdeCallback, APPCMD_CLIENTONLY, 0);` - C's
         cast to the callback typedef is the local's type here, because Rust has no cast into a
         function-pointer type (Deviation 8).  */
      let callback: PFNCALLBACK = Some (DdeCallback);
      DdeInitialize (&raw mut dde, callback, APPCMD_CLIENTONLY, 0);
      progman = DdeCreateStringHandle (dde, c"PROGMAN".as_ptr (), CP_WINANSI);            /* C 329 */
      conversation = DdeConnect (dde, progman, progman, std::ptr::null_mut ());           /* C 330 */
      if ! conversation.is_null ()                                                        /* C 331 */
        {
          DdeCommand! (c"[CreateGroup (\"Gnu Emacs\")]".as_ptr (), conversation);         /* C 333 */
          DdeCommand! (c"[ReplaceItem (Emacs)]".as_ptr (), conversation);                  /* C 334 */
          // C 335-336
          sprintf (add_item.as_mut_ptr (), c"[AddItem (\"%s\\bin\\%s\", Emacs)]".as_ptr (),
                   emacs_path, prog_name);
          DdeCommand! (add_item.as_ptr (), conversation);                                  /* C 337 */

          DdeDisconnect (conversation);                                                    /* C 339 */
        }

      DdeFreeStringHandle (dde, progman);                                                  /* C 342 */
      DdeUninitialize (dde);                                                               /* C 343 */
    }

  return 0;                                                /* C 346 */
}

