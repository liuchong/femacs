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

/* C 31's `<errno.h>`: `ESRCH`, `EFAULT` and (slice 15's) `ENOEXEC` are **the mirror's** rows.
   Slice 1 declared the first two here because `femacs/lib/include/errno.rs` did not carry them
   yet; status 3.446/3.514 moved all three back into that mirror (`errno.rs` 112-120, whose own
   note records the same P-88 (5) route for `E2BIG`), so the unit's copies became three `E0428`s
   against it.  Slice 15 removes them and records the reading; the three call sites (C 1318, C
   1368, C 3073) resolve through the mirror.  */
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
   text are inside C 66-69's macro (written out in slice 13) and C 74-75's dropped prototype.  */
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
/* C 57's lisp.h `#include "min-max.h"` (C 26 of that header): the `min!`/`max!` pair, which C
   2501's `max (section->SizeOfRawData, section->Misc.VirtualSize)` reaches through lisp.h - the same
   reading as `<string.h>`'s line above.  `include/lisp.rs` pulls neither mirror, so it is included
   here under the lisp.h line it belongs to (the ledger records the deviation).  */
#[cfg (feature = "windowsnt")]
include!("../lib/include/min-max.rs");            // C 57's lisp.h 26: `#include "min-max.h"` (`max!`)
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

/* C 66-69: `#define RVA_TO_PTR(var,section,filedata) …`, the file's own macro.  Slice 1 carried its
   text because the port had no `IMAGE_SECTION_HEADER` (`include/w32heap.rs` 63-76 declares
   `rva_to_section` with the record as `*mut c_void`, P-72 ③/P-55); slice 12 spelled the record, so
   the macro is written out here, at its C position, and its two users (C 2635, 2644) call it.
   The three operands are C's own: two field reads and `file_data`'s `file_base`, whose pointer
   addition is the `as usize` arithmetic below.  */
/// C: #define RVA_TO_PTR(var,section,filedata) ((void *) ((section)->PointerToRawData
/// C:   + ((DWORD_PTR) (var) - (section)->VirtualAddress) + (filedata).file_base))
#[cfg (feature = "windowsnt")]
macro_rules! RVA_TO_PTR {
  ($var:expr, $section:expr, $filedata:expr) => {
    (((*$section).PointerToRawData as usize
      + (($var) as DWORD_PTR - (*$section).VirtualAddress as DWORD_PTR)
      + ($filedata).file_base as usize) as *mut c_void)
  };
}

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

/* C 50's `<windows.h>`/`<winuser.h>`: the message-aware wait entry point C 1628/1657/1681 calls and
   the wake mask its last argument carries.  `MsgWaitForMultipleObjects` is `<winuser.h>`'s (C
   reaches it through `<windows.h>`), and `QS_ALLINPUT` is that header's `#define QS_ALLINPUT
   0x04FF` - the OR of every input-queue class the SDK names.  No mirror in this unit's graph
   carries either, so both stand at the use site with the `UNVERIFIED` mark the acquisition note
   gives every Windows name on this machine (P-88 ①).  */
/// C: #define QS_ALLINPUT 0x04FF
#[cfg (feature = "windowsnt")]
pub const QS_ALLINPUT: DWORD = 0x04FF;
/// C: WINUSERAPI DWORD WINAPI MsgWaitForMultipleObjects (DWORD nCount, const HANDLE *pHandles,
/// C:   BOOL fWaitAll, DWORD dwMilliseconds, DWORD dwWakeMask);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn MsgWaitForMultipleObjects (nCount: DWORD, pHandles: *const HANDLE, fWaitAll: BOOL,
                                    dwMilliseconds: DWORD, dwWakeMask: DWORD) -> DWORD;
}

/* C 1513-1693: the file's two wait entry points - C 1513-1526's `wait_for_objects` overview
   comment, C 1527-1597's `wait_for_objects`, C 1599-1616's `msg_wait_for_objects` overview comment
   and C 1617-1693's `msg_wait_for_objects`.  C 1695's subprocess overview comment opens the next
   slice.  */

/* Replacement of WaitForMultipleObjects, with the bWaitAll parameter
   removed.  The function's return values are as follows:

   [0 ~ nCount-1], the return value indicates the lpHandles array
   index of the object that satisfied the wait.

   [WFO_ABANDONED ~ nCount-1 + WFO_ABANDONED], the return value minus
   WFO_ABANDONED indicates the lpHandles array index of an abandoned
   mutex object that satisfied the wait.

   [WFO_TIMEOUT], The time-out interval elapsed.

   [WFO_FAILED], The function has failed.  To get extended error
   information, call GetLastError.  */                              // C 1513-1526
/// C 1527-1597: `static DWORD wait_for_objects (DWORD nCount, HANDLE *lpHandles, DWORD
/// dwMilliseconds)` - C 1531-1533's shrink guard, C 1536-1550's `nCount <= 64` fast path (C
/// 1538-1539's `WaitForMultipleObjects`, C 1540-1541's timeout and C 1542-1549's three-range
/// answer), C 1551-1578's busy-wait loop over 64-sized groups (C 1554-1555's `rest`/`group`, C
/// 1556-1557's two loop variables, C 1559's `count` choice, C 1564-1575's per-group answer) and C
/// 1579-1596's multithreaded path (C 1581's `start_wait_objects`, C 1583-1584's
/// `WaitForMultipleObjects (wait_info.nh, wait_info.hs, FALSE, dwMilliseconds)`, C 1585-1590's
/// sixty-four-hertz comment and C 1591-1595's zero-time retry).  C's `int` loop offset scales a
/// `HANDLE *` (`lpHandles.add (…)`) and its `int`/`DWORD` mix is spelled with the `as` conversions
/// C's own arithmetic performs (Deviations 39).
/// C: static DWORD
/// C: wait_for_objects (DWORD nCount, HANDLE *lpHandles, DWORD dwMilliseconds)
#[cfg (feature = "windowsnt")]
unsafe fn wait_for_objects (nCount: DWORD, lpHandles: *mut HANDLE,
                           dwMilliseconds: DWORD) -> DWORD
{
  /* Check inactive worker threads and terminate them.  */
  if shrink_wait_pool () == WFO_FAILED as c_int                        // C 1532
    { return WFO_FAILED; }                                             // C 1533
  /* If the number of wait objects does not exceed 64, directly call
     WaitForMultipleObjects and convert the return value.  */
  if nCount <= 64                                                       // C 1536
    {
      let res: DWORD = WaitForMultipleObjects (nCount, lpHandles, FALSE, dwMilliseconds);    // C 1538-1539
      if res == WAIT_TIMEOUT                                            // C 1540
        { return WFO_TIMEOUT; }                                        // C 1541
      else if res >= WAIT_OBJECT_0                                      // C 1542
               && res < WAIT_OBJECT_0 + nCount                          // C 1543
        { return res - WAIT_OBJECT_0; }                                 // C 1544
      else if res >= WAIT_ABANDONED_0                                   // C 1545
               && res < WAIT_ABANDONED_0 + nCount                       // C 1546
        { return res + WFO_ABANDONED - WAIT_ABANDONED_0; }              // C 1547
      else
        { return WFO_FAILED; }                                          // C 1549
    }
  /* If the wait time is 0, perform busy waiting.  */
  if dwMilliseconds == 0                                                // C 1552
    {
      let rest: c_int = (nCount % 64) as c_int;                         // C 1554
      let group: c_int = (nCount / 64) as c_int;                        // C 1555
      let mut res: DWORD = 0;
      let mut count: DWORD = 0;                                         // C 1556
      let mut i: c_int = 0;
      let mut offset: c_int = 0;                                        // C 1557
      while i <= group
        {
          count = if i == group { rest as DWORD } else { 64 };          // C 1559
          /* When the number of waits is a multiple of 64, skipping the
             last wait with a count of 0.  */
          if count == 0                                                 // C 1562
            { break; }                                                  // C 1563
          res = WaitForMultipleObjects (count, lpHandles.add (offset as usize),    // C 1564-1565
                                        FALSE, 0);
          if res == WAIT_TIMEOUT                                        // C 1566
            { i += 1; offset += 64; continue; }                         // C 1567
          else if res >= WAIT_OBJECT_0                                  // C 1568
                   && res < WAIT_OBJECT_0 + count                        // C 1569
            { return offset as DWORD + res - WAIT_OBJECT_0; }           // C 1570
          else if res >= WAIT_ABANDONED_0                               // C 1571
                   && res < WAIT_ABANDONED_0 + count                     // C 1572
            { return offset as DWORD + res + WFO_ABANDONED - WAIT_ABANDONED_0; }    // C 1573
          else
            { return WFO_FAILED; }                                      // C 1575
        }
      return WFO_TIMEOUT;                                               // C 1577
    }
  /* If the number of objects is greater than 64 and the wait time is
     not 0, use multithreaded waiting.  */
  if start_wait_objects (&raw mut wait_info, nCount, lpHandles)         // C 1581
      == WFO_FAILED as c_int
    { return WFO_FAILED; }                                              // C 1582
  let mut res: DWORD = WaitForMultipleObjects (wait_info.nh, wait_info.hs.as_mut_ptr (),    // C 1583-1584
                                               FALSE, dwMilliseconds);
  /* If the main thread wait times out, call WaitForMultipleObjects
     again with zero time-out interval to check if any objects have
     completed.  The default clock resolution of Windows is 64 Hz.  If a
     time less than 15.625ms is specified, the waiting time may be
     longer than the specified time, and some objects that were not
     completed in the previous call may now be completed.  */
  if res == WAIT_TIMEOUT                                                // C 1591
    {
      res = WaitForMultipleObjects (wait_info.nh, wait_info.hs.as_mut_ptr (),    // C 1593-1594
                                    FALSE, 0);
    }
  return end_wait_and_return (&raw mut wait_info, res);                 // C 1596
}

/* Replacement of MsgWaitForMultipleObjects, with the bWaitAll and
   dwWakeMask parameters removed.  The function's return values are as
   follows:

   [0 ~ nCount-1], the return value indicates the lpHandles array
   index of the object that satisfied the wait.

   [nCount], New input of the type QS_ALLINPUT is available in the
   thread's input queue.

   [WFO_ABANDONED ~ nCount-1 + WFO_ABANDONED], the return value minus
   WFO_ABANDONED indicates the lpHandles array index of an abandoned
   mutex object that satisfied the wait.

   [WFO_TIMEOUT], The time-out interval elapsed.

   [WFO_FAILED], The function has failed.  To get extended error
   information, call GetLastError.  */                              // C 1599-1616
/// C 1617-1693: `static DWORD msg_wait_for_objects (DWORD nCount, HANDLE *lpHandles, DWORD
/// dwMilliseconds)` - the same shape as `wait_for_objects` over `MsgWaitForMultipleObjects`, with
/// C's two differences carried in place: C 1635-1636's `res <= WAIT_OBJECT_0 + nCount` (`<=`,
/// because the call can also report `nCount` itself) and C 1667-1668's `res == WAIT_OBJECT_0 +
/// count` answer, which returns `nCount` for a message rather than the group offset.  C 1621-1623's
/// shrink guard, C 1626-1643's `nCount <= 63` fast path, C 1644-1676's busy-wait loop over
/// 63-sized groups and C 1677-1692's multithreaded path (C 1687-1691's zero-time retry) round it
/// out; the two numbers C waits on are `63` and `QS_ALLINPUT` (the wake mask of every call).
/// C: static DWORD
/// C: msg_wait_for_objects (DWORD nCount, HANDLE *lpHandles, DWORD dwMilliseconds)
#[cfg (feature = "windowsnt")]
unsafe fn msg_wait_for_objects (nCount: DWORD, lpHandles: *mut HANDLE,
                               dwMilliseconds: DWORD) -> DWORD
{
  /* Check inactive worker threads and terminate them.  */
  if shrink_wait_pool () == WFO_FAILED as c_int                        // C 1622
    { return WFO_FAILED; }                                             // C 1623
  /* If the number of wait objects does not exceed 63, directly call
     MsgWaitForMultipleObjects and convert the return value.  */
  if nCount <= 63                                                       // C 1626
    {
      let res: DWORD = MsgWaitForMultipleObjects (nCount, lpHandles, FALSE,    // C 1628-1629
                                                  dwMilliseconds, QS_ALLINPUT);
      if res == WAIT_TIMEOUT                                            // C 1630
        { return WFO_TIMEOUT; }                                        // C 1631
      /* The return value of MsgWaitForMultipleObjects can be
         WAIT_OBJECT_0 + nCount, indicating that a message was
         received.  So use (<=) rather than (<) here.  */
      else if res >= WAIT_OBJECT_0                                      // C 1635
               && res <= WAIT_OBJECT_0 + nCount                         // C 1636
        { return res - WAIT_OBJECT_0; }                                 // C 1637
      else if res >= WAIT_ABANDONED_0                                   // C 1638
               && res < WAIT_ABANDONED_0 + nCount                       // C 1639
        { return res + WFO_ABANDONED - WAIT_ABANDONED_0; }              // C 1640
      else
        { return WFO_FAILED; }                                          // C 1642
    }
  /* If the wait time is 0, perform busy waiting.  */
  if dwMilliseconds == 0                                                // C 1645
    {
      let rest: c_int = (nCount % 63) as c_int;                         // C 1647
      let group: c_int = (nCount / 63) as c_int;                        // C 1648
      let mut res: DWORD = 0;
      let mut count: DWORD = 0;                                         // C 1649
      let mut i: c_int = 0;
      let mut offset: c_int = 0;                                        // C 1650
      while i <= group
        {
          count = if i == group { rest as DWORD } else { 63 };          // C 1652
          /* When the number of waits is a multiple of 63, skipping the
             last wait with a count of 0.  */
          if count == 0                                                 // C 1655
            { break; }                                                  // C 1656
          res = MsgWaitForMultipleObjects (count, lpHandles.add (offset as usize),    // C 1657-1658
                                           FALSE, 0, QS_ALLINPUT);
          if res == WAIT_TIMEOUT                                        // C 1659
            { i += 1; offset += 63; continue; }                         // C 1660
          else if res >= WAIT_OBJECT_0                                  // C 1661
                   && res < WAIT_OBJECT_0 + count                       // C 1662
            { return offset as DWORD + res - WAIT_OBJECT_0; }           // C 1663
          /* When a message is received during the wait, return nCount
             directly.  This is the distinction that needs to be made
             compared to wait_for_objects.  */
          else if res == WAIT_OBJECT_0 + count                          // C 1667
            { return nCount; }                                          // C 1668
          else if res >= WAIT_ABANDONED_0                               // C 1669
                   && res < WAIT_ABANDONED_0 + count                     // C 1670
            { return offset as DWORD + res + WFO_ABANDONED - WAIT_ABANDONED_0; }    // C 1671
          else
            { return WFO_FAILED; }                                      // C 1673
        }
      return WFO_TIMEOUT;                                               // C 1675
    }
  /* If the number of objects is greater than 63 and the wait time is
     not 0, use multithreaded waiting.  */
  if start_wait_objects (&raw mut wait_info, nCount, lpHandles)         // C 1679
      == WFO_FAILED as c_int
    { return WFO_FAILED; }                                              // C 1680
  let mut res: DWORD = MsgWaitForMultipleObjects (wait_info.nh, wait_info.hs.as_mut_ptr (),    // C 1681-1683
                                                 FALSE, dwMilliseconds, QS_ALLINPUT);
  /* If the main thread wait times out, call MsgWaitForMultipleObjects
     again with zero time-out interval to check if any objects have
     completed.  */
  if res == WAIT_TIMEOUT                                                // C 1687
    {
      res = MsgWaitForMultipleObjects (wait_info.nh, wait_info.hs.as_mut_ptr (),   // C 1689-1690
                                       FALSE, 0, QS_ALLINPUT);
    }
  return end_wait_and_return (&raw mut wait_info, res);                 // C 1692
}

/* C 1695-1832: the child-process block's opening - C 1695-1823's overview of how subprocesses and
   network/serial streams are supported on this platform, C 1825-1826's `_P_NOWAIT`, C 1828-1830's
   child-process management list and C 1832's `reader_thread` forward declaration.  `new_child`
   (C 1834) opens the next slice.  */

/* C 1695-1823: the file's overview comment, carried as C's own text at its C position (the reading
   slices 1/2 give every one of this file's banner comments).  It is the map the whole child-process
   block below it is read against: the `child_procs[]` array and its `MAX_CHILDREN` (= 32) bound,
   the `reader_thread`/`sys_select` handshake through `char_avail`/`char_consumed`, the
   `WaitForMultipleObjects` ceiling that bound comes from, the `fd_info[]` conversion and the
   `sys_kill` emulation.  */
/* Here's an overview of how support for subprocesses and
   network/serial streams is implemented on MS-Windows.

   The management of both subprocesses and network/serial streams
   circles around the child_procs[] array, which can record up to the
   grand total of MAX_CHILDREN (= 32) of these.  (The reasons for the
   32 limitation will become clear below.)  Each member of
   child_procs[] is a child_process structure, defined on w32.h.

   A related data structure is the fd_info[] array, which holds twice
   as many members, 64, and records the information about file
   descriptors used for communicating with subprocesses and
   network/serial devices.  Each member of the array is the filedesc
   structure, which records the Windows handle for communications,
   such as the read end of the pipe to a subprocess, a socket handle,
   etc.

   Both these arrays reference each other: there's a member of
   child_process structure that records the corresponding file
   descriptor, and there's a member of filedesc structure that holds a
   pointer to the corresponding child_process.

   Whenever Emacs starts a subprocess or opens a network/serial
   stream, the function new_child is called to prepare a new
   child_process structure.  new_child looks for the first vacant slot
   in the child_procs[] array, initializes it, and starts a "reader
   thread" that will watch the output of the subprocess/stream and its
   status.  (If no vacant slot can be found, new_child returns a
   failure indication to its caller, and the higher-level Emacs
   primitive that called it will then fail with EMFILE or EAGAIN.)

   The reader thread started by new_child communicates with the main
   (a.k.a. "Lisp") thread via two event objects and a status, all of
   them recorded by the members of the child_process structure in
   child_procs[].  The event objects serve as semaphores between the
   reader thread and the 'pselect' emulation in sys_select, as follows:

     . Initially, the reader thread is waiting for the char_consumed
       event to become signaled by sys_select, which is an indication
       for the reader thread to go ahead and try reading more stuff
       from the subprocess/stream.

     . The reader thread then attempts to read by calling a
       blocking-read function.  When the read call returns, either
       successfully or with some failure indication, the reader thread
       updates the status of the read accordingly, and signals the 2nd
       event object, char_avail, on whose handle sys_select is
       waiting.  This tells sys_select that the file descriptor
       allocated for the subprocess or the stream is ready to be
       read from.

   When the subprocess exits or the network/serial stream is closed,
   the reader thread sets the status accordingly and exits.  It also
   exits when the main thread sets the status to STATUS_READ_ERROR
   and/or the char_avail and char_consumed event handles become NULL;
   this is how delete_child, called by Emacs when a subprocess or a
   stream is terminated, terminates the reader thread as part of
   deleting the child_process object.

   The sys_select function emulates the Posix 'pselect' functionality;
   it is needed because the Windows 'select' function supports only
   network sockets, while Emacs expects 'pselect' to work for any file
   descriptor, including pipes and serial streams.

   When sys_select is called, it uses the information in fd_info[]
   array to convert the file descriptors which it was asked to watch
   into Windows handles.  In general, the handle to watch is the
   handle of the char_avail event of the child_process structure that
   corresponds to the file descriptor.  In addition, for subprocesses,
   sys_select watches one more handle: the handle for the subprocess,
   so that it could emulate the SIGCHLD signal when the subprocess
   exits.

   If file descriptor zero (stdin) doesn't have its bit set in the
   'rfds' argument to sys_select, the function always watches for
   keyboard interrupts, to be able to interrupt the wait and return
   when the user presses C-g.

   Having collected the handles to watch, sys_select calls
   WaitForMultipleObjects to wait for any one of them to become
   signaled.  Since WaitForMultipleObjects can only watch up to 64
   handles, Emacs on Windows is limited to maximum 32 child_process
   objects (since a subprocess consumes 2 handles to be watched, see
   above).

   When any of the handles become signaled, sys_select does whatever
   is appropriate for the corresponding child_process object:

     . If it's a handle to the char_avail event, sys_select marks the
       corresponding bit in 'rfds', and Emacs will then read from that
       file descriptor.

     . If it's a handle to the process, sys_select calls the SIGCHLD
       handler, to inform Emacs of the fact that the subprocess
       exited.

   The waitpid emulation works very similar to sys_select, except that
   it only watches handles of subprocesses, and doesn't synchronize
   with the reader thread.

   Because socket descriptors on Windows are handles, while Emacs
   expects them to be file descriptors, all low-level I/O functions,
   such as 'read' and 'write', and all socket operations, like
   'connect', 'recvfrom', 'accept', etc., are redirected to the
   corresponding 'sys_*' functions, which must convert a file
   descriptor to a handle using the fd_info[] array, and then invoke
   the corresponding Windows API on the handle.  Most of these
   redirected 'sys_*' functions are implemented on w32.c.

   When the file descriptor was produced by functions such as 'open',
   the corresponding handle is obtained by calling _get_osfhandle.  To
   produce a file descriptor for a socket handle, which has no file
   descriptor as far as Windows is concerned, the function
   socket_to_fd opens the null device; the resulting file descriptor
   will never be used directly in any I/O API, but serves as an index
   into the fd_info[] array, where the socket handle is stored.  The
   SOCK_HANDLE macro retrieves the handle when given the file
   descriptor.

   The function sys_kill emulates the Posix 'kill' functionality to
   terminate other processes.  It does that by attaching to the
   foreground window of the process and sending a Ctrl-C or Ctrl-BREAK
   signal to the process; if that doesn't work, then it calls
   TerminateProcess to forcibly terminate the process.  Note that this
   only terminates the immediate process whose PID was passed to
   sys_kill; it doesn't terminate the child processes of that process.
   This means, for example, that an Emacs subprocess run through a
   shell might not be killed, because sys_kill will only terminate the
   shell.  (In practice, however, such problems are very rare.)  */

/* C 1825-1826: C's one-line note "Defined in <process.h> which conflicts with the local copy" and
   the `#define _P_NOWAIT 1` under it.  The constant is this file's own (`src/callproc.rs` 339 spells the
   same one for `callproc.c`'s `#ifdef WINDOWSNT` arm, and two units spelling one constant is what
   C's own headers do once per translation unit); the note above it is C's text, and the spelling
   `<process.h>` is C's.  */
/// C: #define _P_NOWAIT 1
#[cfg (feature = "windowsnt")]
pub const _P_NOWAIT: c_int = 1;

/* C 1828-1830: C's "Child process management list." note and the two file-scope objects under it.
   `child_process` is the mirror's record (`include/w32.rs` 326-355, C `src/w32.h` 84-110, which is
   in this unit's graph) and the array's bound is the mirror's `MAX_CHILDREN` (`include/w32.rs` 242,
   C's `MAXDESC/2`); the record is not re-spelled here (PORTING.md 0.1-3), and both objects are
   statics C leaves zero-initialised, which Rust spells `core::mem::zeroed ()` exactly as the
   `wait_pool`/`wait_info` pair above does.  */
/// C: static int child_proc_count = 0;
#[cfg (feature = "windowsnt")]
static mut child_proc_count: c_int = 0;
/// C: static child_process child_procs[ MAX_CHILDREN ];
#[cfg (feature = "windowsnt")]
static mut child_procs: [child_process; MAX_CHILDREN as usize] = unsafe { core::mem::zeroed () };

/* C 1832: `static DWORD WINAPI reader_thread (void *arg);` - dropped (PORTING.md 0.1-12: C's
   forward declaration has no Rust counterpart, and the definition is C 2031, past this slice).
   `new_child` (C 1834) and its "Find an unused process slot." comment open the next slice.  */

/* C 50's `<windows.h>`/`<winbase.h>` and the CRT's `<io.h>`: the two names this slice's child-process
   functions need beyond the blocks above.  `GetExitCodeProcess` is `<winbase.h>`'s (C 1855/1953
   call it), `_close` is the CRT's underscore spelling (`<io.h>` has no file in the reference tree,
   P-88 ①, and the unit's own `<unistd.h>` note at C 35 records why `lib/include/unistd.in.rs` is
   *not* pulled in), and `ULONG_PTR` is the pointer-sized SDK type `OVERLAPPED`'s first two members
   are - `src/w32.rs` 470-474, `src/w32fns.rs` 466 and `src/w32menu.rs` 185 each spell it at their
   own root, so this unit does too.  Every one is **UNVERIFIED** (no Windows SDK here).  */
/// C: typedef ULONG_PTR DWORD_PTR;   [`<winnt.h>`; `usize`, PORTING.md 1.1's pointer rule]
#[cfg (feature = "windowsnt")]
pub type ULONG_PTR = usize;
/// C: WINBASEAPI BOOL WINAPI GetExitCodeProcess (HANDLE hProcess, LPDWORD lpExitCode);
/// C: _CRTIMP int __cdecl _close (int _FileHandle);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetExitCodeProcess (hProcess: HANDLE, lpExitCode: LPDWORD) -> BOOL;
}
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _close (fd: c_int) -> c_int;
}

/* C 74-113's subprocess record, which the `include/w32.rs` mirror *defers* (its note 34-42: C 103's
   `PROCESS_INFORMATION` and C 107-108's two `OVERLAPPED` records are members **by value** and come
   from `<windows.h>`, which has no mirror on this machine, so the mirror keeps the pointers to the
   record at C's pointer depth as `*mut c_void` and leaves the record itself unchecked).  This unit
   is a reader of the record's *members* (`new_child`, `delete_child` and `reader_thread` read
   `procinfo.hProcess`/`hThread`/`dwProcessId`), so it spells the record and the three SDK records
   it embeds - exactly the device `src/w32.rs` 11847-11916 documents for the same record (that unit
   was its first reader), under the same `windowsnt` gate: the mirror's own copy is the *dead* arm's
   text under `feature = "windows"` (its note 319-323), so the two never coexist in one crate.  C's
   member names, order and types are kept; C's per-member prose (w32.h 76-112) is not carried, as in
   the mirror's deferred note.  Every record below is **UNVERIFIED** (P-88 ①'s class).  */
/// C: typedef struct _PROCESS_INFORMATION { HANDLE hProcess; HANDLE hThread; DWORD dwProcessId;
/// C:   DWORD dwThreadId; } PROCESS_INFORMATION;   [`<winbase.h>`; 24 bytes]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct PROCESS_INFORMATION
{
  pub hProcess: HANDLE,
  pub hThread: HANDLE,
  pub dwProcessId: DWORD,
  pub dwThreadId: DWORD,
}
/// C: typedef struct _OVERLAPPED { ULONG_PTR Internal; ULONG_PTR InternalHigh; union { struct {
/// C:   DWORD Offset; DWORD OffsetHigh; }; PVOID Pointer; }; HANDLE hEvent; } OVERLAPPED;
/// [`<winbase.h>`; 32 bytes.  C 107-108's union and its inner struct are anonymous, so the port
/// names them after the container (P-67, the `src/w32.rs` 11870-11890 spelling).]
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
/// C: w32.h 74-113: `typedef struct _child_process { int fd; int pid; HANDLE char_avail; HANDLE
/// C:   char_consumed; HANDLE thrd; HWND hwnd; PROCESS_INFORMATION procinfo; int status; int
/// C:   errcode; char chr; OVERLAPPED ovl_read; OVERLAPPED ovl_write; } child_process;`
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

/* C 1834-2115: the child-process block's five working functions - C 1834-1933's `new_child`, C
   1935-1998's `delete_child`, C 2000-2011's `find_child_pid`, C 2013-2024's
   `release_listen_threads` and C 2026-2115's `reader_thread`.  C 2117-2121's `process_dir` comment
   and declaration open the next slice.  */

/* Find an unused process slot.  */                               // C 1834
/// C 1834-1933: `child_process * new_child (void)` - C 1841-1843's descending scan for a free slot,
/// C 1844-1885's dead-process sweep (C 1849/1857/1864's three `DebPrint`s, C 1855-1860's
/// `GetExitCodeProcess`/`GetLastError` pair and C 1861-1862's `STILL_ACTIVE`/`WaitForSingleObject`
/// test), C 1886-1888's `MAX_CHILDREN` guard and slot grab, C 1890-1901's `Initialize` tail (C
/// 1893-1896's two handle closes, C 1897's `memset`, C 1898-1901's four zeroed members), and C
/// 1903-1932's event/thread creation with C 1910-1924's stack-flag comment and C 1931's
/// `delete_child` failure path.  C's two `goto Initialize` jumps (C 1843, C 1883) are the port's
/// `break 'initialize` out of a labeled block - a forward jump into the tail, which `match`/`loop`
/// cannot express (the `doprnt` tail-flag precedent, PORTING.md 2.2) - and C's `cp--` pointer walk
/// is `cp.sub (1)` (a `*mut child_process`, the unit's own record).
/// C: child_process *
/// C: new_child (void)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_child () -> *mut child_process
{
  let mut cp: *mut child_process = core::ptr::null_mut ();     // C 1838
  let mut id: DWORD = 0;                                       // C 1839

  'initialize: {
    let base: *mut child_process = child_procs.as_mut_ptr ();
    cp = base.add ((child_proc_count - 1) as usize);           // C 1841
    while cp >= base
      {
        if !CHILD_ACTIVE! (*cp) && (*cp).procinfo.hProcess.is_null ()    // C 1842
          { break 'initialize; }                               // C 1843
        cp = cp.sub (1);
      }
    if child_proc_count == MAX_CHILDREN                        // C 1844
      {
        let mut i: c_int = 0;                                  // C 1846
        let mut dead_cp: *mut child_process = core::ptr::null_mut ();   // C 1847

        DebPrint! (("new_child: No vacant slots, looking for dead processes\n"));   // C 1849
        cp = base.add ((child_proc_count - 1) as usize);       // C 1850
        while cp >= base
          {
            if !CHILD_ACTIVE! (*cp) && !(*cp).procinfo.hProcess.is_null ()    // C 1851
              {
                let mut status: DWORD = 0;                     // C 1853

                if GetExitCodeProcess ((*cp).procinfo.hProcess, &raw mut status) == 0   // C 1855
                  {
                    DebPrint! (("new_child.GetExitCodeProcess: error %lu for PID %lu\n",
                                GetLastError (), (*cp).procinfo.dwProcessId));   // C 1857-1858
                    status = STILL_ACTIVE;                     // C 1859
                  }
                if status != STILL_ACTIVE                      // C 1861
                    || WaitForSingleObject ((*cp).procinfo.hProcess, 0) == WAIT_OBJECT_0   // C 1862
                  {
                    DebPrint! (("new_child: Freeing slot of dead process %d, fd %d\n",
                                (*cp).procinfo.dwProcessId, (*cp).fd));   // C 1864-1865
                    CloseHandle ((*cp).procinfo.hProcess);     // C 1866
                    (*cp).procinfo.hProcess = core::ptr::null_mut ();   // C 1867
                    CloseHandle ((*cp).procinfo.hThread);      // C 1868
                    (*cp).procinfo.hThread = core::ptr::null_mut ();    // C 1869
                    /* Free up to 2 dead slots at a time, so that if we
                       have a lot of them, they will eventually all be
                       freed when the tornado ends.  */
                    if i == 0                                  // C 1873
                      { dead_cp = cp; }
                    else
                      { break; }                               // C 1876
                    i += 1;                                    // C 1877
                  }
              }
            cp = cp.sub (1);
          }
        if !dead_cp.is_null ()                                 // C 1880
          {
            cp = dead_cp;                                      // C 1882
            break 'initialize;                                 // C 1883
          }
      }
  }
  if child_proc_count == MAX_CHILDREN                          // C 1886
    { return core::ptr::null_mut (); }                         // C 1887
  cp = child_procs.as_mut_ptr ().add (child_proc_count as usize);   // C 1888
  child_proc_count += 1;

  /* C 1890's `Initialize:` label.  */
  /* Last opportunity to avoid leaking handles before we forget them
     for good.  */
  if !(*cp).procinfo.hProcess.is_null ()                       // C 1893
    { CloseHandle ((*cp).procinfo.hProcess); }                 // C 1894
  if !(*cp).procinfo.hThread.is_null ()                        // C 1895
    { CloseHandle ((*cp).procinfo.hThread); }                  // C 1896
  memset (cp as *mut c_void, 0, core::mem::size_of::<child_process> ());   // C 1897
  (*cp).fd = -1;                                               // C 1898
  (*cp).pid = -1;                                              // C 1899
  (*cp).procinfo.hProcess = core::ptr::null_mut ();            // C 1900
  (*cp).status = STATUS_READ_ERROR;                            // C 1901

  /* use manual reset event so that select() will function properly */
  (*cp).char_avail = CreateEvent (core::ptr::null_mut (), TRUE, FALSE,
                                  core::ptr::null ());         // C 1904
  if !(*cp).char_avail.is_null ()                              // C 1905
    {
      (*cp).char_consumed = CreateEvent (core::ptr::null_mut (), FALSE, FALSE,
                                         core::ptr::null ());  // C 1907
      if !(*cp).char_consumed.is_null ()                       // C 1908
        {
          /* The 0x00010000 flag is STACK_SIZE_PARAM_IS_A_RESERVATION.
             It means that the 64K stack we are requesting in the 2nd
             argument is how much memory should be reserved for the
             stack.  If we don't use this flag, the memory requested
             by the 2nd argument is the amount actually _committed_,
             but Windows reserves 8MB of memory for each thread's
             stack.  (The 8MB figure comes from the -stack
             command-line argument we pass to the linker when building
             Emacs, but that's because we need a large stack for
             Emacs's main thread.)  The 8MB reservation for each thread
             might causes failures in starting subprocesses, especially
             in 32-bit builds, because we create a thread running
             reader_thread for each subprocess.  As 8MB of stack is
             way too much for reader_thread, forcing Windows to
             reserve less wins the day.  */                    // C 1910-1924
          (*cp).thrd = CreateThread (core::ptr::null_mut (), 64 * 1024, Some (reader_thread),
                                     cp as *mut c_void, 0x00010000, &raw mut id);   // C 1925-1926
          if !(*cp).thrd.is_null ()                            // C 1927
            { return cp; }                                     // C 1928
        }
    }
  delete_child (cp);                                           // C 1931
  return core::ptr::null_mut ();                               // C 1932
}

/// C 1935-1998: `void delete_child (child_process *cp)` - C 1941-1943's `fd_info` guard (C's
/// `fd_info[i].cp == cp` against the mirror's deferred `*mut c_void` member, cast as the mirror's
/// note spells it), C 1945-1946's inactive guard, C 1948-1973's thread reap (C 1953's
/// `GetExitCodeThread`/`STILL_ACTIVE` test, C 1956-1957's status-and-wake pair, C 1958-1969's
/// `#if 0` region carried as C's own text, and C 1971-1972's close), C 1974-1983's two event
/// closes, C 1985-1997's `child_proc_count` rewind.  C's `MAXDESC`, `FILE_LISTEN`-free body and the
/// `int` loop variable are unchanged; the `#if 0` region is the file's usual reading (a region no
/// configuration compiles is carried as C's text, P-51).
/// C: void
/// C: delete_child (child_process *cp)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_child (cp: *mut child_process)
{
  let mut i: c_int;                                            // C 1938

  /* Should not be deleting a child that is still needed.  */
  i = 0;                                                       // C 1941
  while i < MAXDESC
    {
      if fd_info[i as usize].cp == cp as *mut c_void           // C 1942
        { emacs_abort (); }                                    // C 1943
      i += 1;
    }

  if !CHILD_ACTIVE! (*cp) && (*cp).procinfo.hProcess.is_null ()     // C 1945
    { return; }                                                // C 1946

  /* reap thread if necessary */
  if !(*cp).thrd.is_null ()                                    // C 1949
    {
      let mut rc: DWORD = 0;                                   // C 1951

      if GetExitCodeThread ((*cp).thrd, &raw mut rc) != 0 && rc == STILL_ACTIVE   // C 1953
        {
          /* let the thread exit cleanly if possible */
          (*cp).status = STATUS_READ_ERROR;                    // C 1956
          SetEvent ((*cp).char_consumed);                      // C 1957
          /* C 1958-1969: the file's `#if 0` region - the forcibly-terminate arm C no longer
             compiles, carried as C's own text (P-51).  `TerminateThread` is declared in the
             `<windows.h>` block above and is reachable only from this text.  */
          /*
#if 0
          /* We used to forcibly terminate the thread here, but it
             is normally unnecessary, and in abnormal cases, the worst that
             will happen is we have an extra idle thread hanging around
             waiting for the zombie process.  */
	  if (WaitForSingleObject (cp->thrd, 1000) != WAIT_OBJECT_0)
	    {
	      DebPrint (("delete_child.WaitForSingleObject (thread) failed "
			 "with %lu for fd %ld\n", GetLastError (), cp->fd));
	      TerminateThread (cp->thrd, 0);
	    }
#endif
          */                                                  // C 1958-1969
        }
      CloseHandle ((*cp).thrd);                                // C 1971
      (*cp).thrd = core::ptr::null_mut ();                     // C 1972
    }
  if !(*cp).char_avail.is_null ()                              // C 1974
    {
      CloseHandle ((*cp).char_avail);                          // C 1976
      (*cp).char_avail = core::ptr::null_mut ();               // C 1977
    }
  if !(*cp).char_consumed.is_null ()                           // C 1979
    {
      CloseHandle ((*cp).char_consumed);                       // C 1981
      (*cp).char_consumed = core::ptr::null_mut ();            // C 1982
    }

  /* update child_proc_count (highest numbered slot in use plus one) */
  if cp == child_procs.as_mut_ptr ().add ((child_proc_count - 1) as usize)   // C 1986
    {
      i = child_proc_count - 1;                                // C 1988
      while i >= 0
        {
          if CHILD_ACTIVE! (child_procs[i as usize])           // C 1989
              || !child_procs[i as usize].procinfo.hProcess.is_null ()   // C 1990
            {
              child_proc_count = i + 1;                        // C 1992
              break;                                           // C 1993
            }
          i -= 1;
        }
    }
  if i < 0                                                     // C 1996
    { child_proc_count = 0; }                                  // C 1997
}

/* Find a child by pid.  */                                    // C 2000
/// C 2000-2011: `static child_process * find_child_pid (DWORD pid)` - the descending scan C 2006
/// runs against `CHILD_ACTIVE!` and the live-process test, returning the record or `NULL`.
/// C: static child_process *
/// C: find_child_pid (DWORD pid)
#[cfg (feature = "windowsnt")]
unsafe fn find_child_pid (pid: DWORD) -> *mut child_process
{
  let base: *mut child_process = child_procs.as_mut_ptr ();
  let mut cp: *mut child_process = base.add ((child_proc_count - 1) as usize);   // C 2006
  while cp >= base
    {
      if (CHILD_ACTIVE! (*cp) || !(*cp).procinfo.hProcess.is_null ())    // C 2007
          && pid == (*cp).pid as DWORD                                   // C 2008
        { return cp; }                                       // C 2009
      cp = cp.sub (1);
    }
  return core::ptr::null_mut ();                             // C 2010
}

/// C 2013-2024: `void release_listen_threads (void)` - the descending scan that marks every
/// listening descriptor's record with `STATUS_READ_ERROR` (C 2020-2022's `CHILD_ACTIVE!` and
/// `FILE_LISTEN` test).  C's `fd_info[...].flags & FILE_LISTEN` keeps the mirror's `c_uint` member
/// as C's `unsigned` and the mask as the mirror's `c_int` constant (the same spelling the other
/// reader, `src/process.rs` 7492, uses).
/// C: void
/// C: release_listen_threads (void)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn release_listen_threads ()
{
  let mut i: c_int = child_proc_count - 1;                   // C 2018
  while i >= 0
    {
      if CHILD_ACTIVE! (child_procs[i as usize])              // C 2020
          && (fd_info[child_procs[i as usize].fd as usize].flags & FILE_LISTEN as c_uint) != 0
        { child_procs[i as usize].status = STATUS_READ_ERROR; }    // C 2022
      i -= 1;
    }
}

/* Thread proc for child process and socket reader threads.  Each thread
   is normally blocked until woken by select() to check for input by
   reading one char.  When the read completes, char_avail is signaled
   to wake up the select emulator and the thread blocks itself again.  */    // C 2026-2029
/// C 2030-2115: `static DWORD WINAPI reader_thread (void *arg)` - C 2037's cast of the thread
/// argument, C 2040-2043's three-way start guard, C 2045-2095's `for (;;)` (C 2049-2055's
/// descriptor dispatch on `FILE_CONNECT`/`FILE_LISTEN`/read-ahead, C 2059-2060's and C 2081-2082's
/// `status`/handle exits, C 2064-2070's `SetEvent` with its `DebPrint`, C 2072-2077's two
/// status returns, C 2085-2090's acknowledge wait and C 2093-2094's exit check) and C 2096-2113's
/// post-loop close of a `FILE_DONT_CLOSE` descriptor (five tries, `Sleep (5)` between them),
/// ending at C 2114's `return 0`.  C's `fd` starts unset (the port writes C's own zero value,
/// PORTING.md 0.1-13); the thread argument keeps C's `void *` and is cast to the record, the
/// `new_child` call site being the only producer.
/// C: static DWORD WINAPI
/// C: reader_thread (void *arg)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn reader_thread (arg: *mut c_void) -> DWORD
{
  let mut cp: *mut child_process;                             // C 2033
  let mut fd: c_int = 0;                                      // C 2034

  /* Our identity */
  cp = arg as *mut child_process;                             // C 2037

  /* We have to wait for the go-ahead before we can start */
  if cp.is_null ()                                            // C 2040
      || WaitForSingleObject ((*cp).char_consumed, INFINITE) != WAIT_OBJECT_0   // C 2041
      || (*cp).fd < 0                                         // C 2042
    { return 1; }                                             // C 2043

  loop
    {
      let mut rc: c_int;                                      // C 2047

      fd = (*cp).fd;                                          // C 2049
      if fd >= 0 && (fd_info[fd as usize].flags & FILE_CONNECT as c_uint) != 0   // C 2050
        { rc = _sys_wait_connect (fd); }                      // C 2051
      else if fd >= 0 && (fd_info[fd as usize].flags & FILE_LISTEN as c_uint) != 0   // C 2052
        { rc = _sys_wait_accept (fd); }                       // C 2053
      else
        { rc = _sys_read_ahead (fd); }                        // C 2055

      /* Don't bother waiting for the event if we already have been
         told to exit by delete_child.  */
      if (*cp).status == STATUS_READ_ERROR || (*cp).char_avail.is_null ()   // C 2059
        { break; }                                            // C 2060

      /* The name char_avail is a misnomer - it really just means the
         read-ahead has completed, whether successfully or not.  */
      if SetEvent ((*cp).char_avail) == 0                     // C 2064
        {
          DebPrint! (("reader_thread.SetEvent(0x%x) failed with %lu for fd %ld (PID %d)\n",
                      (*cp).char_avail as DWORD_PTR, GetLastError (), fd, (*cp).pid));   // C 2066-2068
          return 1;                                           // C 2069
        }

      if rc == STATUS_READ_ERROR || rc == STATUS_CONNECT_FAILED   // C 2072
        { return 2; }                                         // C 2073

      /* If the read died, the child has died so let the thread die */
      if rc == STATUS_READ_FAILED                             // C 2076
        { break; }                                            // C 2077

      /* Don't bother waiting for the acknowledge if we already have
         been told to exit by delete_child.  */
      if (*cp).status == STATUS_READ_ERROR || (*cp).char_consumed.is_null ()   // C 2081
        { break; }                                            // C 2082

      /* Wait until our input is acknowledged before reading again */
      if WaitForSingleObject ((*cp).char_consumed, INFINITE) != WAIT_OBJECT_0   // C 2085
        {
          DebPrint! (("reader_thread.WaitForSingleObject failed with "
                      "%lu for fd %ld\n", GetLastError (), (*cp).fd));   // C 2087-2088
          break;                                              // C 2089
        }
      /* delete_child sets status to STATUS_READ_ERROR when it wants
         us to exit.  */
      if (*cp).status == STATUS_READ_ERROR                    // C 2093
        { break; }                                            // C 2094
    }
  /* If this thread was reading from a pipe process, close the
     descriptor used for reading, as sys_close doesn't in that case.  */
  if (fd_info[fd as usize].flags & FILE_DONT_CLOSE as c_uint) == FILE_DONT_CLOSE as c_uint   // C 2098
    {
      let mut i: c_int;                                       // C 2100
      /* If w32.c:sys_close is still processing this descriptor, wait
         for a while for it to finish.  */
      i = 0;                                                  // C 2103
      while i < 5
        {
          if fd_info[fd as usize].flags == FILE_DONT_CLOSE as c_uint   // C 2105
            {
              fd_info[fd as usize].flags = 0;                 // C 2107
              _close (fd);                                    // C 2108
              break;                                          // C 2109
            }
          Sleep (5);                                          // C 2111
          i += 1;
        }
    }
  return 0;                                                   // C 2114
}

/* C 2117-2121: `process_dir`, the directory the next child is to start in - the object
   `create_child` (C 2123) reads and `set_process_dir` (C 3903-3907) writes, both past this slice.
   C's four-line comment is carried as C's own text at its C position (the reading slices 1/2/6 give
   this file's comments), and the object is a file-scope `static char *` C leaves NULL-initialised,
   which the port spells `core::ptr::null_mut ()` in the `static mut` shape of slice 6's list.  */
/* To avoid Emacs changing directory, we just record here the
   directory the new process should start in.  This is set just before
   calling sys_spawnve, and is not generally valid at any other time.
   Note that this directory's name is UTF-8 encoded.  */
/// C: static char * process_dir;
#[cfg (feature = "windowsnt")]
static mut process_dir: *mut c_char = core::ptr::null_mut ();

/* C 2123's `create_child` (the `static BOOL` whose record block needs `<windows.h>`'s
   `STARTUPINFO`/`SECURITY_ATTRIBUTES` and `CreateProcessA`) opens the next slice.  */

/* C 50's `<windows.h>`/`<winbase.h>`/`<winuser.h>`: the process-creation surface C 2123-2189 names,
   none of which any mirror in this unit's graph carries (the same reading as the blocks above; no
   Windows SDK on this machine, so every record and constant is UNVERIFIED, P-88 ①).  `STARTUPINFO`
   and `SECURITY_ATTRIBUTES` are the two records C 2127-2128 place by value, `MAX_PATH` bounds C
   2133's `dir`, the three `STARTF_*`/`SW_HIDE`/`STD_*` values are the `HAVE_NTGUI` arm's, and the
   three `CREATE_*` flags are C 2183-2187's.  C 2188's `CreateProcessA`, C 2149-2151's
   `GetStdHandle` and C 2170's `CharNextA` are the three entry points; `xstrcasecmp` is C
   `dispextern.h` 104's inline (the `src/xfns.rs` 10563 / `src/fontset.rs` 108 device for a unit
   whose include graph has no `dispextern.rs` - this file names `dispextern.h` nowhere).  */
/// C: #define MAX_PATH 260
/// C: typedef struct _STARTUPINFOA { DWORD cb; LPSTR lpReserved; LPSTR lpDesktop; LPSTR lpTitle;
/// C:   DWORD dwX; DWORD dwY; DWORD dwXSize; DWORD dwYSize; DWORD dwXCountChars;
/// C:   DWORD dwYCountChars; DWORD dwFillAttribute; DWORD dwFlags; WORD wShowWindow;
/// C:   WORD cbReserved2; LPBYTE lpReserved2; HANDLE hStdInput; HANDLE hStdOutput;
/// C:   HANDLE hStdError; } STARTUPINFO, *LPSTARTUPINFO;   [`<winbase.h>`; 104 bytes]
/// C: typedef struct _SECURITY_ATTRIBUTES { DWORD nLength; LPVOID lpSecurityDescriptor;
/// C:   BOOL bInheritHandle; } SECURITY_ATTRIBUTES, *LPSECURITY_ATTRIBUTES;   [`<winbase.h>`]
/// C: #define STARTF_USESHOWWINDOW 0x00000001
/// C: #define STARTF_USESTDHANDLES 0x00000100
/// C: #define SW_HIDE 0
/// C: #define STD_INPUT_HANDLE ((DWORD) -10)
/// C: #define STD_OUTPUT_HANDLE ((DWORD) -11)
/// C: #define STD_ERROR_HANDLE ((DWORD) -12)
/// C: #define CREATE_NEW_CONSOLE 0x00000010
/// C: #define CREATE_NEW_PROCESS_GROUP 0x00000200
/// C: #define CREATE_DEFAULT_ERROR_MODE 0x04000000
#[cfg (feature = "windowsnt")]
pub const MAX_PATH: c_int = 260;
#[cfg (feature = "windowsnt")]
#[repr(C)]
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
  pub lpReserved2: LPBYTE,
  pub hStdInput: HANDLE,
  pub hStdOutput: HANDLE,
  pub hStdError: HANDLE,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct SECURITY_ATTRIBUTES
{
  pub nLength: DWORD,
  pub lpSecurityDescriptor: LPVOID,
  pub bInheritHandle: BOOL,
}
#[cfg (feature = "windowsnt")]
pub const STARTF_USESHOWWINDOW: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const STARTF_USESTDHANDLES: DWORD = 0x00000100;
#[cfg (feature = "windowsnt")]
pub const SW_HIDE: WORD = 0;
#[cfg (feature = "windowsnt")]
pub const STD_INPUT_HANDLE: DWORD = -10i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const STD_ERROR_HANDLE: DWORD = -12i32 as DWORD;
#[cfg (feature = "windowsnt")]
pub const CREATE_NEW_CONSOLE: DWORD = 0x00000010;
#[cfg (feature = "windowsnt")]
pub const CREATE_NEW_PROCESS_GROUP: DWORD = 0x00000200;
#[cfg (feature = "windowsnt")]
pub const CREATE_DEFAULT_ERROR_MODE: DWORD = 0x04000000;
/// C: WINBASEAPI BOOL WINAPI CreateProcessA (LPCSTR lpApplicationName, LPSTR lpCommandLine,
/// C:   LPSECURITY_ATTRIBUTES lpProcessAttributes, LPSECURITY_ATTRIBUTES lpThreadAttributes,
/// C:   BOOL bInheritHandles, DWORD dwCreationFlags, LPVOID lpEnvironment,
/// C:   LPCSTR lpCurrentDirectory, LPSTARTUPINFOA lpStartupInfo,
/// C:   LPPROCESS_INFORMATION lpProcessInformation);
/// C: WINBASEAPI HANDLE WINAPI GetStdHandle (DWORD nStdHandle);
/// C: WINUSERAPI LPSTR WINAPI CharNextA (LPCSTR lpsz);
/// C: extern int xstrcasecmp (const char *, const char *);   [dispextern.h 104]
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn CreateProcessA (lpApplicationName: LPCSTR, lpCommandLine: LPSTR,
                         lpProcessAttributes: *mut SECURITY_ATTRIBUTES,
                         lpThreadAttributes: *mut SECURITY_ATTRIBUTES, bInheritHandles: BOOL,
                         dwCreationFlags: DWORD, lpEnvironment: LPVOID,
                         lpCurrentDirectory: LPCSTR, lpStartupInfo: *mut STARTUPINFO,
                         lpProcessInformation: *mut PROCESS_INFORMATION) -> BOOL;
  pub fn GetStdHandle (nStdHandle: DWORD) -> HANDLE;
  pub fn CharNextA (lpsz: LPCSTR) -> LPSTR;
}
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int;
}

/* C 2123-2205: `create_child` - the one place this file starts a process.  C's live body keeps its
   order: the C 2137 abort guard, C 2139-2140's zeroed `STARTUPINFO` and its size, C 2142-2152's
   `HAVE_NTGUI` arm (the two `dwFlags` spellings, `SW_HIDE` and the three standard handles), C
   2154-2160's `#if 0` security region carried as C's text (P-51), C 2161-2163's
   `SECURITY_ATTRIBUTES`, C 2165-2172's `filename_to_ansi`/`CharNextA` path fix-up, C 2174-2181's
   batch-file `exe` clearing, C 2183-2187's three flags, C 2188-2190's `CreateProcessA`, C
   2192-2199's pid fix-up and C 2200's `TRUE`, with C 2202-2204's `EH_Fail:` tail after it (both
   `goto EH_Fail`s inside the carried region are dead text; C 2190's is the live one, and the port
   spells the forward jump as `break 'eh_fail` - Deviations 48's device).
   C 2129-2131's second `#if 0` declares `SECURITY_DESCRIPTOR sec_desc`; it is carried as C's text
   with the other region, so the identifier appears in this file only inside that text.  */
/// C: static BOOL
/// C: create_child (char *exe, char *cmdline, char *env, int is_gui_app, pid_t * pPid,
/// C:              child_process *cp)
#[cfg (feature = "windowsnt")]
unsafe fn create_child (exe: *mut c_char, cmdline: *mut c_char, env: *mut c_char,
                        is_gui_app: c_int, pPid: *mut pid_t,
                        cp: *mut child_process) -> BOOL
{
  let mut start: STARTUPINFO = core::mem::zeroed ();           // C 2127
  let mut sec_attrs: SECURITY_ATTRIBUTES = core::mem::zeroed ();   // C 2128
/* C 2129-2131:
#if 0
  SECURITY_DESCRIPTOR sec_desc;
#endif
*/
  let mut flags: DWORD = 0;                                    // C 2132
  let mut dir: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   // C 2133
  let mut p: *mut c_char = core::ptr::null_mut ();             // C 2134
  let mut ext: *const c_char = core::ptr::null ();             // C 2135

  if cp.is_null () { emacs_abort (); }                         // C 2137

  memset (&raw mut start as *mut c_void, 0, core::mem::size_of::<STARTUPINFO> ());   // C 2139
  start.cb = core::mem::size_of::<STARTUPINFO> () as DWORD;    // C 2140

  #[cfg (feature = "have_ntgui")]
  {
    if NILP (Vw32_start_process_show_window) && is_gui_app == 0   // C 2143
      { start.dwFlags = STARTF_USESTDHANDLES | STARTF_USESHOWWINDOW; }   // C 2144
    else
      { start.dwFlags = STARTF_USESTDHANDLES; }                // C 2146
    start.wShowWindow = SW_HIDE;                               // C 2147

    start.hStdInput = GetStdHandle (STD_INPUT_HANDLE);         // C 2149
    start.hStdOutput = GetStdHandle (STD_OUTPUT_HANDLE);       // C 2150
    start.hStdError = GetStdHandle (STD_ERROR_HANDLE);         // C 2151
  }

/* C 2154-2160:
#if 0
  /* Explicitly specify no security */
  if (!InitializeSecurityDescriptor (&sec_desc, SECURITY_DESCRIPTOR_REVISION))
    goto EH_Fail;
  if (!SetSecurityDescriptorDacl (&sec_desc, TRUE, NULL, FALSE))
    goto EH_Fail;
#endif
*/
  sec_attrs.nLength = core::mem::size_of::<SECURITY_ATTRIBUTES> () as DWORD;   // C 2161
  sec_attrs.lpSecurityDescriptor = core::ptr::null_mut ();     /* &sec_desc */    // C 2162
  sec_attrs.bInheritHandle = FALSE;                            // C 2163

  filename_to_ansi (process_dir, dir.as_mut_ptr ());           // C 2165
  /* Can't use unixtodos_filename here, since that needs its file name
     argument encoded in UTF-8.  OTOH, process_dir, which _is_ in
     UTF-8, points, to the directory computed by our caller, and we
     don't want to modify that, either.  */
  p = dir.as_mut_ptr ();                                       // C 2170
  while *p != 0
    {
      if *p == b'/' as c_char                                 // C 2171
        { *p = b'\\' as c_char; }                            // C 2172
      p = CharNextA (p);
    }

  /* CreateProcess handles batch files as exe specially.  This special
     handling fails when both the batch file and arguments are quoted.
     We pass NULL as exe to avoid the special handling.  */
  if !exe.is_null () && *cmdline == b'"' as c_char             // C 2177
      && { ext = strrchr (exe, b'.' as c_int); !ext.is_null () }   // C 2178
      && (xstrcasecmp (ext, c".bat".as_ptr ()) == 0            // C 2179
          || xstrcasecmp (ext, c".cmd".as_ptr ()) == 0)        // C 2180
    { exe = core::ptr::null_mut (); }                          // C 2181

  flags = if !NILP (Vw32_start_process_share_console)          // C 2183
            { CREATE_NEW_PROCESS_GROUP }                       // C 2184
          else
            { CREATE_NEW_CONSOLE };                            // C 2185
  if NILP (Vw32_start_process_inherit_error_mode)              // C 2186
    { flags |= CREATE_DEFAULT_ERROR_MODE; }                    // C 2187
  'eh_fail: {
    if CreateProcessA (exe, cmdline, &raw mut sec_attrs, core::ptr::null_mut (), TRUE,
                       flags, env, dir.as_mut_ptr (), &raw mut start,
                       &raw mut (*cp).procinfo) == 0           // C 2188-2189
      { break 'eh_fail; }                                      // C 2190 (C's `goto EH_Fail`)
    (*cp).pid = (*cp).procinfo.dwProcessId as c_int;           // C 2192

    /* Hack for Windows 95, which assigns large (ie negative) pids */
    if (*cp).pid < 0                                           // C 2195
      { (*cp).pid = -(*cp).pid; }                              // C 2196
    *pPid = (*cp).pid;                                         // C 2198
    return TRUE;                                               // C 2200
  }
  /* C 2202's `EH_Fail:` label.  C's own line has a stray `;` before the closing parenthesis
     (`DebPrint ((...););`), which the port's macro call cannot carry (the file's rule for a C
     typo that compiles: the macro takes the tuple C's parentheses hold).  */
  DebPrint! (("create_child.CreateProcess failed: %ld\n", GetLastError ()));   // C 2203
  return FALSE;                                                // C 2204
}

/* C 2207-2268: the two functions that pair a started child with a descriptor and reap it when it
   exits - C 2207-2211's comment with C 2212-2242's `register_child`, and C 2244's comment with
   C 2245-2268's `reap_subprocess`.  C 2270-2273's `waitpid` comment opens the next slice.  */

/* create_child doesn't know what emacs's file handle will be for waiting
   on output from the child, so we need to make this additional call
   to register the handle with the process
   This way the select emulator knows how to match file handles with
   entries in child_procs.  */
/// C 2212-2242: `void register_child (pid_t pid, int fd)` - C 2217's `find_child_pid` with C's
/// `(DWORD)` cast, C 2218-2222's not-found exit, C 2224-2226's dead `FULL_DEBUG` arm carried as
/// C's own text (P-51: `FULL_DEBUG` is not in the frozen table), C 2228's descriptor, C 2230-2232's
/// `STATUS_READ_ACKNOWLEDGED` and C 2234-2241's in-use guard, whose `fd_info[fd].cp` is the
/// mirror's deferred `*mut c_void` member (Deviations 49's cast).
/// C: void
/// C: register_child (pid_t pid, int fd)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_child (pid: pid_t, fd: c_int)
{
  let mut cp: *mut child_process;                              // C 2215

  cp = find_child_pid (pid as DWORD);                          // C 2217
  if cp.is_null ()                                             // C 2218
    {
      DebPrint! (("register_child unable to find pid %lu\n", pid));   // C 2220
      return;                                                  // C 2221
    }

  /* C 2224-2226:
#ifdef FULL_DEBUG
  DebPrint (("register_child registered fd %d with pid %lu\n", fd, pid));
#endif
*/
  (*cp).fd = fd;                                               // C 2228

  /* thread is initially blocked until select is called; set status so
     that select will release thread */
  (*cp).status = STATUS_READ_ACKNOWLEDGED;                     // C 2232

  /* attach child_process to fd_info */
  if !fd_info[fd as usize].cp.is_null ()                       // C 2235
    {
      DebPrint! (("register_child: fd_info[%d] apparently in use!\n", fd));   // C 2237
      emacs_abort ();                                          // C 2238
    }

  fd_info[fd as usize].cp = cp as *mut c_void;                 // C 2241
}

/* Called from waitpid when a process exits.  */
/// C 2245-2268: `static void reap_subprocess (child_process *cp)` - C 2248-2260's handle pair (C
/// 2251-2255's dead `FULL_DEBUG` arm carried as C's text between C 2250's comment and the two
/// `CloseHandle`s) and C 2266-2267's `delete_child` for a descriptor already closed.
/// C: static void
/// C: reap_subprocess (child_process *cp)
#[cfg (feature = "windowsnt")]
unsafe fn reap_subprocess (cp: *mut child_process)
{
  if !(*cp).procinfo.hProcess.is_null ()                       // C 2248
    {
      /* Reap the process */
      /* C 2251-2255:
#ifdef FULL_DEBUG
      /* Process should have already died before we are called.  */
      if (WaitForSingleObject (cp->procinfo.hProcess, 0) != WAIT_OBJECT_0)
	DebPrint (("reap_subprocess: child for fd %d has not died yet!", cp->fd));
#endif
*/
      CloseHandle ((*cp).procinfo.hProcess);                   // C 2256
      (*cp).procinfo.hProcess = core::ptr::null_mut ();        // C 2257
      CloseHandle ((*cp).procinfo.hThread);                    // C 2258
      (*cp).procinfo.hThread = core::ptr::null_mut ();         // C 2259
    }

  /* If cp->fd was not closed yet, we might be still reading the
     process output, so don't free its resources just yet.  The call
     to delete_child on behalf of this subprocess will be made by
     sys_read when the subprocess output is fully read.  */
  if (*cp).fd < 0                                              // C 2266
    { delete_child (cp); }                                     // C 2267
}

/* C 2270-2449: `waitpid` - the file's longest single function, and the caller `reap_subprocess`
   (slice 10) was landed for.  C's two values it needs and no mirror in this unit's graph carries
   stand at the use site: `WNOHANG` (`<sys/wait.h>`'s `#define WNOHANG 1`, reached in C through
   C 61's `"syswait.h"`; the port's `include/syswait.rs` records the dead fallbacks instead and
   `nt/inc/sys/include/wait.rs` - whose only other item is the `waitpid` prototype C's own
   definition here supplies - is included by no unit, `docs/ports/include-wait.md` 23-27) and
   `STATUS_CONTROL_C_EXIT` (`<winbase.h>`/`<ntstatus.h>`'s `0xC000013A`, the exit code C 2431
   compares; `nt/cmdproxy.rs` 118-120 spells the same value for cmdproxy.c).  `SIGINT` (C 2432's
   value) is *not* one of these: `include/syssignal.rs` 18 includes `lib/include/signal.rs`, so the
   six standard signals are in this unit's graph through C 62's line.  */
/// C: #define WNOHANG 1
#[cfg (feature = "windowsnt")]
pub const WNOHANG: c_int = 1;
/// C: #define STATUS_CONTROL_C_EXIT ((NTSTATUS) 0xC000013A)
#[cfg (feature = "windowsnt")]
pub const STATUS_CONTROL_C_EXIT: DWORD = 0xC000013A;

/* Wait for a child process specified by PID, or for any of our
   existing child processes (if PID is nonpositive) to die.  When it
   does, close its handle.  Return the pid of the process that died
   and fill in STATUS if non-NULL.  */
/// C 2275-2449: `pid_t waitpid (pid_t pid, int *status, int options)` - C 2278-2283's locals and
/// the `WNOHANG` decode, the C 2286-2303 Posix comment carried as C's text, C 2304-2343's
/// single-pid arm (the descending scan, the `FILE_AT_EOF` test page and the two failures), C
/// 2344-2363's any-child arm, C 2365-2368's two timeouts, C 2370-2378's `do`/`while` (the port's
/// `loop` + negated test, P-95 - `check-do-while.py` reads this pair), C 2380-2404's five-way
/// answer, C 2406-2411's `GetExitCodeProcess`, C 2412-2425's `STILL_ACTIVE` punt and C
/// 2427-2437's exit-code massage, ending at C 2438-2448's reap and return.  C's two dead
/// `FULL_DEBUG` arms are carried as C's text at their positions; `active` indexes the two arrays
/// (`as usize`) and C's `retval <<= 8` is the port's own shift.
/// C: pid_t
/// C: waitpid (pid_t pid, int *status, int options)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
{
  let mut active: DWORD = 0;                                     // C 2278
  let mut retval: DWORD = 0;
  let mut nh: c_int = 0;                                         // C 2279
  let mut cp: *mut child_process = core::ptr::null_mut ();       // C 2280
  let mut cps: [*mut child_process; MAX_CHILDREN as usize]
    = [core::ptr::null_mut (); MAX_CHILDREN as usize];
  let mut wait_hnd: [HANDLE; MAX_CHILDREN as usize]              // C 2281
    = [core::ptr::null_mut (); MAX_CHILDREN as usize];
  let mut timeout_ms: DWORD = 0;                                 // C 2282
  let dont_wait: c_int = ((options & WNOHANG) != 0) as c_int;    // C 2283

  nh = 0;                                                        // C 2285
  /* According to Posix:

     PID = -1 means status is requested for any child process.

     PID > 0 means status is requested for a single child process
     whose pid is PID.

     PID = 0 means status is requested for any child process whose
     process group ID is equal to that of the calling process.  But
     since Windows has only a limited support for process groups (only
     for console processes and only for the purposes of passing
     Ctrl-BREAK signal to them), and since we have no documented way
     of determining whether a given process belongs to our group, we
     treat 0 as -1.

     PID < -1 means status is requested for any child process whose
     process group ID is equal to the absolute value of PID.  Again,
     since we don't support process groups, we treat that as -1.  */
  if pid > 0                                                     // C 2304
    {
      let mut our_child: c_int = 0;                              // C 2306

      /* We are requested to wait for a specific child.  */
      let base: *mut child_process = child_procs.as_mut_ptr ();
      cp = base.add ((child_proc_count - 1) as usize);           // C 2309
      while cp >= base
        {
          /* Some child_procs might be sockets; ignore them.  Also
             ignore subprocesses whose output is not yet completely
             read.  */
          if CHILD_ACTIVE! (*cp)                                 // C 2314
              && !(*cp).procinfo.hProcess.is_null ()             // C 2315
              && (*cp).pid == pid                                // C 2316
            {
              our_child = 1;                                     // C 2318
              break;                                             // C 2319
            }
          cp = cp.sub (1);
        }
      if our_child != 0                                          // C 2322
        {
          if (*cp).fd < 0                                        // C 2324
              || (fd_info[(*cp).fd as usize].flags & FILE_AT_EOF as c_uint) != 0
            {
              wait_hnd[nh as usize] = (*cp).procinfo.hProcess;   // C 2326
              cps[nh as usize] = cp;                             // C 2327
              nh += 1;                                           // C 2328
            }
          else if dont_wait != 0                                 // C 2330
            {
              /* PID specifies our subprocess, but its status is not
                 yet available.  */
              return 0;                                          // C 2334
            }
        }
      if nh == 0                                                 // C 2337
        {
          /* No such child process, or nothing to wait for, so fail.  */
          *errno () = ECHILD;                                    // C 2340
          return -1;                                             // C 2341
        }
    }
  else
    {
      let base: *mut child_process = child_procs.as_mut_ptr ();
      cp = base.add ((child_proc_count - 1) as usize);           // C 2346
      while cp >= base
        {
          if CHILD_ACTIVE! (*cp)                                 // C 2348
              && !(*cp).procinfo.hProcess.is_null ()             // C 2349
              && ((*cp).fd < 0                                   // C 2350
                  || (fd_info[(*cp).fd as usize].flags & FILE_AT_EOF as c_uint) != 0)
            {
              wait_hnd[nh as usize] = (*cp).procinfo.hProcess;   // C 2352
              cps[nh as usize] = cp;                             // C 2353
              nh += 1;                                           // C 2354
            }
          cp = cp.sub (1);
        }
      if nh == 0                                                 // C 2357
        {
          /* Nothing to wait on, so fail.  */
          *errno () = ECHILD;                                    // C 2360
          return -1;                                             // C 2361
        }
    }

  if dont_wait != 0                                              // C 2365
    { timeout_ms = 0; }                                          // C 2366
  else
    { timeout_ms = 1000; }        /* Check for quit about once a second.  */    // C 2368

  loop                                                           // C 2370
    {
      /* When child_status_changed calls us with WNOHANG in OPTIONS,
         we are supposed to be non-interruptible, so don't allow
         quitting in that case.  */
      if dont_wait == 0                                          // C 2375
        { maybe_quit (); }                                       // C 2376
      active = wait_for_objects (nh as DWORD, wait_hnd.as_mut_ptr (), timeout_ms);   // C 2377
      if !(active == WFO_TIMEOUT && dont_wait == 0) { break; }    // C 2378
    }

  if active == WFO_FAILED                                        // C 2380
    {
      *errno () = EBADF;                                         // C 2382
      return -1;                                                 // C 2383
    }
  else if active == WFO_TIMEOUT && dont_wait != 0                // C 2385
    {
      /* PID specifies our subprocess, but it didn't exit yet, so its
         status is not yet available.  */
/* C 2389-2391:
#ifdef FULL_DEBUG
      DebPrint (("Wait: PID %d not reap yet\n", cp->pid));
#endif
*/
      return 0;                                                  // C 2392
    }
  else if active >= 0 && active < WFO_MAX_WAIT as DWORD          // C 2394
    {
      ;                                                          // C 2396
    }
  else if active >= WFO_ABANDONED                                // C 2398
           && active < WFO_ABANDONED + WFO_MAX_WAIT as DWORD     // C 2399
    {
      active -= WFO_ABANDONED;                                   // C 2401
    }
  else
    { emacs_abort (); }                                          // C 2404

  if GetExitCodeProcess (wait_hnd[active as usize], &raw mut retval) == 0   // C 2406
    {
      DebPrint! (("Wait.GetExitCodeProcess failed with %lu\n",    // C 2408-2409
                  GetLastError ()));
      retval = 1;                                                // C 2410
    }
  if retval == STILL_ACTIVE                                      // C 2412
    {
      /* Should never happen.  But it does, with invoking git-gui.exe
         asynchronously.  So we punt, and just report this process as
         exited with exit code 259, when we are called with WNOHANG
         from child_status_changed, because in that case we already
         _know_ the process has died.  */
      DebPrint! (("Wait.WaitForMultipleObjects returned an active process\n"));   // C 2419
      if !(pid > 0 && dont_wait != 0)                            // C 2420
        {
          *errno () = EINVAL;                                    // C 2422
          return -1;                                             // C 2423
        }
    }

  /* Massage the exit code from the process to match the format expected
     by the WIFSTOPPED et al macros in syswait.h.  Only WIFSIGNALED and
     WIFEXITED are supported; WIFSTOPPED doesn't make sense under NT.  */

  if retval == STATUS_CONTROL_C_EXIT                             // C 2431
    { retval = SIGINT as DWORD; }                                // C 2432
  else
    { retval <<= 8; }                                            // C 2434

  if pid > 0 && active != 0                                      // C 2436
    { emacs_abort (); }                                          // C 2437
  cp = cps[active as usize];                                     // C 2438
  pid = (*cp).pid;                                               // C 2439
/* C 2440-2442:
#ifdef FULL_DEBUG
  DebPrint (("Wait signaled with process pid %d\n", cp->pid));
#endif
*/

  if !status.is_null ()                                          // C 2444
    { *status = retval as c_int; }                               // C 2445
  reap_subprocess (cp);                                          // C 2446

  return pid;                                                    // C 2448
}

/* C 50's `<windows.h>`/`<winbase.h>`/`<winnt.h>`: the memory-mapped-file surface and the PE record
   family C 2451-2528 names.  None of it is in any mirror of this unit's graph: `include/w32heap.rs`
   63-76 declares `find_section`/`rva_to_section` with `IMAGE_NT_HEADERS *` and
   `IMAGE_SECTION_HEADER *` spelled `*mut c_void` (P-72 ③, its own note), and this file's C 66-69
   note carries `RVA_TO_PTR` as text for the same reason - so the unit spells the four records (and
   the union C's `Misc` member is) exactly as `src/w32.rs` 11847-11916 spells `child_process`.
   Every record is **UNVERIFIED** on this machine (P-88 ①); the sizes are the PE/COFF
   specification's, and the `/// C:` lines carry C's own field order.  */
/// C: #define GENERIC_READ 0x80000000L
/// C: #define FILE_SHARE_READ 0x00000001
/// C: #define OPEN_EXISTING 3
/// C: #define FILE_ATTRIBUTE_NORMAL 0x00000080
/// C: #define PAGE_READONLY 0x02
/// C: #define FILE_MAP_READ 0x0004
/// C: #define FILE_BEGIN 0
/// C: #define IMAGE_NT_OPTIONAL_HDR_MAGIC 0x10b
/// C: #define IMAGE_NT_OPTIONAL_HDR32_MAGIC IMAGE_NT_OPTIONAL_HDR_MAGIC
#[cfg (feature = "windowsnt")]
pub const GENERIC_READ: DWORD = 0x80000000;
#[cfg (feature = "windowsnt")]
pub const FILE_SHARE_READ: DWORD = 0x00000001;
#[cfg (feature = "windowsnt")]
pub const OPEN_EXISTING: DWORD = 3;
#[cfg (feature = "windowsnt")]
pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x00000080;
#[cfg (feature = "windowsnt")]
pub const PAGE_READONLY: DWORD = 0x02;
#[cfg (feature = "windowsnt")]
pub const FILE_MAP_READ: DWORD = 0x0004;
#[cfg (feature = "windowsnt")]
pub const FILE_BEGIN: DWORD = 0;
#[cfg (feature = "windowsnt")]
pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: WORD = 0x10b;
#[cfg (feature = "windowsnt")]
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: WORD = IMAGE_NT_OPTIONAL_HDR_MAGIC;
/// C: WINBASEAPI HANDLE WINAPI CreateFileA (LPCSTR lpFileName, DWORD dwDesiredAccess,
/// C:   DWORD dwShareMode, LPSECURITY_ATTRIBUTES lpSecurityAttributes, DWORD dwCreationDisposition,
/// C:   DWORD dwFlagsAndAttributes, HANDLE hTemplateFile);
/// C: WINBASEAPI DWORD WINAPI GetFileSize (HANDLE hFile, LPDWORD lpFileSizeHigh);
/// C: WINBASEAPI HANDLE WINAPI CreateFileMappingA (HANDLE hFile,
/// C:   LPSECURITY_ATTRIBUTES lpFileMappingAttributes, DWORD flProtect, DWORD dwMaximumSizeHigh,
/// C:   DWORD dwMaximumSizeLow, LPCSTR lpName);
/// C: WINBASEAPI LPVOID WINAPI MapViewOfFile (HANDLE hFileMappingObject, DWORD dwDesiredAccess,
/// C:   DWORD dwFileOffsetHigh, DWORD dwFileOffsetLow, SIZE_T dwNumberOfBytesToMap);
/// C: WINBASEAPI BOOL WINAPI UnmapViewOfFile (LPCVOID lpBaseAddress);
/// C: WINBASEAPI DWORD WINAPI SetFilePointer (HANDLE hFile, LONG lDistanceToMove,
/// C:   PLONG lpDistanceToMoveHigh, DWORD dwMoveMethod);
/// C: WINBASEAPI BOOL WINAPI SetEndOfFile (HANDLE hFile);
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn CreateFileA (lpFileName: LPCSTR, dwDesiredAccess: DWORD, dwShareMode: DWORD,
                      lpSecurityAttributes: *mut SECURITY_ATTRIBUTES,
                      dwCreationDisposition: DWORD, dwFlagsAndAttributes: DWORD,
                      hTemplateFile: HANDLE) -> HANDLE;
  pub fn GetFileSize (hFile: HANDLE, lpFileSizeHigh: LPDWORD) -> DWORD;
  pub fn CreateFileMapping (hFile: HANDLE, lpFileMappingAttributes: *mut SECURITY_ATTRIBUTES,
                            flProtect: DWORD, dwMaximumSizeHigh: DWORD, dwMaximumSizeLow: DWORD,
                            lpName: LPCSTR) -> HANDLE;
  pub fn MapViewOfFile (hFileMappingObject: HANDLE, dwDesiredAccess: DWORD,
                        dwFileOffsetHigh: DWORD, dwFileOffsetLow: DWORD,
                        dwNumberOfBytesToMap: SIZE_T) -> LPVOID;
  pub fn UnmapViewOfFile (lpBaseAddress: *const c_void) -> BOOL;
  pub fn SetFilePointer (hFile: HANDLE, lDistanceToMove: LONG, lpDistanceToMoveHigh: *mut LONG,
                         dwMoveMethod: DWORD) -> DWORD;
  pub fn SetEndOfFile (hFile: HANDLE) -> BOOL;
}
/// C: typedef struct _IMAGE_FILE_HEADER { WORD Machine; WORD NumberOfSections; DWORD TimeDateStamp;
/// C:   DWORD PointerToSymbolTable; DWORD NumberOfSymbols; WORD SizeOfOptionalHeader;
/// C:   WORD Characteristics; } IMAGE_FILE_HEADER;   [`<winnt.h>`; 20 bytes]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_FILE_HEADER
{
  pub Machine: WORD,
  pub NumberOfSections: WORD,
  pub TimeDateStamp: DWORD,
  pub PointerToSymbolTable: DWORD,
  pub NumberOfSymbols: DWORD,
  pub SizeOfOptionalHeader: WORD,
  pub Characteristics: WORD,
}
/// C: typedef struct _IMAGE_DATA_DIRECTORY { DWORD VirtualAddress; DWORD Size; }
/// C:   IMAGE_DATA_DIRECTORY;   [`<winnt.h>`; 8 bytes]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY
{
  pub VirtualAddress: DWORD,
  pub Size: DWORD,
}
/// C: typedef struct _IMAGE_OPTIONAL_HEADER { WORD Magic; BYTE MajorLinkerVersion;
/// C:   BYTE MinorLinkerVersion; DWORD SizeOfCode; DWORD SizeOfInitializedData;
/// C:   DWORD SizeOfUninitializedData; DWORD AddressOfEntryPoint; DWORD BaseOfCode;
/// C:   DWORD BaseOfData; DWORD ImageBase; DWORD SectionAlignment; DWORD FileAlignment;
/// C:   WORD MajorOperatingSystemVersion; WORD MinorOperatingSystemVersion; WORD MajorImageVersion;
/// C:   WORD MinorImageVersion; WORD MajorSubsystemVersion; WORD MinorSubsystemVersion;
/// C:   DWORD Win32VersionValue; DWORD SizeOfImage; DWORD SizeOfHeaders; DWORD CheckSum;
/// C:   WORD Subsystem; WORD DllCharacteristics; DWORD SizeOfStackReserve;
/// C:   DWORD SizeOfStackCommit; DWORD SizeOfHeapReserve; DWORD SizeOfHeapCommit;
/// C:   DWORD LoaderFlags; DWORD NumberOfRvaAndSizes;
/// C:   IMAGE_DATA_DIRECTORY DataDirectory [IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
/// C:   } IMAGE_OPTIONAL_HEADER32;   [`<winnt.h>`; 224 bytes, `DataDirectory`'s sixteen entries
/// included]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER32
{
  pub Magic: WORD,
  pub MajorLinkerVersion: BYTE,
  pub MinorLinkerVersion: BYTE,
  pub SizeOfCode: DWORD,
  pub SizeOfInitializedData: DWORD,
  pub SizeOfUninitializedData: DWORD,
  pub AddressOfEntryPoint: DWORD,
  pub BaseOfCode: DWORD,
  pub BaseOfData: DWORD,
  pub ImageBase: DWORD,
  pub SectionAlignment: DWORD,
  pub FileAlignment: DWORD,
  pub MajorOperatingSystemVersion: WORD,
  pub MinorOperatingSystemVersion: WORD,
  pub MajorImageVersion: WORD,
  pub MinorImageVersion: WORD,
  pub MajorSubsystemVersion: WORD,
  pub MinorSubsystemVersion: WORD,
  pub Win32VersionValue: DWORD,
  pub SizeOfImage: DWORD,
  pub SizeOfHeaders: DWORD,
  pub CheckSum: DWORD,
  pub Subsystem: WORD,
  pub DllCharacteristics: WORD,
  pub SizeOfStackReserve: DWORD,
  pub SizeOfStackCommit: DWORD,
  pub SizeOfHeapReserve: DWORD,
  pub SizeOfHeapCommit: DWORD,
  pub LoaderFlags: DWORD,
  pub NumberOfRvaAndSizes: DWORD,
  pub DataDirectory: [IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES as usize],
}
/// C: #define IMAGE_OPTIONAL_HEADER32 IMAGE_OPTIONAL_HEADER  (the `#ifndef` fallback's second
/// line; `<winnt.h>` gives the generic name to the same record, so the port carries
/// both spellings - this unit's record is the 32-bit one.)
#[cfg (feature = "windowsnt")]
pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER32;
/// C: #define IMAGE_NUMBEROF_DIRECTORY_ENTRIES 16
#[cfg (feature = "windowsnt")]
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: c_int = 16;
/// C: typedef struct _IMAGE_NT_HEADERS { DWORD Signature; IMAGE_FILE_HEADER FileHeader;
/// C:   IMAGE_OPTIONAL_HEADER32 OptionalHeader; } IMAGE_NT_HEADERS;   [`<winnt.h>`]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_NT_HEADERS
{
  pub Signature: DWORD,
  pub FileHeader: IMAGE_FILE_HEADER,
  pub OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}
/// C: typedef struct _IMAGE_SECTION_HEADER { BYTE Name [IMAGE_SIZEOF_SHORT_NAME];
/// C:   union { DWORD PhysicalAddress; DWORD VirtualSize; } Misc; DWORD VirtualAddress;
/// C:   DWORD SizeOfRawData; DWORD PointerToRawData; DWORD PointerToRelocations;
/// C:   DWORD PointerToLinenumbers; WORD NumberOfRelocations; WORD NumberOfLinenumbers;
/// C:   DWORD Characteristics; } IMAGE_SECTION_HEADER;   [`<winnt.h>`; 40 bytes.  C's `Misc` is an
/// anonymous union, so the port names it after its container (P-67, this file's `OVERLAPPED`).]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub union IMAGE_SECTION_HEADER_Misc
{
  pub PhysicalAddress: DWORD,
  pub VirtualSize: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_SECTION_HEADER
{
  pub Name: [BYTE; IMAGE_SIZEOF_SHORT_NAME as usize],
  pub Misc: IMAGE_SECTION_HEADER_Misc,
  pub VirtualAddress: DWORD,
  pub SizeOfRawData: DWORD,
  pub PointerToRawData: DWORD,
  pub PointerToRelocations: DWORD,
  pub PointerToLinenumbers: DWORD,
  pub NumberOfRelocations: WORD,
  pub NumberOfLinenumbers: WORD,
  pub Characteristics: DWORD,
}
/// C: #define IMAGE_SIZEOF_SHORT_NAME 8
#[cfg (feature = "windowsnt")]
pub const IMAGE_SIZEOF_SHORT_NAME: c_int = 8;
#[cfg (feature = "windowsnt")]
pub type PIMAGE_SECTION_HEADER = *mut IMAGE_SECTION_HEADER;
#[cfg (feature = "windowsnt")]
pub type PIMAGE_NT_HEADERS = *mut IMAGE_NT_HEADERS;
/// C: #define IMAGE_FIRST_SECTION(ntheader) ((PIMAGE_SECTION_HEADER) ((ULONG_PTR) (ntheader)
/// C:   + FIELD_OFFSET (IMAGE_NT_HEADERS, OptionalHeader)
/// C:   + ((ntheader))->FileHeader.SizeOfOptionalHeader))
#[cfg (feature = "windowsnt")]
macro_rules! IMAGE_FIRST_SECTION {
  ($nt:expr) => {
    ((($nt) as usize
      + core::mem::offset_of! (IMAGE_NT_HEADERS, OptionalHeader)
      + (*$nt).FileHeader.SizeOfOptionalHeader as usize) as PIMAGE_SECTION_HEADER)
  };
}

/* C 2451-2528: the memory-mapped-file helpers C 2451-2481's `open_input_file` and C 2511-2521's
   `close_file_data`, the section lookup C 2483-2509's `rva_to_section` between them, and C
   2523-2528's `#ifndef IMAGE_NT_OPTIONAL_HDR32_MAGIC` fallback.  C 2530-2532's
   `w32_executable_type` comment opens the next slice.  */

/// C 2451-2481: `int open_input_file (file_data *p_file, char *filename)` - C 2459-2460's
/// `CreateFileA`, C 2461-2462's `INVALID_HANDLE_VALUE` guard, C 2464-2468's `GetFileSize`/
/// `CreateFileMapping` pair, C 2470-2472's `MapViewOfFile` and C 2474-2478's five field writes.
/// C's `unsigned long size` is the mirror's `c_ulong`, so C 2464's `DWORD` return and C 2466/2470's
/// use of `size` as `DWORD`/`SIZE_T` are the two `as` conversions (Deviations 39's class), and C
/// 2478's `unsigned char *` field takes C's own implicit pointer conversion as a cast.
/// C: int
/// C: open_input_file (file_data *p_file, char *filename)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_input_file (p_file: *mut file_data, filename: *mut c_char) -> c_int
{
  let mut file: HANDLE;                                        // C 2454
  let mut file_mapping: HANDLE;                                // C 2455
  let mut file_base: *mut c_void;                              // C 2456
  let mut size: c_ulong = 0;
  let mut upper_size: c_ulong = 0;                             // C 2457

  file = CreateFileA (filename, GENERIC_READ, FILE_SHARE_READ, core::ptr::null_mut (),
                      OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);   // C 2459-2460
  if file == INVALID_HANDLE_VALUE                              // C 2461
    { return FALSE; }                                          // C 2462

  size = GetFileSize (file, &raw mut upper_size) as c_ulong;   // C 2464
  file_mapping = CreateFileMapping (file, core::ptr::null_mut (), PAGE_READONLY,
                                    0, size as DWORD, core::ptr::null ());   // C 2465-2466
  if file_mapping.is_null ()                                   // C 2467
    { return FALSE; }                                          // C 2468

  file_base = MapViewOfFile (file_mapping, FILE_MAP_READ, 0, 0, size as SIZE_T);   // C 2470
  if file_base.is_null ()                                      // C 2471
    { return FALSE; }                                          // C 2472

  (*p_file).name = filename;                                   // C 2474
  (*p_file).size = size;                                       // C 2475
  (*p_file).file = file;                                       // C 2476
  (*p_file).file_mapping = file_mapping;                       // C 2477
  (*p_file).file_base = file_base as *mut u8;                  // C 2478

  return TRUE;                                                 // C 2480
}

/* Return pointer to section header for section containing the given
   relative virtual address.  */                             // C 2483-2484
/// C 2483-2509: `IMAGE_SECTION_HEADER * rva_to_section (DWORD_PTR rva, IMAGE_NT_HEADERS
/// *nt_header)` - C 2491's `IMAGE_FIRST_SECTION` (the port's `macro_rules!` above), C 2493's
/// `NumberOfSections` loop, C 2495-2500's linker comment carried as C's text, C 2501-2502's
/// `max (SizeOfRawData, Misc.VirtualSize)` (the `max!` mirror this round includes under C 57's
/// lisp.h line) and C 2503-2505's range test.  The mirror's prototype keeps the record as
/// `*mut c_void` (`include/w32heap.rs` 128-132); the definition spells the record it reads
/// (Deviations 47's device), which is the same symbol either way.
/// C: IMAGE_SECTION_HEADER *
/// C: rva_to_section (DWORD_PTR rva, IMAGE_NT_HEADERS * nt_header)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rva_to_section (rva: DWORD_PTR, nt_header: *mut IMAGE_NT_HEADERS)
                                         -> *mut IMAGE_SECTION_HEADER
{
  let mut section: PIMAGE_SECTION_HEADER;                      // C 2488
  let mut i: c_int;                                            // C 2489

  section = IMAGE_FIRST_SECTION! (nt_header);                  // C 2491

  i = 0;                                                       // C 2493
  while i < (*nt_header).FileHeader.NumberOfSections as c_int
    {
      /* Some linkers (eg. the NT SDK linker I believe) swapped the
	 meaning of these two values - or rather, they ignored
	 VirtualSize entirely and always set it to zero.  This affects
	 some very old exes (eg. gzip dated Dec 1993).  Since
	 w32_executable_type relies on this function to work reliably,
	 we need to cope with this.  */
      let real_size: DWORD_PTR = max! ((*section).SizeOfRawData as DWORD_PTR,
                                      (*section).Misc.VirtualSize as DWORD_PTR);   // C 2501-2502
      if rva >= (*section).VirtualAddress as DWORD_PTR         // C 2503
          && rva < (*section).VirtualAddress as DWORD_PTR + real_size   // C 2504
        { return section; }                                    // C 2505
      section = section.add (1);                               // C 2506
      i += 1;
    }
  return core::ptr::null_mut ();                               // C 2508
}

/* Close the system structures associated with the given file.  */    // C 2511
/// C 2511-2521: `void close_file_data (file_data *p_file)` - C 2515's `UnmapViewOfFile`, C 2516's
/// mapping close, C 2518-2519's `SetFilePointer`/`SetEndOfFile` pair (C's `p_file->size` narrows
/// to the `LONG` parameter, the `as LONG` C's own conversion performs) and C 2520's file close.
/// C: void
/// C: close_file_data (file_data *p_file)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_file_data (p_file: *mut file_data)
{
  UnmapViewOfFile ((*p_file).file_base as *const c_void);      // C 2515
  CloseHandle ((*p_file).file_mapping);                        // C 2516
  /* For the case of output files, set final size.  */
  SetFilePointer ((*p_file).file, (*p_file).size as LONG, core::ptr::null_mut (),
                  FILE_BEGIN);                                 // C 2518
  SetEndOfFile ((*p_file).file);                               // C 2519
  CloseHandle ((*p_file).file);                                // C 2520
}

/* Old versions of w32api headers don't have separate 32-bit and
   64-bit defines, but the one they have matches the 32-bit variety.  */    // C 2523-2524
/* C 2525-2528: the `#ifndef IMAGE_NT_OPTIONAL_HDR32_MAGIC` fallback.  The guard is *false* with a
   modern `<winnt.h>` (both names are defined there), so the two `#define`s below never fire; the
   port carries C's own text here and spells the two names in its `<winnt.h>` block above
   (`IMAGE_NT_OPTIONAL_HDR32_MAGIC`, the value `<winnt.h>` gives it, and the
   `IMAGE_OPTIONAL_HEADER32` record this round's block declares).  */
/*
#ifndef IMAGE_NT_OPTIONAL_HDR32_MAGIC
# define IMAGE_NT_OPTIONAL_HDR32_MAGIC IMAGE_NT_OPTIONAL_HDR_MAGIC
# define IMAGE_OPTIONAL_HEADER32 IMAGE_OPTIONAL_HEADER
#endif
*/

/* C 50's `<windows.h>`/`<winbase.h>`/`<winnt.h>`: the DOS/NT header surface C 2530-2674 reads - the
   two records, the four signatures, the two subsystem/directory constants, the SDK's `LOWORD`, and
   the 64-bit pair C 2610's `#ifdef IMAGE_NT_OPTIONAL_HDR64_MAGIC` arm needs (which follows the
   file's `__MINGW64__` reading: a guard the *header* defines becomes a feature named after it, and
   the arm's text is carried under it - P-51; a modern `<winnt.h>` defines the macro, so a build
   that sets the feature gets the arm).  No mirror in this unit's graph carries any of them
   (UNVERIFIED, P-88 ①).  */
/// C: #define IMAGE_DOS_SIGNATURE 0x5A4D
/// C: #define IMAGE_NT_SIGNATURE 0x00004550
/// C: #define IMAGE_OS2_SIGNATURE 0x454E
/// C: #define IMAGE_NT_OPTIONAL_HDR64_MAGIC 0x20b
/// C: #define IMAGE_SUBSYSTEM_WINDOWS_GUI 2
/// C: #define IMAGE_DIRECTORY_ENTRY_IMPORT 1
#[cfg (feature = "windowsnt")]
pub const IMAGE_DOS_SIGNATURE: WORD = 0x5A4D;
#[cfg (feature = "windowsnt")]
pub const IMAGE_NT_SIGNATURE: DWORD = 0x00004550;
#[cfg (feature = "windowsnt")]
pub const IMAGE_OS2_SIGNATURE: WORD = 0x454E;
#[cfg (feature = "windowsnt")]
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: WORD = 2;
#[cfg (feature = "windowsnt")]
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: c_int = 1;
/// C: typedef struct _IMAGE_DOS_HEADER { WORD e_magic; WORD e_cblp; WORD e_cp; WORD e_crlc;
/// C:   WORD e_cparhdr; WORD e_minalloc; WORD e_maxalloc; WORD e_ss; WORD e_sp; WORD e_csum;
/// C:   WORD e_ip; WORD e_cs; WORD e_lfarlc; WORD e_ovno; WORD e_res [4]; WORD e_oemid;
/// C:   WORD e_oeminfo; WORD e_res2 [10]; LONG e_lfanew; } IMAGE_DOS_HEADER;   [`<winnt.h>`; 64
/// bytes]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_DOS_HEADER
{
  pub e_magic: WORD,
  pub e_cblp: WORD,
  pub e_cp: WORD,
  pub e_crlc: WORD,
  pub e_cparhdr: WORD,
  pub e_minalloc: WORD,
  pub e_maxalloc: WORD,
  pub e_ss: WORD,
  pub e_sp: WORD,
  pub e_csum: WORD,
  pub e_ip: WORD,
  pub e_cs: WORD,
  pub e_lfarlc: WORD,
  pub e_ovno: WORD,
  pub e_res: [WORD; 4],
  pub e_oemid: WORD,
  pub e_oeminfo: WORD,
  pub e_res2: [WORD; 10],
  pub e_lfanew: LONG,
}
#[cfg (feature = "windowsnt")]
pub type PIMAGE_DOS_HEADER = *mut IMAGE_DOS_HEADER;
/// C: typedef struct _IMAGE_IMPORT_DESCRIPTOR { union { DWORD Characteristics;
/// C:   DWORD OriginalFirstThunk; }; DWORD TimeDateStamp; DWORD ForwarderChain; DWORD Name;
/// C:   DWORD FirstThunk; } IMAGE_IMPORT_DESCRIPTOR;   [`<winnt.h>`; 20 bytes.  C's first member is
/// an anonymous union, named here after its container (P-67).]
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub union IMAGE_IMPORT_DESCRIPTOR_u
{
  pub Characteristics: DWORD,
  pub OriginalFirstThunk: DWORD,
}
#[cfg (feature = "windowsnt")]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR
{
  pub u: IMAGE_IMPORT_DESCRIPTOR_u,
  pub TimeDateStamp: DWORD,
  pub ForwarderChain: DWORD,
  pub Name: DWORD,
  pub FirstThunk: DWORD,
}
/// C: #define LOWORD(l) ((WORD) (((DWORD_PTR) (l)) & 0xffff))
#[cfg (feature = "windowsnt")]
macro_rules! LOWORD {
  ($l:expr) => { (($l) as DWORD_PTR & 0xffff) as WORD };
}
#[cfg (feature = "image_nt_optional_hdr64_magic")]
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: WORD = 0x20b;
/// C: typedef struct _IMAGE_OPTIONAL_HEADER64 { WORD Magic; BYTE MajorLinkerVersion;
/// C:   BYTE MinorLinkerVersion; DWORD SizeOfCode; DWORD SizeOfInitializedData;
/// C:   DWORD SizeOfUninitializedData; DWORD AddressOfEntryPoint; DWORD BaseOfCode;
/// C:   ULONGLONG ImageBase; DWORD SectionAlignment; DWORD FileAlignment;
/// C:   WORD MajorOperatingSystemVersion; WORD MinorOperatingSystemVersion; WORD MajorImageVersion;
/// C:   WORD MinorImageVersion; WORD MajorSubsystemVersion; WORD MinorSubsystemVersion;
/// C:   DWORD Win32VersionValue; DWORD SizeOfImage; DWORD SizeOfHeaders; DWORD CheckSum;
/// C:   WORD Subsystem; WORD DllCharacteristics; ULONGLONG SizeOfStackReserve;
/// C:   ULONGLONG SizeOfStackCommit; ULONGLONG SizeOfHeapReserve; ULONGLONG SizeOfHeapCommit;
/// C:   DWORD LoaderFlags; DWORD NumberOfRvaAndSizes;
/// C:   IMAGE_DATA_DIRECTORY DataDirectory [IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
/// C:   } IMAGE_OPTIONAL_HEADER64;   [`<winnt.h>`; 240 bytes]
#[cfg (feature = "image_nt_optional_hdr64_magic")]
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER64
{
  pub Magic: WORD,
  pub MajorLinkerVersion: BYTE,
  pub MinorLinkerVersion: BYTE,
  pub SizeOfCode: DWORD,
  pub SizeOfInitializedData: DWORD,
  pub SizeOfUninitializedData: DWORD,
  pub AddressOfEntryPoint: DWORD,
  pub BaseOfCode: DWORD,
  pub ImageBase: ULONGLONG,
  pub SectionAlignment: DWORD,
  pub FileAlignment: DWORD,
  pub MajorOperatingSystemVersion: WORD,
  pub MinorOperatingSystemVersion: WORD,
  pub MajorImageVersion: WORD,
  pub MinorImageVersion: WORD,
  pub MajorSubsystemVersion: WORD,
  pub MinorSubsystemVersion: WORD,
  pub Win32VersionValue: DWORD,
  pub SizeOfImage: DWORD,
  pub SizeOfHeaders: DWORD,
  pub CheckSum: DWORD,
  pub Subsystem: WORD,
  pub DllCharacteristics: WORD,
  pub SizeOfStackReserve: ULONGLONG,
  pub SizeOfStackCommit: ULONGLONG,
  pub SizeOfHeapReserve: ULONGLONG,
  pub SizeOfHeapCommit: ULONGLONG,
  pub LoaderFlags: DWORD,
  pub NumberOfRvaAndSizes: DWORD,
  pub DataDirectory: [IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES as usize],
}

/* Implementation note: This function works with file names encoded in
   the current ANSI codepage.  */                             // C 2530-2531
/// C 2532-2674: `static int w32_executable_type (char * filename, int * is_dos_app, int
/// * is_cygnus_app, int * is_msys_app, int * is_gui_app)` - C 2543-2547's four defaults, C
/// 2549-2550's `open_input_file` guard, C 2552-2570's extension test (`.com`/`.bat`/`.cmd` and the
/// `COMSPEC` recursion), C 2571-2669's DOS/NT header walk (C 2578-2585's two records, C 2587-2596's
/// two "it is a DOS program" answers, C 2597-2607's 32-bit optional-header read, C 2610-2619's
/// `IMAGE_NT_OPTIONAL_HDR64_MAGIC` arm, C 2620-2667's import-directory walk with `RVA_TO_PTR` and
/// the two DLL-name tests) and C 2671-2673's `unwind` tail.  C's `goto unwind` is forward, into
/// that tail, so it is the port's `break 'unwind` out of a labeled block (Deviations 48's device).
/// C: static int
/// C: w32_executable_type (char * filename,
/// C:                      int * is_dos_app,
/// C:                      int * is_cygnus_app,
/// C:                      int * is_msys_app,
/// C:                      int * is_gui_app)
#[cfg (feature = "windowsnt")]
unsafe fn w32_executable_type (filename: *mut c_char, is_dos_app: *mut c_int,
                               is_cygnus_app: *mut c_int, is_msys_app: *mut c_int,
                               is_gui_app: *mut c_int) -> c_int
{
  let mut executable: file_data = core::mem::zeroed ();        // C 2539
  let mut p: *mut c_char;                                      // C 2540
  let mut retval: c_int = 0;                                   // C 2541

  /* Default values in case we can't tell for sure.  */
  *is_dos_app = FALSE;                                         // C 2544
  *is_cygnus_app = FALSE;                                      // C 2545
  *is_msys_app = FALSE;                                        // C 2546
  *is_gui_app = FALSE;                                         // C 2547

  if open_input_file (&raw mut executable, filename) == 0      // C 2549
    { return -1; }                                             // C 2550

  p = strrchr (filename, b'.' as c_int);                       // C 2552

  /* We can only identify DOS .com programs from the extension. */
  if !p.is_null () && xstrcasecmp (p, c".com".as_ptr ()) == 0  // C 2555
    { *is_dos_app = TRUE; }                                    // C 2556
  else if !p.is_null () && (xstrcasecmp (p, c".bat".as_ptr ()) == 0   // C 2557
                            || xstrcasecmp (p, c".cmd".as_ptr ()) == 0)   // C 2558
    {
      /* A DOS shell script - it appears that CreateProcess is happy to
	 accept this (somewhat surprisingly); presumably it looks at
	 COMSPEC to determine what executable to actually invoke.
	 Therefore, we have to do the same here as well. */
      /* Actually, I think it uses the program association for that
	 extension, which is defined in the registry.  */
      p = egetenv (c"COMSPEC".as_ptr ());                      // C 2566
      if !p.is_null ()                                         // C 2567
        {
          retval = w32_executable_type (p, is_dos_app, is_cygnus_app, is_msys_app,
                                        is_gui_app);           // C 2568-2569
        }
    }
  else
    {
      /* Look for DOS .exe signature - if found, we must also check that
	 it isn't really a 16- or 32-bit Windows exe, since both formats
	 start with a DOS program stub.  Note that 16-bit Windows
	 executables use the OS/2 1.x format. */

      let mut dos_header: *mut IMAGE_DOS_HEADER;                // C 2578
      let mut nt_header: *mut IMAGE_NT_HEADERS;                 // C 2579

      'unwind: {
        dos_header = executable.file_base as PIMAGE_DOS_HEADER;   // C 2581
        if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE         // C 2582
          { break 'unwind; }                                    // C 2583 (C's `goto unwind`)

        nt_header = (dos_header as *mut u8).add ((*dos_header).e_lfanew as usize)
          as PIMAGE_NT_HEADERS;                                 // C 2585

        if nt_header as *mut c_char
          > (dos_header as *mut c_char).add (executable.size as usize)   // C 2587
          {
            /* Some dos headers (pkunzip) have bogus e_lfanew fields.  */
            *is_dos_app = TRUE;                                 // C 2590
          }
        else if (*nt_header).Signature != IMAGE_NT_SIGNATURE    // C 2592
                 && LOWORD! ((*nt_header).Signature) != IMAGE_OS2_SIGNATURE   // C 2593
          {
            *is_dos_app = TRUE;                                 // C 2595
          }
        else if (*nt_header).Signature == IMAGE_NT_SIGNATURE    // C 2597
          {
            let mut data_dir: *mut IMAGE_DATA_DIRECTORY = core::ptr::null_mut ();   // C 2599
            if (*nt_header).OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC   // C 2600
              {
                /* Ensure we are using the 32 bit structure.  */
                let opt: *mut IMAGE_OPTIONAL_HEADER32
                  = &raw mut (*nt_header).OptionalHeader;       // C 2603-2604
                data_dir = (*opt).DataDirectory.as_mut_ptr ();  // C 2605
                *is_gui_app = ((*opt).Subsystem == IMAGE_SUBSYSTEM_WINDOWS_GUI) as c_int;   // C 2606
              }
            /* MingW 3.12 has the required 64 bit structs, but in case older
               versions don't, only check 64 bit exes if we know how.  */
            #[cfg (feature = "image_nt_optional_hdr64_magic")]
            if (*nt_header).OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC   // C 2611-2612
              {
                let opt: *mut IMAGE_OPTIONAL_HEADER64
                  = &raw mut (*nt_header).OptionalHeader as *mut _ as *mut IMAGE_OPTIONAL_HEADER64;   // C 2614-2615
                data_dir = (*opt).DataDirectory.as_mut_ptr ();  // C 2616
                *is_gui_app = ((*opt).Subsystem == IMAGE_SUBSYSTEM_WINDOWS_GUI) as c_int;   // C 2617
              }
            if !data_dir.is_null ()                             // C 2620
              {
                /* Look for Cygwin DLL in the DLL import list. */
                let import_dir: IMAGE_DATA_DIRECTORY
                  = *data_dir.add (IMAGE_DIRECTORY_ENTRY_IMPORT as usize);   // C 2623-2624

                /* Import directory can be missing in .NET DLLs.  */
                if import_dir.VirtualAddress != 0               // C 2627
                  {
                    let mut section: *mut IMAGE_SECTION_HEADER
                      = rva_to_section (import_dir.VirtualAddress as DWORD_PTR, nt_header);   // C 2629-2630
                    if section.is_null ()                       // C 2631
                      { emacs_abort (); }                       // C 2632

                    let mut imports: *mut IMAGE_IMPORT_DESCRIPTOR
                      = RVA_TO_PTR! (import_dir.VirtualAddress, section, executable)
                        as *mut IMAGE_IMPORT_DESCRIPTOR;        // C 2634-2636

                    while (*imports).Name != 0                  // C 2638
                      {
                        section = rva_to_section ((*imports).Name as DWORD_PTR,
                                                  nt_header);   // C 2640
                        if section.is_null ()                   // C 2641
                          { emacs_abort (); }                   // C 2642

                        let dllname: *mut c_char
                          = RVA_TO_PTR! ((*imports).Name, section, executable)
                            as *mut c_char;                     // C 2644-2645

                        /* The exact name of the Cygwin DLL has changed with
			   various releases, but hopefully this will be
			   reasonably future-proof.  */
                        if strncmp (dllname, c"cygwin".as_ptr (), 6) == 0   // C 2650
                          {
                            *is_cygnus_app = TRUE;              // C 2652
                            break;                              // C 2653
                          }
                        else if strncmp (dllname, c"msys-".as_ptr (), 5) == 0   // C 2655
                          {
                            /* This catches both MSYS 1.x and MSYS2
			       executables (the DLL name is msys-1.0.dll and
			       msys-2.0.dll, respectively).  There doesn't
			       seem to be a reason to distinguish between
			       the two, for now.  */
                            *is_msys_app = TRUE;                // C 2662
                            break;                              // C 2663
                          }
                        imports = imports.add (1);              // C 2638's `imports++`
                      }
                  }
              }
          }
      }
    }

  /* C 2671's `unwind:` label.  */
  close_file_data (&raw mut executable);                        // C 2672
  return retval;                                                // C 2673
}

/* C 2676-2720: the environment pair `sys_spawnve` (C 2725, the next slice) calls - C 2676-2698's
   `compare_env` comparator and C 2700-2720's `merge_and_sort_env`.  C's `<stdlib.h>`'s `qsort` is
   the one C library entry the pair needs and the port's `stdlib.rs` mirror does not carry
   (`src/buffer.rs` 129-134 declares the same row with C's own text; the comparator is C's
   `__compar_fn_t`), so it stands at the use site.  `compare_env` keeps C's `static` (no
   `no_mangle`: this file's rule for the C helpers) but takes the `extern "C"` ABI C's callback
   needs, and `toupper` is the `ctype.rs` mirror's.  */

/* C's `<stdlib.h>`: `void qsort (void *, size_t, size_t, __compar_fn_t);` - C 2717's call, whose
   `compare_env` is the `Some (...)` below (`src/buffer.rs` 129-134's spelling).
   */
/// C: extern void qsort (void *__base, size_t __nmemb, size_t __size, __compar_fn_t __compar);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
}

/// C 2676-2698: `static int compare_env (const void *strp1, const void *strp2)` - C 2679's two
/// dereferences, C 2681's four-part loop test, C 2683-2688's `toupper` comparison pair and C
/// 2689's two increments, then C 2692-2697's three-way answer on `=`.
/// C: static int
/// C: compare_env (const void *strp1, const void *strp2)
#[cfg (feature = "windowsnt")]
unsafe extern "C" fn compare_env (strp1: *const c_void, strp2: *const c_void) -> c_int
{
  let mut str1: *const c_char = *(strp1 as *const *const c_char);
  let mut str2: *const c_char = *(strp2 as *const *const c_char);    // C 2679

  while *str1 != 0 && *str2 != 0                                    // C 2681
        && *str1 != b'=' as c_char && *str2 != b'=' as c_char
    {
      /* Sort order in command.com/cmd.exe is based on uppercasing
         names, so do the same here.  */
      if toupper (*str1 as c_int) > toupper (*str2 as c_int)         // C 2685
        { return 1; }                                                // C 2686
      else if toupper (*str1 as c_int) < toupper (*str2 as c_int)    // C 2687
        { return -1; }                                               // C 2688
      str1 = str1.add (1);
      str2 = str2.add (1);                                           // C 2689
    }

  if *str1 == b'=' as c_char && *str2 == b'=' as c_char              // C 2692
    { return 0; }                                                    // C 2693
  else if *str1 == b'=' as c_char                                    // C 2694
    { return -1; }                                                   // C 2695
  else
    { return 1; }                                                    // C 2697
}

/// C 2700-2720: `static void merge_and_sort_env (char **envp1, char **envp2, char **new_envp)` -
/// C 2706-2710's first copy loop and its count, C 2712-2715's second, C 2717's `qsort` (C's
/// `sizeof (char *)` is the element size below) and C 2719's terminator.  C's `*nptr++ = *optr++`
/// is the port's assignment plus the two advances (the same order), and the two element counts are
/// C's own pointer differences.
/// C: static void
/// C: merge_and_sort_env (char **envp1, char **envp2, char **new_envp)
#[cfg (feature = "windowsnt")]
unsafe fn merge_and_sort_env (envp1: *mut *mut c_char, envp2: *mut *mut c_char,
                              new_envp: *mut *mut c_char)
{
  let mut optr: *mut *mut c_char;                                    // C 2703
  let mut nptr: *mut *mut c_char;
  let mut num: c_int;                                                // C 2704

  nptr = new_envp;                                                   // C 2706
  optr = envp1;                                                      // C 2707
  while !(*optr).is_null ()                                          // C 2708
    {
      *nptr = *optr;                                                 // C 2709
      nptr = nptr.add (1);
      optr = optr.add (1);
    }
  num = optr.offset_from (envp1) as c_int;                           // C 2710

  optr = envp2;                                                      // C 2712
  while !(*optr).is_null ()                                          // C 2713
    {
      *nptr = *optr;                                                 // C 2714
      nptr = nptr.add (1);
      optr = optr.add (1);
    }
  num += optr.offset_from (envp2) as c_int;                          // C 2715

  qsort (new_envp as *mut c_void, num as usize,
         core::mem::size_of::<*mut c_char> (), Some (compare_env));   // C 2717

  *nptr = core::ptr::null_mut ();                                    // C 2719
}

/* C 2722-3078: `sys_spawnve` - the file's ``spawn'' entry point: the executable-name check and its
   `openp` fallback, the ANSI conversion and the `_mbspbrk` guard, the DOS-app `CMDPROXY` branch,
   the two quoting passes over `argv` (with C 2976-2988's `#if 0` arm carried as C's text) and the
   `envp` pass that merges and sorts the environment, ending at C 3062-3077's `new_child`/
   `create_child` pair.  C's four `alloca` sites are PORTING.md P-05's stack pool `sa_stack_alloc`
   (the `src/w32.rs` 1770-1774 reading: the device is chosen and this unit calls it through
   `include/lisp.rs`'s declaration).  Names no mirror in this unit's graph carries stand at the use
   site with C's own text (P-88 1): `openp` (the `src/callproc.rs` 98-103 device), the CRT's
   `faccessat` and `_mbspbrk`, `GetCurrentProcessId`/`CharPrevA` (SDK), and `ENOEXEC` (the errno
   mirror's three remaining rows, this file's slice 1 note).  */

/* When a new child process is created we need to register it in our list,
   so intercept spawn requests.  */
/// C 2724-3078: `int sys_spawnve (int mode, char *cmdname, char **argv, char **envp)`.
/// C: int
/// C: sys_spawnve (int mode, char *cmdname, char **argv, char **envp)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_spawnve (mode: c_int, cmdname: *mut c_char,
                                      argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int
{
  let mut program: Lisp_Object = Qnil ();                            // C 2727
  let mut full: Lisp_Object = Qnil ();
  let mut cmdline: *mut c_char;                                      // C 2728
  let mut env: *mut c_char;
  let mut parg: *mut c_char;
  let mut targ: *mut *mut c_char;
  let mut arglen: c_int;                                             // C 2729
  let mut numenv: c_int;
  let mut pid: pid_t;                                                // C 2730
  let mut cp: *mut child_process;                                    // C 2731
  let mut is_dos_app: c_int = 0;
  let mut is_cygnus_app: c_int = 0;
  let mut is_msys_app: c_int = 0;
  let mut is_gui_app: c_int = 0;                                     // C 2732
  let mut do_quoting: c_int = 0;                                     // C 2733
  /* We pass our process ID to our children by setting up an environment
     variable in their environment.  */
  let mut ppid_env_var_buffer: [c_char; 64] = [0; 64];               // C 2736
  let mut extra_env: [*mut c_char; 2]
    = [ppid_env_var_buffer.as_mut_ptr (), core::ptr::null_mut ()];   // C 2737
  /* These are the characters that cause an argument to need quoting.
     Arguments with whitespace characters need quoting to prevent the
     argument being split into two or more. Arguments with wildcards
     are also quoted, for consistency with posix platforms, where wildcards
     are not expanded if we run the program directly without a shell.
     Some extra whitespace characters need quoting in Cygwin/MSYS programs,
     so this list is conditionally modified below.  */
  let mut sepchars: *const c_char = c" \t*?".as_ptr ();             // C 2745
  /* This is for native w32 apps; modified below for Cygwin/MSUS apps.  */
  let mut escape_char: c_char = b'\\' as c_char;                    // C 2747
  let mut cmdname_a: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   // C 2748

  /* We don't care about the other modes */
  if mode != _P_NOWAIT                                               // C 2751
    {
      *errno () = EINVAL;                                            // C 2753
      return -1;                                                     // C 2754
    }

  /* Handle executable names without an executable suffix.  The caller
     already searched exec-path and verified the file is executable,
     but start-process doesn't do that for file names that are already
     absolute.  So we double-check this here, just in case.  */
  if faccessat (AT_FDCWD, cmdname, X_OK, AT_EACCESS) != 0            // C 2761
    {
      program = build_string (cmdname);                              // C 2763
      full = Qnil ();                                                // C 2764
      openp (Vexec_path, program, Fdefault_value (Qexec_suffixes),    // C 2765
             &raw mut full, make_fixnum (X_OK), false, false,        // C 2766
             core::ptr::null_mut ());
      if NILP (full)                                                 // C 2767
        {
          *errno () = EINVAL;                                        // C 2769
          return -1;                                                 // C 2770
        }
      program = ENCODE_FILE (full);                                  // C 2772
      cmdname = SSDATA (program);                                    // C 2773
    }
  else
    {
      let p: *mut c_char = sa_stack_alloc (strlen (cmdname) + 1) as *mut c_char;   // C 2777

      /* Don't change the command name we were passed by our caller
	 (unixtodos_filename below will destructively mirror forward
	 slashes).  */
      cmdname = strcpy (p, cmdname);                                 // C 2782
    }

  /* make sure argv[0] and cmdname are both in DOS format */
  unixtodos_filename (cmdname);                                      // C 2786
  /* argv[0] was encoded by caller using ENCODE_FILE, so it is in
     UTF-8.  All the other arguments are encoded by ENCODE_SYSTEM or
     some such, and are in some ANSI codepage.  We need to have
     argv[0] encoded in ANSI codepage.  */
  filename_to_ansi (cmdname, cmdname_a.as_mut_ptr ());               // C 2791
  /* We explicitly require that the command's file name be encodable
     in the current ANSI codepage, because we will be invoking it via
     the ANSI APIs.  */
  if !_mbspbrk (cmdname_a.as_mut_ptr () as *const c_uchar,           // C 2795
                c"?".as_ptr () as *const c_uchar).is_null ()
    {
      *errno () = ENOENT;                                            // C 2797
      return -1;                                                     // C 2798
    }
  /* From here on, CMDNAME is an ANSI-encoded string.  */
  cmdname = cmdname_a.as_mut_ptr ();                                 // C 2801
  *argv = cmdname;                                                   // C 2802

  /* Determine whether program is a 16-bit DOS executable, or a 32-bit
     Windows executable that is implicitly linked to the Cygnus or
     MSYS dll (implying it was compiled with the Cygnus/MSYS GNU
     toolchain and hence relies on cygwin.dll or MSYS DLL to parse the
     command line - we use this to decide how to escape quote chars in
     command line args that must be quoted).

     Also determine whether it is a GUI app, so that we don't hide its
     initial window unless specifically requested.  */
  w32_executable_type (cmdname, &raw mut is_dos_app, &raw mut is_cygnus_app,
                       &raw mut is_msys_app, &raw mut is_gui_app);   // C 2813-2814

  /* On Windows 95, if cmdname is a DOS app, we invoke a helper
     application to start it by specifying the helper app as cmdname,
     while leaving the real app name as argv[0].  */
  if is_dos_app != 0                                                 // C 2819
    {
      let mut p: *mut c_char;                                        // C 2821

      cmdname = sa_stack_alloc (MAX_PATH as usize) as *mut c_char;   // C 2823
      if !egetenv (c"CMDPROXY".as_ptr ()).is_null ()                 // C 2824
        {
	  /* Implementation note: since process-environment, where
	     'egetenv' looks, is encoded in the system codepage, we
	     don't need to encode the cmdproxy file name if we get it
	     from the environment.  */
          strcpy (cmdname, egetenv (c"CMDPROXY".as_ptr ()));         // C 2830
        }
      else
        {
          let mut q: *mut c_char = lispstpcpy (cmdname,              // C 2834
                                               /* exec-directory needs to be encoded.  */
                                               ansi_encode_filename (Vexec_directory));   // C 2836

	  /* If we are run from the source tree, use cmdproxy.exe from
	     the same source tree.  */
          p = q.offset (-2);                                         // C 2839
          while p > cmdname
            {
              if *p == b'/' as c_char { break; }                     // C 2840-2841
              p = CharPrevA (cmdname, p);
            }
          if *p == b'/' as c_char                                    // C 2842
              && xstrcasecmp (p, c"/lib-src/".as_ptr ()) == 0
            { q = stpcpy (p, c"/nt/".as_ptr ()); }                   // C 2843
          strcpy (q, c"cmdproxy.exe".as_ptr ());                     // C 2844
        }

      /* Can't use unixtodos_filename here, since that needs its file
	 name argument encoded in UTF-8.  */
      p = cmdname;                                                   // C 2849
      while *p != 0
        {
          if *p == b'/' as c_char { *p = b'\\' as c_char; }        // C 2850-2851
          p = CharNextA (p);
        }
    }

  /* we have to do some conjuring here to put argv and envp into the
     form CreateProcess wants...  argv needs to be a space separated/null
     terminated list of parameters, and envp is a null
     separated/double-null terminated list of parameters.

     Additionally, zero-length args and args containing whitespace or
     quote chars need to be wrapped in double quotes - for this to work,
     embedded quotes need to be escaped as well.  The aim is to ensure
     the child process reconstructs the argv array we start with
     exactly, so we treat quotes at the beginning and end of arguments
     as embedded quotes.

     The w32 GNU-based library from Cygnus doubles quotes to escape
     them, while MSVC uses backslash for escaping.  (Actually the MSVC
     startup code does attempt to recognize doubled quotes and accept
     them, but gets it wrong and ends up requiring three quotes to get a
     single embedded quote!)  So by default we decide whether to use
     quote or backslash as the escape character based on whether the
     binary is apparently a Cygnus compiled app.

     Note that using backslash to escape embedded quotes requires
     additional special handling if an embedded quote is already
     preceded by backslash, or if an arg requiring quoting ends with
     backslash.  In such cases, the run of escape characters needs to be
     doubled.  For consistency, we apply this special handling as long
     as the escape character is not quote.

     Since we have no idea how large argv and envp are likely to be we
     figure out list lengths on the fly and allocate them.  */

  if !NILP (Vw32_quote_process_args)                                 // C 2884
    {
      do_quoting = 1;                                                // C 2886
      /* Override escape char by binding w32-quote-process-args to
	 desired character, or use t for auto-selection.  */
      if FIXNUMP (Vw32_quote_process_args)                           // C 2889
        { escape_char = XFIXNUM (Vw32_quote_process_args) as c_char; }   // C 2890
      else
        { escape_char = if is_cygnus_app != 0 || is_msys_app != 0    // C 2892
                          { b'"' as c_char } else { b'\\' as c_char }; }
    }

  /* Cygwin/MSYS apps need quoting a bit more often.  */
  if escape_char == b'"' as c_char                                   // C 2896
    { sepchars = c"\r\n\t\x0c '".as_ptr (); }   /* C's \f is Rust's \x0c */    // C 2897

  /* do argv...  */
  arglen = 0;                                                        // C 2900
  targ = argv;                                                       // C 2901
  while !(*targ).is_null ()                                          // C 2902
    {
      let mut p: *mut c_char = *targ;                                // C 2904
      let mut need_quotes: c_int = 0;                                // C 2905
      let mut escape_char_run: c_int = 0;                            // C 2906

      if *p == 0                                                     // C 2908
        { need_quotes = 1; }                                         // C 2909
      while *p != 0                                                  // C 2910
        {
          if escape_char == b'"' as c_char && *p == b'\\' as c_char   // C 2912
            /* If it's a Cygwin/MSYS app, \ needs to be escaped.  */
            { arglen += 1; }                                         // C 2914
          else if *p == b'"' as c_char                               // C 2915
            {
              /* allow for embedded quotes to be escaped */
              arglen += 1;                                           // C 2918
              need_quotes = 1;                                       // C 2919
              /* handle the case where the embedded quote is already escaped */
              if escape_char_run > 0                                 // C 2921
                {
		  /* To preserve the arg exactly, we need to double the
		     preceding escape characters (plus adding one to
		     escape the quote character itself).  */
                  arglen += escape_char_run;                         // C 2926
                }
            }
          else if !strchr (sepchars, *p as c_int).is_null ()         // C 2929
            {
              need_quotes = 1;                                       // C 2931
            }

          if *p == escape_char && escape_char != b'"' as c_char      // C 2934
            { escape_char_run += 1; }                                // C 2935
          else
            { escape_char_run = 0; }                                 // C 2937
          p = p.add (1);
        }
      if need_quotes != 0                                            // C 2939
        {
          arglen += 2;                                               // C 2941
	  /* handle the case where the arg ends with an escape char - we
	     must not let the enclosing quote be escaped.  */
          if escape_char_run > 0                                     // C 2944
            { arglen += escape_char_run; }                           // C 2945
        }
      arglen += strlen (*targ) as c_int + 1;                         // C 2947
      targ = targ.add (1);
    }
  cmdline = sa_stack_alloc (arglen as usize) as *mut c_char;         // C 2949
  targ = argv;                                                       // C 2950
  parg = cmdline;                                                    // C 2951
  while !(*targ).is_null ()                                          // C 2952
    {
      let mut p: *mut c_char = *targ;                                // C 2954
      let mut need_quotes: c_int = 0;                                // C 2955

      if *p == 0                                                     // C 2957
        { need_quotes = 1; }                                         // C 2958

      if do_quoting != 0                                             // C 2960
        {
          while *p != 0                                              // C 2962
            {
              if !strchr (sepchars, *p as c_int).is_null ()          // C 2963
                  || *p == b'"' as c_char
                { need_quotes = 1; }                                 // C 2964
              p = p.add (1);
            }
        }
      if need_quotes != 0                                            // C 2966
        {
          let mut escape_char_run: c_int = 0;                        // C 2968
          /* char * first; */
          /* char * last; */

          p = *targ;                                                 // C 2972
          /* first = p; */
          /* last = p + strlen (p) - 1; */
          *parg = b'"' as c_char; parg = parg.add (1);                // C 2975
          /* C 2976-2988: the first (commented-out) arm of C's
             `#if 0`/`#else` pair, carried as C's own text (P-51).  The live arm below is C 2989's
             `#else`.  */
          /*
#if 0
	  /* This version does not escape quotes if they occur at the
	     beginning or end of the arg - this could lead to incorrect
	     behavior when the arg itself represents a command line
	     containing quoted args.  I believe this was originally done
	     as a hack to make some things work, before
	     `w32-quote-process-args' was added.  */
	  while (*p)
	    {
	      if (*p == '"' && p > first && p < last)
		*parg++ = escape_char;	/* escape embedded quotes */
	      *parg++ = *p++;
	    }
          */
          while *p != 0                                              // C 2990
            {
              if *p == b'"' as c_char                                // C 2992
                {
                  /* double preceding escape chars if any */
                  while escape_char_run > 0                          // C 2995
                    {
                      *parg = escape_char; parg = parg.add (1);      // C 2997
                      escape_char_run -= 1;                          // C 2998
                    }
                  /* escape all quote chars, even at beginning or end */
                  *parg = escape_char; parg = parg.add (1);          // C 3001
                }
              else if escape_char == b'"' as c_char && *p == b'\\' as c_char
                { *parg = b'\\' as c_char; parg = parg.add (1); }  // C 3003-3004
              *parg = *p; parg = parg.add (1);                       // C 3005

              if *p == escape_char && escape_char != b'"' as c_char   // C 3007
                { escape_char_run += 1; }                            // C 3008
              else
                { escape_char_run = 0; }                             // C 3010
              p = p.add (1);
            }
          /* double escape chars before enclosing quote */
          while escape_char_run > 0                                   // C 3013
            {
              *parg = escape_char; parg = parg.add (1);               // C 3015
              escape_char_run -= 1;                                   // C 3016
            }
          *parg = b'"' as c_char; parg = parg.add (1);                // C 3019
        }
      else
        {
          strcpy (parg, *targ);                                       // C 3023
          parg = parg.add (strlen (*targ));                           // C 3024
        }
      *parg = b' ' as c_char; parg = parg.add (1);                    // C 3026
      targ = targ.add (1);                                            // C 3027
    }
  parg = parg.sub (1);
  *parg = 0;                                                          // C 3029

  /* and envp...  */
  arglen = 1;                                                        // C 3032
  targ = envp;                                                        // C 3033
  numenv = 1; /* for end null */                                     // C 3034
  while !(*targ).is_null ()                                          // C 3035
    {
      arglen += strlen (*targ) as c_int + 1;                         // C 3037
      targ = targ.add (1);
      numenv += 1;                                                   // C 3038
    }
  /* extra env vars... */
  sprintf (ppid_env_var_buffer.as_mut_ptr (),                        // C 3041
           c"EM_PARENT_PROCESS_ID=%lu".as_ptr (), GetCurrentProcessId ());   // C 3042
  arglen += strlen (ppid_env_var_buffer.as_ptr ()) as c_int + 1;     // C 3043
  numenv += 1;                                                        // C 3044

  /* merge env passed in and extra env into one, and sort it.  */
  targ = sa_stack_alloc (numenv as usize * core::mem::size_of::<*mut c_char> ())
    as *mut *mut c_char;                                             // C 3047
  merge_and_sort_env (envp, extra_env.as_mut_ptr (), targ);           // C 3048

  /* concatenate env entries.  */
  env = sa_stack_alloc (arglen as usize) as *mut c_char;              // C 3051
  parg = env;                                                         // C 3052
  while !(*targ).is_null ()                                          // C 3053
    {
      strcpy (parg, *targ);                                           // C 3055
      parg = parg.add (strlen (*targ));                               // C 3056
      targ = targ.add (1);
      *parg = 0; parg = parg.add (1);                                 // C 3057
    }
  *parg = 0; parg = parg.add (1);                                     // C 3059
  *parg = 0;                                                          // C 3060

  cp = new_child ();                                                  // C 3062
  if cp.is_null ()                                                    // C 3063
    {
      *errno () = EAGAIN;                                             // C 3065
      return -1;                                                      // C 3066
    }

  /* Now create the process.  */
  if create_child (cmdname, cmdline, env, is_gui_app, &raw mut pid, cp) == 0   // C 3070
    {
      delete_child (cp);                                              // C 3072
      *errno () = ENOEXEC;                                            // C 3073
      return -1;                                                      // C 3074
    }

  return pid;                                                         // C 3077
}

/* C's CRT and SDK surface this round needs and no mirror in this unit's graph carries (P-88 1):
   `openp` (the `src/callproc.rs` 98-103 shape, from `lisp.h` 4743/lread.c 1756), the gnulib
   `faccessat` and its `AT_FDCWD`/`AT_EACCESS` (the latter two are in this unit's graph through
   `nt/inc/include/unistd.rs` 31 and `lib/include/fcntl.in.rs` 34), the CRT's `_mbspbrk` and the
   SDK's `GetCurrentProcessId`/`CharPrevA`.  `ENOEXEC` is *not* here: `lib/include/errno.rs` 116
   carries it (the note above).  */
/// C: int faccessat (int fd, const char *file, int mode, int flag);
/// C: int openp (Lisp_Object path, Lisp_Object str, Lisp_Object suffixes, Lisp_Object *storeptr,
/// C:            Lisp_Object predicate, bool newer, bool no_native, void **platform);
/// C: unsigned char *_mbspbrk (const unsigned char *, const unsigned char *);
/// C: DWORD WINAPI GetCurrentProcessId (void);
/// C: LPSTR WINAPI CharPrevA (LPCSTR lpszStart, LPCSTR lpszCurrent);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn faccessat (fd: c_int, file: *const c_char, mode: c_int, flag: c_int) -> c_int;
  pub fn openp (path: Lisp_Object, str_: Lisp_Object, suffixes: Lisp_Object,
                storeptr: *mut Lisp_Object, predicate: Lisp_Object, newer: bool,
                no_native: bool, platform: *mut *mut c_void) -> c_int;
  pub fn _mbspbrk (s1: *const c_uchar, s2: *const c_uchar) -> *mut c_uchar;
}
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetCurrentProcessId () -> DWORD;
  pub fn CharPrevA (lpszStart: LPCSTR, lpszCurrent: LPCSTR) -> LPSTR;
}

/* C 3099-3106: the three extern objects `sys_select` (C 3108, the next slice) reads - C 3099's
   `keyboard_handle` (`w32console.c`), C 3102's `interrupt_handle` (`w32xfns.c`) and C 3105's
   `proc_buffered_char` (`process.c`).  No mirror in this unit's graph declares any of them (that is
   why C declares them here), and the array's length is C's own `FD_SETSIZE` - the mirror's 2048,
   spelled `w32_system_names::FD_SETSIZE` as slice 2's `WFO_MAX_WAIT` is (Deviation 36); the
   definitions are `src/process.rs` 489 (the array, whose `-1` fill is its own C's) and the two
   units of `w32console.c`/`w32xfns.c`.  C's own one-line comments stand above each declaration and
   each declaration is cited in its `/// C:` line (the file's rule for a declaration).  */
/* From w32console.c */
/// C: extern HANDLE keyboard_handle;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut keyboard_handle: HANDLE;
}
/* From w32xfns.c */
/// C: extern HANDLE interrupt_handle;
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut interrupt_handle: HANDLE;
}
/* From process.c */
/// C: extern int proc_buffered_char[];
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut proc_buffered_char: [c_int; w32_system_names::FD_SETSIZE as usize];
}

/* C 3080-3097's `sys_select` overview comment travels with C 3108's function, which opens the next
   slice (the file's rule that a C comment belongs to the item it documents).  */
/* Emulate the select call.
   Wait for available input on any of the given rfds, or timeout if
   a timeout is given and no input is detected.  wfds are supported
   only for asynchronous 'connect' calls.  efds are not supported
   and must be NULL.

   For simplicity, we detect the death of child processes here and
   synchronously call the SIGCHLD handler.  Since it is possible for
   children to be created without a corresponding pipe handle from which
   to read output, we wait separately on the process handles as well as
   the char_avail events for each process pipe.  We only call
   wait/reap_process when the process actually terminates.

   To reduce the number of places in which Emacs can be hung such that
   C-g is not able to interrupt it, we always wait on interrupt_handle
   (which is signaled by the input thread when C-g is detected).  If we
   detect that we were woken up by C-g, we return -1 with errno set to
   EINTR as on Unix.  */
/// C 3108-3483: `int sys_select (int nfds, SELECT_TYPE *rfds, SELECT_TYPE *wfds, SELECT_TYPE
/// *efds, const struct timespec *timeout, const sigset_t *ignored)` - the file's longest function.
/// C 3112-3115's four function-local statics (`orfds`/`owfds`, `cps`, `wait_hnd`, `fdindex`), C
/// 3121-3151's timeout decode and the two set copies plus the two `FD_ZERO`s, C 3154-3162's
/// `interrupt_handle` entry, C 3165-3274's descriptor scan (C 3168-3190's stdin arm with
/// `detect_input_pending`/`handle_file_notifications`, C 3192-3272's child/socket arm with the
/// `STATUS_READ_ACKNOWLEDGED` handshake), C 3276-3292's `count_children` label and its child scan,
/// C 3307-3314's wait, C 3316-3346's five-way answer, C 3352-3461's fairness `do`/`while`, C
/// 3463-3470's noninteractive drain and the timeout recheck, and C 3482's `return nr`.  C's two
/// `goto count_children`-shaped jumps are one labeled `loop` (the `start:`/`goto start` device) and
/// the `do`/`while` is the port's negated `break` (P-95, `check-do-while.py`).  C's `CHECK_INTERLOCK`
/// and two `FULL_DEBUG` arms are carried as C's text at their positions.
/// C: int
/// C: sys_select (int nfds, SELECT_TYPE *rfds, SELECT_TYPE *wfds, SELECT_TYPE *efds,
/// C:             const struct timespec *timeout, const sigset_t *ignored)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_select (nfds: c_int, rfds: *mut SELECT_TYPE! (),
                                     wfds: *mut SELECT_TYPE! (), efds: *mut SELECT_TYPE! (),
                                     timeout: *const timespec, ignored: *const sigset_t) -> c_int
{
  static mut orfds: SELECT_TYPE! () = unsafe { core::mem::zeroed () };    // C 3112
  static mut owfds: SELECT_TYPE! () = unsafe { core::mem::zeroed () };
  static mut cps: [*mut child_process; MAX_CHILDREN as usize]             // C 3113
    = [core::ptr::null_mut (); MAX_CHILDREN as usize];
  static mut wait_hnd: [HANDLE; (MAXDESC + MAX_CHILDREN) as usize]        // C 3114
    = [core::ptr::null_mut (); (MAXDESC + MAX_CHILDREN) as usize];
  static mut fdindex: [c_int; MAXDESC as usize] = [0; MAXDESC as usize];   // C 3115
  let mut timeout_ms: DWORD;                                              // C 3116
  let mut start_time: DWORD = 0;
  let mut i: c_int = 0;                                                   // C 3117
  let mut nh: c_int = 0;
  let mut nc: c_int = 0;
  let mut nr: c_int = 0;
  let mut active: DWORD = 0;                                              // C 3118
  let mut cp: *mut child_process = core::ptr::null_mut ();                // C 3119

  timeout_ms = if !timeout.is_null ()                                     // C 3121-3122
    { ((*timeout).tv_sec * 1000 + (*timeout).tv_nsec / 1000000) as DWORD }
    else { INFINITE };

  /* If the descriptor sets are NULL but timeout isn't, then just Sleep.  */
  if rfds.is_null () && wfds.is_null () && efds.is_null () && !timeout.is_null ()   // C 3125
    {
      Sleep (timeout_ms);                                                 // C 3127
      return 0;                                                           // C 3128
    }

  /* Otherwise, we only handle rfds and wfds, so fail otherwise.  */
  if (rfds.is_null () && wfds.is_null ()) || !efds.is_null ()              // C 3132
    {
      *errno () = EINVAL;                                                 // C 3134
      return -1;                                                          // C 3135
    }

  if !rfds.is_null ()                                                     // C 3138
    {
      orfds = *rfds;                                                      // C 3140
      FD_ZERO! (rfds);                                                    // C 3141
    }
  else
    { FD_ZERO! (&raw mut orfds); }                                        // C 3144
  if !wfds.is_null ()                                                     // C 3145
    {
      owfds = *wfds;                                                      // C 3147
      FD_ZERO! (wfds);                                                    // C 3148
    }
  else
    { FD_ZERO! (&raw mut owfds); }                                        // C 3151
  nr = 0;                                                                 // C 3152

  /* If interrupt_handle is available and valid, always wait on it, to
     detect C-g (quit).  */
  nh = 0;                                                                 // C 3156
  if !interrupt_handle.is_null () && interrupt_handle != INVALID_HANDLE_VALUE   // C 3157
    {
      wait_hnd[0] = interrupt_handle;                                     // C 3159
      fdindex[0] = -1;                                                    // C 3160
      nh += 1;                                                            // C 3161
    }

  /* Build a list of pipe handles to wait on.  */
  i = 0;                                                                  // C 3165
  while i < nfds
    {
      if FD_ISSET! (i, &orfds) || FD_ISSET! (i, &owfds)                   // C 3166
        {
          if i == 0                                                       // C 3168
            {
              if !keyboard_handle.is_null ()                              // C 3170
                {
                  /* Handle stdin specially */
                  wait_hnd[nh as usize] = keyboard_handle;                // C 3173
                  fdindex[nh as usize] = i;                               // C 3174
                  nh += 1;                                                // C 3175
                }

              /* Check for any emacs-generated input in the queue since
                 it won't be detected in the wait */
              if !rfds.is_null () && detect_input_pending ()              // C 3180
                {
                  FD_SET! (i, rfds);                                      // C 3182
                  return 1;                                               // C 3183
                }
              else if noninteractive != 0                                 // C 3185
                {
                  if handle_file_notifications (core::ptr::null_mut ()) != 0   // C 3187
                    { return 1; }                                         // C 3188
                }
            }
          else
            {
              /* Child process and socket/comm port input.  */
              cp = fd_info[i as usize].cp as *mut child_process;           // C 3194
              if FD_ISSET! (i, &owfds)                                    // C 3195
                  && !cp.is_null ()                                       // C 3196
                  && (fd_info[i as usize].flags & FILE_CONNECT as c_uint) == 0   // C 3197
                {
                  DebPrint! (("sys_select: fd %d is in wfds, but FILE_CONNECT is reset!\n", i));   // C 3199
                  cp = core::ptr::null_mut ();                            // C 3200
                }
              if !cp.is_null ()                                           // C 3202
                {
                  let mut current_status: c_int = (*cp).status;            // C 3204

                  if current_status == STATUS_READ_ACKNOWLEDGED            // C 3206
                    {
                      /* Tell reader thread which file handle to use. */
                      (*cp).fd = i;                                       // C 3209
                      /* Zero out the error code.  */
                      (*cp).errcode = 0;                                  // C 3211
                      /* Wake up the reader thread for this process */
                      (*cp).status = STATUS_READ_READY;                   // C 3213
                      if SetEvent ((*cp).char_consumed) == 0              // C 3214
                        { DebPrint! (("sys_select.SetEvent failed with "
                                      "%lu for fd %ld\n", GetLastError (), i)); }   // C 3215-3216
                    }

                  /* C 3219-3248: the `CHECK_INTERLOCK` arm (a debug-only macro no configuration
                     defines), carried as C's own text (P-51).  The declarations below it keep C's
                     names: `current_status` is C 3222/3228's own assignment.  */
                  /*
#ifdef CHECK_INTERLOCK
		/* slightly crude cross-checking of interlock between threads */

		current_status = cp->status;
		if (WaitForSingleObject (cp->char_avail, 0) == WAIT_OBJECT_0)
		  {
		    /* char_avail has been signaled, so status (which may
		       have changed) should indicate read has completed
		       but has not been acknowledged. */
		    current_status = cp->status;
		    if (current_status != STATUS_READ_SUCCEEDED
			&& current_status != STATUS_READ_FAILED)
		      DebPrint (("char_avail set, but read not completed: status %d\n",
				 current_status));
		  }
		else
		  {
		    /* char_avail has not been signaled, so status should
		       indicate that read is in progress; small possibility
		       that read has completed but event wasn't yet signaled
		       when we tested it (because a context switch occurred
		       or if running on separate CPUs). */
		    if (current_status != STATUS_READ_READY
			&& current_status != STATUS_READ_IN_PROGRESS
			&& current_status != STATUS_READ_SUCCEEDED
			&& current_status != STATUS_READ_FAILED)
		      DebPrint (("char_avail reset, but read status is bad: %d\n",
				 current_status));
		  }
#endif
                  */
                  wait_hnd[nh as usize] = (*cp).char_avail;               // C 3249
                  fdindex[nh as usize] = i;                               // C 3250
                  if wait_hnd[nh as usize].is_null () { emacs_abort (); }   // C 3251
                  nh += 1;                                               // C 3252
                  /* C 3253-3256: the dead `FULL_DEBUG` arm.  */
                  /*
#ifdef FULL_DEBUG
		DebPrint (("select waiting on child %d fd %d\n",
			   cp-child_procs, i));
#endif
                  */
                }
              else
                {
                  /* Unable to find something to wait on for this fd, skip */

		/* Note that this is not a fatal error, and can in fact
		   happen in unusual circumstances.  Specifically, if
		   sys_spawnve fails, eg. because the program doesn't
		   exist, and debug-on-error is t so Fsignal invokes a
		   nested input loop, then the process output pipe is
		   still included in input_wait_mask with no child_proc
		   associated with it.  (It is removed when the debugger
		   exits the nested input loop and the error is thrown.)  */
                  DebPrint! (("sys_select: fd %ld is invalid! ignoring\n", i));   // C 3271
                }
            }
        }
      i += 1;
    }

  /* C 3276's `count_children:` label - C's two `goto count_children` jumps (C 3478) are this
     loop's `continue`.  */
  'count_children: loop
    {
      /* Add handles of child processes.  */
      nc = 0;                                                            // C 3278
      let base: *mut child_process = child_procs.as_mut_ptr ();
      cp = base.add ((child_proc_count - 1) as usize);                   // C 3279
      while cp >= base
        {
    /* Some child_procs might be sockets; ignore them.  Also some
       children may have died already, but we haven't finished reading
       the process output; ignore them too.  */
          if CHILD_ACTIVE! (*cp) && !(*cp).procinfo.hProcess.is_null ()   // C 3283
              && ((*cp).fd < 0                                            // C 3284
                  || (fd_info[(*cp).fd as usize].flags & FILE_SEND_SIGCHLD as c_uint) == 0   // C 3285
                  || (fd_info[(*cp).fd as usize].flags & FILE_AT_EOF as c_uint) != 0)        // C 3286
            {
              wait_hnd[(nh + nc) as usize] = (*cp).procinfo.hProcess;     // C 3289
              cps[nc as usize] = cp;                                      // C 3290
              nc += 1;                                                    // C 3291
            }
          cp = cp.sub (1);
        }

      /* Nothing to look for, so we didn't find anything */
      if nh + nc == 0                                                     // C 3295
        {
          if !timeout.is_null ()                                          // C 3297
            { Sleep (timeout_ms); }                                       // C 3298
          if noninteractive != 0                                          // C 3299
            {
              if handle_file_notifications (core::ptr::null_mut ()) != 0   // C 3301
                { return 1; }                                             // C 3302
            }
          return 0;                                                       // C 3304
        }

      start_time = GetTickCount ();                                       // C 3307

      /* Wait for input or child death to be signaled.  If user input is
         allowed, then also accept window messages.  */
      if FD_ISSET! (0, &orfds)                                            // C 3311
        {
          active = msg_wait_for_objects ((nh + nc) as DWORD, wait_hnd.as_mut_ptr (),
                                         timeout_ms);                     // C 3312
        }
      else
        {
          active = wait_for_objects ((nh + nc) as DWORD, wait_hnd.as_mut_ptr (),
                                     timeout_ms);                         // C 3314
        }

      if active == WFO_FAILED                                             // C 3316
        {
          DebPrint! (("select.WaitForMultipleObjects (%d, %lu) failed with %lu\n",
                      nh + nc, timeout_ms, GetLastError ()));             // C 3318-3319
      /* don't return EBADF - this causes wait_reading_process_output to
	 abort; WAIT_FAILED is returned when single-stepping under
	 Windows 95 after switching thread focus in debugger, and
	 possibly at other times. */
          *errno () = EINTR;                                              // C 3324
          return -1;                                                      // C 3325
        }
      else if active == WFO_TIMEOUT                                       // C 3327
        {
          if noninteractive != 0                                          // C 3329
            {
              if handle_file_notifications (core::ptr::null_mut ()) != 0   // C 3331
                { return 1; }                                             // C 3332
            }
          return 0;                                                       // C 3334
        }
      else if active >= 0 && active < WFO_MAX_WAIT as DWORD               // C 3336
        {
          ;                                                               // C 3338
        }
      else if active >= WFO_ABANDONED                                     // C 3340
               && active < WFO_ABANDONED + WFO_MAX_WAIT as DWORD          // C 3341
        {
          active -= WFO_ABANDONED;                                        // C 3343
        }
      else
        { emacs_abort (); }                                               // C 3346

  /* Loop over all handles after active (now officially documented as
     being the first signaled handle in the array).  We do this to
     ensure fairness, so that all channels with data available will be
     processed - otherwise higher numbered channels could be starved. */
      loop                                                                // C 3352
        {
          if active == (nh + nc) as DWORD                                 // C 3354
            {
	  /* There are messages in the lisp thread's queue; we must
             drain the queue now to ensure they are processed promptly,
             because if we don't do so, we will not be woken again until
             further messages arrive.

	     NB. If ever we allow window message procedures to callback
	     into lisp, we will need to ensure messages are dispatched
	     at a safe time for lisp code to be run (*), and we may also
	     want to provide some hooks in the dispatch loop to cater
	     for modeless dialogs created by lisp (ie. to register
	     window handles to pass to IsDialogMessage).

	     (*) Note that MsgWaitForMultipleObjects above is an
	     internal dispatch point for messages that are sent to
	     windows created by this thread.  */
              if drain_message_queue () != 0                              // C 3371
	      /* If drain_message_queue returns non-zero, that means
		 we received a WM_EMACS_FILENOTIFY message.  If this
		 is a TTY frame, we must signal the caller that keyboard
		 input is available, so that w32_console_read_socket
		 will be called to pick up the notifications.  If we
		 don't do that, file notifications will only work when
		 the Emacs TTY frame has focus.  */
                  && FRAME_TERMCAP_P (SELECTED_FRAME ())                  // C 3379
                  /* they asked for stdin reads */
                  && FD_ISSET! (0, &orfds)                                // C 3381
                  /* the stdin handle is valid */
                  && !keyboard_handle.is_null ()                          // C 3383
                {
                  FD_SET! (0, rfds);                                      // C 3385
                  if nr == 0                                              // C 3386
                    { nr = 1; }                                           // C 3387
                }
            }
          else if active >= nh as DWORD                                   // C 3390
            {
              cp = cps[(active - nh as DWORD) as usize];                  // C 3392

	  /* We cannot always signal SIGCHLD immediately; if we have not
	     finished reading the process output, we must delay sending
	     SIGCHLD until we do.  */

              if (*cp).fd >= 0                                            // C 3398
                  && (fd_info[(*cp).fd as usize].flags & FILE_AT_EOF as c_uint) == 0
                { fd_info[(*cp).fd as usize].flags |= FILE_SEND_SIGCHLD as c_uint; }   // C 3399
              /* SIG_DFL for SIGCHLD is ignored */
              else if sig_handlers[SIGCHLD as usize] != SIG_DFL           // C 3401
                      && sig_handlers[SIGCHLD as usize] != SIG_IGN        // C 3402
                {
                  /* C 3404-3407: the dead `FULL_DEBUG` arm.  */
                  /*
#ifdef FULL_DEBUG
	      DebPrint (("select calling SIGCHLD handler for pid %d\n",
			 cp->pid));
#endif
                  */
                  sig_handlers[SIGCHLD as usize].unwrap () (SIGCHLD);      // C 3408
                }
            }
          else if fdindex[active as usize] == -1                          // C 3411
            {
              /* Quit (C-g) was detected.  */
              *errno () = EINTR;                                          // C 3414
              return -1;                                                  // C 3415
            }
          else if !rfds.is_null () && fdindex[active as usize] == 0        // C 3417
            {
              /* Keyboard input available */
              FD_SET! (0, rfds);                                          // C 3420
              nr += 1;                                                    // C 3421
            }
          else
            {
	  /* Must be a socket or pipe - read ahead should have
             completed, either succeeding or failing.  If this handle
             was waiting for an async 'connect', reset the connect
             flag, so it could read from now on.  */
              if !wfds.is_null ()                                         // C 3429
                  && (fd_info[fdindex[active as usize] as usize].flags
                      & FILE_CONNECT as c_uint) != 0
                {
                  cp = fd_info[fdindex[active as usize] as usize].cp
                    as *mut child_process;                                // C 3431
                  if !cp.is_null ()                                       // C 3432
                    {
		  /* Don't reset the FILE_CONNECT bit and don't
		     acknowledge the read if the status is
		     STATUS_CONNECT_FAILED or some other
		     failure. That's because the thread exits in those
		     cases, so it doesn't need the ACK, and we want to
		     keep the FILE_CONNECT bit as evidence that the
		     connect failed, to be checked in sys_read.  */
                      if (*cp).status == STATUS_READ_SUCCEEDED            // C 3441
                        {
                          fd_info[(*cp).fd as usize].flags
                            &= !(FILE_CONNECT as c_uint);                 // C 3443
                          (*cp).status = STATUS_READ_ACKNOWLEDGED;        // C 3444
                        }
                      ResetEvent ((*cp).char_avail);                      // C 3446
                    }
                  FD_SET! (fdindex[active as usize], wfds);               // C 3448
                }
              else if !rfds.is_null ()
                { FD_SET! (fdindex[active as usize], rfds); }             // C 3451
              nr += 1;                                                    // C 3452
            }

      /* Even though wait_reading_process_output only reads from at most
	 one channel, we must process all channels here so that we reap
	 all children that have died.  */
          active += 1;                                                    // C 3458
          while active < (nh + nc) as DWORD
            {
              if WaitForSingleObject (wait_hnd[active as usize], 0)       // C 3459
                  == WAIT_OBJECT_0
                { break; }                                                // C 3460
              active += 1;
            }
          if !(active < (nh + nc) as DWORD) { break; }                    // C 3461
        }

      if noninteractive != 0                                              // C 3463
        {
          if handle_file_notifications (core::ptr::null_mut ()) != 0       // C 3465
            { nr += 1; }                                                  // C 3466
        }

      /* If no input has arrived and timeout hasn't expired, wait again.  */
      if nr == 0                                                          // C 3470
        {
          let elapsed: DWORD = GetTickCount () - start_time;              // C 3472

          if timeout_ms > elapsed                                         // C 3474
            {
              if timeout_ms != INFINITE                                   // C 3476
                { timeout_ms -= elapsed; }                                // C 3477
              continue 'count_children;                                   // C 3478 (C's `goto`)
            }
        }

      return nr;                                                          // C 3482
    }
}

/* C's `<keyboard.h>`/`<winbase.h>`: the two entries this function calls and no mirror in this
   unit's graph carries (`handle_file_notifications` and `drain_message_queue` are in the
   `w32term.rs` mirror).  */
/// C: extern bool detect_input_pending (void);   [lisp.h 5095]
/// C: DWORD WINAPI GetTickCount (void);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn detect_input_pending () -> bool;
}
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetTickCount () -> DWORD;
}

/* C 3485-3526: the three items between `sys_select` and `sys_kill` (C 3528, the next slice) - C
   3485-3496's `w32_valid_process_id`, C 3498-3523's `find_child_console` and C 3525-3526's
   `DebugBreakProcess_Proc`.  `os_subtype`/`OS_SUBTYPE_9X` are the `w32common.rs` mirror's, so the
   round declares only the two `<winuser.h>` entry points and the `LPARAM` they need (a `LONG_PTR`
   at the ABI, spelled `isize`; P-88 1's `UNVERIFIED` class).  */

/* This is called from filelock.c:current_lock_owner to validate a PID.
   Return true if PID could identify a process on the current host,
   false otherwise.  */
/// C 3488-3496: `bool w32_valid_process_id (intmax_t id)` - C 3491-3493's four-way test (C's
/// `UINT32_MAX`/`INT32_MIN` are the `stdint.in.rs` mirror's).
/// C: bool
/// C: w32_valid_process_id (intmax_t id)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn w32_valid_process_id (id: intmax_t) -> bool
{
  if id == -1 || id == 0                                             // C 3491
      || id > UINT32_MAX as intmax_t                                 // C 3492
      || id < INT32_MIN as intmax_t                                  // C 3493
    { return false; }                                                // C 3494
  return true;                                                       // C 3495
}

/* Substitute for certain kill () operations */

/// C 3500-3523: `static BOOL CALLBACK find_child_console (HWND hwnd, LPARAM arg)` - C 3503's cast
/// of the enumeration argument, C 3506's process-id read, C 3507's match, C 3509-3515's
/// `GetClassName`/`strcmp` pair with C 3513's `os_subtype` choice, and C 3517-3518's record of the
/// window (C's `return FALSE` stops the enumeration).
/// C: static BOOL CALLBACK
/// C: find_child_console (HWND hwnd, LPARAM arg)
#[cfg (feature = "windowsnt")]
unsafe extern "system" fn find_child_console (hwnd: HWND, arg: LPARAM) -> BOOL
{
  let cp: *mut child_process = arg as *mut child_process;            // C 3503
  let mut process_id: DWORD = 0;                                     // C 3504

  GetWindowThreadProcessId (hwnd, &raw mut process_id);              // C 3506
  if process_id == (*cp).procinfo.dwProcessId                        // C 3507
    {
      let mut window_class: [c_char; 32] = [0; 32];                  // C 3509

      GetClassName (hwnd, window_class.as_mut_ptr (),                // C 3511
                    core::mem::size_of::<[c_char; 32]> () as c_int);
      if strcmp (window_class.as_ptr (),                             // C 3512
                 if os_subtype == OS_SUBTYPE_9X                      // C 3513
                   { c"tty".as_ptr () } else { c"ConsoleWindowClass".as_ptr () }) == 0   // C 3514-3515
        {
          (*cp).hwnd = hwnd;                                         // C 3517
          return FALSE;                                              // C 3518
        }
    }
  /* keep looking */
  return TRUE;                                                       // C 3522
}

/// C: typedef BOOL (WINAPI * DebugBreakProcess_Proc) (HANDLE hProcess);
#[cfg (feature = "windowsnt")]
pub type DebugBreakProcess_Proc
  = Option<unsafe extern "system" fn (hProcess: HANDLE) -> BOOL>;    // C 3525-3526

/// C: typedef LONG_PTR LPARAM;   [`<windef.h>`; pointer-sized, `isize`]
/// C: WINUSERAPI DWORD WINAPI GetWindowThreadProcessId (HWND hWnd, LPDWORD lpdwProcessId);
/// C: WINUSERAPI int WINAPI GetClassNameA (HWND hWnd, LPSTR lpClassName, int nMaxCount);
#[cfg (feature = "windowsnt")]
pub type LPARAM = isize;
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn GetWindowThreadProcessId (hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
  pub fn GetClassName (hWnd: HWND, lpClassName: LPSTR, nMaxCount: c_int) -> c_int;
}

/* C 3528's `sys_kill` opens the next slice.  */

/* C 50's `<windows.h>`/`<winbase.h>`/`<winuser.h>`/`<io.h>`: the surface `sys_kill` (C 3528) and
   the standard-handle pair (C 3838-3901) name, none of which any mirror in this unit's graph
   carries (UNVERIFIED, P-88 1): the process/console entry points, the two window-message users
   with `WM_CLOSE`/`WM_QUIT`, the keyboard-simulation set with its four virtual-key and flag
   constants, the four `OpenProcess` access/error values, and the CRT's `_get_osfhandle`.  */
/// C: #define PROCESS_TERMINATE 0x0001
/// C: #define PROCESS_QUERY_INFORMATION 0x0400
/// C: #define PROCESS_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0xFFFF)
/// C: #define ERROR_ACCESS_DENIED 5
/// C: #define ERROR_INVALID_PARAMETER 87
/// C: #define VK_CANCEL 0x03
/// C: #define VK_CONTROL 0x11
/// C: #define VK_LCONTROL 0xA2
/// C: #define KEYEVENTF_EXTENDEDKEY 0x0001
/// C: #define KEYEVENTF_KEYUP 0x0002
/// C: #define CTRL_BREAK_EVENT 1
/// C: #define WM_CLOSE 0x0010
/// C: #define WM_QUIT 0x0012
#[cfg (feature = "windowsnt")]
pub const PROCESS_TERMINATE: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
#[cfg (feature = "windowsnt")]
pub const PROCESS_ALL_ACCESS: DWORD = 0x001F0FFF;
#[cfg (feature = "windowsnt")]
pub const ERROR_ACCESS_DENIED: DWORD = 5;
#[cfg (feature = "windowsnt")]
pub const ERROR_INVALID_PARAMETER: DWORD = 87;
#[cfg (feature = "windowsnt")]
pub const VK_CANCEL: c_int = 0x03;
#[cfg (feature = "windowsnt")]
pub const VK_CONTROL: c_int = 0x11;
#[cfg (feature = "windowsnt")]
pub const VK_LCONTROL: c_int = 0xA2;
#[cfg (feature = "windowsnt")]
pub const KEYEVENTF_EXTENDEDKEY: DWORD = 0x0001;
#[cfg (feature = "windowsnt")]
pub const KEYEVENTF_KEYUP: DWORD = 0x0002;
#[cfg (feature = "windowsnt")]
pub const CTRL_BREAK_EVENT: DWORD = 1;
#[cfg (feature = "windowsnt")]
pub const WM_CLOSE: UINT = 0x0010;
#[cfg (feature = "windowsnt")]
pub const WM_QUIT: UINT = 0x0012;
/// C: typedef BOOL (CALLBACK *WNDENUMPROC) (HWND, LPARAM);
/// C: WINBASEAPI HANDLE WINAPI OpenProcess (DWORD dwDesiredAccess, BOOL bInheritHandle,
/// C:   DWORD dwProcessId);
/// C: WINUSERAPI BOOL WINAPI EnumWindows (WNDENUMPROC lpEnumFunc, LPARAM lParam);
/// C: WINUSERAPI UINT WINAPI MapVirtualKeyA (UINT uCode, UINT uMapType);
/// C: WINUSERAPI HWND WINAPI GetForegroundWindow (void);
/// C: WINBASEAPI DWORD WINAPI GetCurrentThreadId (void);
/// C: WINUSERAPI BOOL WINAPI AttachThreadInput (DWORD idAttach, DWORD idAttachTo, BOOL fAttach);
/// C: WINUSERAPI BOOL WINAPI SetForegroundWindow (HWND hWnd);
/// C: WINUSERAPI SHORT WINAPI GetKeyState (int nVirtKey);
/// C: WINUSERAPI void WINAPI keybd_event (BYTE bVk, BYTE bScan, DWORD dwFlags,
/// C:   ULONG_PTR dwExtraInfo);
/// C: WINBASEAPI BOOL WINAPI GenerateConsoleCtrlEvent (DWORD dwCtrlEvent, DWORD dwProcessGroupId);
/// C: WINUSERAPI BOOL WINAPI PostMessageA (HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
/// C: WINBASEAPI BOOL WINAPI TerminateProcess (HANDLE hProcess, UINT uExitCode);
/// C: WINBASEAPI BOOL WINAPI SetStdHandle (DWORD nStdHandle, HANDLE hHandle);
/// C: _CRTIMP intptr_t __cdecl _get_osfhandle (int);
#[cfg (feature = "windowsnt")]
pub type WNDENUMPROC = Option<unsafe extern "system" fn (HWND, LPARAM) -> BOOL>;
/// C: typedef UINT_PTR WPARAM;   [`<windef.h>`; pointer-sized, `usize`]
#[cfg (feature = "windowsnt")]
pub type WPARAM = usize;
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn OpenProcess (dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
  pub fn EnumWindows (lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
  pub fn MapVirtualKey (uCode: UINT, uMapType: UINT) -> UINT;
  pub fn GetForegroundWindow () -> HWND;
  pub fn GetCurrentThreadId () -> DWORD;
  pub fn AttachThreadInput (idAttach: DWORD, idAttachTo: DWORD, fAttach: BOOL) -> BOOL;
  pub fn SetForegroundWindow (hWnd: HWND) -> BOOL;
  pub fn GetKeyState (nVirtKey: c_int) -> c_short;
  pub fn keybd_event (bVk: BYTE, bScan: BYTE, dwFlags: DWORD, dwExtraInfo: ULONG_PTR);
  pub fn GenerateConsoleCtrlEvent (dwCtrlEvent: DWORD, dwProcessGroupId: DWORD) -> BOOL;
  pub fn PostMessage (hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
  pub fn TerminateProcess (hProcess: HANDLE, uExitCode: UINT) -> BOOL;
  pub fn SetStdHandle (nStdHandle: DWORD, hHandle: HANDLE) -> BOOL;
}
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn _get_osfhandle (fd: c_int) -> isize;
}
/* Emulate 'kill', but only for other processes.  */
/// C 3529-3819: `int sys_kill (pid_t pid, int sig)` - the file's third-longest function: C
/// 3536/3539-3540's negative-pid handling and C 3543-3549's signal whitelist, C 3551-3601's
/// `sig == 0` probe (the `pid <= 4` guard, `OpenProcess` with its two nested `switch`es on
/// `GetLastError`), C 3603-3630's child lookup and the two `OpenProcess`/`EnumWindows` arms, C
/// 3632-3719's `SIGINT`/`SIGQUIT` arm (the `Ctrl-C`/`Ctrl-Break` keyboard simulation with its
/// thread attaching, and the `GenerateConsoleCtrlEvent` fallback), C 3720-3756's `SIGTRAP` arm
/// (`DebugBreakProcess` resolved through `get_proc_addr` on first use) and C 3757-3813's
/// terminate arm (the `os_subtype` choice between `TerminateProcess` and `PostMessage (WM_CLOSE)`),
/// ending at C 3815-3818's `need_to_free` close and `return rc`.
/// C: int
/// C: sys_kill (pid_t pid, int sig)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_kill (pid: pid_t, sig: c_int) -> c_int
{
  let mut cp: *mut child_process = core::ptr::null_mut ();           // C 3532
  let mut proc_hand: HANDLE;                                         // C 3533
  let mut need_to_free: c_int = 0;                                   // C 3534
  let mut rc: c_int = 0;                                             // C 3535
  let mut pid: pid_t = pid;                                          // (C 3536's `orig_pid`)
  let orig_pid: pid_t = pid;

  /* Each process is in its own process group.  */
  if pid < 0                                                         // C 3539
    { pid = -pid; }                                                  // C 3540

  /* Only handle signals that can be mapped to a similar behavior on Windows */
  if sig != 0                                                        // C 3543
      && sig != SIGINT && sig != SIGKILL && sig != SIGQUIT          // C 3544
      && sig != SIGHUP && sig != SIGTRAP                            // C 3545
    {
      *errno () = EINVAL;                                            // C 3547
      return -1;                                                     // C 3548
    }

  if sig == 0                                                        // C 3551
    {
      /* It will take _some_ time before PID 4 or less on Windows will
	 be Emacs...  */
      if pid <= 4                                                    // C 3555
        {
          *errno () = EPERM;                                         // C 3557
          return -1;                                                 // C 3558
        }
      proc_hand = OpenProcess (PROCESS_QUERY_INFORMATION, 0, pid as DWORD);   // C 3560
      if proc_hand.is_null ()                                        // C 3561
        {
          let mut err: DWORD = GetLastError ();                       // C 3563

          match err
            {
              /* existing process, but access denied */
              ERROR_ACCESS_DENIED =>                             // C 3567
                {
                  *errno () = EPERM;                                 // C 3568
                  return -1;                                         // C 3569
                }
              /* process PID does not exist */
              ERROR_INVALID_PARAMETER =>                         // C 3570
                {
                  if orig_pid == pid                                 // C 3572
                    {
                      *errno () = ESRCH;                             // C 3574
                      return -1;                                     // C 3575
                    }
		/* If we received a negative value, try again with the
                   original one we received.  */
                  proc_hand = OpenProcess (PROCESS_QUERY_INFORMATION, 0, orig_pid as DWORD);   // C 3579-3580
                  if proc_hand.is_null ()                            // C 3581
                    {
                      err = GetLastError ();                         // C 3583
                      match err
                        {
                          ERROR_ACCESS_DENIED =>                 // C 3586
                            {
                              *errno () = EPERM;                     // C 3587
                              return -1;                             // C 3588
                            }
                          ERROR_INVALID_PARAMETER =>             // C 3589
                            {
                              *errno () = ESRCH;                     // C 3590
                              return -1;                             // C 3591
                            }
                          _ => { }
                        }
                    }
                  /* C 3594's `break` - the arm's end.  */
                }
              _ => { }
            }
        }
      if !proc_hand.is_null ()                                       // C 3598
        { CloseHandle (proc_hand); }                                 // C 3599
      return 0;                                                      // C 3600
    }

  cp = find_child_pid (pid as DWORD);                                // C 3603
  if cp.is_null ()                                                   // C 3604
    {
      /* We were passed a PID of something other than our subprocess.
	 If that is our own PID, we will send to ourself a message to
	 close the selected frame, which does not necessarily
	 terminates Emacs.  But then we are not supposed to call
	 sys_kill with our own PID.  */

      let desiredAccess: DWORD = if sig == SIGTRAP                   // C 3612
        { PROCESS_ALL_ACCESS } else { PROCESS_TERMINATE };            // C 3613

      proc_hand = OpenProcess (desiredAccess, 0, pid as DWORD);       // C 3615
      if proc_hand.is_null ()                                         // C 3616
        {
          *errno () = EPERM;                                          // C 3618
          return -1;                                                  // C 3619
        }
      need_to_free = 1;                                               // C 3621
    }
  else
    {
      proc_hand = (*cp).procinfo.hProcess;                            // C 3625
      pid = (*cp).procinfo.dwProcessId as pid_t;                      // C 3626

      /* Try to locate console window for process. */
      EnumWindows (Some (find_child_console), cp as LPARAM);           // C 3629
    }

  if sig == SIGINT || sig == SIGQUIT                                 // C 3632
    {
      if NILP (Vw32_start_process_share_console) && !cp.is_null ()     // C 3634
          && !(*cp).hwnd.is_null ()
        {
          let control_scan_code: BYTE = MapVirtualKey (VK_CONTROL as UINT, 0) as BYTE;   // C 3636
          /* Fake Ctrl-C for SIGINT, and Ctrl-Break for SIGQUIT.  */
          let mut vk_break_code: BYTE = if sig == SIGINT               // C 3638
            { b'C' as BYTE } else { VK_CANCEL as BYTE };
          let mut break_scan_code: BYTE
            = MapVirtualKey (vk_break_code as UINT, 0) as BYTE;        // C 3639
          let mut foreground_window: HWND;                             // C 3640

          if break_scan_code == 0                                      // C 3642
            {
              /* Fake Ctrl-C for SIGQUIT if we can't manage Ctrl-Break. */
              vk_break_code = b'C' as BYTE;                             // C 3645
              break_scan_code = MapVirtualKey (vk_break_code as UINT, 0) as BYTE;   // C 3646
            }

          foreground_window = GetForegroundWindow ();                  // C 3649
          if !foreground_window.is_null ()                             // C 3650
            {
              /* NT 5.0, and apparently also Windows 98, will not allow
		 a Window to be set to foreground directly without the
		 user's involvement. The workaround is to attach
		 ourselves to the thread that owns the foreground
		 window, since that is the only thread that can set the
		 foreground window.  */
              let mut foreground_thread: DWORD;                        // C 3658
              let mut child_thread: DWORD;
              foreground_thread = GetWindowThreadProcessId (foreground_window,
                                                            core::ptr::null_mut ());   // C 3659-3660
              if foreground_thread == GetCurrentThreadId ()            // C 3661
                  || AttachThreadInput (GetCurrentThreadId (), foreground_thread,
                                        TRUE) == 0                     // C 3662-3663
                { foreground_thread = 0; }                             // C 3664

              child_thread = GetWindowThreadProcessId ((*cp).hwnd,
                                                       core::ptr::null_mut ());   // C 3666
              if child_thread == GetCurrentThreadId ()                 // C 3667
                  || AttachThreadInput (GetCurrentThreadId (), child_thread, TRUE) == 0   // C 3668-3669
                { child_thread = 0; }                                  // C 3670

              /* Set the foreground window to the child.  */
              if SetForegroundWindow ((*cp).hwnd) != 0                 // C 3673
                {
		  /* Record the state of the left Ctrl key: the user
		     could have it depressed while we are simulating
		     Ctrl-C, in which case we will have to leave the
		     state of that Ctrl depressed when we are done.  */
                  let ctrl_state: c_short = GetKeyState (VK_LCONTROL) & 0x8000u16 as c_short;   // C 3679

                  /* Generate keystrokes as if user had typed Ctrl-Break or
                     Ctrl-C.  */
                  keybd_event (VK_CONTROL as BYTE, control_scan_code, 0, 0);   // C 3683
                  keybd_event (vk_break_code, break_scan_code,
                               if vk_break_code == b'C' as BYTE
                                 { 0 } else { KEYEVENTF_EXTENDEDKEY }, 0);   // C 3684-3685
                  keybd_event (vk_break_code, break_scan_code,
                               (if vk_break_code == b'C' as BYTE
                                  { 0 } else { KEYEVENTF_EXTENDEDKEY })
                               | KEYEVENTF_KEYUP, 0);                  // C 3686-3688
                  keybd_event (VK_CONTROL as BYTE, control_scan_code,
                               KEYEVENTF_KEYUP, 0);                    // C 3689-3690

                  /* Sleep for a bit to give time for Emacs frame to respond
                     to focus change events (if Emacs was active app).  */
                  Sleep (100);                                         // C 3694

                  SetForegroundWindow (foreground_window);              // C 3696
                  /* If needed, restore the state of Ctrl.  */
                  if ctrl_state != 0                                   // C 3698
                    { keybd_event (VK_CONTROL as BYTE, control_scan_code, 0, 0); }   // C 3699
                }
              /* Detach from the foreground and child threads now that
                 the foreground switching is over.  */
              if foreground_thread != 0                                // C 3703
                { AttachThreadInput (GetCurrentThreadId (), foreground_thread, FALSE); }   // C 3704-3705
              if child_thread != 0                                     // C 3706
                { AttachThreadInput (GetCurrentThreadId (), child_thread, FALSE); }        // C 3707-3708
            }
        }
      /* Ctrl-Break is NT equivalent of SIGINT.  */
      else if GenerateConsoleCtrlEvent (CTRL_BREAK_EVENT, pid as DWORD) == 0   // C 3712
        {
          DebPrint! (("sys_kill.GenerateConsoleCtrlEvent return %d "
                      "for pid %lu\n", GetLastError (), pid));        // C 3714-3715
          *errno () = EINVAL;                                          // C 3716
          rc = -1;                                                     // C 3717
        }
    }
  else if sig == SIGTRAP                                            // C 3720
    {
      static mut s_pfn_Debug_Break_Process: DebugBreakProcess_Proc = None;   // C 3722

      if g_b_init_debug_break_process == 0                          // C 3724
        {
          g_b_init_debug_break_process = 1;                          // C 3726
          s_pfn_Debug_Break_Process
            = core::mem::transmute::<VOIDFNPTR, DebugBreakProcess_Proc> (
                get_proc_addr (GetModuleHandle (c"kernel32.dll".as_ptr ()),
                               c"DebugBreakProcess".as_ptr ()));      // C 3727-3729
        }

      if s_pfn_Debug_Break_Process.is_none ()                        // C 3732
        {
          *errno () = ENOTSUP;                                       // C 3734
          rc = -1;                                                   // C 3735
        }
      else if (s_pfn_Debug_Break_Process.unwrap ()) (proc_hand) == 0   // C 3737
        {
          let err: DWORD = GetLastError ();                           // C 3739

          DebPrint! (("sys_kill.DebugBreakProcess return %d "
                      "for pid %lu\n", err, pid));                    // C 3741-3742

          match err
            {
              ERROR_ACCESS_DENIED =>                             // C 3746
                { *errno () = EPERM; }                                // C 3747
              _ =>                                                 // C 3749
                { *errno () = EINVAL; }                               // C 3750
            }

          rc = -1;                                                    // C 3754
        }
    }
  else
    {
      if NILP (Vw32_start_process_share_console) && !cp.is_null ()     // C 3759
          && !(*cp).hwnd.is_null ()
        {
          /* C 3760's `#if 1` takes this arm in every configuration; C 3799's `else` pairs with
             C 3762's `if`, and C 3800's `#endif` closes the `#if 1`.  */
          if os_subtype == OS_SUBTYPE_9X                               // C 3762
            {
              /* C 3764-3779: the Shell VxD note, carried as C's comment.  */
              /*
/*
   Another possibility is to try terminating the VDM out-right by
   calling the Shell VxD (id 0x17) V86 interface, function #4
   "SHELL_Destroy_VM", ie.

     mov edx,4
     mov ebx,vm_handle
     call shellapi

   First need to determine the current VM handle, and then arrange for
   the shellapi call to be made from the system vm (by using
   Switch_VM_and_callback).

   Could try to invoke DestroyVM through CallVxD.

*/
              */
              /* C 3780-3790: the `#if 0` arm.  */
              /*
#if 0
	      /* On Windows 95, posting WM_QUIT causes the 16-bit subsystem
		 to hang when cmdproxy is used in conjunction with
		 command.com for an interactive shell.  Posting
		 WM_CLOSE pops up a dialog that, when Yes is selected,
		 does the same thing.  TerminateProcess is also less
		 than ideal in that subprocesses tend to stick around
		 until the machine is shutdown, but at least it
		 doesn't freeze the 16-bit subsystem.  */
	      PostMessage (cp->hwnd, WM_QUIT, 0xff, 0);
#endif
              */
              if TerminateProcess (proc_hand, 0xff) == 0               // C 3791
                {
                  DebPrint! (("sys_kill.TerminateProcess returned %d "
                              "for pid %lu\n", GetLastError (), pid));   // C 3793-3794
                  *errno () = EINVAL;                                   // C 3795
                  rc = -1;                                              // C 3796
                }
            }
          else
            { PostMessage ((*cp).hwnd, WM_CLOSE, 0, 0); }               // C 3801
        }
      /* Kill the process.  On W32 this doesn't kill child processes
	 so it doesn't work very well for shells which is why it's not
	 used in every case.  */
      else if TerminateProcess (proc_hand, 0xff) == 0                  // C 3806
        {
          DebPrint! (("sys_kill.TerminateProcess returned %d "
                      "for pid %lu\n", GetLastError (), pid));         // C 3808-3809
          *errno () = EINVAL;                                           // C 3810
          rc = -1;                                                      // C 3811
        }
    }

  if need_to_free != 0                                                 // C 3815
    { CloseHandle (proc_hand); }                                       // C 3816

  return rc;                                                           // C 3818
}

/* The following two routines are used to manipulate stdin, stdout, and
   stderr of our child processes.

   Assuming that in, out, and err are *not* inheritable, we make them
   stdin, stdout, and stderr of the child as follows:

   - Save the parent's current standard handles.
   - Set the std handles to inheritable duplicates of the ones being passed in.
     (Note that _get_osfhandle() is an io.h procedure that retrieves the
     NT file handle for a crt file descriptor.)
   - Spawn the child, which inherits in, out, and err as stdin,
     stdout, and stderr. (see Spawnve)
   - Close the std handles passed to the child.
   - Reset the parent's standard handles to the saved handles.
     (see reset_standard_handles)
   We assume that the caller closes in, out, and err after calling us.  */
/// C 3838-3887: `void prepare_standard_handles (int in, int out, int err, HANDLE handles[3])` -
/// C 3844-3848's parent handle and the three saved std handles, C 3851-3876's three
/// `DuplicateHandle` calls with their `report_file_error` paths, and C 3879-3886's three
/// `SetStdHandle` calls.  C's `(HANDLE) _get_osfhandle (in)` is the `_get_osfhandle` cast the
/// use-site block above declares.
/// C: void
/// C: prepare_standard_handles (int in, int out, int err, HANDLE handles[3])
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_standard_handles (in_: c_int, out: c_int, err: c_int,
                                                   handles: *mut HANDLE)
{
  let mut parent: HANDLE;                                              // C 3841
  let mut newstdin: HANDLE = core::ptr::null_mut ();                   // C 3842
  let mut newstdout: HANDLE = core::ptr::null_mut ();
  let mut newstderr: HANDLE = core::ptr::null_mut ();

  parent = GetCurrentProcess ();                                       // C 3844

  *handles = GetStdHandle (STD_INPUT_HANDLE);                          // C 3846
  *handles.add (1) = GetStdHandle (STD_OUTPUT_HANDLE);                 // C 3847
  *handles.add (2) = GetStdHandle (STD_ERROR_HANDLE);                  // C 3848

  /* make inheritable copies of the new handles */
  if DuplicateHandle (parent, _get_osfhandle (in_) as HANDLE, parent, // C 3851-3853
                      &raw mut newstdin, 0, TRUE,
                      DUPLICATE_SAME_ACCESS) == 0                      // C 3857
    { report_file_error (c"Duplicating input handle for child".as_ptr (), Qnil ()); }   // C 3858

  if DuplicateHandle (parent, _get_osfhandle (out) as HANDLE, parent,   // C 3860-3862
                      &raw mut newstdout, 0, TRUE,
                      DUPLICATE_SAME_ACCESS) == 0                       // C 3866
    { report_file_error (c"Duplicating output handle for child".as_ptr (), Qnil ()); }   // C 3867

  if DuplicateHandle (parent, _get_osfhandle (err) as HANDLE, parent,   // C 3869-3871
                      &raw mut newstderr, 0, TRUE,
                      DUPLICATE_SAME_ACCESS) == 0                       // C 3875
    { report_file_error (c"Duplicating error handle for child".as_ptr (), Qnil ()); }   // C 3876

  /* and store them as our std handles */
  if SetStdHandle (STD_INPUT_HANDLE, newstdin) == 0                     // C 3879
    { report_file_error (c"Changing stdin handle".as_ptr (), Qnil ()); }    // C 3880

  if SetStdHandle (STD_OUTPUT_HANDLE, newstdout) == 0                   // C 3882
    { report_file_error (c"Changing stdout handle".as_ptr (), Qnil ()); }    // C 3883

  if SetStdHandle (STD_ERROR_HANDLE, newstderr) == 0                    // C 3885
    { report_file_error (c"Changing stderr handle".as_ptr (), Qnil ()); }    // C 3886
}

/// C 3889-3901: `void reset_standard_handles (int in, int out, int err, HANDLE handles[3])` - the
/// three closes of the duplicated handles C 3893-3895 and the three `SetStdHandle` restores C
/// 3898-3900.  C's `in`/`out`/`err` parameters are unused here (the unit's `unused_variables`
/// allow covers them).
/// C: void
/// C: reset_standard_handles (int in, int out, int err, HANDLE handles[3])
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_standard_handles (in_: c_int, out: c_int, err: c_int,
                                                 handles: *mut HANDLE)
{
  /* close the duplicated handles passed to the child */
  CloseHandle (GetStdHandle (STD_INPUT_HANDLE));                        // C 3893
  CloseHandle (GetStdHandle (STD_OUTPUT_HANDLE));                       // C 3894
  CloseHandle (GetStdHandle (STD_ERROR_HANDLE));                        // C 3895

  /* now restore parent's saved std handles */
  SetStdHandle (STD_INPUT_HANDLE, *handles.add (0));                    // C 3898
  SetStdHandle (STD_OUTPUT_HANDLE, *handles.add (1));                   // C 3899
  SetStdHandle (STD_ERROR_HANDLE, *handles.add (2));                    // C 3900
}

/* C 3903's `set_process_dir` (C 3903-3907) opens the next slice, with the twenty `DEFUN`s and
   `syms_of_ntproc`.  */

/// C 3903-3907: `void set_process_dir (const char *dir)` - the writer of the `process_dir` static
/// slice 8 landed; C's `(char *) dir` is the port's cast.
/// C: void
/// C: set_process_dir (const char * dir)
#[cfg (feature = "windowsnt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_process_dir (dir: *const c_char)
{
  process_dir = dir as *mut c_char;                                  // C 3906
}

/* To avoid problems with winsock implementations that work over dial-up
   connections causing or requiring a connection to exist while Emacs is
   running, Emacs no longer automatically loads winsock on startup if it
   is present.  Instead, it will be loaded when open-network-stream is
   first called.

   To allow full control over when winsock is loaded, we provide these
   two functions to dynamically load and unload winsock.  This allows
   dial-up users to only be connected when they actually need to use
   socket services.  */

/* From w32.c */
/* C 3920-3922's two `w32.c` names: this unit declares them (the mirror `include/w32.rs` carries
   neither - `init_winsock` is there, at its 534, and is the one the two DEFUNs below call).  */
/// C: extern HANDLE winsock_lib;
/// C: extern BOOL term_winsock (void);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub static mut winsock_lib: HANDLE;
  pub fn term_winsock () -> BOOL;
}

/// C 3924-3956: `DEFUN ("w32-has-winsock", Fw32_has_winsock, Sw32_has_winsock, 0, 1, 0, …)` - C
/// 3937's `init_winsock (!NILP (load_now))`, C 3940's `winsock_lib` guard with C 3945-3950's
/// save/`init_system_name`/restore of `Vsystem_name`, and the `Qt`/`Qnil` answers.  C's docstring
/// is carried in the invocation (the `doc:` label the macro takes).
/// C: DEFUN ("w32-has-winsock", Fw32_has_winsock, Sw32_has_winsock, 0, 1, 0,
/// C:        doc: /* Test for presence of the Windows socket library `winsock'.
/// C: Returns non-nil if winsock support is present, nil otherwise.
/// C:
/// C: If the optional argument LOAD-NOW is non-nil, the winsock library is
/// C: also loaded immediately if not already loaded.  If winsock is loaded,
/// C: the winsock local hostname is returned (since this may be different from
/// C: the value of `system-name' and should supplant it), otherwise t is
/// C: returned to indicate winsock support is present.  */)
/// C:   (Lisp_Object load_now)
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-has-winsock", Fw32_has_winsock, Sw32_has_winsock, 0, 1, 0,
	doc: "Test for presence of the Windows socket library `winsock'.\nReturns non-nil if winsock support is present, nil otherwise.\n\nIf the optional argument LOAD-NOW is non-nil, the winsock library is\nalso loaded immediately if not already loaded.  If winsock is loaded,\nthe winsock local hostname is returned (since this may be different from\nthe value of `system-name' and should supplant it), otherwise t is\nreturned to indicate winsock support is present.  ",
	(load_now: Lisp_Object),
	{
  let have_winsock: c_int;                                           // C 3935

  have_winsock = init_winsock (!NILP (load_now) as c_int);           // C 3937
  if have_winsock != 0                                               // C 3938
    {
      if !winsock_lib.is_null ()                                     // C 3940
	{
	  /* Return new value for system-name.  The best way to do this
	     is to call init_system_name, saving and restoring the
	     original value to avoid side-effects.  */
	  let orig_hostname: Lisp_Object = Vsystem_name;              // C 3945
	  let hostname: Lisp_Object;                                  // C 3946

	  init_system_name ();                                        // C 3948
	  hostname = Vsystem_name;                                    // C 3949
	  Vsystem_name = orig_hostname;                               // C 3950
	  return hostname;                                            // C 3951
	}
      return Qt ();                                                   // C 3953
    }
  return Qnil ();                                                     // C 3955
});

/// C 3958-3967: `DEFUN ("w32-unload-winsock", Fw32_unload_winsock, Sw32_unload_winsock, 0, 0, 0, …)`
/// - the one-line body C 3966, whose `term_winsock ()` is the extern above.
/// C: DEFUN ("w32-unload-winsock", Fw32_unload_winsock, Sw32_unload_winsock,
/// C:        0, 0, 0,
/// C:        doc: /* Unload the Windows socket library `winsock' if loaded.
/// C: This is provided to allow dial-up socket connections to be disconnected
/// C: when no longer needed.  Returns nil without unloading winsock if any
/// C: socket connections still exist.  */)
/// C:   (void)
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-unload-winsock", Fw32_unload_winsock, Sw32_unload_winsock, 0, 0, 0,
	doc: "Unload the Windows socket library `winsock' if loaded.\nThis is provided to allow dial-up socket connections to be disconnected\nwhen no longer needed.  Returns nil without unloading winsock if any\nsocket connections still exist.  ",
	(),
	{
  return if term_winsock () != 0 { Qt () } else { Qnil () };         // C 3966
});

/* C's `<process.h>`/`<emacs.c>`: `init_system_name`, the one name the first DEFUN needs and no
   mirror in this unit's graph carries (`src/process.rs` defines it; the declaration shape is
   `src/emacs.rs`'s).  */
/// C: extern void init_system_name (void);
#[cfg (feature = "windowsnt")]
unsafe extern "C" {
  pub fn init_system_name ();
}

/* C 3969-3971's banner comment and C 3973's `w32-short-file-name` open the next slice.  */

/* Some miscellaneous functions that are Windows specific, but not GUI
   specific (ie. are applicable in terminal or batch mode as well).  */
/// C 3973-3995: `DEFUN ("w32-short-file-name", …)` - C 3981's `CHECK_STRING`, C 3984's
/// `Fexpand_file_name`, C 3987-3989's `w32_get_short_filename` guard and C 3991/3994's
/// `dostounix_filename`/`build_string` pair.
/// C: DEFUN ("w32-short-file-name", Fw32_short_file_name, Sw32_short_file_name, 1, 1, 0,
/// C:        doc: /* Return the short file name version (8.3) of the full path of FILENAME.
/// C: If FILENAME does not exist, return nil.
/// C: All path elements in FILENAME are converted to their short names.  */)
/// C:   (Lisp_Object filename)
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-short-file-name", Fw32_short_file_name, Sw32_short_file_name, 1, 1, 0,
	doc: "       doc: /* Return the short file name version (8.3) of the full path of FILENAME.
If FILENAME does not exist, return nil.
All path elements in FILENAME are converted to their short names.  */)  ",
	(filename: Lisp_Object),
	{
  let mut shortname: [c_char; MAX_PATH as usize] = [0; MAX_PATH as usize];   // C 3979

  CHECK_STRING (filename);                                           // C 3981

  /* first expand it.  */
  filename = Fexpand_file_name (filename, Qnil ());                  // C 3984

  /* luckily, this returns the short version of each element in the path.  */
  if w32_get_short_filename (SSDATA (ENCODE_FILE (filename)),       // C 3987
			     shortname.as_mut_ptr (), MAX_PATH) == 0     // C 3988
    { return Qnil (); }                                              // C 3989

  dostounix_filename (shortname.as_mut_ptr ());                      // C 3991

  /* No need to DECODE_FILE, because 8.3 names are pure ASCII.   */
  return build_string (shortname.as_ptr ());                         // C 3994
});

/// C 3998-4030: `DEFUN ("w32-long-file-name", …)` - C 4005-4012's buffer and the two-character
/// drive test, C 4015's `Fexpand_file_name`, C 4017-4019's `w32_get_long_filename` guard, C 4021's
/// `dostounix_filename` and C 4023-4029's drive-only fixup with `DECODE_FILE`/`build_unibyte_string`.
/// C: DEFUN ("w32-long-file-name", Fw32_long_file_name, Sw32_long_file_name,
/// C:        1, 1, 0,
/// C:        doc: /* Return the long file name version of the full path of FILENAME.
/// C: If FILENAME does not exist, return nil.
/// C: All path elements in FILENAME are converted to their long names.  */)
/// C:   (Lisp_Object filename)
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-long-file-name", Fw32_long_file_name, Sw32_long_file_name, 1, 1, 0,
	doc: "       doc: /* Return the long file name version of the full path of FILENAME.
If FILENAME does not exist, return nil.
All path elements in FILENAME are converted to their long names.  */)  ",
	(filename: Lisp_Object),
	{
  let mut longname: [c_char; MAX_UTF8_PATH as usize] = [0; MAX_UTF8_PATH as usize];   // C 4005
  let mut drive_only: c_int = 0;                                     // C 4006

  CHECK_STRING (filename);                                           // C 4008

  if SBYTES (filename) == 2                                          // C 4010
      && *(SDATA (filename)).add (1) == b':' as c_char               // C 4011
    { drive_only = 1; }                                              // C 4012

  /* first expand it.  */
  filename = Fexpand_file_name (filename, Qnil ());                  // C 4015

  if w32_get_long_filename (SSDATA (ENCODE_FILE (filename)),         // C 4017
			    longname.as_mut_ptr (), MAX_UTF8_PATH) == 0   // C 4018
    { return Qnil (); }                                              // C 4019

  dostounix_filename (longname.as_mut_ptr ());                       // C 4021

  /* If we were passed only a drive, make sure that a slash is not appended
     for consistency with directories.  Allow for drive mapping via SUBST
     in case expand-file-name is ever changed to expand those.  */
  if drive_only != 0 && longname[1] == b':' as c_char                // C 4026
      && longname[2] == b'/' as c_char && longname[3] == 0
    { longname[2] = 0; }                                             // C 4027

  return DECODE_FILE (build_unibyte_string (longname.as_ptr ()));    // C 4029
});

/// C 4032-4082: `DEFUN ("w32-set-process-priority", …)` - C 4043-4045's three locals, C 4047's
/// `CHECK_SYMBOL`, C 4049-4066's process arm (C 4054's `CHECK_FIXNUM`, C 4060-4063's
/// `find_child_pid` fixup and C 4065's `OpenProcess`), C 4068-4071's two `EQ` tests and C
/// 4073-4079's `SetPriorityClass`/`CloseHandle` pair.
/// C: DEFUN ("w32-set-process-priority", Fw32_set_process_priority,
/// C:        Sw32_set_process_priority, 2, 2, 0,
/// C:        doc: /* Set the priority of PROCESS to PRIORITY.
/// C: If PROCESS is nil, the priority of Emacs is changed, otherwise the
/// C: priority of the process whose pid is PROCESS is changed.
/// C: PRIORITY should be one of the symbols high, normal, or low;
/// C: any other symbol will be interpreted as normal.
/// C:
/// C: If successful, the return value is t, otherwise nil.  */)
/// C:   (Lisp_Object process, Lisp_Object priority)
#[cfg (feature = "windowsnt")]
DEFUN! ("w32-set-process-priority", Fw32_set_process_priority, Sw32_set_process_priority,
	2, 2, 0,
	doc: "       doc: /* Set the priority of PROCESS to PRIORITY.
If PROCESS is nil, the priority of Emacs is changed, otherwise the
priority of the process whose pid is PROCESS is changed.
PRIORITY should be one of the symbols high, normal, or low;
any other symbol will be interpreted as normal.

If successful, the return value is t, otherwise nil.  */)  ",
	(process: Lisp_Object, priority: Lisp_Object),
	{
  let mut proc_handle: HANDLE = GetCurrentProcess ();                 // C 4043
  let mut priority_class: DWORD = NORMAL_PRIORITY_CLASS;              // C 4044
  let mut result: Lisp_Object = Qnil ();                              // C 4045

  CHECK_SYMBOL (priority);                                           // C 4047

  if !NILP (process)                                                 // C 4049
    {
      let mut pid: DWORD;                                            // C 4051
      let mut cp: *mut child_process;                                // C 4052

      CHECK_FIXNUM (process);                                        // C 4054

      /* Allow pid to be an internally generated one, or one obtained
	 externally.  This is necessary because real pids on Windows 95 are
	 negative.  */

      pid = XFIXNUM (process) as DWORD;                              // C 4060
      cp = find_child_pid (pid);                                     // C 4061
      if !cp.is_null ()                                              // C 4062
	{ pid = (*cp).procinfo.dwProcessId; }                        // C 4063

      proc_handle = OpenProcess (PROCESS_SET_INFORMATION, FALSE, pid);   // C 4065
    }

  if EQ (priority, Qhigh)                                            // C 4068
    { priority_class = HIGH_PRIORITY_CLASS; }                        // C 4069
  else if EQ (priority, Qlow)                                        // C 4070
    { priority_class = IDLE_PRIORITY_CLASS; }                        // C 4071

  if !proc_handle.is_null ()                                         // C 4073
    {
      if SetPriorityClass (proc_handle, priority_class) != 0          // C 4075
	{ result = Qt (); }                                          // C 4076
      if !NILP (process)                                             // C 4077
	{ CloseHandle (proc_handle); }                               // C 4078
    }

  return result;                                                     // C 4081
});

/* C's `<winbase.h>`: the `OpenProcess` access right and the three priority classes C 4032-4082
   names, plus `SetPriorityClass` (`OpenProcess` itself is slice 19's).  UNVERIFIED (P-88 1).  */
/// C: #define PROCESS_SET_INFORMATION 0x0200
/// C: #define NORMAL_PRIORITY_CLASS 0x00000020
/// C: #define IDLE_PRIORITY_CLASS 0x00000040
/// C: #define HIGH_PRIORITY_CLASS 0x00000080
/// C: WINBASEAPI BOOL WINAPI SetPriorityClass (HANDLE hProcess, DWORD dwPriorityClass);
#[cfg (feature = "windowsnt")]
pub const PROCESS_SET_INFORMATION: DWORD = 0x0200;
#[cfg (feature = "windowsnt")]
pub const NORMAL_PRIORITY_CLASS: DWORD = 0x00000020;
#[cfg (feature = "windowsnt")]
pub const IDLE_PRIORITY_CLASS: DWORD = 0x00000040;
#[cfg (feature = "windowsnt")]
pub const HIGH_PRIORITY_CLASS: DWORD = 0x00000080;
#[cfg (feature = "windowsnt")]
unsafe extern "system" {
  pub fn SetPriorityClass (hProcess: HANDLE, dwPriorityClass: DWORD) -> BOOL;
}

/* C 4084's `w32-application-type` opens the next slice.  */
