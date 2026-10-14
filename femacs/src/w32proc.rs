// femacs: port of src/w32proc.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  `w32proc.o`
// sits in `W32_OBJ` (`emacs/src/Makefile.in` 482, `W32_OBJ=@W32_OBJ@`, filled by the `else` arm of
// `configure.ac` 3051's `HAVE_W32` Cygwin/non-Cygwin split - i.e. the MinGW build, whose
// `nt/inc/ms-w32.h` is what `#define`s `WINDOWSNT`), and `docs/baseline/frozen-build-sources.txt`
// lists no `w32proc.o` for this target - so `extract-abi.py`'s checklist for the file is empty
// (P-86 ②) and the ledger `docs/ports/w32proc.md` carries that clause's substitute: the **C
// line-range table**, the `## Evidence` readings and the `## Deviations`.  This batch is P-86 ④'s
// first slice: **C 1-912**, ending at the `}` of `alarm` (C 896-912); the next item is the C
// 914-962 banner and **the next live definition is `wait_objects_thread` (C 1052)**.
//
//   * the **file gate** is `feature = "windowsnt"`, on every item *and on the include graph*,
//     which is why the frozen configuration compiles nothing here.  It is `WINDOWSNT`, not
//     `HAVE_NTGUI`: `w32fns.o`/`w32term.o` are in `W32_OBJ` on *both* arms of the split
//     (configure.ac 3030-3031) while `w32proc.o` is added only at 3051, the non-Cygwin one;
//   * the slice's own `#if`s, each carried with both arms: C 52 `HAVE_LANGINFO_CODESET` (1 in
//     `docs/baseline/config/darwin-arm64.macros.tsv`) -> `feature = "have_langinfo_codeset"`;
//     C 120 `__MINGW64__` (not in the table) -> `feature = "mingw64"`; C 136 `#if 1` (true in
//     every configuration, and it has no `#else` arm); C 899 `HAVE_SETITIMER` (1 here) with the
//     C 909 `#else` arm -> `feature = "have_setitimer"`.  `tools/check-cond-eval.py
//     src/w32proc.c` reads `26 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and the four
//     of them in this slice are listed in the ledger's `## Gate mapping`;
//   * the `#undef signal … #undef kill` block (C 44-48) and C 318's `#undef raise` cancel the
//     renames `nt/inc/ms-w32.h` installs; the ms-w32.h mirror spells those renames as
//     `macro_rules!`, which fire on the `name! (…)` form only, so the port's plain call
//     `signal (…)` (C 187) reaches the CRT entry C's `#undef` selects (the ledger records it);
//   * the `<windows.h>` section below stands where C 50's `#include <windows.h>` is, and the
//     MinGW-runtime section where C 36's `#include <signal.h>` is: neither header has a file in
//     the reference tree (P-88 ①), so the names this slice takes from them are declared here, and
//     **no `<windows.h>` and no mingw header exist on this machine** (see the acquisition note) -
//     every record's size is `UNVERIFIED` and the layout assertions are owed.
//
// **Acquisition note - this batch is `text-only` (P-86 ③⑵).**  The machine has no Windows and no
// mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
// Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h` anywhere), so there is no
// `<windows.h>` to compile or measure against, **no arm flip on a Windows target and no layout
// measurement**: the ledger's readings are the frozen `build-tu`, the machine-checked coverage
// table, the `#if` verdicts, the arm-reach pre-flight over the unit's own text, and the `probe`
// reading of the `windowsnt` arm (`rustc` on this machine, whose errors are the mirror graph's
// gaps) - which are not a Windows-target compile.
//
// `docs/ports/w32proc.md` carries the C line ranges, the acceptance readings, every deviation (the
// two stand-in sections, the dropped forward declarations, the `RVA_TO_PTR` macro, the four
// names this slice defines that a mirror also declares, the Windows `<sys/time.h>` shim) and the
// items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

/* C 20-23: the file's attribution comment, carried as C's own text at its C position.

/*
   Drew Bliss                   Oct 14, 1993
     Adapted from alarm.c by Tim Fleehart
*/
*/

include!("include/config.rs");                  // C 26: `#include <config.h>`
include!("include/conf_post.rs");               // C 26's AH_BOTTOM `#include <conf_post.h>` (DebPrint, C 459)

/* C 28-64: the include graph.  Every header sits inside the file gate, as C's own build has it (no
   translation unit the frozen configuration builds reaches this file; P-86 ⑧).  The headers the
   tree has no file for (`<io.h>`, `<mbstring.h>`, `<locale.h>`, `<windows.h>`) are recorded at
   their C lines with a `// C …` line (P-88 ①): the names this slice takes from them stand in the
   two sections below.  Where the reference tree carries *two* headers of one name, the one
   `nt/inc` overrides is the one included - that is what `<unistd.h>`, `<signal.h>`, `<sys/file.h>`
   and `<mingw_time.h>` resolve to in the MinGW build, and the `nt/inc/include/` mirrors' own notes
   name `src/w32proc.c` as one of the units that reaches them.  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/mingw_time.rs");     // C 28: `#include <mingw_time.h>`
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdio.rs");             // C 29: `#include <stdio.h>` (`fprintf`, `stderr`)
#[cfg (feature = "windowsnt")]
include!("../lib/include/stdlib.rs");            // C 30: `#include <stdlib.h>`

/// C 30's `<stdlib.h>`: `exit`, the entry the three `*_before_init` functions call (C 96, 104, 112).
/// The port's `<stdlib.h>` mirror does not carry it (`femacs/lib/include/stdlib.rs`), so this unit
/// declares it with the header's own text, the way `src/emacs.rs` 320-321 does; the ledger's owed
/// list names the mirror row.
/// C: <stdlib.h>: `_Noreturn void exit (int);`
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}
#[cfg (feature = "windowsnt")]
include!("../lib/include/errno.rs");             // C 31: `#include <errno.h>` (`EINVAL`, `EAGAIN`)

/// C 31's `<errno.h>`: the two rows this slice sets that the port's `<errno.h>` mirror does not
/// carry yet (`femacs/lib/include/errno.rs` has EINVAL, EAGAIN and the rest of this file's set).
/// The values are the frozen configuration's own - `docs/baseline/config/darwin-arm64.macros.tsv`
/// 841 `EFAULT 14` and 936 `ESRCH 3` - and the runtime's `<errno.h>` uses the same two on a
/// Windows target.  The ledger's owed list names the two mirror rows.
/// C: #define ESRCH 3
/// C: #define EFAULT 14
#[cfg (feature = "windowsnt")]
pub const ESRCH: c_int = 3;
#[cfg (feature = "windowsnt")]
pub const EFAULT: c_int = 14;
#[cfg (feature = "windowsnt")]
include!("../lib/include/ctype.rs");             // C 32: `#include <ctype.h>`
// C 33: `#include <io.h>` - the runtime's own header; this slice names nothing from it.
#[cfg (feature = "windowsnt")]
include!("../lib/include/fcntl.in.rs");          // C 34: `#include <fcntl.h>`
/// C 35's `<sys/types.h>` (through `nt/inc/unistd.h` 6): `ssize_t`, the type the nt/inc
/// `<unistd.h>` mirror's own note leaves to the including unit (`femacs/lib/include/
/// sys_types.in.rs` does not carry it, and pulling `lib/include/unistd.in.rs` in would clash with
/// that mirror's `AT_*`/`O_*` names).  Spelled the way the port's `<unistd.h>` mirror spells it
/// (`femacs/lib/include/unistd.in.rs` 34); the MinGW runtime's own width is UNVERIFIED.
/// C: typedef long ssize_t;
#[cfg (feature = "windowsnt")]
pub type ssize_t = c_long;
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/unistd.rs");         // C 35: `#include <unistd.h>` (`getpgrp`…)
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/signal.rs");         // C 36: `#include <signal.h>`

/* ----------------------------------------------------------------------------------------------
   The MinGW runtime's own `<signal.h>` names.

   C 36's `#include <signal.h>` resolves, in the MinGW build, to `nt/inc/signal.h` and then
   (`#include_next`) to the runtime's own header; the tree has a file for the first half only
   (`femacs/nt/inc/include/signal.rs`, which adds `SIG2STR_MAX` and `sig2str`), and that mirror's
   own note says the `#include_next` target is MinGW's header, **not** the frozen target's - which
   is why `lib/include/signal.rs` (darwin's `<signal.h>`) is deliberately not pulled in here.
   P-88 ①: the names this slice's text needs are declared at C's own `#include` line, with the
   header's text above each one.  **No measurement was possible** - there is no mingw header on this
   machine - so every value below is **UNVERIFIED** and the ledger's owed list names the toolchain.
   ---------------------------------------------------------------------------------------------- */

/// C 36's `<signal.h>`: the signal set `sigemptyset`/`sigfillset`/`sigaddset` write.  C spells it
/// as a typedef of the runtime's `_sigset_t`, `nt/inc/ms-w32.h` 345-349's
/// `typedef _sigset_t sigset_t;` (under `MINGW_W64 && !_POSIX`), and that name is a MinGW runtime
/// type no mirror owns (`docs/ports/include-ms-w32.md`'s "Names with no mirror", which is why
/// `femacs/nt/inc/include/ms-w32.rs` leaves it to the including unit).  C's own operations on it -
/// `*set = 0` (C 223), `*set |= (1U << signo)` (C 241), `*set = 0xFFFFFFFF` (C 255), `*set & (1U
/// << signo)` (C 311) - require a 32-bit unsigned scalar, and `sigismember`'s
/// `sizeof (*set) * CHAR_BIT` (C 308) reads 32 against `NSIG`'s 23; the *width* is nevertheless
/// UNVERIFIED, because only a mingw toolchain can measure it.
/// C: typedef _sigset_t sigset_t;   [nt/inc/ms-w32.h 345-349]
/// The type is the include graph's: `femacs/lib/include/signal.rs`'s `sigset_t`, whose
/// `windowsnt` arm is this 32-bit scalar and whose frozen arm is Darwin's record (P-87 ③).

/// C 36's `<signal.h>`: the seven `SIG*` numbers this slice tests.  `SIGALRM`, `SIGCHLD` and
/// `SIGPROF` are `nt/inc/ms-w32.h`'s own (C 328-330, carried by that mirror as 14/18/19); the rest
/// are the runtime header's, and C's `lib/include/signal.rs` (the frozen target's `<signal.h>`)
/// records the same seven values, which is the cross-check this port has - the runtime header
/// itself is UNVERIFIED here.
/// C: #define SIGINT 2      #define SIGILL 4      #define SIGABRT 6     #define SIGFPE 8
/// C: #define SIGSEGV 11    #define SIGTERM 15    #define SIGBREAK 21
///
/// Only `SIGBREAK` is declared here: the other six are in the include graph already
/// (`femacs/lib/include/signal.rs`, the same seven values recorded from this target's runtime
/// header), and a second declaration of one name in one translation unit is `E0428`
/// (PORTING.md P-87 ③).
#[cfg (feature = "windowsnt")]
pub const SIGBREAK: c_int = 21;

/// C 36's `<signal.h>`: the three handler *words* this file compares a recorded handler against.
/// `SIG_DFL` is the zero handler, which the port's `signal_handler` (`Option<fn>`, P-27) spells
/// `None` (the `process.rs` 8823-8825 reading); `SIG_IGN` and `SIG_ERR` are the words 1 and -1,
/// and such an address is not a valid `Option<fn>` *constant* (measured: `transmute::<usize,
/// Option<fn>> (1)` in a `const` is `E0080 constructing invalid value`), so they stay the integers
/// the port's own `<signal.h>` mirror keeps them as for the frozen target
/// (`femacs/lib/include/signal.rs` 44-50) and each comparison transmutes, as `process.rs` 8671 and
/// `sysdep.rs` 1710 do.
/// C: #define SIG_DFL ((void (_CALLBACK_ *) (int)) 0)
#[cfg (feature = "windowsnt")]
pub const SIG_DFL: signal_handler = None;
/// `SIG_IGN` (the word 1) is in the include graph already (`femacs/lib/include/signal.rs` 50);
/// only `SIG_IGN`'s sibling `SIG_ERR` is this file's own.
/// C: #define SIG_ERR ((void (_CALLBACK_ *) (int)) -1)
#[cfg (feature = "windowsnt")]
pub const SIG_ERR: isize = -1;

/// C 36's `<signal.h>`: the CRT's `signal` entry.  C's `#undef signal` (C 44) cancels the
/// `#define signal sys_signal` that `nt/inc/ms-w32.h` 225 installs, so C 187's call is the CRT's;
/// the mirror spells that rename as `macro_rules! signal` (call-form only), which the port's plain
/// call cannot reach, so the effect C's `#undef` has is what the port gets for free.  The handler
/// type is the ms-w32.h mirror's `signal_handler` (`Option<fn>`, P-27).
/// C: void (*signal (int, void (*) (int))) (int);
/// The declaration is the include graph's - `femacs/lib/include/signal.rs`'s `signal`, whose
/// `windowsnt` arm carries the runtime header's function-pointer shape (P-87 ③).

/// C 36's `<signal.h>` (through `<process.h>`): the CRT's `raise`.  `nt/inc/ms-w32.h` 379+
/// redirects `raise` to `w32_raise` for the rest of the tree, but this file `#undef`s it (C 318)
/// and calls the CRT entry from its own emulation; the redirect is a macro in C, not in the port,
/// and `include/syssignal.rs` 81 keeps `emacs_raise`.
/// C: int raise (int);
/// The declaration is the include graph's (`femacs/lib/include/signal.rs`'s `raise`).

// C 37: `#include <sys/file.h>` - the `nt/inc` override (`O_RDONLY`-shaped names this slice does
// not use is recorded in that mirror's ledger).
#[cfg (feature = "windowsnt")]
include!("../nt/inc/sys/include/file.rs");
// C 38: `#include <mbstring.h>` - the runtime's own header (`_mbspbrk`, `_mbslwr`, `_mbsrchr`);
// this slice names nothing from it.
// C 39: `#include <locale.h>` - the runtime's own header (the `LC_*` names and `setlocale`);
// this slice names nothing from it.

/* C 41-42: the "Include CRT headers *before* ms-w32.h" note and the include itself.  */
#[cfg (feature = "windowsnt")]
include!("../nt/inc/include/ms-w32.rs");         // C 42: `#include <ms-w32.h>`

/* C 44-48: the `#undef signal` … `#undef kill` block.  It cancels the `#define signal sys_signal`
   family `nt/inc/ms-w32.h` installs above, so that this file's own definitions and calls are not
   rewritten.  Rust has no object-like rename to cancel and the mirror spells those renames as
   `macro_rules!` (bound at the `name! (…)` call form, not at a definition or a plain call), so the
   block is carried as C's own text (the `cm.rs` 86 reading).

#undef signal
#undef wait
#undef spawnve
#undef select
#undef kill
*/

// C 50: `#include <windows.h>` - the header the section below stands for.

/* ----------------------------------------------------------------------------------------------
   `<windows.h>`: the SDK surface this slice names.

   C 50's `#include <windows.h>` has no file in the reference tree (P-88 ①: the declaration site is
   C's own `#include` line, and no file is added under `femacs/src/include/`).  What follows is the
   names *this slice* uses, each with the SDK's own text above it; the spellings repeat
   `femacs/src/w32.rs`'s section so that a `windowsnt` translation unit sees one set of names.

   **No measurement was possible**: this machine has no `<windows.h>` and no mingw toolchain (see
   the acquisition note), so every scalar is spelled from PORTING.md 1.1's fixed widths (`DWORD`,
   `LONG`, `ULONG` and `UINT` are the 32-bit Windows types whatever this target's `c_long` is,
   `HANDLE` and the SDK's handle records are `void *`), and every record below is **UNVERIFIED** -
   its layout assertions are owed (the ledger's owed list).
   ---------------------------------------------------------------------------------------------- */

/// C 50's `<windows.h>`: the scalar types and the handle typedefs this slice's declarations use.
/// C: typedef unsigned char BYTE; typedef unsigned short WORD; typedef unsigned short WCHAR;
/// C: typedef unsigned long DWORD; typedef long LONG; typedef unsigned int UINT;
/// C: typedef int BOOL; typedef unsigned long ULONG; typedef unsigned long long ULONGLONG;
/// C: typedef DWORD *LPDWORD; typedef UINT_PTR SIZE_T; typedef ULONG_PTR DWORD_PTR;
/// C: typedef void *HANDLE, *HMODULE, *HINSTANCE, *LPVOID, *LPSECURITY_ATTRIBUTES;
/// C: typedef const char *LPCSTR, *LPCTSTR; typedef char *LPSTR, *LPTSTR;
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
pub type UINT = c_uint;
#[cfg (feature = "windowsnt")]
pub type BOOL = c_int;
#[cfg (feature = "windowsnt")]
pub type ULONG = u32;
#[cfg (feature = "windowsnt")]
pub type ULONGLONG = u64;
#[cfg (feature = "windowsnt")]
pub type LPDWORD = *mut DWORD;
#[cfg (feature = "windowsnt")]
pub type SIZE_T = usize;
/* `DWORD_PTR` belongs to `include/w32common.rs` 42 (`w32common.h` 26-37 names it), which is in
   this unit's graph; the SDK section does not repeat it.  The only occurrences in this slice's
   text are inside C 66-69's macro and C 74-75's dropped prototype, both carried as text.  */
#[cfg (feature = "windowsnt")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HMODULE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type HINSTANCE = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPVOID = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPSECURITY_ATTRIBUTES = *mut c_void;
#[cfg (feature = "windowsnt")]
pub type LPCSTR = *const c_char;
#[cfg (feature = "windowsnt")]
pub type LPSTR = *mut c_char;
#[cfg (feature = "windowsnt")]
pub type LPHANDLE = *mut HANDLE;

/// C 50's `<windows.h>`: the `WINAPI`/`CALLBACK` convention, which Rust spells
/// `extern "system"`; the name appears in the `/// C:` lines of the declarations below.
/// C: #define WINAPI __stdcall
/// C: #define CALLBACK __stdcall
#[cfg (feature = "windowsnt")]
pub const WINAPI: bool = true;

/// C 50's `<windows.h>`: the two values `BOOL` carries, which C's text uses directly.
/// C: #define FALSE 0
/// C: #define TRUE 1
#[cfg (feature = "windowsnt")]
pub const FALSE: BOOL = 0;
#[cfg (feature = "windowsnt")]
pub const TRUE: BOOL = 1;

/// C 50's `<windows.h>`: the constants this slice tests or passes.  `STILL_ACTIVE` is the exit
/// code `GetExitCodeThread` reports for a running thread (259), `ERROR_INVALID_HANDLE` the
/// `GetLastError` value `stop_timer_thread` compares against (6), `DUPLICATE_SAME_ACCESS` the
/// `DuplicateHandle` option (2), `THREAD_PRIORITY_TIME_CRITICAL` the `SetThreadPriority` level
/// (15), `SEM_FAILCRITICALERRORS` the `SetErrorMode` flag (1) and `INVALID_HANDLE_VALUE` the
/// pseudo-handle `w32_get_timer_time` tests (-1).
/// C: #define STILL_ACTIVE 259
/// C: #define ERROR_INVALID_HANDLE 6
/// C: #define DUPLICATE_SAME_ACCESS 0x00000002
/// C: #define THREAD_PRIORITY_TIME_CRITICAL 15
/// C: #define SEM_FAILCRITICALERRORS 0x0001
/// C: #define INVALID_HANDLE_VALUE ((HANDLE) (LONG_PTR) -1)
#[cfg (feature = "windowsnt")]
pub const STILL_ACTIVE: DWORD = 259;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_HANDLE: DWORD = 6;
#[cfg (feature = "windowsnt")]
pub const DUPLICATE_SAME_ACCESS: DWORD = 0x00000002;
#[cfg (feature = "windowsnt")]
pub const THREAD_PRIORITY_TIME_CRITICAL: c_int = 15;
#[cfg (feature = "windowsnt")]
pub const SEM_FAILCRITICALERRORS: UINT = 0x0001;
#[cfg (feature = "windowsnt")]
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

/// C 50's `<windows.h>`: `FILETIME`, the two 100-ns-unit records `w32_get_timer_time` reads.
/// C: typedef struct _FILETIME { DWORD dwLowDateTime; DWORD dwHighDateTime; } FILETIME, *PFILETIME,
/// C:   *LPFILETIME;
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILETIME
{
  pub dwLowDateTime: DWORD,
  pub dwHighDateTime: DWORD,
}
#[cfg (feature = "windowsnt")]
pub type LPFILETIME = *mut FILETIME;

/// C 50's `<windows.h>`: `ULARGE_INTEGER`, the union the timers assemble and divide.  Its
/// `LowPart`/`HighPart` views live in a *nameless* inner struct in the SDK; Rust has no nameless
/// struct, so the port names it `u` and C's `temp.LowPart` reads as `temp.u.LowPart` (the same
/// deviation `femacs/src/w32.rs` records for `LARGE_INTEGER`).  UNVERIFIED here.
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
  pub QuadPart: ULONGLONG,
}

/// C 50's `<windows.h>`: `CRITICAL_SECTION`, the lock record this file keeps three of (C 82, 410)
/// and hands to the four entry points below.  C only ever passes `&crit` or a `CRITICAL_SECTION *`,
/// but the three objects are *statics* here, so the record's size is what the port's `.bss` has to
/// carry: the fields are the SDK's, and every width is **UNVERIFIED** (no mingw toolchain).
/// C: typedef struct _RTL_CRITICAL_SECTION { PRTL_CRITICAL_SECTION_DEBUG DebugInfo; LONG LockCount;
/// C:   LONG RecursionCount; HANDLE OwningThread; HANDLE LockSemaphore; ULONG_PTR SpinCount;
/// C:   } CRITICAL_SECTION, *PCRITICAL_SECTION, *LPCRITICAL_SECTION;
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
  pub SpinCount: usize,
}
#[cfg (feature = "windowsnt")]
pub type LPCRITICAL_SECTION = *mut CRITICAL_SECTION;

/// C 50's `<windows.h>`: the console-control handler the file registers, and its callback shape.
/// C: typedef BOOL (CALLBACK *PHANDLER_ROUTINE) (DWORD CtrlType);
#[cfg (feature = "windowsnt")]
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn (CtrlType: DWORD) -> BOOL>;

/// C 50's `<windows.h>`: the thread entry point `CreateThread` starts, and the entry points this
/// slice calls.  The `…A`/`…W` alias pair of the SDK is kept as C's own spelling (this
/// configuration leaves `_UNICODE` undefined, and the alias resolves to the ANSI entry).
/// C: typedef DWORD (WINAPI *LPTHREAD_START_ROUTINE) (LPVOID lpThreadParameter);
/// C: WINBASEAPI DWORD WINAPI GetEnvironmentVariableA (LPCSTR lpName, LPSTR lpBuffer, DWORD nSize);
/// C: WINBASEAPI void WINAPI DebugBreak (void);
/// C: WINBASEAPI BOOL WINAPI SetConsoleCtrlHandler (PHANDLER_ROUTINE HandlerRoutine, BOOL Add);
/// C: WINBASEAPI UINT WINAPI SetErrorMode (UINT uMode);
/// C: WINBASEAPI HANDLE WINAPI GetCurrentProcess (void);
/// C: WINBASEAPI HANDLE WINAPI GetCurrentThread (void);
/// C: WINBASEAPI HANDLE WINAPI CreateThread (LPSECURITY_ATTRIBUTES, SIZE_T, LPTHREAD_START_ROUTINE,
/// C:   LPVOID, DWORD, LPDWORD);
/// C: WINBASEAPI BOOL WINAPI SetThreadPriority (HANDLE hThread, int nPriority);
/// C: WINBASEAPI BOOL WINAPI GetExitCodeThread (HANDLE hThread, LPDWORD lpExitCode);
/// C: WINBASEAPI BOOL WINAPI TerminateThread (HANDLE hThread, DWORD dwExitCode);
/// C: WINBASEAPI DWORD WINAPI SuspendThread (HANDLE hThread);
/// C: WINBASEAPI DWORD WINAPI ResumeThread (HANDLE hThread);
/// C: WINBASEAPI BOOL WINAPI CloseHandle (HANDLE hObject);
/// C: WINBASEAPI BOOL WINAPI DuplicateHandle (HANDLE, HANDLE, HANDLE, LPHANDLE, DWORD, BOOL, DWORD);
/// C: WINBASEAPI DWORD WINAPI GetLastError (void);
/// C: WINBASEAPI void WINAPI Sleep (DWORD dwMilliseconds);
/// C: WINBASEAPI void WINAPI GetSystemTimeAsFileTime (LPFILETIME lpSystemTimeAsFileTime);
/// C: WINBASEAPI void WINAPI InitializeCriticalSection (LPCRITICAL_SECTION lpCriticalSection);
/// C: WINBASEAPI void WINAPI EnterCriticalSection (LPCRITICAL_SECTION lpCriticalSection);
/// C: WINBASEAPI void WINAPI LeaveCriticalSection (LPCRITICAL_SECTION lpCriticalSection);
/// C: WINBASEAPI void WINAPI DeleteCriticalSection (LPCRITICAL_SECTION lpCriticalSection);
/// C: WINBASEAPI HMODULE WINAPI GetModuleHandleA (LPCSTR lpModuleName);
#[cfg (feature = "windowsnt")]
pub type LPTHREAD_START_ROUTINE = Option<unsafe extern "system" fn (lpThreadParameter: LPVOID) -> DWORD>;
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetEnvironmentVariable (lpName: LPCSTR, lpBuffer: LPSTR, nSize: DWORD) -> DWORD;
  pub fn DebugBreak ();
  pub fn SetConsoleCtrlHandler (HandlerRoutine: PHANDLER_ROUTINE, Add: BOOL) -> BOOL;
  pub fn SetErrorMode (uMode: UINT) -> UINT;
  pub fn GetCurrentProcess () -> HANDLE;
  pub fn GetCurrentThread () -> HANDLE;
  pub fn CreateThread (lpThreadAttributes: LPSECURITY_ATTRIBUTES, dwStackSize: SIZE_T,
                       lpStartAddress: LPTHREAD_START_ROUTINE, lpParameter: LPVOID,
                       dwCreationFlags: DWORD, lpThreadId: LPDWORD) -> HANDLE;
  pub fn SetThreadPriority (hThread: HANDLE, nPriority: c_int) -> BOOL;
  pub fn GetExitCodeThread (hThread: HANDLE, lpExitCode: LPDWORD) -> BOOL;
  pub fn TerminateThread (hThread: HANDLE, dwExitCode: DWORD) -> BOOL;
  pub fn SuspendThread (hThread: HANDLE) -> DWORD;
  pub fn ResumeThread (hThread: HANDLE) -> DWORD;
  pub fn CloseHandle (hObject: HANDLE) -> BOOL;
  pub fn DuplicateHandle (hSourceProcessHandle: HANDLE, hSourceHandle: HANDLE,
                          hTargetProcessHandle: HANDLE, lpTargetHandle: LPHANDLE,
                          dwDesiredAccess: DWORD, bInheritHandle: BOOL,
                          dwOptions: DWORD) -> BOOL;
  pub fn GetLastError () -> DWORD;
  pub fn Sleep (dwMilliseconds: DWORD);
  pub fn GetSystemTimeAsFileTime (lpSystemTimeAsFileTime: LPFILETIME);
  pub fn InitializeCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn EnterCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn LeaveCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn DeleteCriticalSection (lpCriticalSection: LPCRITICAL_SECTION);
  pub fn GetModuleHandle (lpModuleName: LPCSTR) -> HMODULE;
}

/* C 52-55: `#ifdef HAVE_LANGINFO_CODESET` around `<nl_types.h>` and `<langinfo.h>` - the two
   headers the C 4137-4234 `nl_langinfo` emulation needs.  The condition is *true* in the frozen
   table (macos has langinfo), so this is one of the four `#if`s of this slice that carry both
   arms; both headers have mirrors under `nt/inc/include/` because the MinGW build is the one that
   resolves them.  */
#[cfg (all (feature = "windowsnt", feature = "have_langinfo_codeset"))]
include!("../nt/inc/include/nl_types.rs");       // C 53: `#include <nl_types.h>`
#[cfg (all (feature = "windowsnt", feature = "have_langinfo_codeset"))]
include!("../nt/inc/include/langinfo.rs");       // C 54: `#include <langinfo.h>`

/* C 57-64: the project's own headers.  `"lisp.h"` carries `<string.h>` (its C 30) and
   `"lisp.h"`'s C 33 `<limits.h>` comes in through `stdint.in.rs` -> `limits.in.rs`, but
   `include/lisp.rs` pulls neither, and this file's `memset` (C 680-681) and `CHAR_BIT` (C 308)
   come from exactly those two lines: the two mirrors are included here, under the lisp.h line
   they belong to (the ledger records the deviation).  */
#[cfg (feature = "windowsnt")]
include!("include/lisp.rs");                     // C 57: `#include "lisp.h"`
#[cfg (feature = "windowsnt")]
include!("../lib/include/string.rs");            // C 57's lisp.h 30: `#include <string.h>` (`memset`)
#[cfg (feature = "windowsnt")]
include!("include/w32.rs");                      // C 58: `#include "w32.h"` (`term_ntproc`)
#[cfg (feature = "windowsnt")]
include!("include/w32common.rs");                // C 59: `#include "w32common.h"` (`os_subtype`,
                                                 //   `cache_system_info`, `get_proc_addr`)
#[cfg (feature = "windowsnt")]
include!("include/w32heap.rs");                  // C 60: `#include "w32heap.h"`
#[cfg (feature = "windowsnt")]
include!("include/syswait.rs");                  // C 61: `#include "syswait.h"`	/* for WNOHANG */
#[cfg (feature = "windowsnt")]
include!("include/syssignal.rs");                // C 62: `#include "syssignal.h"` (`empty_mask`)
#[cfg (feature = "windowsnt")]
include!("include/w32term.rs");                  // C 63: `#include "w32term.h"`
#[cfg (feature = "windowsnt")]
include!("include/coding.rs");                   // C 64: `#include "coding.h"`

/* C 66-69: `#define RVA_TO_PTR(var,section,filedata) …`, the file's own macro.  Its body is
   three field reads on `<windows.h>`'s `IMAGE_SECTION_HEADER` and on `struct file_data`, and the
   port has no mirror for the SDK record: `include/w32heap.rs` 63-76 declares `find_section` and
   `rva_to_section` with the record as `*mut c_void` (P-72 ③/P-55), so a Rust body cannot be
   written without inventing that record's layout.  The macro is carried as C's own text at its C
   position, and its two users (C 2635, 2644) are past this slice; the ledger records the
   deviation and the integrator owes either a measured record or an `excluded-macros.txt` row.

#define RVA_TO_PTR(var,section,filedata) \
  ((void *)((section)->PointerToRawData					\
	    + ((DWORD_PTR)(var) - (section)->VirtualAddress)		\
	    + (filedata).file_base))
*/

/* C 71-72: the two "the entry point has been resolved" flags.  `w32.c` owns the definitions, and
   this file's `w32_compare_strings` (C 4657+) and `sys_kill` (C 3530+) are their readers, so C
   declares them here (PORTING.md 0.1-3: a declaration-only mirror would be the header's, but this
   pair is declared by the `.c` itself).  */
/// C: extern BOOL g_b_init_compare_string_w;
/// C: extern BOOL g_b_init_debug_break_process;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut g_b_init_compare_string_w: BOOL;
  pub static mut g_b_init_debug_break_process: BOOL;
}

/* C 74-75: `int sys_select (int, SELECT_TYPE *, …);` - dropped (PORTING.md 0.1-12: C's forward
   declaration has no Rust counterpart, and the definition is C 3109, past this slice).  It is also
   the slice's only mention of `SELECT_TYPE`, which `include/w32.rs` 177 carries as a nullary
   `macro_rules!`.  */

/* C 77: "Signal handlers...SIG_DFL == 0 so this is initialized correctly." - C's three file-scope
   statics.  `NSIG` is `nt/inc/ms-w32.h` 333's 23, carried by that mirror.  */
#[cfg (feature = "windowsnt")]
static mut sig_handlers: [signal_handler; NSIG as usize] = [SIG_DFL; NSIG as usize];
#[cfg (feature = "windowsnt")]
static mut sig_mask: sigset_t = 0;
#[cfg (feature = "windowsnt")]
static mut crit_sig: CRITICAL_SECTION = unsafe { core::mem::zeroed () };

/* C 84-89: "Catch memory allocation before the heap allocation scheme is set up.  These functions
   should never be called, unless code is added early on in 'main' that runs before init_heap is
   called." - the three `_Noreturn` prototypes, dropped (PORTING.md 0.1-12: the definitions are
   C 91-113 below).  */

/* C 91-97: `malloc_before_init`.  C's `_Noreturn void *` keeps the declared return type - C 143
   assigns the three to `malloc_fn`/`realloc_fn`/`free_fn`, whose signatures are
   `nt/inc/include/ms-w32.rs` 149-154's - and the `exit` tail is `!`, so "never returns" is the
   body's own statement (the ledger records the `-> !` reading of `AVOID`).  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_before_init (size: size_t) -> *mut c_void
{
  fprintf (stderr, c"error: 'malloc' called before setting up heap allocation; exiting.\n".as_ptr ());
  exit (-1);
}

/* C 99-105: `realloc_before_init`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc_before_init (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  fprintf (stderr, c"error: 'realloc' called before setting up heap allocation; exiting.\n".as_ptr ());
  exit (-1);
}

/* C 107-113: `free_before_init`.  */
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_before_init (ptr: *mut c_void)
{
  fprintf (stderr, c"error: 'free' called before setting up heap allocation; exiting.\n".as_ptr ());
  exit (-1);
}

/* C 115: `extern BOOL ctrl_c_handler (unsigned long type);` - the console-control handler is
   defined in `w32console.c` (C 92), so this is a declaration.  */
/// C: extern BOOL ctrl_c_handler (unsigned long type);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn ctrl_c_handler (type_: c_ulong) -> BOOL;
}

/* C 117-122: "MinGW64 doesn't add a leading underscore to external symbols, whereas configure.ac
   sets up LD_SWITCH_SYSTEM_TEMACS to force the entry point at __start, with two underscores." -
   `#ifdef __MINGW64__` / `#define _start __start` / `#endif`.  C's macro renames the declaration
   C 130 *and* the definition C 132; Rust has no object-like rename, so the port keeps C's *source*
   name and carries the rename as the symbol the arm produces (the `regex-emacs.rs` 6148-6150
   `re_compile_pattern`/`rpl_re_compile_pattern` shape), the `#else` arm being C's own absent one -
   the name `_start` with no export attribute.  */

/// C 124: `extern void mainCRTStartup (void);` - the CRT's entry point, called at the end of
/// `_start`.
/// C: extern void mainCRTStartup (void);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn mainCRTStartup ();
}

/* C 126-129: "Startup code for running on NT.  When we are running as the dumped version, we need
   to bootstrap our heap and .bss section into our address space before we can actually hand off
   control to the startup code supplied by NT (primarily because that code relies upon malloc ())."
   C 130's `void _start (void);` is the forward declaration of C 132 (dropped, PORTING.md 0.1-12).  */
/// C 132-133: `void _start (void)`.
/// C: void _start (void);
#[cfg (feature = "windowsnt")]
#[cfg_attr (not (feature = "mingw64"), unsafe (no_mangle))]
#[cfg_attr (feature = "mingw64", unsafe (export_name = "__start"))]
pub unsafe extern "C" fn _start ()
{

  /* C 136-141: `#if 1` around the debug hook - the condition is true in every configuration and
     C has no `#else` arm, so the body is live text and the other arm is C's own empty one.  */
  /* Give us a way to debug problems with crashes on startup when
     running under the MSVC profiler. */
  if GetEnvironmentVariable (c"EMACS_DEBUG".as_ptr (), core::ptr::null_mut (), 0) > 0
    { DebugBreak (); }

  the_malloc_fn = Some (malloc_before_init);
  the_realloc_fn = Some (realloc_before_init);
  the_free_fn = Some (free_before_init);

  /* Cache system info, e.g., the NT page size.  */
  cache_system_info ();

  /* This prevents ctrl-c's in shells running while we're suspended from
     having us exit.  */
  /* C's `(PHANDLER_ROUTINE) ctrl_c_handler` reinterprets one function-pointer type as another;
     Rust's `as` refuses that pair (`E0605`, measured), so the port binds C's cast to a local of
     the alias's own type and transmutes the pointer - the operation the `DEF_DLL_FN` expansions
     use, and the shape `process.rs` 8671 gives the same kind of cast.  */
  let handler: PHANDLER_ROUTINE
    = Some (core::mem::transmute::<unsafe extern "C" fn (c_ulong) -> BOOL,
				   unsafe extern "system" fn (DWORD) -> BOOL> (ctrl_c_handler));
  SetConsoleCtrlHandler (handler, TRUE);

  /* Prevent Emacs from being locked up (eg. in batch mode) when
     accessing devices that aren't mounted (eg. removable media drives).  */
  SetErrorMode (SEM_FAILCRITICALERRORS);
  mainCRTStartup ();
}

/* C 160-161: "Improve on the CRT 'signal' implementation so that we could record the SIGCHLD
   handler and fake interval timers."  */
/// C 162-163: `signal_handler sys_signal (int sig, signal_handler handler)`.
/// C: extern signal_handler sys_signal (int, signal_handler);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_signal (sig: c_int, handler: signal_handler) -> signal_handler
{
  let mut old: signal_handler;

  /* SIGCHLD is needed for supporting subprocesses, see sys_kill
     below.  SIGALRM and SIGPROF are used by setitimer.  All the
     others are the only ones supported by the MS runtime.  */
  if !(sig == SIGINT || sig == SIGSEGV || sig == SIGILL || sig == SIGBREAK
       || sig == SIGFPE || sig == SIGABRT || sig == SIGTERM
       || sig == SIGCHLD || sig == SIGALRM || sig == SIGPROF)
    {
      *errno () = EINVAL;
      return core::mem::transmute::<isize, signal_handler> (SIG_ERR);
    }
  old = sig_handlers[sig as usize];
  /* SIGABRT is treated specially because w32.c installs term_ntproc
     as its handler, so we don't want to override that afterwards.
     Aborting Emacs works specially anyway: either by calling
     emacs_abort directly or through terminate_due_to_signal, which
     calls emacs_abort through emacs_raise.  */
  if !(sig == SIGABRT && old == Some (term_ntproc as unsafe extern "C" fn (c_int)))
    {
      sig_handlers[sig as usize] = handler;
      if !(sig == SIGCHLD || sig == SIGALRM || sig == SIGPROF)
	{ signal (sig, handler); }
    }
  return old;
}

/* C 192: "Emulate sigaction."  */
/// C 193-194: `int sigaction (int sig, const struct sigaction *act, struct sigaction *oact)`.
/// C: extern int sigaction (int, const struct sigaction *, struct sigaction *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigaction (sig: c_int, act: *const sigaction, oact: *mut sigaction)
  -> c_int
{
  let mut old: signal_handler = SIG_DFL;
  let mut retval: c_int = 0;

  if !act.is_null ()
    { old = sys_signal (sig, (*act).sa_handler); }
  else if !oact.is_null ()
    { old = sig_handlers[sig as usize]; }

  if old == core::mem::transmute::<isize, signal_handler> (SIG_ERR)
    {
      *errno () = EINVAL;
      retval = -1;
    }
  if !oact.is_null ()
    {
      (*oact).sa_handler = old;
      (*oact).sa_flags = 0;
      (*oact).sa_mask = empty_mask;
    }
  return retval;
}

/* C 218: "Emulate signal sets and blocking of signals used by timers."  */

/// C 220-221: `int sigemptyset (sigset_t *set)`.
/// C: extern int sigemptyset (sigset_t *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigemptyset (set: *mut sigset_t) -> c_int
{
  *set = 0;
  return 0;
}

/// C 227-228: `int sigaddset (sigset_t *set, int signo)`.
/// C: extern int sigaddset (sigset_t *, int);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int
{
  if set.is_null ()
    {
      *errno () = EINVAL;
      return -1;
    }
  if signo < 0 || signo >= NSIG
    {
      *errno () = EINVAL;
      return -1;
    }

  *set |= (1u32 << signo);

  return 0;
}

/// C 246-247: `int sigfillset (sigset_t *set)`.
/// C: extern int sigfillset (sigset_t *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigfillset (set: *mut sigset_t) -> c_int
{
  if set.is_null ()
    {
      *errno () = EINVAL;
      return -1;
    }

  *set = 0xFFFFFFFF;
  return 0;
}

/// C 259-260: `int sigprocmask (int how, const sigset_t *set, sigset_t *oset)`.
/// C: extern int sigprocmask (int, const sigset_t *, sigset_t *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigprocmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t)
  -> c_int
{
  if !(how == SIG_BLOCK || how == SIG_UNBLOCK || how == SIG_SETMASK)
    {
      *errno () = EINVAL;
      return -1;
    }

  if !oset.is_null ()
    { *oset = sig_mask; }

  if set.is_null ()
    { return 0; }

  /* C's `switch` has no `default:`; the guard above is its whole domain, so the port's extra arm
     is the empty one Rust requires.  */
  match how
    {
      SIG_BLOCK =>
	{ sig_mask |= *set; }
      SIG_SETMASK =>
	{ sig_mask = *set; }
      SIG_UNBLOCK =>
	{
	  /* FIXME: Catch signals that are blocked and reissue them when
	     they are unblocked.  Important for SIGALRM and SIGPROF only.  */
	  sig_mask &= !(*set);
	}
      _ => {}
    }

  return 0;
}

/// C 292-293: `int pthread_sigmask (int how, const sigset_t *set, sigset_t *oset)`.  `nt/inc/ms-w32.h`
/// 368-373 `#undef`s the runtime's zero macro for this name, which is why the file defines it.
/// C: extern int pthread_sigmask (int, const sigset_t *, sigset_t *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_sigmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t)
  -> c_int
{
  if sigprocmask (how, set, oset) == -1
    { return EINVAL; }
  return 0;
}

/// C 300-301: `int sigismember (const sigset_t *set, int signo)`.
/// C: extern int sigismember (const sigset_t *, int);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigismember (set: *const sigset_t, signo: c_int) -> c_int
{
  if signo < 0 || signo >= NSIG
    {
      *errno () = EINVAL;
      return -1;
    }
  if signo > core::mem::size_of::<sigset_t> () as c_int * CHAR_BIT
    { emacs_abort (); }

  return ((*set & (1u32 << signo)) != 0) as c_int;
}

/* C 314-316: "A fuller emulation of 'raise', which supports signals that MS runtime doesn't know
   about, and avoids the danger of invoking the invalid-argument handler.  syssignal.h redirects
   'raise' to this."  C 318's `#undef raise` cancels `nt/inc/ms-w32.h` 379's redirect for this file
   (which is why the definition below is named `w32_raise` and the *call* inside it is the CRT's
   `raise`); the mirror spells the redirect as a `macro_rules!`, whose call form this file's plain
   call cannot reach (the ledger records the reading).

#undef raise
*/

/// C 320-321: `int w32_raise (int signo)`.
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_raise (signo: c_int) -> c_int
{
  let mut handler: signal_handler;

  match signo
    {
      /* Signals supported by MS runtime: */
      SIGINT | SIGILL | SIGFPE | SIGSEGV | SIGTERM | SIGBREAK | SIGABRT =>
	return raise (signo),
      /* For signals for which we have custom support in Emacs, call the
	 handler directly.  */
      SIGCHLD | SIGALRM | SIGPROF =>
	{
	  handler = sig_handlers[signo as usize];
	  /* Implementation note: SIG_DFL does nothing, since these signals
	     are not supported by the MS runtime.  */
	  if handler == core::mem::transmute::<usize, signal_handler> (SIG_IGN) || handler == SIG_DFL
	    { return 0; }
	  else if handler == core::mem::transmute::<isize, signal_handler> (SIG_ERR)
	    {
	      *errno () = EINVAL;
	      return -1;
	    }
	  sig_handlers[signo as usize] = SIG_DFL; /* in case handler raises same signal */
	  handler.unwrap () (signo);
	  if sig_handlers[signo as usize] == SIG_DFL
	    { sig_handlers[signo as usize] = handler; }
	  return 0;
	}
      _ =>  /* Any unsupported signal.  */
	{
	  *errno () = EINVAL;
	  return -1;
	}
    }
}

/// C 362-363: `pid_t getpgrp (void)`.  `getpid` is `nt/inc/ms-w32.h` 282's rename of `_getpid`
/// (the mirror carries it as a `macro_rules!`), and the two agree in width (`pid_t` is `c_int`).
/// C: extern pid_t getpgrp (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getpgrp () -> pid_t
{
  return getpid! ();
}

/// C 368-369: `pid_t tcgetpgrp (int fd)`.
/// C: extern pid_t tcgetpgrp (int);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcgetpgrp (fd: c_int) -> pid_t
{
  return getpid! ();
}

/// C 374-375: `int setpgid (pid_t pid, pid_t pgid)`.
/// C: extern int setpgid (pid_t, pid_t);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setpgid (pid: pid_t, pgid: pid_t) -> c_int
{
  return 0;
}

/// C 380-381: `pid_t setsid (void)`.
/// C: extern pid_t setsid (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setsid () -> pid_t
{
  return getpid! ();
}

/* C 386-392: "Emulations of interval timers.  Limitations: only ITIMER_REAL and ITIMER_PROF are
   supported.  Implementation: a separate thread is started for each timer type, the thread calls
   the appropriate signal handler when the timer expires, after stopping the thread which installed
   the timer."  The two `ITIMER_*` selectors and `struct itimerval` reach this unit through
   C 63's `"w32term.h"` -> `w32gui.h` -> `systime.h` -> `<sys/time.h>`, which in the MinGW build is
   `nt/inc/sys/time.h` and in the port is `femacs/lib/include/sys_time.rs` (the frozen target's
   mirror, reached the same way); that shim's own note records the conflict, and the ledger's
   `## Deviations` records the reading (`ITIMER_PROF` is 2 there, 1 in the shim).  */
/// C 394-401: `struct itimer_data`.
/// C: struct itimer_data {
/// C:   volatile ULONGLONG expire;
/// C:   volatile ULONGLONG reload;
/// C:   volatile int terminate;
/// C:   int type;
/// C:   HANDLE caller_thread;
/// C:   HANDLE timer_thread;
/// C: };
#[cfg (feature = "windowsnt")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct itimer_data
{
  /* C's `volatile` has no Rust field form and is dropped (the ledger records it).  */
  pub expire: ULONGLONG,
  pub reload: ULONGLONG,
  pub terminate: c_int,
  pub r#type: c_int,
  pub caller_thread: HANDLE,
  pub timer_thread: HANDLE,
}

/// C 403-408: the file-scope timer state.  "If non-zero, itimers are disabled.  Used during
/// shutdown, when we delete the critical sections used by the timer threads."
#[cfg (feature = "windowsnt")]
static mut ticks_now: ULONGLONG = 0;
#[cfg (feature = "windowsnt")]
static mut real_itimer: itimer_data = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut prof_itimer: itimer_data = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut clocks_min: ULONGLONG = 0;
#[cfg (feature = "windowsnt")]
static mut disable_itimers: c_int = 0;

/// C 410: `static CRITICAL_SECTION crit_real, crit_prof;`.
#[cfg (feature = "windowsnt")]
static mut crit_real: CRITICAL_SECTION = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut crit_prof: CRITICAL_SECTION = unsafe { core::mem::zeroed () };

/// C 412-418: "GetThreadTimes is not available on Windows 9X and possibly also on 2K." - the
/// entry-point typedef `init_timers` resolves through `get_proc_addr`.
/// C: typedef BOOL (WINAPI *GetThreadTimes_Proc) (HANDLE hThread, LPFILETIME lpCreationTime,
/// C:   LPFILETIME lpExitTime, LPFILETIME lpKernelTime, LPFILETIME lpUserTime);
#[cfg (feature = "windowsnt")]
pub type GetThreadTimes_Proc = Option<unsafe extern "system" fn (hThread: HANDLE,
                                                                lpCreationTime: LPFILETIME,
                                                                lpExitTime: LPFILETIME,
                                                                lpKernelTime: LPFILETIME,
                                                                lpUserTime: LPFILETIME) -> BOOL>;

/// C 420: `static GetThreadTimes_Proc s_pfn_Get_Thread_Times;`.
#[cfg (feature = "windowsnt")]
static mut s_pfn_Get_Thread_Times: GetThreadTimes_Proc = None;

/// C 422-423: the two `#define`s the timer loop and the conversions use.
/// C: #define MAX_SINGLE_SLEEP    30
/// C: #define TIMER_TICKS_PER_SEC 1000
#[cfg (feature = "windowsnt")]
pub const MAX_SINGLE_SLEEP: c_int = 30;
#[cfg (feature = "windowsnt")]
pub const TIMER_TICKS_PER_SEC: c_int = 1000;

/* C 425-429: "Return a suitable time value, in 1-ms units, for THREAD, a handle to a thread.  If
   THREAD is NULL or an invalid handle, return the current wall-clock time since January 1, 1601
   (UTC).  Otherwise, return the sum of kernel and user times used by THREAD since it was created,
   plus its creation time."  */
/// C 430-431: `static ULONGLONG w32_get_timer_time (HANDLE thread)`.
#[cfg (feature = "windowsnt")]
unsafe fn w32_get_timer_time (thread: HANDLE) -> ULONGLONG
{
  /* C leaves `retval` unset on the paths that abort; the port writes C's own zero value
     (PORTING.md 0.1-13).  */
  let mut retval: ULONGLONG = 0;
  let mut use_system_time: c_int = 1;
  /* The functions below return times in 100-ns units.  */
  let tscale: c_int = 10 * TIMER_TICKS_PER_SEC;

  if !thread.is_null () && thread != INVALID_HANDLE_VALUE
      && s_pfn_Get_Thread_Times.is_some ()
    {
      /* C leaves the six records unset; the port writes C's own zero values (PORTING.md 0.1-13). */
      let mut creation_ftime: FILETIME = core::mem::zeroed ();
      let mut exit_ftime: FILETIME = core::mem::zeroed ();
      let mut kernel_ftime: FILETIME = core::mem::zeroed ();
      let mut user_ftime: FILETIME = core::mem::zeroed ();
      let mut temp_creation: ULARGE_INTEGER = core::mem::zeroed ();
      let mut temp_kernel: ULARGE_INTEGER = core::mem::zeroed ();
      let mut temp_user: ULARGE_INTEGER = core::mem::zeroed ();

      if s_pfn_Get_Thread_Times.unwrap () (thread, &raw mut creation_ftime, &raw mut exit_ftime,
					   &raw mut kernel_ftime, &raw mut user_ftime) != 0
	{
	  use_system_time = 0;
	  temp_creation.u.LowPart = creation_ftime.dwLowDateTime;
	  temp_creation.u.HighPart = creation_ftime.dwHighDateTime;
	  temp_kernel.u.LowPart = kernel_ftime.dwLowDateTime;
	  temp_kernel.u.HighPart = kernel_ftime.dwHighDateTime;
	  temp_user.u.LowPart = user_ftime.dwLowDateTime;
	  temp_user.u.HighPart = user_ftime.dwHighDateTime;
	  retval =
	    temp_creation.QuadPart / tscale as ULONGLONG + temp_kernel.QuadPart / tscale as ULONGLONG
	    + temp_user.QuadPart / tscale as ULONGLONG;
	}
      else
	{ DebPrint! (("GetThreadTimes failed with error code %lu\n", GetLastError ())); }
    }

  if use_system_time != 0
    {
      let mut current_ftime: FILETIME = core::mem::zeroed ();
      let mut temp: ULARGE_INTEGER = core::mem::zeroed ();

      GetSystemTimeAsFileTime (&raw mut current_ftime);

      temp.u.LowPart = current_ftime.dwLowDateTime;
      temp.u.HighPart = current_ftime.dwHighDateTime;

      retval = temp.QuadPart / tscale as ULONGLONG;
    }

  return retval;
}

/* C 479: "Thread function for a timer thread."  */
/// C 480-481: `static DWORD WINAPI timer_loop (LPVOID arg)`.
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn timer_loop (arg: LPVOID) -> DWORD
{
  let itimer: *mut itimer_data = arg as *mut itimer_data;
  let which: c_int = (*itimer).r#type;
  let sig: c_int = if which == ITIMER_REAL { SIGALRM } else { SIGPROF };
  let crit: *mut CRITICAL_SECTION =
    if which == ITIMER_REAL { &raw mut crit_real } else { &raw mut crit_prof };
  let max_sleep: DWORD = (MAX_SINGLE_SLEEP * 1000 / TIMER_TICKS_PER_SEC) as DWORD;
  let hth: HANDLE =
    if which == ITIMER_REAL { core::ptr::null_mut () } else { (*itimer).caller_thread };

  while true
    {
      let mut sleep_time: DWORD;
      let mut handler: signal_handler;
      let mut now: ULONGLONG;
      let mut expire: ULONGLONG;
      let mut reload: ULONGLONG;

      /* Load new values if requested by setitimer.  */
      EnterCriticalSection (crit);
      expire = (*itimer).expire;
      reload = (*itimer).reload;
      LeaveCriticalSection (crit);
      if (*itimer).terminate != 0
	{ return 0; }

      if expire == 0
	{
	  /* We are idle.  */
	  Sleep (max_sleep);
	  continue;
	}

      now = w32_get_timer_time (hth);
      if expire > now
	{ sleep_time = (expire - now) as DWORD; }
      else
	{ sleep_time = 0; }
      /* Don't sleep too long at a time, to be able to see the
	 termination flag without too long a delay.  */
      while sleep_time > max_sleep
	{
	  if (*itimer).terminate != 0
	    { return 0; }
	  Sleep (max_sleep);
	  EnterCriticalSection (crit);
	  expire = (*itimer).expire;
	  LeaveCriticalSection (crit);
	  now = w32_get_timer_time (hth);
	  sleep_time = if expire > now { (expire - now) as DWORD } else { 0 };
	}
      if (*itimer).terminate != 0
	{ return 0; }
      if sleep_time > 0
	{
	  Sleep (sleep_time * 1000 / TIMER_TICKS_PER_SEC as DWORD);
	  /* Always sleep past the expiration time, to make sure we
	     never call the handler _before_ the expiration time,
	     always slightly after it.  Sleep(5) makes sure we don't
	     hog the CPU by calling 'w32_get_timer_time' with high
	     frequency, and also let other threads work.  */
	  while w32_get_timer_time (hth) < expire
	    { Sleep (5); }
	}

      EnterCriticalSection (crit);
      expire = (*itimer).expire;
      LeaveCriticalSection (crit);
      if expire == 0
	{ continue; }

      /* Time's up.  */
      handler = sig_handlers[sig as usize];
      if !(handler == SIG_DFL || handler == core::mem::transmute::<usize, signal_handler> (SIG_IGN)
	   || handler == core::mem::transmute::<isize, signal_handler> (SIG_ERR))
	  /* FIXME: Don't ignore masked signals.  Instead, record that
	     they happened and reissue them when the signal is
	     unblocked.  */
	  && sigismember (&raw const sig_mask, sig) == 0
	  /* Simulate masking of SIGALRM and SIGPROF when processing
	     fatal signals.  */
	  && !fatal_error_in_progress
	  && !(*itimer).caller_thread.is_null ()
	{
	  /* Simulate a signal delivered to the thread which installed
	     the timer, by suspending that thread while the handler
	     runs.  */
	  let th: HANDLE = (*itimer).caller_thread;
	  let result: DWORD = SuspendThread (th);

	  if result == -1i32 as DWORD	/* C: `result == (DWORD)-1` */
	    { return 2; }

	  handler.unwrap () (sig);
	  ResumeThread (th);
	}

      /* Update expiration time and loop.  */
      EnterCriticalSection (crit);
      expire = (*itimer).expire;
      if expire == 0
	{
	  LeaveCriticalSection (crit);
	  continue;
	}
      reload = (*itimer).reload;
      if reload > 0
	{
	  now = w32_get_timer_time (hth);
	  if expire <= now
	    {
	      let lag: ULONGLONG = now - expire;

	      /* If we missed some opportunities (presumably while
		 sleeping or while the signal handler ran), skip
		 them.  */
	      if lag > reload
		{ expire = now - (lag % reload); }

	      expire += reload;
	    }
	}
      else
	{ expire = 0; }	/* become idle */
      (*itimer).expire = expire;
      LeaveCriticalSection (crit);
    }
  return 0;
}

/// C 606-607: `static void stop_timer_thread (int which)`.
#[cfg (feature = "windowsnt")]
unsafe fn stop_timer_thread (which: c_int)
{
  let itimer: *mut itimer_data =
    if which == ITIMER_REAL { &raw mut real_itimer } else { &raw mut prof_itimer };
  let mut i: c_int;
  let mut err: DWORD = 0;
  let mut exit_code: DWORD = 255;
  /* C leaves `status` unset where its loop is not entered; the port writes C's own zero value
     (PORTING.md 0.1-13).  */
  let mut status: BOOL = 0;

  /* Signal the thread that it should terminate.  */
  (*itimer).terminate = 1;

  if (*itimer).timer_thread.is_null ()
    { return; }

  /* Wait for the timer thread to terminate voluntarily, then kill it
     if it doesn't.  This loop waits twice more than the maximum
     amount of time a timer thread sleeps, see above.  */
  i = 0;
  while i < MAX_SINGLE_SLEEP / 5
    {
      status = GetExitCodeThread ((*itimer).timer_thread, &raw mut exit_code);
      if !(status != 0 && exit_code == STILL_ACTIVE)
	{ break; }
      Sleep (10);
      i += 1;
    }
  /* C writes `(status == FALSE && (err = GetLastError ()) == ERROR_INVALID_HANDLE)`, so the
     call is short-circuited away when `status` is not FALSE; the port's block expression keeps
     exactly that order.  */
  if (status == FALSE && { err = GetLastError (); err == ERROR_INVALID_HANDLE })
      || exit_code == STILL_ACTIVE
    {
      if !(status == FALSE && err == ERROR_INVALID_HANDLE)
	{ TerminateThread ((*itimer).timer_thread, 0); }
    }

  /* Clean up.  */
  CloseHandle ((*itimer).timer_thread);
  (*itimer).timer_thread = core::ptr::null_mut ();
  if !(*itimer).caller_thread.is_null ()
    {
      CloseHandle ((*itimer).caller_thread);
      (*itimer).caller_thread = core::ptr::null_mut ();
    }
}

/* C 648: "This is called at shutdown time from term_ntproc."  */
/// C 649-650: `void term_timers (void)`.
/// C: extern void term_timers (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn term_timers ()
{
  if !real_itimer.timer_thread.is_null ()
    { stop_timer_thread (ITIMER_REAL); }
  if !prof_itimer.timer_thread.is_null ()
    { stop_timer_thread (ITIMER_PROF); }

  /* We are going to delete the critical sections, so timers cannot
     work after this.  */
  disable_itimers = 1;

  DeleteCriticalSection (&raw mut crit_real);
  DeleteCriticalSection (&raw mut crit_prof);
  DeleteCriticalSection (&raw mut crit_sig);
}

/* C 666: "This is called at initialization time from init_ntproc."  */
/// C 667-668: `void init_timers (void)`.
/// C: extern void init_timers (void);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_timers ()
{
  /* GetThreadTimes is not available on all versions of Windows, so
     need to probe for its availability dynamically, and call it
     through a pointer.  */
  s_pfn_Get_Thread_Times = None; /* in case dumped Emacs comes with a value */
  if os_subtype != OS_SUBTYPE_9X
    {
      /* C's cast `(GetThreadTimes_Proc)` reinterprets the loader's untyped entry point; the port
	 transmutes between the two `Option<fn>` types of equal width (the
	 `comp.rs` 137 / `decompress.rs` 237 `DEF_DLL_FN` reading).  */
      s_pfn_Get_Thread_Times = core::mem::transmute::<VOIDFNPTR, GetThreadTimes_Proc>
	(get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
			c"GetThreadTimes".as_ptr ()));
    }

  /* Make sure we start with zeroed out itimer structures, since
     dumping may have left there traces of threads long dead.  */
  memset (&raw mut real_itimer as *mut c_void, 0, core::mem::size_of::<itimer_data> ());
  memset (&raw mut prof_itimer as *mut c_void, 0, core::mem::size_of::<itimer_data> ());

  InitializeCriticalSection (&raw mut crit_real);
  InitializeCriticalSection (&raw mut crit_prof);
  InitializeCriticalSection (&raw mut crit_sig);

  disable_itimers = 0;
}

/// C 690-691: `static int start_timer_thread (int which)`.
#[cfg (feature = "windowsnt")]
unsafe fn start_timer_thread (which: c_int) -> c_int
{
  let mut exit_code: DWORD = 0;
  let mut tid: DWORD = 0;
  let mut th: HANDLE = core::ptr::null_mut ();
  let itimer: *mut itimer_data =
    if which == ITIMER_REAL { &raw mut real_itimer } else { &raw mut prof_itimer };

  if !(*itimer).timer_thread.is_null ()
      && GetExitCodeThread ((*itimer).timer_thread, &raw mut exit_code) != 0
      && exit_code == STILL_ACTIVE
    { return 0; }

  /* Clean up after possibly exited thread.  */
  if !(*itimer).timer_thread.is_null ()
    {
      CloseHandle ((*itimer).timer_thread);
      (*itimer).timer_thread = core::ptr::null_mut ();
    }
  if !(*itimer).caller_thread.is_null ()
    {
      CloseHandle ((*itimer).caller_thread);
      (*itimer).caller_thread = core::ptr::null_mut ();
    }

  /* Start a new thread.  */
  if DuplicateHandle (GetCurrentProcess (), GetCurrentThread (),
		      GetCurrentProcess (), &raw mut th, 0, FALSE,
		      DUPLICATE_SAME_ACCESS) == 0
    {
      *errno () = ESRCH;
      return -1;
    }
  (*itimer).terminate = 0;
  (*itimer).r#type = which;
  (*itimer).caller_thread = th;
  /* Request that no more than 64KB of stack be reserved for this
     thread, to avoid reserving too much memory, which would get in
     the way of threads we start to wait for subprocesses.  See also
     new_child below.  */
  (*itimer).timer_thread = CreateThread (core::ptr::null_mut (), 64 * 1024, Some (timer_loop),
					 itimer as *mut c_void, 0x00010000, &raw mut tid);

  if (*itimer).timer_thread.is_null ()
    {
      CloseHandle ((*itimer).caller_thread);
      (*itimer).caller_thread = core::ptr::null_mut ();
      *errno () = EAGAIN;
      return -1;
    }

  /* This is needed to make sure that the timer thread running for
     profiling gets CPU as soon as the Sleep call terminates. */
  if which == ITIMER_PROF
    { SetThreadPriority ((*itimer).timer_thread, THREAD_PRIORITY_TIME_CRITICAL); }

  return 0;
}

/* C 749-751: "Most of the code of getitimer and setitimer (but not of their subroutines) was
   shamelessly stolen from itimer.c in the DJGPP library, see www.delorie.com/djgpp."  */
/// C 752-753: `int getitimer (int which, struct itimerval *value)`.
/// C: int getitimer (int, struct itimerval *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getitimer (which: c_int, value: *mut itimerval) -> c_int
{
  let mut t_expire: *mut ULONGLONG;
  let mut t_reload: *mut ULONGLONG;
  let mut expire: ULONGLONG;
  let mut reload: ULONGLONG;
  let mut usecs: i64;
  let mut crit: *mut CRITICAL_SECTION;
  let mut itimer: *mut itimer_data;

  if disable_itimers != 0
    { return -1; }

  if value.is_null ()
    {
      *errno () = EFAULT;
      return -1;
    }

  if which != ITIMER_REAL && which != ITIMER_PROF
    {
      *errno () = EINVAL;
      return -1;
    }

  itimer = if which == ITIMER_REAL { &raw mut real_itimer } else { &raw mut prof_itimer };

  ticks_now = w32_get_timer_time (if which == ITIMER_REAL
				  { core::ptr::null_mut () }
				  else { GetCurrentThread () });

  t_expire = &raw mut (*itimer).expire;
  t_reload = &raw mut (*itimer).reload;
  crit = if which == ITIMER_REAL { &raw mut crit_real } else { &raw mut crit_prof };

  EnterCriticalSection (crit);
  reload = *t_reload;
  expire = *t_expire;
  LeaveCriticalSection (crit);

  if expire != 0
    { expire -= ticks_now; }

  (*value).it_value.tv_sec = (expire / TIMER_TICKS_PER_SEC as ULONGLONG) as time_t;
  usecs =
    (((expire % TIMER_TICKS_PER_SEC as ULONGLONG) * 1000000) / TIMER_TICKS_PER_SEC as ULONGLONG)
    as i64;
  (*value).it_value.tv_usec = usecs as suseconds_t;
  (*value).it_interval.tv_sec = (reload / TIMER_TICKS_PER_SEC as ULONGLONG) as time_t;
  usecs =
    (((reload % TIMER_TICKS_PER_SEC as ULONGLONG) * 1000000) / TIMER_TICKS_PER_SEC as ULONGLONG)
    as i64;
  (*value).it_interval.tv_usec = usecs as suseconds_t;

  return 0;
}

/// C 807-808: `int setitimer (int which, struct itimerval *value, struct itimerval *ovalue)`.
/// C: int setitimer (int, struct itimerval *, struct itimerval *);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setitimer (which: c_int, value: *mut itimerval, ovalue: *mut itimerval)
  -> c_int
{
  let mut t_expire: *mut ULONGLONG;
  let mut t_reload: *mut ULONGLONG;
  let mut expire: ULONGLONG;
  let mut reload: ULONGLONG;
  let mut expire_old: ULONGLONG;
  let mut reload_old: ULONGLONG;
  let mut usecs: i64;
  let mut crit: *mut CRITICAL_SECTION;
  /* C leaves `tem` unset and only reads it when `ovalue` is null; the port writes C's own zero
     value (PORTING.md 0.1-13).  */
  let mut tem: itimerval = core::mem::zeroed ();
  let mut ptem: *mut itimerval;

  if disable_itimers != 0
    { return -1; }

  /* Posix systems expect timer values smaller than the resolution of
     the system clock be rounded up to the clock resolution.  First
     time we are called, measure the clock tick resolution.  */
  if clocks_min == 0
    {
      let mut t1: ULONGLONG;
      let mut t2: ULONGLONG;

      t1 = w32_get_timer_time (core::ptr::null_mut ());
      loop
	{
	  t2 = w32_get_timer_time (core::ptr::null_mut ());
	  if !(t2 == t1)
	    { break; }
	}
      clocks_min = t2 - t1;
    }

  if !ovalue.is_null ()
    { ptem = ovalue; }
  else
    { ptem = &raw mut tem; }

  if getitimer (which, ptem) != 0 /* also sets ticks_now */
    { return -1; }		       /* errno already set */

  t_expire =
    if which == ITIMER_REAL { &raw mut real_itimer.expire } else { &raw mut prof_itimer.expire };
  t_reload =
    if which == ITIMER_REAL { &raw mut real_itimer.reload } else { &raw mut prof_itimer.reload };

  crit = if which == ITIMER_REAL { &raw mut crit_real } else { &raw mut crit_prof };

  if value.is_null ()
      || ((*value).it_value.tv_sec == 0 && (*value).it_value.tv_usec == 0)
    {
      EnterCriticalSection (crit);
      /* Disable the timer.  */
      *t_expire = 0;
      *t_reload = 0;
      LeaveCriticalSection (crit);
      return 0;
    }

  reload = ((*value).it_interval.tv_sec * TIMER_TICKS_PER_SEC as time_t) as ULONGLONG;

  usecs = (*value).it_interval.tv_usec as i64;
  if (*value).it_interval.tv_sec == 0
      && usecs != 0
      && ((usecs * TIMER_TICKS_PER_SEC as i64) as ULONGLONG) < clocks_min * 1000000
    { reload = clocks_min; }
  else
    {
      usecs *= TIMER_TICKS_PER_SEC as i64;
      reload += (usecs / 1000000) as ULONGLONG;
    }

  expire = ((*value).it_value.tv_sec * TIMER_TICKS_PER_SEC as time_t) as ULONGLONG;
  usecs = (*value).it_value.tv_usec as i64;
  if (*value).it_value.tv_sec == 0
      && ((usecs * TIMER_TICKS_PER_SEC as i64) as ULONGLONG) < clocks_min * 1000000
    { expire = clocks_min; }
  else
    {
      usecs *= TIMER_TICKS_PER_SEC as i64;
      expire += (usecs / 1000000) as ULONGLONG;
    }

  expire += ticks_now;

  EnterCriticalSection (crit);
  expire_old = *t_expire;
  reload_old = *t_reload;
  if !(expire == expire_old && reload == reload_old)
    {
      *t_reload = reload;
      *t_expire = expire;
    }
  LeaveCriticalSection (crit);

  return start_timer_thread (which);
}

/// C 896-897: `int alarm (int seconds)`.  C 899's `#ifdef HAVE_SETITIMER` is true in the frozen
/// table (and the MinGW build defines it, which is why this file defines `setitimer`), so the
/// C 909 `#else` arm - `return seconds;` - is the other arm carried here.
/// C: extern int alarm (int);
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alarm (seconds: c_int) -> c_int
{
  #[cfg (feature = "have_setitimer")]
  {
    /* C leaves the two records unset and fills every member before use; the port writes C's own
       zero values (PORTING.md 0.1-13).  */
    let mut new_values: itimerval = core::mem::zeroed ();
    let mut old_values: itimerval = core::mem::zeroed ();

    new_values.it_value.tv_sec = seconds as time_t;
    new_values.it_value.tv_usec = 0;
    new_values.it_interval.tv_sec = 0;
    new_values.it_interval.tv_usec = 0;

    if setitimer (ITIMER_REAL, &raw mut new_values, &raw mut old_values) < 0
      { return 0; }
    return old_values.it_value.tv_sec as c_int;
  }
  /* C 909-911: the `#else` arm - MinGW's `alarm`, whose emulation this file is: no timer is
     installed and the request is reported back unchanged.  */
  #[cfg (not (feature = "have_setitimer"))]
  {
    return seconds;
  }
}

/* Here's an overview of how support for waiting more than 64 objects
   on MS-Windows.

   As noted in the MS documentation, WaitForMultipleObjects can wait on
   a maximum of MAXIMUM_WAIT_OBJECTS (64) objects.  Due to this
   limitation, earlier versions of Emacs (at least 30) could only
   support up to 32 subprocesses or network connections.

   The documentation suggests using multiple threads or a thread pool to
   wait on a larger number of objects.  With Windows 2000, Microsoft has
   added new thread pooling functions to make thread creation,
   destruction, and general management easier:

   . https://jacobfilipp.com/MSJ/pooling.html

   Thread pools implement waiting by calling WaitForMultipleObjects or
   relying on completion ports (Windows 8 or above) in their internal
   threads, as The Old New Thing said:

   . https://devblogs.microsoft.com/oldnewthing/20081117-00/?p=20183
   . https://devblogs.microsoft.com/oldnewthing/20220406-00/?p=106434

   However, since the system thread pool in versions prior to Windows 7
   does not support adjusting thread stack sizes by calling
   SetThreadpoolStackInformation, using a large number of threads may
   consume a lot of address space.  This can have a significant impact on
   the limited 2GB address space available on 32-bit systems.  To avoid
   this issue on Windows Vista and earlier systems and make the code as
   generic as possible, a simple waiting thread pool is manually
   implemented here.

   The waiting thread pool contains a maximum of 32 threads, with each
   thread capable of waiting on up to 63 objects (one object is reserved
   for communication with the main thread).  Combined with the main
   thread's WaitForMultipleObjects call, this implementation supports
   waiting on a maximum of 2048 objects.  Since Windows only supports
   creating subprocesses using 'pipe method, this allows Emacs to create
   a maximum of approximately 1024 subprocesses.  This limit is close to
   the number of subprocesses that can be created using the 'pty method
   on Linux when the default FD_SETSIZE is 1024.

   Once created, the threads do not exit after finish waiting; instead,
   they enter an infinite loop, waiting for an event to trigger and
   begin their WaitForMultipleObjects tasks, thereby eliminating the
   additional time and power consumption associated with frequently
   creating and destroying threads.  If a thread is not triggered after a
   certain number of sys_select calls, it can be terminated to free up
   resources.  */                                                     // C 914-962

/* The values returned by WaitForMultipleObjects, WAIT_ABANDONED_0
   (0x80) and WAIT_TIMEOUT (0x102), are less than 2048 (0x800), so
   define new constants to replace them.  'WFO' prefix stands for
   'WaitForObjects'.  */                                              // C 964-967
/// C 968: `#define WFO_ABANDONED 0x10000`
#[cfg (feature = "windowsnt")]
pub const WFO_ABANDONED: DWORD = 0x10000;
/// C 969: `#define WFO_TIMEOUT   0x20002`
#[cfg (feature = "windowsnt")]
pub const WFO_TIMEOUT: DWORD = 0x20002;
/// C 970: `#define WFO_FAILED    0xfffff`
#[cfg (feature = "windowsnt")]
pub const WFO_FAILED: DWORD = 0xfffff;
/// C 971: `#define WFO_MAX_WAIT  FD_SETSIZE` - `FD_SETSIZE` is `include/w32.rs` 94's 2048, which
/// is what this file's overview comment (C 954) names.
#[cfg (feature = "windowsnt")]
pub const WFO_MAX_WAIT: c_int = w32_system_names::FD_SETSIZE;

/* When debugging, use SetThreadDescription to provide additional
   debugging information.  They are already defined in w32fns.c.  */  // C 973-974
/// C 975: `typedef BOOL (WINAPI *IsDebuggerPresent_Proc) (void);` - the `src/systhread.rs`
/// 717-721 spelling of the same two typedefs (that unit ports `systhread.c`, which declares them
/// too); `PCWSTR`/`HRESULT` are declared at the use site below because no mirror in this unit's
/// graph carries them (P-72 rule 4).
#[cfg (feature = "windowsnt")]
pub type IsDebuggerPresent_Proc = Option<unsafe extern "system" fn () -> BOOL>;
/// C 976: `typedef HRESULT (WINAPI *SetThreadDescription_Proc) (HANDLE, PCWSTR);`
#[cfg (feature = "windowsnt")]
pub type SetThreadDescription_Proc = Option<unsafe extern "system" fn (thread: HANDLE,
								      description: PCWSTR) -> HRESULT>;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  /// C 977: `extern IsDebuggerPresent_Proc is_debugger_present;`
  pub static mut is_debugger_present: IsDebuggerPresent_Proc;
  /// C 978: `extern SetThreadDescription_Proc set_thread_description;`
  pub static mut set_thread_description: SetThreadDescription_Proc;
}

/* Structure for waiting thread's context.  */                        // C 980
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct wait_objects_context
{
  /* Handle to the thread itself.  */                                 // C 983
  pub thread: HANDLE,                     // C 984
  /* Handle to an event object that is signaled by the main thread
     to tell thread start waiting.  */                                // C 985-986
  pub wait_start: HANDLE,                 // C 987
  /* Handle to an event object that is signaled when this thread is not
     used for a period of time, it tells thread to quit.  */          // C 988-989
  pub wait_quit: HANDLE,                  // C 990
  /* Handle to an event object that is signaled when wokrer thread is
     ready to call WaitForMultipleObjects on the given objects.  */    // C 991-992
  pub wait_ready: HANDLE,                 // C 993
  /* pHandles and nCount are part of the WaitForMultipleObjects
     parameters, specifying the array of objects to wait on and the
     number of objects.  bWaitAll and dwMilliseconds are not needed.  */   // C 994-996
  pub pHandles: *mut HANDLE,              // C 997
  pub nCount: c_int,                      // C 998
  /* The return value of the thread's call to WaitForMultipleObjects.  */   // C 999
  pub result: DWORD,                      // C 1000
  /* Used to store GetLastError value of failed wait.  */              // C 1001
  pub errcode: DWORD,                     // C 1002
  /* The thread's wait count.  */                                      // C 1003
  pub call_count: c_int,                  // C 1004
}

/* Structure for the waiting thread pool  */                           // C 1007
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct wait_objects_pool
{
  /* An array for the context of each worker thread.  */               // C 1010
  pub wocs: [wait_objects_context; 32],   // C 1011
  /* The number of the current threads for waiting.  */                // C 1012
  pub count: c_int,                       // C 1013
  /* times of waiting in main thread.  */                              // C 1014
  pub main_thread_waits: c_int,           // C 1015
  /* The event handle that signals the worker thread's timeout.  */     // C 1016
  pub timeout_event: HANDLE,              // C 1017
  /* The flag that indicates whether the thread pool is initialized.  */   // C 1018
  pub init_flag: c_int,                   // C 1019
}

/* Structure for information about the grouping of waits.  */          // C 1022
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct wait_objects_info
{
  /* An array of handles used as parameters for WaitForMultipleObjects
     in the main thread, which includes event handles wait_ready
     signaled by the worker threads, as well as any possible remaining
     wait objects.  */                                                 // C 1025-1028
  pub hs: [HANDLE; 64],                   // C 1029
  /* The number of threads to be used.  */                             // C 1030
  pub nt: c_int,                          // C 1031
  /* The number of handles the main thread needs to wait for.  */       // C 1032
  pub nh: c_int,                          // C 1033
}

/* Declare wait_pool and wait_info as static variables to avoid
   unnecessary stack allocation.  */                                  // C 1036-1037
#[cfg (feature = "windowsnt")]
static mut wait_pool: wait_objects_pool = unsafe { core::mem::zeroed () };
#[cfg (feature = "windowsnt")]
static mut wait_info: wait_objects_info = unsafe { core::mem::zeroed () };

/* C 50's `<windows.h>` names this file takes from `winnt.h`: `PCWSTR`, the `wchar_t *` the C 976
   typedef spells, and `HRESULT`, the `long` it returns.  No mirror in this unit's graph carries
   either (the unit's own `<windows.h>` section names the scalars it needs, this pair among them
   being new here), so both stand at the use site with the `UNVERIFIED` mark the acquisition note
   gives every Windows name on this machine (P-88 ①).  */
/// C: `typedef const wchar_t *PCWSTR;   [<winnt.h>; UNVERIFIED here]`
#[cfg (feature = "windowsnt")]
pub type PCWSTR = *const u16;
/// C: `typedef long HRESULT;   [<winnt.h>; UNVERIFIED here]`
#[cfg (feature = "windowsnt")]
pub type HRESULT = c_long;

/* C 50's `<windows.h>`/`<winbase.h>`: the wait-object entry points this slice calls and the values
   they return.  C 1078 and C 1094 make the two `WaitForMultipleObjects` calls whose result C
   1081-1111 tests and `switch`es on, C 1147 makes `start_wait_objects`'s `WaitForSingleObject`
   probe, and C 1131/1245/1248/1253 open the events that C 1084/1089/1290/1291 then `SetEvent`s and
   `ResetEvent`s; C 1158 hands the failing thread's error to `SetLastError`.  `INFINITE` is
   `<winbase.h>`'s wait sentinel and `S_OK` is `<winerror.h>`'s `((HRESULT) 0L)`, which C 1275
   compares `SetThreadDescription`'s return against.  The unit's own `<windows.h>` section above
   names slice 1's surface only, and no mirror in this unit's graph carries any of these names, so
   all of them stand at the use site with the `UNVERIFIED` mark the acquisition note gives every
   Windows name on this machine (P-88 ①); C's `…A`/`…W` alias pair is spelled as C spells it
   (`_UNICODE` is undefined, so the alias resolves to the ANSI entry), exactly as the section above
   does for `GetEnvironmentVariable`.  */
/// C: #define WAIT_OBJECT_0 0
/// C: #define WAIT_ABANDONED_0 (0x00000080)
/// C: #define WAIT_TIMEOUT 0x00000102
/// C: #define WAIT_FAILED ((DWORD) 0xFFFFFFFF)
/// C: #define INFINITE 0xFFFFFFFF
/// C: #define S_OK ((HRESULT) 0L)
#[cfg (feature = "windowsnt")]
pub const WAIT_OBJECT_0: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub const WAIT_ABANDONED_0: DWORD = 0x00000080;
#[cfg (feature = "windowsnt")]
pub const WAIT_TIMEOUT: DWORD = 0x00000102;
#[cfg (feature = "windowsnt")]
pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;
#[cfg (feature = "windowsnt")]
pub const INFINITE: DWORD = 0xFFFFFFFF;
#[cfg (feature = "windowsnt")]
pub const S_OK: HRESULT = 0;
/// C: WINBASEAPI DWORD WINAPI WaitForMultipleObjects (DWORD nCount, const HANDLE *lpHandles,
/// C:   BOOL bWaitAll, DWORD dwMilliseconds);
/// C: WINBASEAPI DWORD WINAPI WaitForSingleObject (HANDLE hHandle, DWORD dwMilliseconds);
/// C: WINBASEAPI HANDLE WINAPI CreateEventA (LPSECURITY_ATTRIBUTES lpEventAttributes,
/// C:   BOOL bManualReset, BOOL bInitialState, LPCSTR lpName);
/// C: WINBASEAPI BOOL WINAPI SetEvent (HANDLE hEvent);
/// C: WINBASEAPI BOOL WINAPI ResetEvent (HANDLE hEvent);
/// C: WINBASEAPI void WINAPI SetLastError (DWORD dwErrCode);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn WaitForMultipleObjects (nCount: DWORD, lpHandles: *const HANDLE, bWaitAll: BOOL,
                                 dwMilliseconds: DWORD) -> DWORD;
  pub fn WaitForSingleObject (hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
  pub fn CreateEvent (lpEventAttributes: LPSECURITY_ATTRIBUTES, bManualReset: BOOL,
                      bInitialState: BOOL, lpName: LPCSTR) -> HANDLE;
  pub fn SetEvent (hEvent: HANDLE) -> BOOL;
  pub fn ResetEvent (hEvent: HANDLE) -> BOOL;
  pub fn SetLastError (dwErrCode: DWORD);
}

/* C 1041-1299: the wait-object worker pair - C 1041-1050's overview comment, C 1051-1117's
   `wait_objects_thread` and C 1119-1299's `start_wait_objects`.  C 1301's `stop_wait_objects` opens
   the next slice.  */

/* Thread proc for worker threads waiting on objects.  Each thread is
   normally blocked until woken by the main thread to wait on objects.
   When the wait completes, wait_ready is signaled to wake up the main
   thread and the thread blocks itself again.

   The worker thread needs to wait for the timeout event from the main
   thread.  When timeout_event is signaled by the main thread, it will
   end the current wait and start the next round.  Similarly, threads
   also need to wait for an quit event, which usually occurs when the
   thread has not been used for a certain period of time.  */

/// C 1051-1117: `static DWORD WINAPI wait_objects_thread (void *arg)` - C 1054-1060's four locals
/// (C's `HANDLE hs[64] = {NULL}` / `start_or_quit[2] = {NULL}` are the `[null; N]` arrays C 1056 and
/// C 1058 spell with initialisers), C 1062's cast of the thread argument, C 1064/1066's two event
/// placements, and C 1067-1115's `for (;;)`: C 1070's call count, C 1073's timeout event at the end
/// of `hs`, C 1076's `memcpy`, C 1078-1079/1094's two `WaitForMultipleObjects` calls with
/// C 1081-1093's error paths and C 1095-1114's `switch` over the second result (whose two summed
/// labels are guarded match arms, the `src/w32.rs` 13819 device - here the port keeps C's
/// `if`/`else if` chain because the arms fall out of the loop instead of breaking).  C 1116's
/// trailing `return 0` is C's own (the compilation unit is `unreachable_code`-allowed in this
/// file's header).
/// C: static DWORD WINAPI
/// C: wait_objects_thread (void *arg)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn wait_objects_thread (arg: *mut c_void) -> DWORD
{
  let mut ctx: *mut wait_objects_context = core::ptr::null_mut ();     // C 1054
  /* Thread's wait handle array.  */
  let mut hs: [HANDLE; 64] = [core::ptr::null_mut (); 64];             // C 1056
  /* start and quit event handle array.  */
  let mut start_or_quit: [HANDLE; 2] = [core::ptr::null_mut (); 2];    // C 1058
  /* return value of waiting.  */
  let mut res: DWORD = 0;                                              // C 1060
  /* Thread's context.  */
  ctx = arg as *mut wait_objects_context;                              // C 1062
  /* Place start event in start_or_quit[0].  */
  start_or_quit[0] = (*ctx).wait_start;                                // C 1064
  /* Place quit event in start_or_quit[1].  */
  start_or_quit[1] = (*ctx).wait_quit;                                 // C 1066
  loop
    {
      /* increase thread's wait call count by 1.  */
      (*ctx).call_count += 1;                                          // C 1070
      /* The timeout event object is placed at the end.
	 nCount will not exceed 63.  */
      hs[(*ctx).nCount as usize] = wait_pool.timeout_event;            // C 1073
      /* The contents copied by different threads do not overlap, so it
	 is safe to use memcpy.  */
      memcpy (hs.as_mut_ptr () as *mut c_void, (*ctx).pHandles as *const c_void,
	      (*ctx).nCount as usize * core::mem::size_of::<HANDLE> ()); // C 1076
      /* Start waiting.  */
      (*ctx).result = WaitForMultipleObjects (((*ctx).nCount + 1) as DWORD, hs.as_mut_ptr (),
					      FALSE, INFINITE);          // C 1078-1079
      /* Get the error code when the wait fails.  */
      if (*ctx).result == WAIT_FAILED                                  // C 1081
	{
	  (*ctx).errcode = GetLastError ();                              // C 1083
	  SetEvent ((*ctx).wait_ready);                                  // C 1084
	  return WFO_FAILED;                                             // C 1085
	}
      /* After waiting, signal wait_ready and wait for the wait_start
	 and wait_quit events from the main thread.  */
      if SetEvent ((*ctx).wait_ready) == 0                             // C 1089
	{
	  (*ctx).errcode = GetLastError ();                              // C 1091
	  return WFO_FAILED;                                             // C 1092
	}
      res = WaitForMultipleObjects (2, start_or_quit.as_mut_ptr (), FALSE, INFINITE); // C 1094
      /* wait_start, continue loop.  */
      if res == WAIT_OBJECT_0                                          // C 1095-1099
	{ }
      /* wait_quit, exit with code 0.  */
      else if res == WAIT_OBJECT_0 + 1                                 // C 1101
	{
	  /* Close its wait_quit event handle.  */
	  if CloseHandle (start_or_quit[1]) == 0                       // C 1103
	    { return GetLastError (); }                                // C 1104
	  return 0;                                                    // C 1105
	}
      /* Failure to wait for the start and quit event from the main
	 thread should not occur.  */
      else if res == WAIT_ABANDONED_0 || res == WAIT_ABANDONED_0 + 1   // C 1108-1111
	       || res == WAIT_TIMEOUT || res == WAIT_FAILED
	{
	  (*ctx).errcode = GetLastError ();                              // C 1112
	  return WFO_FAILED;                                             // C 1113
	}
    }
  return 0;                                                            // C 1116
}

/* Determine the grouping based on the given wait objects and assign
   them to the worker threads to begin waiting, creating new worker
   threads if necessary.  The function also performs initialization of
   some static resources.  */
/// C 1123-1299: `static int start_wait_objects (wait_objects_info *p, DWORD nCount, HANDLE
/// *lpHandles)` - C 1129-1137's lazy pool initialisation, C 1140-1165's live-thread scan
/// (`WaitForSingleObject (... , 0)` against `WAIT_TIMEOUT`/`WAIT_OBJECT_0`), C 1166-1209's
/// forty-line sizing comment (carried as C's text above the two assignments it explains),
/// C 1210-1236's group arithmetic with C's `memcpy` and per-thread `nCount`/`pHandles`,
/// C 1241-1278's thread creation loop (three `CreateEvent`s, `CreateThread` with C's
/// `64 * 1024`/`0x00010000`, and C 1268-1276's optional `SetThreadDescription`), C 1282-1297's
/// wait-array fill with `ResetEvent`/`SetEvent`, and C 1298's `return 0`.  C's `int` returns of the
/// `WFO_FAILED` macro are `as c_int` (C's own u32-to-int conversion).
/// C: static int
/// C: start_wait_objects (wait_objects_info *p, DWORD nCount, HANDLE *lpHandles)
#[cfg (feature = "windowsnt")]
unsafe fn start_wait_objects (p: *mut wait_objects_info, nCount: DWORD,
			      lpHandles: *mut HANDLE) -> c_int
{
  /* Initialization of static resources.  */
  if wait_pool.init_flag == 0                                         // C 1129
    {
      wait_pool.timeout_event = CreateEvent (core::ptr::null_mut (), TRUE, FALSE,
					     core::ptr::null ());      // C 1131
      /* If resource initialization fails, exit immediately.  */
      if wait_pool.timeout_event.is_null ()                           // C 1133
	{ return WFO_FAILED as c_int; }                                // C 1134
      /* Set init_flag.  */
      wait_pool.init_flag = TRUE;                                     // C 1136
    }
  /* Check if all threads in the thread pool are working properly.  If
     any thread has exited, exit immediately.  */
  let mut i: c_int = 0;                                               // C 1140
  while i < wait_pool.count
    {
      /* The MS documentation suggests that callers should call the
	 GetExitCodeThread function only after the thread has been
	 confirmed to have exited.  Use the WaitForSingleObject with a
	 wait duration of zero to determine whether a thread has
	 exited.  */
      let res: DWORD = WaitForSingleObject (wait_pool.wocs[i as usize].thread, 0); // C 1147
      /* Thread is alive.  */
      if res == WAIT_TIMEOUT                                          // C 1149
	{ i += 1; continue; }
      /* The thread unexpectedly terminated when waiting
	 wait_start and wait_quit.  */
      else if res == WAIT_OBJECT_0                                    // C 1153
	{
	  /* The last-error code is kept in thread local storage so that
	     multiple threads do not overwrite each other's values.  Pass
	     the error by using SetLastError to set the error value.  */
	  SetLastError (wait_pool.wocs[i as usize].errcode);           // C 1158
	  return WFO_FAILED as c_int;                                  // C 1159
	}
      /* Unexpected return value from WaitForSingleObject.
         WAIT_FAILED or WAIT_ABANDONED.  */
      else
	{ return WFO_FAILED as c_int; }                                // C 1164
    }
  /* Calculate the required number of threads and the number of objects
     the main thread needs to wait for based on the number of objects to
     be waited on.  To avoid increasing the number of threads for a small
     increase in the number of objects (e.g., using two threads for 65
     objects), the following calculation allows the main thread's array
     to hold up to 32 wait objects, excluding the thread event
     handles.  With this approach, the minimum number of wait objects for
     the worker threads is 32, and it allows a maximum of 63 * 32 + 32 =
     2048 objects to be waited on.

     The number of child process in Emacs is limited by MAX_CHILDREN,
     which is half of MAXDESC (FD_SETSIZE).  When FD_SETSIZE is set to
     2048, a maximum of 1024 child processes and network/serial are
     allowed.  Compared to network/serial, the process handles of child
     processes are also waited on, which is why only a maximum of 1024
     child processes can be created.

     When there are 1024 child processes, the main thread needs to wait
     for 64 objects, which seems to exceed the 63 allowed by
     MsgWaitForMultipleObjects.  However, due to the influence of
     emacs_pipe, a maximum of only 1021 child processes can be created,
     so the number of elements in the main thread's wait array will not
     exceed 63.  The explanation is as follows:

     . emacs_pipe calls the pipe2 function located in w32.c.
     . pipe2 makes sure that file descriptors return by _pipe less than
       MAXDESC (2048), Therefore, the range of available file descriptor
       values is from 3 to 2047 (0, 1, 2 for stdin, stdout and
       stderr).  Since pipe file descriptors are opened in pairs, the
       actual range is from 3 to 2046, meaning there are 2044 available
       file descriptors.
     . When create_process creates a process using the 'pipe' method, it
       creates two pipes, one for reading and one for writing.  It then
       closes one file descriptor for each pipe, as each pipe is only
       used for reading or writing.  This results in 4 file descriptors
       being used initially for each process creation, and then 2 are
       released.  When only 2 empty slots remain, no new child processes
       can be created.
     . In the end, we can use 2042 file descriptors, which allows for
       1021 child processes.  */

  /* Compared to using (nCount / 63), we use (nCount - 33) / 63, which
     will cause a new thread to be added only when the number of wait
     objects in the main thread's array exceeds 32.  */
  (*p).nt = (1 + (nCount - 33) / 63) as c_int;                        // C 1210
  /* The number of objects the main thread needs to wait for is the
     required number of threads plus the remaining objects.  If the
     existing threads are sufficient to wait for all the objects, then
     nh is the number of threads.  */
  (*p).nh = if ((*p).nt * 63) as DWORD >= nCount                       // C 1215
	      { (*p).nt }
	    else
	      { ((*p).nt as DWORD + nCount - ((*p).nt * 63) as DWORD) as c_int };
  /* In the main thread's wait array hs, the first nt are thread event
     handles, and the remaining are the remaining objects.  */
  if (*p).nh != (*p).nt                                               // C 1218
    {
      memcpy ((*p).hs.as_mut_ptr ().add ((*p).nt as usize) as *mut c_void, // C 1219-1220
	      lpHandles.add (((*p).nt * 63) as usize) as *const c_void,
	      core::mem::size_of::<HANDLE> ()
	      * (nCount - ((*p).nt * 63) as DWORD) as usize);
    }
  /* Set the pHandles and nCount parameters for each thread.  Since the
     waiting task is relatively simple, we do not need a dedicated
     task queue mechanism.  We can simply wait for the corresponding
     objects in the order of the thread context array wait_pool.wocs.  */
  let mut i: c_int = 0;                                               // C 1225
  while i < (*p).nt
    {
      /* If it is the last thread and the thread is sufficient to wait
	 all the objects, then the count needs to be calculated;
	 otherwise, it will be 63.  */
      let count: c_int = if i == (*p).nt - 1                          // C 1230-1233
			   {
			     if (*p).nt == (*p).nh
			       { (nCount - (i * 63) as DWORD) as c_int }
			     else { 63 }
			   }
			 else { 63 };
      wait_pool.wocs[i as usize].nCount = count;                      // C 1234
      wait_pool.wocs[i as usize].pHandles = lpHandles.add ((i * 63) as usize); // C 1235
      i += 1;
    }
  /* Get current threads count.  */
  let orig_thread_count: c_int = wait_pool.count;                     // C 1238
  /* If the current thread pool is insufficient to wait for all the
     objects, create new threads and initialize the event handles.  */
  while wait_pool.count < (*p).nt                                     // C 1241
    {
      let ctx: *mut wait_objects_context                              // C 1243
	= wait_pool.wocs.as_mut_ptr ().add (wait_pool.count as usize);
      /* Create wait_start, wait_quit and wait_ready events.  */
      (*ctx).wait_start = CreateEvent (core::ptr::null_mut (), FALSE, FALSE,
				       core::ptr::null ());           // C 1245
      if (*ctx).wait_start.is_null ()                                 // C 1246
	{ return WFO_FAILED as c_int; }                                // C 1247
      (*ctx).wait_quit = CreateEvent (core::ptr::null_mut (), FALSE, FALSE,
				      core::ptr::null ());            // C 1248
      if (*ctx).wait_quit.is_null ()                                  // C 1249
	{ return WFO_FAILED as c_int; }                                // C 1250
      /* The wait_ready event is set to be manually reset because it may
	 be waited on multiple times.  */
      (*ctx).wait_ready = CreateEvent (core::ptr::null_mut (), TRUE, FALSE,
				       core::ptr::null ());           // C 1253
      if (*ctx).wait_ready.is_null ()                                 // C 1254
	{ return WFO_FAILED as c_int; }                                // C 1255
      /* Set call_count and errcode to ZERO.  */
      (*ctx).call_count = 0;                                          // C 1257
      (*ctx).errcode = 0;                                             // C 1258
      /* Creating new worker threads.  Please refer to the comment in
	 w32proc.c within the new_child function, where the
	 reader_thread thread is created, to understand why these
	 parameters are needed.  */
      (*ctx).thread = CreateThread (core::ptr::null_mut (), 64 * 1024, // C 1263-1264
				    Some (wait_objects_thread), ctx as *mut c_void,
				    0x00010000, core::ptr::null_mut ());
      /* CreateThread failed.  */
      if (*ctx).thread.is_null ()                                     // C 1266
	{ return WFO_FAILED as c_int; }                                // C 1267
      /* Set Thread Description information.  This may be handy with
	 debugging.  */
      if is_debugger_present.is_some ()                               // C 1270-1271
	  && (is_debugger_present.unwrap ()) () != 0
	  && set_thread_description.is_some ()
	{
	  if (set_thread_description.unwrap ()) ((*ctx).thread,          // C 1273-1274
						 [0x0073u16, 0x0079, 0x0073, 0x005f, 0x0073,
						  0x0065, 0x006c, 0x0065, 0x0063, 0x0074, 0x005f,
						  0x0077, 0x006f, 0x0072, 0x006b, 0x0065, 0x0072,
						  0x005f, 0x0074, 0x0068, 0x0072, 0x0065, 0x0061,
						  0x0064, 0x0000].as_ptr ())
	      != S_OK
	    { return WFO_FAILED as c_int; }                            // C 1275
	}
      wait_pool.count += 1;                                           // C 1277
    }
  /* Fill the main thread's wait array with the wait_ready event
     handles of the required worker threads, and set them to be
     unsignaled.  */
  let mut i: c_int = 0;                                               // C 1282
  while i < (*p).nt
    {
      (*p).hs[i as usize] = wait_pool.wocs[i as usize].wait_ready;     // C 1284
      /* Reset the wait_ready event and set wait_start event for threads
	 that are not newly created.  Newly created threads start
	 execution immediately.  */
      if i < orig_thread_count                                        // C 1288
	{
	  if ResetEvent (wait_pool.wocs[i as usize].wait_ready) != 0     // C 1290-1291
	      && SetEvent (wait_pool.wocs[i as usize].wait_start) != 0
	    { i += 1; continue; }
	  /* SetEvent or ResetEvent failed.  */
	  else
	    { return WFO_FAILED as c_int; }                            // C 1295
	}
      i += 1;
    }
  return 0;                                                           // C 1298
}

/* C 50's `<windows.h>`: `WAIT_ABANDONED`, the base value the `WAIT_ABANDONED_0` above is computed
   from - `<winbase.h>` spells both, and C 1314 compares a `WaitForMultipleObjects` result against
   the base one here.  Like the five values above it is not in any mirror of this unit's graph, so
   it stands at the use site with the `UNVERIFIED` mark the acquisition note gives every Windows
   name on this machine (P-88 ①).  */
/// C: #define WAIT_ABANDONED 0x00000080
#[cfg (feature = "windowsnt")]
pub const WAIT_ABANDONED: DWORD = 0x00000080;

/* C 1301-1511: the pool's four control functions - C 1301-1347's `stop_wait_objects`, C
   1349-1437's `end_wait_and_return`, C 1439-1483's `shrink_wait_pool` and C 1485-1511's
   `free_wait_pool`.  C 1513's `wait_for_objects` opens the next slice.  */

/// C 1301-1347: `static int stop_wait_objects (wait_objects_info *p)` - C 1306-1308's
/// `timeout_event` signal, C 1313's `WaitForMultipleObjects (p->nt, p->hs, TRUE, 20)`, C 1314-1315's
/// `WAIT_FAILED`/`WAIT_ABANDONED` guard, C 1319-1331's timeout branch with its dead-thread scan,
/// C 1336-1342's `errcode` scan and C 1344-1345's `ResetEvent`.  C's three `int` returns of the
/// `WFO_FAILED` macro are `as c_int` (the C 1134 conversion, Deviations 39).
/// C: static int
/// C: stop_wait_objects (wait_objects_info *p)
#[cfg (feature = "windowsnt")]
unsafe fn stop_wait_objects (p: *mut wait_objects_info) -> c_int
{
  /* Ensure that all worker threads have completed their wait and are in
     the ready state.  */
  /* Set timeout_event to tell all worker threads stop waiting.  */      // C 1306
  if SetEvent (wait_pool.timeout_event) == 0                            // C 1307
    { return WFO_FAILED as c_int; }                                     // C 1308
  /* Wait for all the used worker threads to signal wait_ready.  This
     typically takes no more than a few dozen microseconds, so a
     timeout indicates that an error has occurred.  */
  let result: DWORD = WaitForMultipleObjects ((*p).nt, (*p).hs.as_mut_ptr (),    // C 1313
                                              TRUE, 20);
  if result == WAIT_FAILED || result == WAIT_ABANDONED                   // C 1314
    { return WFO_FAILED as c_int; }                                     // C 1315
  /* Timeout means there exists a thread exit abnormally.  Since the
     WAIT_FAILED error in the worker threads signals wait_ready, this
     can only be an error occurring when the worker threads are waiting
     for curr_start and wait_quit.  */
  if result == WAIT_TIMEOUT                                             // C 1319
    {
      /* Find the first dead thread.  */
      let mut i: c_int = 0;                                             // C 1322
      while i < (*p).nt
        {
          if WaitForSingleObject (wait_pool.wocs[i as usize].thread, 0) // C 1324
              == WAIT_OBJECT_0
            {
              SetLastError (wait_pool.wocs[i as usize].errcode);        // C 1326
              return WFO_FAILED as c_int;                               // C 1327
            }
          i += 1;
        }
      return WFO_FAILED as c_int;                                       // C 1330
    }
  /* Even if the wait succeeds, there is still a possibility that some
     wait_ready might be signaled by a WAIT_FAILED error.  To determine
     if no WAIT_FAILED error occurred, check the errcode of all threads.  */
  let mut i: c_int = 0;                                                 // C 1336
  while i < (*p).nt
    {
      if wait_pool.wocs[i as usize].errcode != 0                        // C 1338
        {
          SetLastError (wait_pool.wocs[i as usize].errcode);            // C 1340
          return WFO_FAILED as c_int;                                   // C 1341
        }
      i += 1;
    }
  /* Reset the timeout event.  */
  if ResetEvent (wait_pool.timeout_event) == 0                          // C 1344
    { return WFO_FAILED as c_int; }                                     // C 1345
  return 0;                                                             // C 1346
}

/// C 1349-1437: `static DWORD end_wait_and_return (wait_objects_info *p, DWORD result)` - C
/// 1356-1357's timeout, C 1359-1360's failure, C 1365-1385's `WAIT_ABANDONED` arm (whose two
/// bounds are C's `result >= WAIT_ABANDONED_0 && result < p->nh + WAIT_ABANDONED_0` and whose
/// rewrite adds `62 * p->nt + WFO_ABANDONED`), C 1388-1432's success branch (C 1389's `idx`, C
/// 1391-1405's two main-thread cases, C 1407-1432's worker-thread cases with C's `t_result`
/// comparisons) and C 1434-1435's `stop_wait_objects (&wait_info)` guard.  C's `p->nh + …`,
/// `p->nt + …` and `idx - 1` are `int`, so each is spelled with the `as` conversion C's own
/// arithmetic performs (Deviations 39); C's `result = …` assignments are `DWORD` throughout.
/// C: static DWORD
/// C: end_wait_and_return (wait_objects_info *p, DWORD result)
#[cfg (feature = "windowsnt")]
unsafe fn end_wait_and_return (p: *mut wait_objects_info, mut result: DWORD) -> DWORD
{
  /* Wait timeout in main thread.  */
  if result == WAIT_TIMEOUT                                             // C 1356
    { result = WFO_TIMEOUT; }                                           // C 1357
  /* Wait failed in main thread.  */
  else if result == WAIT_FAILED                                         // C 1359
    { return WFO_FAILED; }                                              // C 1360
  /* It seems that all the wait objects are just process handles and
     event handles.  WAIT_ABANDONED may not occur, but let's just
     handle it here.  */
  else if result >= WAIT_ABANDONED_0                                    // C 1365
           && result < (*p).nh as DWORD + WAIT_ABANDONED_0               // C 1366
    {
      result -= WAIT_ABANDONED_0;                                       // C 1368
      /* The event object wait_ready is not mutex object.  This is
         unlikely to happen...  */
      if result < (*p).nt as DWORD                                      // C 1374
        { return WFO_FAILED; }                                          // C 1375
      /* There are 62 * nt objects before the objects in the main
         thread.  Each thread can wait for 63 objects, and each
         thread's wait_ready occupies one position in the main thread
         array.  Therefore, the index of the waiting object in the main
         thread array should be incremented by (63 - 1) multiplied by
         the number of worker threads.  We add WFO_ABANDONED to
         distinguish it from normal object index.  */
      result = result + (62 * (*p).nt) as DWORD + WFO_ABANDONED;        // C 1384
    }
  /* Wait succeed in main thread.  */
  else
    {
      /* Object index in main thread wait array.  */
      let idx: c_int = (result - WAIT_OBJECT_0) as c_int;               // C 1389
      /* Object is in main thread's wait array.  */
      if idx >= (*p).nt                                                 // C 1391
        {
          /* When the index is equal to the number of worker threads,
             and the number of worker threads is equal to the number of
             wait objects for the main thread, it indicates that the main
             thread is using MsgWaitForMultipleObjects and that there are
             no wait objects in the main thread's array.  We should return
             the total number of wait objects to indicate that a message
             was received during the wait.  */
          if (*p).nt == (*p).nh                                          // C 1399
            { result = ((idx - 1) * 63 + wait_pool.wocs[(idx - 1) as usize].nCount) as DWORD; }
                                                                        // C 1400
          /* Normal case.  */
          else
            { result = (62 * (*p).nt + idx) as DWORD; }                 // C 1403
        }
      /* Object is in worker threads.  */
      else
        {
          let t_result: DWORD = wait_pool.wocs[idx as usize].result;    // C 1407
          /* WAIT_FAILED or WAIT_TIMEOUT.  Since the wait time in the
             worker thread is set to INFINITE, WAIT_TIMEOUT should not
             occur.  */
          if t_result == WAIT_FAILED || t_result == WAIT_TIMEOUT        // C 1412
            {
              SetLastError (wait_pool.wocs[idx as usize].errcode);      // C 1414
              return WFO_FAILED;                                        // C 1415
            }
          /* Abandoned.  Compared to the waiting objects in the main
             thread's array, the index in the worker threads needs to be
             incremented by 63 times the number of preceding threads,
             rather than 62.  */
          else if t_result >= WAIT_ABANDONED_0                          // C 1421
                   && t_result < wait_pool.wocs[idx as usize].nCount as DWORD
                                  + WAIT_ABANDONED_0                    // C 1422
            { result = (idx * 63) as DWORD + t_result + WFO_ABANDONED - WAIT_ABANDONED_0; }
                                                                        // C 1423-1424
          /* The worker thread is signaled by timeout_event, but at this
             point, timeout_event has not yet been signaled.  */
          else if t_result == wait_pool.wocs[idx as usize].nCount as DWORD + WAIT_OBJECT_0
            { return WFO_FAILED; }                                      // C 1429
          /* Normal object index.  */
          else
            { result = (idx * 63) as DWORD + (t_result - WAIT_OBJECT_0); }   // C 1431
        }
    }
  /* Ensure that all the worker threads are ready to begin the next
     round of waiting, and check for any potential errors.  */
  if stop_wait_objects (&raw mut wait_info) == WFO_FAILED as c_int      // C 1434
    { return WFO_FAILED; }                                              // C 1435
  return result;                                                        // C 1436
}

/// C 1439-1483: `static int shrink_wait_pool (void)` - C 1444's `main_thread_waits` bump, C
/// 1447-1448/1451-1452's two early returns, C 1455's `last`, C 1458-1478's retirement of the last
/// worker thread (each `CloseHandle`'s own failure branch and the `NULL` it writes back) and
/// C 1480-1481's `call_count` reset.  C's `wait_pool.count--` is `-= 1` and its `NULL` field
/// writes are `core::ptr::null_mut ()` (the `CreateEvent (core::ptr::null_mut (), …)` spelling of
/// the slice above); C's returns of `WFO_FAILED` are again `as c_int`.
/// C: static int
/// C: shrink_wait_pool (void)
#[cfg (feature = "windowsnt")]
unsafe fn shrink_wait_pool () -> c_int
{
  wait_pool.main_thread_waits += 1;                                     // C 1444
  /* Check the thread every 64 Wait call.  64 is just a value that might
     be reasonably suitable.  */
  if wait_pool.main_thread_waits <= 64                                  // C 1447
    { return 0; }                                                       // C 1448
  wait_pool.main_thread_waits = 0;                                      // C 1449
  /* return if no thread.  */
  if wait_pool.count == 0                                               // C 1451
    { return 0; }                                                       // C 1452
  /* Each time, we only check the last worker thread, which helps avoid
     terminating a large number of threads at the same time.  */
  let last: c_int = wait_pool.count - 1;                                // C 1455
  /* A call count of 0 indicates that the thread has not been used
     during the past period of time.  */
  if wait_pool.wocs[last as usize].call_count == 0                      // C 1458
    {
      /* Signal wait_quit to let the worker thread exit.  */
      if SetEvent (wait_pool.wocs[last as usize].wait_quit) == 0        // C 1461
        { return WFO_FAILED as c_int; }                                 // C 1462
      wait_pool.wocs[last as usize].wait_quit = core::ptr::null_mut (); // C 1463
      /* Close thread handle.  */
      if CloseHandle (wait_pool.wocs[last as usize].thread) == 0        // C 1465
        { return WFO_FAILED as c_int; }                                 // C 1466
      wait_pool.wocs[last as usize].thread = core::ptr::null_mut ();    // C 1467
      /* Close wait_start event handle.  */
      if CloseHandle (wait_pool.wocs[last as usize].wait_start) == 0    // C 1469
        { return WFO_FAILED as c_int; }                                 // C 1470
      wait_pool.wocs[last as usize].wait_start = core::ptr::null_mut ();    // C 1471
      /* Close wait_ready event handle  */
      if CloseHandle (wait_pool.wocs[last as usize].wait_ready) == 0    // C 1473
        { return WFO_FAILED as c_int; }                                 // C 1474
      wait_pool.wocs[last as usize].wait_ready = core::ptr::null_mut ();    // C 1475
      /* decrease the number of workder thread by 1.  */
      wait_pool.count -= 1;                                             // C 1477
    }
  /* Reset call_count for each worker thread.  */
  let mut i: c_int = 0;                                                 // C 1480
  while i <= last
    {
      wait_pool.wocs[i as usize].call_count = 0;
      i += 1;
    }
  return 0;                                                             // C 1482
}

/// C 1485-1511: `void free_wait_pool (void)` - the non-`static` definition `src/w32.c` 10559 calls
/// (its declaration is the mirror's, `include/w32.rs` 578, inside `pub mod w32proc_fn_decls`, so
/// the unit does not repeat it: PORTING.md 0.1-3).  C 1490-1491's `init_flag` guard, C 1492-1500's
/// quit-and-close loop, C 1501-1506's exit wait (C's `for` whose body `break`s is the port's
/// `while`) and C 1509's `timeout_event` close.  C's trailing `return;` is C's own (C 1510).
/// C: void
/// C: free_wait_pool (void)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_wait_pool ()
{
  /* Emacs has never used more than 32 child processes.  */
  if wait_pool.init_flag == FALSE                                       // C 1490
    { return; }                                                         // C 1491
  let mut i: c_int = 0;                                                 // C 1492
  while i < wait_pool.count
    {
      /* Send quit event to earch worker thread.  */
      SetEvent (wait_pool.wocs[i as usize].wait_quit);                  // C 1496
      /* Close wait_start, wait_ready event.  */
      CloseHandle (wait_pool.wocs[i as usize].wait_start);              // C 1498
      CloseHandle (wait_pool.wocs[i as usize].wait_ready);              // C 1499
      i += 1;
    }
  /* Wait all worker threads exit.  Stop if failed.  */
  let mut i: c_int = 0;                                                 // C 1501
  while i < wait_pool.count
    {
      if WaitForSingleObject (wait_pool.wocs[i as usize].thread, 1)     // C 1503-1504
          != WAIT_OBJECT_0
        { break; }                                                      // C 1505
      i += 1;
    }
  /* Close timeout event.  */
  CloseHandle (wait_pool.timeout_event);                                // C 1509
  return;                                                               // C 1510
}
