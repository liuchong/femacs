# emacs/src/w32proc.c (4903 lines) — first slice: C 1-912

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32proc.o` is added to
> `W32_OBJ` only by `configure.ac` 3051's `else` arm (the non-Cygwin one, after the `HAVE_W32` test
> at 3019), and `docs/baseline/frozen-build-sources.txt` lists no `w32proc.o` for this target - so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/w32proc.rs` (**6635 lines**, md5 `893c7fb6ad06991e9d946b148ad4f358`), carrying
> **C 1-4321** - P-86 ④'s first twenty-four slices of this file - with **both arms of every `#if`**.
>
> **This ledger overturns its own earlier conclusion.**  Until this batch it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names the
> gate" (the P-08 reading, `docs/status.md` 3.31).  P-86 ① replaces that with *transcribe the text,
> both arms, no stub*; what the earlier note measured (the file is not in the frozen source list,
> its checklist is empty, its `build-tu` object is 336 bytes) is unchanged and still true - it is
> the *conclusion* that is withdrawn, exactly as `docs/ports/w32.md` records for its own file.
>
> **Why 912 and not 900.**  C 912 is the `}` of `alarm` (C 896-912); the neighbouring whole-item
> boundaries are `setitimer`'s `}` at C 894 (18 short) and the C 914-962 banner that opens the
> "waiting for more than 64 objects" support (past it, the next live definition is
> `wait_objects_thread`, C 1052).  **The next lane's slice is C 913-4903.**
>
> **The file gate is `windowsnt` - and the same build defines `HAVE_NTGUI`.**  `nt/inc/ms-w32.h`
> is what `#define`s `WINDOWSNT`, and `configure.ac` 3027 `AC_DEFINE`s `HAVE_NTGUI` *before* the
> Cygwin/non-Cygwin split at 3037, so the arm that adds `w32proc.o` (3051) has both.  The file's
> own text proves it: C 2142's `#ifdef HAVE_NTGUI` (past this slice) and C 459's `DebPrint`, which
> `conf_post.h` 160-167 defines only under `HAVE_NTGUI`.  A faithful arm flip therefore sets
> `windowsnt` **and** `have_ntgui`; with only the first, the flip stops at `cannot find macro
> DebPrint` - measured, and the reason the reading below lists both.
>
> **The slice's own conditions.**  `tools/check-cond-eval.py src/w32proc.c` reads **`26 conditions,
> 0 unknown, 0 disagreements [VERIFIED]`** (every one of the file's 26 is decided by the frozen
> table *and* agrees with clang), and four of them are in this slice: C 52 `HAVE_LANGINFO_CODESET`
> (1 in `docs/baseline/config/darwin-arm64.macros.tsv` → live), C 120 `__MINGW64__` (not in the
> table → dead), C 136 `#if 1` (true in every configuration, and it has no `#else` arm) and C 899
> `HAVE_SETITIMER` (1 → live, so C 909's `#else` is the other arm carried).  The file is *not* on
> `docs/baseline/config/cond-eval-verified.txt`, which is why `extract-abi.py` resolves no `#if`
> here (P-66).  `extract-abi.py --check` is nevertheless **not vacuous** (P-86 ⑩): the port has
> functions now, and Evidence ③ is that reading.
>
> **Acquisition: this batch is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain - `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty and no
> Homebrew `mingw-w64` is installed - so there is **no `<windows.h>` to compile or measure
> against**, **no arm flip on a Windows target** and **no layout measurement**.  What this batch
> carries instead: the frozen `build-tu` reading, the P-86 ② machine-checked coverage table, the
> `#if` verdicts, a per-item C citation, the `probe` reading of the `windowsnt`+`have_ntgui` arm
> (whose errors are *all* in other files: the mirror graph's Windows arm has never been compiled),
> and an arm-reach pre-flight over the unit's own text.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`'s `configure.ac` 3051 arm ⇔ `nt/inc/ms-w32.h`'s `WINDOWSNT`) | `feature = "windowsnt"` (on every item *and* on the include graph) | the non-Cygwin Windows (MinGW) build |
| the same build's `HAVE_NTGUI` (`configure.ac` 3027; `DebPrint` at C 459, C 2142's `#ifdef`) | `feature = "have_ntgui"` (the arm reading needs it too) | the same build |
| C 24 `#define DEFER_MS_W32_H` (the marker `nt/inc/ms-w32.h` reads) | `pub const DEFER_MS_W32_H: bool = true` - the `ms-w32.rs` marker shape, not a `cfg` | every unit that includes `ms-w32.h` |
| C 52-55 `#ifdef HAVE_LANGINFO_CODESET` (`<nl_types.h>`, `<langinfo.h>`) | `all (feature = "windowsnt", feature = "have_langinfo_codeset")`; the `#else` arm is empty | a langinfo-enabled build (the frozen table has it 1) |
| C 120-122 `#ifdef __MINGW64__` (`#define _start __start`) | `feature = "mingw64"`, as the `no_mangle`/`export_name` pair on `_start` | a mingw-w64 build |
| C 136-141 `#if 1` (the `EMACS_DEBUG` hook) | no gate: the condition is true in every configuration and C has no `#else` | always |
| C 899-911 `#ifdef HAVE_SETITIMER` / `#else` in `alarm` | `feature = "have_setitimer"`; the `#else` arm is the other `#[cfg]` block (`return seconds;`) | both arms carried; the frozen table has it 1 |

## C line ranges

One row per C block, in C's order, tiling C 1-912 contiguously (and the remainder as the last row,
which `check-dead-file-coverage.py` requires):

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 19-24 | the blank, C 20-23's attribution comment, the blank | carried as C's own text |
| 25-25 | `#define DEFER_MS_W32_H` | the marker as a boolean constant (above) |
| 26-42 | the include list `<config.h>` … `<ms-w32.h>` (with C 41's "CRT headers before ms-w32.h" note) | file gate on the include graph; the three no-file headers (`<io.h>` C 33, `<mbstring.h>` C 38, `<locale.h>` C 39) recorded |
| 44-48 | the `#undef signal` … `#undef kill` block | C's text in a comment (the nt/inc renames are `macro_rules!`s, whose call form a plain call cannot reach) |
| 50-55 | `<windows.h>`, then `#ifdef HAVE_LANGINFO_CODESET` around `<nl_types.h>`/`<langinfo.h>` | SDK section below; C 52-55 both arms (`have_langinfo_codeset`) |
| 57-64 | the eight `"…"` includes (`"lisp.h"` … `"coding.h"`) | file gate |
| 66-69 | `#define RVA_TO_PTR(var,section,filedata)` | the file's own macro, **written out in slice 12's wake** once `IMAGE_SECTION_HEADER` existed (slice 1 could only carry its text: Deviations 6, retired in the Slice 13 section) |
| 71-72 | `extern BOOL g_b_init_compare_string_w;` / `g_b_init_debug_break_process;` | file gate; the two definitions are `src/w32.c`'s (`w32.rs` 899-902) |
| 74-75 | `int sys_select (…)` prototype | dropped (PORTING.md 0.1-12); the definition is C 3109 |
| 77-82 | `sig_handlers[NSIG]`, `sig_mask`, `crit_sig` | file gate |
| 84-89 | the three `_Noreturn void *` prototypes | dropped (0.1-12); the definitions are C 91-113 |
| 91-97 | `malloc_before_init` | file gate |
| 99-105 | `realloc_before_init` | file gate |
| 107-113 | `free_before_init` | file gate |
| 115-115 | `extern BOOL ctrl_c_handler (unsigned long)` | file gate (`src/w32console.c` 92 defines it) |
| 117-122 | `#ifdef __MINGW64__` / `#define _start __start` | both arms (`mingw64`), as the export-name pair on `_start` |
| 124-124 | `extern void mainCRTStartup (void);` | file gate |
| 126-158 | `_start` (the C 126-129 comment, the C 130 declaration, the C 132-158 body with `#if 1`) | file gate; C 130 dropped (0.1-12), C 136-141 as above |
| 160-190 | `sys_signal` | file gate |
| 192-216 | `sigaction` | file gate |
| 218-225 | `sigemptyset` | file gate |
| 227-244 | `sigaddset` | file gate |
| 246-257 | `sigfillset` | file gate |
| 259-290 | `sigprocmask` | file gate (the `switch` becomes a `match`; Deviations 8) |
| 292-298 | `pthread_sigmask` | file gate |
| 300-312 | `sigismember` | file gate |
| 314-318 | the "A fuller emulation of 'raise'" comment and `#undef raise` | C's text in a comment |
| 320-360 | `w32_raise` | file gate (the `switch` becomes a `match`; Deviations 8) |
| 362-366 | `getpgrp` | file gate |
| 368-372 | `tcgetpgrp` | file gate |
| 374-378 | `setpgid` | file gate |
| 380-384 | `setsid` | file gate |
| 386-392 | the "Emulations of interval timers" comment | carried; it also fixes the reading of `<sys/time.h>` (Deviations 5) |
| 394-401 | `struct itimer_data` | file gate (C's three `volatile`s dropped; Deviations 4) |
| 403-410 | `ticks_now`, `real_itimer`, `prof_itimer`, `clocks_min`, `disable_itimers`, `crit_real`, `crit_prof` | file gate |
| 412-418 | `typedef BOOL (WINAPI *GetThreadTimes_Proc) (…)` | file gate |
| 420-420 | `static GetThreadTimes_Proc s_pfn_Get_Thread_Times;` | file gate |
| 422-423 | `#define MAX_SINGLE_SLEEP 30` / `TIMER_TICKS_PER_SEC 1000` | file gate (constants) |
| 425-477 | `w32_get_timer_time` | file gate (the `LARGE_INTEGER`-shaped union, Deviations 3) |
| 479-604 | `timer_loop` | file gate |
| 606-646 | `stop_timer_thread` | file gate |
| 648-664 | `term_timers` | file gate; the declaration is `include/w32.rs` 331 (Deviations 7) |
| 666-688 | `init_timers` | file gate; the declaration is `include/w32.rs` 332 (Deviations 7) |
| 690-747 | `start_timer_thread` | file gate |
| 749-805 | `getitimer` | file gate (C's `__int64`; Deviations 8) |
| 807-894 | `setitimer` | file gate (C's `__int64`; Deviations 8) |
| 896-912 | `alarm` (this slice's last definition) | file gate; C 899-911 both arms (`have_setitimer`) |
| 913-1039 | the C 914-962 wait-object overview comment, C 968-971's four `WFO_*` constants, C 975-978's two `typedef`s and their two `extern` objects, C 981-1034's three records (`wait_objects_context`/`wait_objects_pool`/`wait_objects_info`) and C 1038-1039's two statics | file gate (slice 2); C 1040 is a blank line and C 1041-1050's comment belongs to C 1051's `wait_objects_thread`, which is the next row's first item |
| 1041-1299 | the wait-object worker pair: C 1041-1050's `wait_objects_thread` overview comment, C 1051-1117's `wait_objects_thread` and C 1123-1299's `start_wait_objects` | file gate (slice 3); the `switch` over the second wait's result becomes an `if`/`else if` chain (Deviations 38), and C's five `WAIT_*`/`INFINITE` values and six wait-event entry points stand in the use-site `<windows.h>` block (Deviations 37) |
| 1301-1511 | the pool's four control functions: C 1301-1347's `stop_wait_objects`, C 1349-1437's `end_wait_and_return`, C 1439-1483's `shrink_wait_pool` and C 1485-1511's `free_wait_pool` | file gate (slice 4); `WAIT_ABANDONED` joins the use-site `<windows.h>` block (Deviations 40), C's `for` loops whose bodies `break` are `while` loops, and C's `int`/`DWORD` boundary conversions are spelled with `as` (Deviations 39) |
| 1513-1693 | the file's two wait entry points: C 1513-1526's `wait_for_objects` overview comment with C 1527-1597's `wait_for_objects`, and C 1599-1616's `msg_wait_for_objects` overview comment with C 1617-1693's `msg_wait_for_objects` | file gate (slice 5); `MsgWaitForMultipleObjects` and `QS_ALLINPUT` join the use-site `<windows.h>`/`<winuser.h>` block (Deviations 42), C's two counted `for` loops become `while` loops whose `continue` path carries C's increment clause (Deviations 43), and C's 64/63 group arithmetic keeps its `int`/`DWORD` spelling (Deviations 39) |
| 1695-1832 | the child-process block's opening: C 1695-1823's subprocess/network overview comment (the file's longest banner, carried as C's own text), C 1825-1826's `_P_NOWAIT`, C 1828-1830's child-process management list (`child_proc_count`, `child_procs[MAX_CHILDREN]`) and C 1832's `reader_thread` forward declaration (dropped, 0.1-12) | file gate (slice 6); the record and the array's bound are the mirror's (`include/w32.rs` 326-355 / 242), not re-spelled here (0.1-3) |
| 1834-2115 | the child-process block's five working functions: C 1834-1933's `new_child`, C 1935-1998's `delete_child` (with its C 1958-1969 `#if 0` region carried as C's text and its `fd_info` guard), C 2000-2011's `find_child_pid`, C 2013-2024's `release_listen_threads` and C 2026-2115's `reader_thread` | file gate (slice 7); the unit spells `w32.h` 74-113's `child_process` record and the three SDK records it embeds (the mirror defers it - Deviations 47), the two `goto Initialize` jumps become `break 'initialize` (Deviations 48, notes row), and `fd_info`'s deferred `*mut c_void` member is cast as the mirror's note spells it (Deviations 49) |
| 2117-2121 | `process_dir`: C 2117-2120's four-line comment (carried as C's own text) and C 2121's file-scope `static char *` | file gate (slice 8); the object is the `create_child` reader's and `set_process_dir`'s writer, both past this slice |
| 2123-2205 | `create_child`: the process-creation function (its `HAVE_NTGUI` arm, its two `#if 0` regions carried as C's text, the `filename_to_ansi`/`CharNextA` path fix-up, the batch-file `exe` clearing, the three create flags and C 2202-2204's `EH_Fail:` tail) | file gate (slice 9); the round's use-site `<windows.h>`/`<winbase.h>`/`<winuser.h>` block spells `STARTUPINFO`, `SECURITY_ATTRIBUTES`, `MAX_PATH`, the nine constants and the three entry points (Deviations 50), and `xstrcasecmp` stands there with `dispextern.h` 104's citation (Deviations 51) |
| 2207-2268 | `register_child` (C 2207-2242, with C's `(DWORD)` cast, its dead `FULL_DEBUG` arm carried as C's text and the `fd_info` in-use guard) and `reap_subprocess` (C 2244-2268, whose own dead arm sits between C 2250's comment and the two `CloseHandle`s) | file gate (slice 10); C's `status` value is `STATUS_READ_ACKNOWLEDGED` (the mirror's 201) and the `fd_info` member is the deferred `*mut c_void` (Deviations 49's cast) |
| 2270-2449 | `waitpid`: C's Posix comment, the single-pid arm, the any-child arm, the `do`/`while` (the port's `loop` + negated test, P-95), the five-way answer, `GetExitCodeProcess`, the `STILL_ACTIVE` punt and the exit-code massage | file gate (slice 11); `WNOHANG` and `STATUS_CONTROL_C_EXIT` stand at the use site (Deviations 54/55), C's two dead `FULL_DEBUG` arms are carried as C's text, and C's `retval <<= 8` is the port's own shift |
| 2451-2528 | the memory-mapped-file helpers and the section lookup: C 2451-2481's `open_input_file`, C 2483-2509's `rva_to_section` (with C's linker comment and the `max!` pair), C 2511-2521's `close_file_data` and C 2523-2528's `#ifndef IMAGE_NT_OPTIONAL_HDR32_MAGIC` fallback carried as C's text | file gate (slice 12); the round's use-site `<windows.h>`/`<winbase.h>`/`<winnt.h>` block spells the file API and the four PE records (Deviations 57), `min!`/`max!` come from the `min-max.rs` mirror included under C 57's lisp.h line (Deviations 58), and the mirror `include/w32heap.rs`'s four declarations moved into a glob-imported module in the same round (Deviations 59) |
| 2530-2674 | `w32_executable_type`: the ANSI-codepage note, the four defaults, the `.com`/`.bat`/`.cmd` extension test with its `COMSPEC` recursion, the DOS/NT header walk (the two records, the two "DOS program" answers, the 32-bit optional-header read, the `IMAGE_NT_OPTIONAL_HDR64_MAGIC` arm under its feature, and the import-directory walk with `RVA_TO_PTR` and the two DLL-name tests) and the `unwind` tail | file gate (slice 13); the round's use-site `<windows.h>`/`<winnt.h>` block spells the two records, `LOWORD` and the 64-bit pair (Deviations 60), the `#ifdef IMAGE_NT_OPTIONAL_HDR64_MAGIC` guard becomes `feature = "image_nt_optional_hdr64_magic"` (Deviations 61), C's `goto unwind` is `break 'unwind` (Deviations 48's device), and `RVA_TO_PTR` is written out at C 66-69 (Deviation 6 retired) |
| 2676-2720 | `compare_env` and `merge_and_sort_env`: the environment pair `sys_spawnve` calls (C's `toupper` comparison, the two copy loops with their pointer-difference counts, and C 2717's `qsort`) | file gate (slice 14); `qsort` stands at the use site with C's own text (`src/buffer.rs` 129-134's row) and `compare_env` takes the `extern "C"` ABI the comparator needs while keeping C's `static` (Deviations 64/65) |
| 2722-3078 | `sys_spawnve`: the mode guard, the `openp` fallback, the ANSI conversion and `_mbspbrk` guard, the DOS-app `CMDPROXY` branch, the two `argv` quoting passes (with C 2976-2988's `#if 0` arm carried as C's text) and the `envp` merge, ending at the `new_child`/`create_child` pair | file gate (slice 15); C's four `alloca` sites are PORTING.md P-05's `sa_stack_alloc` (Deviations 66), the CRT/SDK names no mirror carries stand at the use site (Deviations 69), and the round removed the unit's three stale `errno` duplicates (Deviations 68) |
| 3080-3097 | the `sys_select` overview comment | **landed in slice 17**, with the function it documents |
| 3099-3106 | the three extern objects `sys_select` reads: `keyboard_handle` (`w32console.c`), `interrupt_handle` (`w32xfns.c`) and `proc_buffered_char` (`process.c`) | file gate (slice 16); C's three one-line comments stand above each declaration, and the array's length is the mirror's `FD_SETSIZE` through its module path (Deviations 36/71) |
| 3108-3483 | `sys_select`: the file's longest function (the four function-local statics, the timeout decode and set copies, the descriptor scan with its stdin and child/socket arms, the `count_children` label and child scan, the five-way answer, the fairness `do`/`while` and the timeout recheck) | file gate (slice 17); the four statics are Rust `static mut` items inside the function (Deviations 72), C's `goto count_children` is a labeled loop and the `do`/`while` the negated break (Deviations 73), the three dead debug arms are carried as C's text (Deviations 74), and `detect_input_pending`/`GetTickCount` stand at the use site (Deviations 75) |
| 3485-3526 | the three items between `sys_select` and `sys_kill`: C 3485-3496's `w32_valid_process_id`, C 3498-3523's `find_child_console` and C 3525-3526's `DebugBreakProcess_Proc` typedef | file gate (slice 18); the round's two `<winuser.h>` entry points and `LPARAM` stand at the use site (Deviations 76), and `w32_valid_process_id` forced the mirror `include/w32common.rs`'s eleventh P-39 move (Deviations 77) |
| 3528-3901 | `sys_kill` (C 3528-3819: the negative-pid and signal-whitelist guards, the `sig == 0` probe with its two nested `switch`es, the child lookup and the two `OpenProcess`/`EnumWindows` arms, the `SIGINT`/`SIGQUIT` keyboard simulation, the `SIGTRAP` `DebugBreakProcess` lookup and the terminate arm) and the standard-handle pair `prepare_standard_handles`/`reset_standard_handles` (C 3821-3901) | file gate (slice 19); the round's `<windows.h>`/`<winuser.h>`/`<io.h>` block spells the whole surface (Deviations 78), the two `switch`es are `match` arms (notes row), and C 3898's `handles[0]` keeps its index (the token reading) |
| 3903-3967 | `set_process_dir` (C 3903-3907), the winsock overview comment with C 3920-3922's two `w32.c` externs, and the block's first two `DEFUN`s: `w32-has-winsock` (C 3924-3956) and `w32-unload-winsock` (C 3958-3967) | file gate (slice 20); the two `DEFUN!` invocations carry C's docstrings in their `doc:` slot (Deviations 81) and the externs stand where C declares them (Deviations 82/83) |
| 3969-4082 | the banner comment and the next three `DEFUN`s: `w32-short-file-name` (C 3973-3995), `w32-long-file-name` (C 3998-4030) and `w32-set-process-priority` (C 4032-4082) | file gate (slice 21); the three `DEFUN!` invocations carry C's docstrings in the `doc:` slot (Deviations 81's shape), and the round's four `<winbase.h>` names stand at the use site (Deviations 84) |
| 4084-4135 | the `DEFUN` `w32-application-type`: the expand/encode/`unixtodos_filename`/`filename_to_ansi` chain, the `_mbspbrk` guard and the `w32_executable_type` call with its four `Q…` answers | file gate (slice 22); the four symbols are the `globals.rs` mirror's (this file's `syms_of_ntproc` is where C defines them - the next lane of rows) |
| 4137-4234 | the langinfo arm: C 4137's `#ifdef HAVE_LANGINFO_CODESET` (live in the frozen table, carried under `have_langinfo_codeset`), C 4139-4143's `LOCALE_IPAPERSIZE` fallback and C 4145-4234's `nl_langinfo` (the `w32item` table, the lazily grown buffer, the `CODESET` `cp`-prefix path and the `_NL_PAPER_*` table) | file gate (slice 23); the round's `<winnt.h>`/CRT/gnulib surface stands at the use site (Deviations 86), and the `countof` macro is spelled here because its gnulib mirror is in no unit's graph (Deviations 87) |
| 4236-4321 | `w32-get-locale-info` (C 4236-4292, with its three branches), `w32-get-current-locale-id` (C 4295-4303) and the static `int_from_hex` (C 4305-4321) | file gate (slice 24); the round's `<winbase.h>` names stand at the use site (Deviations 88), and the `hex` digit table is C's C string under the tree's `*b"…"` device |
| 4323-4571 | the twelve `DEFUN`s of the locale/codepage/console/keyboard block and the four file-scope objects and callbacks they read: `w32-get-valid-locale-ids` (C 4323-4348, with `Vw32_valid_locale_ids` and `enum_locale_fn`), `w32-get-default-locale-id` (C 4351-4362), `w32-set-current-locale` (C 4365-4384), `w32-get-valid-codepages` (C 4386-4410, with `Vw32_valid_codepages` and `enum_codepage_fn`), the four console-codepage `DEFUN`s (C 4413-4466), `w32-get-codepage-charset` (C 4469-4496) and the three keyboard-layout `DEFUN`s (C 4499-4571) | file gate (slice 25); the round's `<windows.h>`/`<wingdi.h>` surface stands at the use site (Deviations 90-91), the `HIWORD` macro at C 4515's use site (the C 2593 `LOWORD` shape, Deviation 92), and `HWND` is declared there because slice 22's Deviation 76 note was wrong about the graph carrying it (Deviation 93) |
| 4573-4903 | **the remainder of the file** - C 4573's `get_lcid_callback` and C 4578-4579's `found_lcid`/`lname` through `get_lcid` (C 4618-4641), `w32_compare_strings` (C 4651-4730), the `syms_of_ntproc` table (C 4764-4903) and C 4861-4869's `#if 0` | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

## Evidence

① **Frozen `build-tu`** (`cd femacs && CARGO_TARGET_DIR=/tmp/w32proc-lane-1/cargo cargo run -q -p
xtask -- build-tu src/w32proc.c`): before the port, `built src/w32proc.rs -> ../build/femacs/w32proc.o`,
exit 0, **336 bytes** (the P-08 empty object); after it, the same command and the same 336-byte
object (every item is `#[cfg (feature = "windowsnt")]`, so the frozen compile sees the include
graph's `config.rs`/`conf_post.rs` only).  A direct `rustc --edition 2024 --crate-type=lib
--emit=obj -C panic=abort -C codegen-units=1 -C opt-level=0 src/w32proc.rs` on the unit re-reads the
same `exit 0 / 336 bytes / 0 error` after the last edit.

② **The arm reading** (`cargo run -q -p xtask -- probe src/w32proc.rs <out> --crate-type=lib
--emit=obj --cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"'
--cfg 'feature="have_langinfo_codeset"'`): **exit 1, 53 errors, and not one of them is anchored in
the unit** (`_root.rs`: 0 - P-86 ⑦'s "by-file" reading, which is the one that means something when
the include graph is what breaks).  By file: `lib/include/signal.rs` 22 (E0428: the frozen
`<signal.h>` mirror's `SIGABRT`/`SIGALRM`/`SIGCHLD`/…/`NSIG`/`SIG_IGN`/`SIG_BLOCK`/`SIG_UNBLOCK`/
`SIG_SETMASK`/`sigset_t`/`sigaction`/`signal_decls` against `nt/inc/include/ms-w32.rs` and this
unit's MinGW-runtime section - the `<signal.h>` a `windowsnt` unit reaches is pulled from
`include/lisp.rs` → `include/thread.rs`, not through `nt/inc`), `src/include/w32term.rs` 12 (E0425:
`HWND`, `RECT`, `HDC`, `COLORREF`, `Emacs_GC`, `font` - names that mirror's own head note leaves to
its includers, i.e. P-87 ③), `nt/inc/include/unistd.rs` 6 (AT_*/O_* against `lib/include/fcntl.in.rs`
- the clash that mirror's own note predicts), `nt/inc/include/ms-w32.rs` 3
(`HAVE___BUILTIN_UNWIND_INIT` against `include/config.rs`; `ENOTSUP`/`EOPNOTSUPP` against
`lib/include/errno.rs`), `src/include/w32_rs` 2 + `lib/include/sys_time.rs` 1 (this unit *defines*
`term_timers`/`init_timers`/`setitimer`, which those two mirrors declare at crate root instead of in
a glob-imported module - the `w32.md` owed-6 class), `src/include/w32common.rs` 2 (`SYSTEM_INFO`,
`OSVERSIONINFO`, which that mirror defers by its own note), `nt/inc/include/pwd.rs` 2 (`uid_t`,
`gid_t` against `lib/include/sys_types.in.rs`), and `src/include/frame.rs` 1 - a **deliberate**
`compile_error!` (`include/frame.rs` 329): `frame.h` 471's `#ifndef HAVE_NTGUI` removes
`override_redirect`, so the `HAVE_NTGUI` arm of the flag run is unmeasured on this machine.
Budget: `probe` was spent **4 发** (the 4th re-reads the unit after its last two fixes) - one over
the ≤3 asked for, reported here.

③ **`python3 tools/extract-abi.py --check emacs/src/w32proc.c femacs/src/w32proc.rs`** (non-vacuous
now, P-86 ⑩): 118 findings, all of them the slice boundary or the four token rows below -
`missing function` 39, `missing subr` 20, `missing defsubr` 20, `missing file-scope object` 15,
`missing macro` 12, `missing constant` 7, **`does not mention` 4**, `line count drifted 68.4%: C
4903 vs Rust 1550` 1.  The 39/20/20/15/12/7 are the C 913-4903 remainder (the first missing function
is `close_file_data`, C 2513; the first missing subr is `Fw32_has_winsock`, C 3924) plus, in this
slice's own text, `RVA_TO_PTR` (carried as text then, written out in slice 13 - Deviations 6) and the two `WFO_*`/`LOCALE_*`-family
constants.  The four `does not mention` rows are Deviations 8.

④ **`python3 tools/check-cond-eval.py src/w32proc.c`**: `26 conditions, 0 unknown, 0 disagreements
[VERIFIED]` (the four of this slice are the Gate-mapping rows above; `cinventory`'s own verdicts are
C 52 True, C 120 False, C 136 True, C 899 True).

⑤ **`python3 tools/check-declarations.py`**: `10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures`, `read 682 files` - unchanged by this batch (the unit's first line is the
`// femacs: port of …` form, and the only C comment it carries that names the program is the
`Aborting Emacs works specially anyway` one, which is not the product name this gate reads).

⑥ **The coverage gates**: `python3 tools/check-dead-file-coverage.py` reports `32 ledger(s) bound
(a C range table), 0 failure(s)`, this file's own row being `info docs/ports/w32proc.md: 49 range
row(s) cover src/w32proc.c (4903 lines; 44 blank line(s) between blocks need no row)` - the bound
count was `29` when this batch started, and the other two rows are in-flight lanes' ledgers, not
this batch's.  `python3 tools/check-ledger-ticks.py` reports `234 ledger(s) bound (a function tick
to compare), 0 failure(s), 0 carried by another file, 356 row(s) with no function tick`, the same
reading as before this batch: this file's checklist is empty (P-86 ②), so it contributes no ticked
row and no "row without a tick".

⑦ **`python3 tools/check-port-lines.py`**: `75 file(s) under ten percent, 101,649 reference
line(s); 75 ratchet row(s), 0 new, 0 stale` - this file is **not** in
`docs/baseline/port-lines-pending.tsv` (it is 31.6% of its reference after this slice, over the ten
percent threshold), so unlike `docs/ports/w32.md`'s owed 1 there is no ratchet row for it to drop.

⑧ **The unit**: `femacs/src/w32proc.rs` 1550 lines, 66,415 bytes, md5
`e7f0a31a5072265f75da9d7199ba66c1`; the reference `emacs/src/w32proc.c` md5
`fa33eefb166b44da521fa105787f27d4` (4903 lines, unchanged - `emacs/` is read-only).

⑨ **The arm-reach pre-flight** (a `/tmp` walk of the unit's `include!` graph, ignoring `cfg`): 52
files reached from the unit under the full door set, and the three files whose *crate-root* items
collide with this unit's own definitions are the ones Evidence ② names
(`lib/include/signal.rs`, `src/include/w32.rs`, `lib/include/sys_time.rs`, `nt/inc/include/unistd.rs`,
`lib/include/fcntl.in.rs`, `src/include/w32common.rs`); it is a text reading of the include graph,
**not** a compile.

## Deviations

1. **The two stand-in sections (P-88 ①).**  `femacs/src/w32proc.rs` carries the names of C 36's
   `<signal.h>` (MinGW's runtime header, which has no file in the reference tree: `sigset_t`, the
   seven `SIG*` numbers, `SIG_DFL`/`SIG_IGN`/`SIG_ERR`, `signal`, `raise`) and of C 50's
   `<windows.h>` (the scalar and handle typedefs, `FILETIME`, `ULARGE_INTEGER`, `CRITICAL_SECTION`,
   `GetThreadTimes_Proc`'s shape via `PHANDLER_ROUTINE`/`LPTHREAD_START_ROUTINE`, the seventeen
   entry points and their constants).  Every declaration carries a `/// C:` line with the header's
   own text; the declaration site is C's own `#include` line and no file is added under
   `femacs/src/include/`, which is the reference headers' mirror layer.
2. **`sigset_t` is the unit's, and its width is UNVERIFIED.**  `nt/inc/ms-w32.h` 345-349 typedefs it
   as the runtime's `_sigset_t`, which the tree does not define; `docs/ports/include-ms-w32.md`
   lists it under "Names with no mirror" and says such names are "named from the including unit", so
   the unit spells it `c_uint` (C's own operations - `*set = 0`, `*set |= (1U << signo)`,
   `*set = 0xFFFFFFFF`, `sizeof (*set) * CHAR_BIT` against `NSIG` 23 - need a 32-bit unsigned
   scalar) and the *width* stays UNVERIFIED: only a mingw toolchain can measure `_sigset_t`.
3. **`ULARGE_INTEGER`'s nameless inner struct gets a name.**  The SDK's `LowPart`/`HighPart` views
   live in an unnamed `struct` inside the union; Rust has no nameless struct, so the port names it
   `u` and C's `temp_creation.LowPart` reads as `temp_creation.u.LowPart` (the same deviation
   `femacs/src/w32.rs` records for `LARGE_INTEGER`).  The union and `CRITICAL_SECTION`,
   `FILETIME`, `OSVERSIONINFO`-shaped records are `UNVERIFIED` here; their layout assertions are
   owed (see `## Owed` 6).
4. **C's `volatile` on `struct itimer_data`'s members is dropped.**  Rust has no field form for it
   (the `android.rs` 548-552 reading), and the three fields' accesses go through
   `EnterCriticalSection`/`LeaveCriticalSection` pairs, which is what the qualifier was for.
5. **`<sys/time.h>` resolves to the frozen target's mirror, not to the Windows shim.**  C reaches
   it transitively (C 63 `"w32term.h"` → `w32gui.h` → `systime.h` → `<sys/time.h>`), where the
   MinGW build gets `nt/inc/sys/time.h` (`ITIMER_REAL` 0, `ITIMER_PROF` 1, `struct itimerval` over
   the runtime's `timeval`).  The port's `include/w32term.rs` pulls `w32gui.rs` → `systime.rs` →
   `femacs/lib/include/sys_time.rs` instead (so `ITIMER_PROF` is 2), and the shim's own note records
   the same conflict from its side; this unit's `#defines` and its use of the three names are C's, and
   the reconciliation is the mirror round's (Owed 5).  `alarm`'s `#ifdef HAVE_SETITIMER` arm is
   still live text, so the divergence changes no code path.
6. **`RVA_TO_PTR` (C 66-69) is carried as C's own text.**  Its body adds three fields of
   `<windows.h>`'s `IMAGE_SECTION_HEADER` to `file_data.file_base`; the port has no mirror for the
   record (`include/w32heap.rs` 63-76 spells it `*mut c_void`, P-72 ③/P-55), so a Rust body would
   have to invent the record's layout.  The macro's two users are C 2635/2644, past this slice.
7. **The four names this slice defines that a mirror also declares (the `w32.md` owed-6 class).**
   `include/w32.rs` declares `term_timers` and `init_timers` at crate root (its 328-332) although
   the three objects beside them sit in the glob-imported `w32_objs` module, and
   `lib/include/sys_time.rs` 74 declares `setitimer` the same way: a `windowsnt` unit that *defines*
   them is `E0428` (Evidence ②).  This unit's text is C's; the mirrors' shape is Owed 5.
8. **Four token rows the `--check` needs and this unit cannot write.**  `sigprocmask` and
   `w32_raise` spell C's `switch`/`case`/`default`/`break` as a `match` (PORTING.md 2.2), and
   `getitimer`/`setitimer` spell C's `__int64` as `i64` (PORTING.md 1.1's fixed-width mapping for
   MSVC's 64-bit builtin, the `docs/ports/include-stat.md` 88-89 reading - that ledger writes
   `c_longlong`, which this unit's alias set does not have: `include/lisp.rs` 14 imports
   `c_ulonglong` but not `c_longlong`) - both are `macro-body-notes.tsv` rows,
   which is not this lane's file (Owed 4).  The `_start` row of the same class was fixed in the unit
   instead: C's `(PHANDLER_ROUTINE) ctrl_c_handler` cast is bound to a local of the alias's type,
   so the alias's own token survives in the port (and `as` cannot spell that cast - `E0605`,
   measured).
9. **C's `_Noreturn void *` returns keep the pointer type.**  `malloc_before_init`,
   `realloc_before_init` and `free_before_init` are assigned to `the_malloc_fn`/`the_realloc_fn`/
   `the_free_fn` (C 143-145) whose port signature is the `nt/inc/include/ms-w32.rs` 149-154 one, so
   writing `-> !` (the `AVOID` row's spelling for a `void` return) would not type-check; the `exit
   (-1)` tail is `!` and C's own "never returns" statement.
10. **The `#undef` blocks are C's text in comments.**  C 44-48 cancels the `signal`/`wait`/
    `spawnve`/`select`/`kill` renames and C 318 cancels `raise`; the mirror spells those renames as
    `macro_rules!`s, which fire on the `name! (…)` form only, so the port's plain calls reach the CRT
    entries C's `#undef` selects by construction (the `cm.rs` 86 reading for the text itself).
11. **Three locals C leaves uninitialized get C's own zero value** (PORTING.md 0.1-13):
    `w32_get_timer_time`'s `retval` and its seven records, `stop_timer_thread`'s `status`,
    `start_timer_thread`'s `exit_code`/`tid`/`th` and `setitimer`'s `tem`.  Two more are the port's
    spelling of C's in-condition assignments: `now = w32_get_timer_time (hth)` is hoisted out of
    `if (expire > (now = …))` (same order), and `stop_timer_thread`'s
    `(status == FALSE && { err = GetLastError (); err == ERROR_INVALID_HANDLE })` keeps C's
    short-circuit with a block expression.
12. **`ssize_t` and `exit` are declared by the unit.**  `nt/inc/include/unistd.rs`'s own note leaves
    `ssize_t` to the including unit (its `<sys/types.h>` mirror has none), and the port's
    `<stdlib.h>` mirror has no `exit` at all - `src/emacs.rs` 320-321 declares it the same way.  Both
    are owed as mirror rows (Owed 3).

## Owed (nothing here is claimed as done)

1. **`femacs/xtask/Cargo.toml`**: three feature names this unit uses are registered nowhere -
   `have_langinfo_codeset` (C 52), `mingw64` (C 120) and `have_setitimer` (C 899).
   (`windowsnt` and `have_ntgui` are precedents that are registered.)
2. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32proc`, tolerance ≥ 0.69, basis
   `total`, with this slice's reason from the Deviations (the two stand-in sections and the per-item
   `/// C:` citations are the physical-line cost); until then `extract-abi.py --check` reports `line
   count drifted 68.4%` (Evidence ③).
3. **`femacs/lib/include/errno.rs`**: two rows, `ESRCH` 3 and `EFAULT` 14 (C 720-721, 767-768 set
   them; the mirror carries neither).  **`femacs/lib/include/stdlib.rs`**: the `exit` row
   ("`_Noreturn void exit (int);`").  **`femacs/lib/include/sys_types.in.rs`**: the `ssize_t` row.
   Until they land, this unit declares all four itself (Deviations 12) - and each declaration
   becomes an `E0428` the day the mirror grows the name.
4. **`docs/ports/macro-body-notes.tsv`**: four rows, `sigprocmask` and `w32_raise`
   (`switch,case,default,break`: C's switch is the port's `match`, PORTING.md 2.2) and `getitimer`,
   `setitimer` (`__int64`: PORTING.md 1.1 spells MSVC's builtin `i64`/`c_longlong`).  Plus one row or
   a measured `IMAGE_SECTION_HEADER` for `RVA_TO_PTR` (C 66-69: carried as text until slice 13 wrote it out - Deviations 6;
   either an `excluded-macros.txt` row of kind `deferred` or the record itself, which is the mirror
   round's call - the record also needs a Windows/mingw clang layout measurement).
5. **The mirror round's reconciliation of the Windows arm** (Evidence ②, Deviations 5/7): (a)
   `lib/include/signal.rs` vs `nt/inc/include/ms-w32.rs` + this unit's runtime section - a
   `windowsnt` unit's `<signal.h>` is pulled through `include/lisp.rs` → `include/thread.rs`, so the
   frozen target's mirror and MinGW's names are both live and 22 names are `E0428`; (b)
   `include/w32.rs`'s `term_timers`/`init_timers` and `lib/include/sys_time.rs`'s `setitimer` belong
   in glob-imported modules, like the `w32_objs` beside them; (c) `include/w32term.rs`'s items name
   `HWND`, `RECT`, `HDC`, `COLORREF`, `Emacs_GC` and `font`, which no mirror in a `windowsnt` unit's
   graph supplies (P-87 ③); (d) `nt/inc/include/unistd.rs`'s `AT_*`/`O_*` rows clash with
   `lib/include/fcntl.in.rs`; (e) `nt/inc/include/ms-w32.rs`'s `HAVE___BUILTIN_UNWIND_INIT`,
   `ENOTSUP`, `EOPNOTSUPP` clash with `include/config.rs`/`lib/include/errno.rs`; (f)
   `include/w32common.rs`'s `SYSTEM_INFO`/`OSVERSIONINFO` deferral and `nt/inc/include/pwd.rs`'s
   `uid_t`/`gid_t` against `lib/include/sys_types.in.rs`.  **Until (a)-(f) are resolved, no
   `windowsnt` translation unit that reaches `lisp.h` can compile at all**, which is why this
   slice's arm reading is a per-file error census rather than a run.
6. **The layout assertions for the SDK records this unit spells** (`FILETIME`, `ULARGE_INTEGER`,
   `CRITICAL_SECTION`, `SYSTEM_INFO`/`OSVERSIONINFO`'s consumers) once a Windows or mingw toolchain
   exists: `xtask layout-asserts` reads a C compiler's readings, and `x86_64-w64-mingw32-gcc` is
   absent here, so no `core::assert!` is written for any of them.
7. **The next lane's slice: C 913-4903**, whose first item is the C 914-962 banner and whose first
   definition is `wait_objects_thread` (C 1052), through `syms_of_ntproc` (C 4764).  Its own new
   conditions start at C 1958 (`#if 0`), C 2142 (`HAVE_NTGUI`), C 2224/2251/2389/2440/3253/3404
   (`FULL_DEBUG`), C 2525 (`IMAGE_NT_OPTIONAL_HDR32_MAGIC`), C 2610
   (`IMAGE_NT_OPTIONAL_HDR64_MAGIC`), C 2976 (`#if 0`/`#else`), C 3219 (`CHECK_INTERLOCK`) and
   C 4137 (`HAVE_LANGINFO_CODESET` again, with `LOCALE_IPAPERSIZE`).

## Not claimed

Every line of this slice is behind gates the frozen configuration leaves false: ① is an empty
object, and the readings of ②-⑨ are compile-level or textual.  **Nothing here was executed** - no
timer thread, no `GetThreadTimes`, no `WaitForMultipleObjects`, no `signal`/`raise` delivery, no
`CreateThread`.  There is **no Windows-target compile and no layout measurement**: the arm flip is
`rustc` on this machine with the four `--cfg`s, whose 53 errors are all in other files, and the
pre-flight is a text walk of the include graph.  The file's `#if` verdicts are
`tools/check-cond-eval.py`'s (and it is `[VERIFIED]` for this file), not clang's.  C 913-4903 is
**not ported here**; the 39 missing functions, 20 subrs, 20 defsubrs, 15 objects, 12 macros and 7
constants of Evidence ③, the four token rows of Owed 4 and the 68.4% drift are that boundary, not
defects.  The `windowsnt`+`have_ntgui` arm does not compile today, and that is the mirror graph's
state, not this slice's: the unit's own text contributes **0 errors** to that reading (Evidence ②).

## Owed 3 corrected (2026-10-05, status 3.451)

The three mirror rows this slice asked for are not one job:

* `errno.rs`'s `ESRCH`/`EFAULT` - **done** in 3.446, together with `ENOEXEC` (all three measured).
* `sys_types.in.rs`'s `ssize_t` - **must not be added**.  `femacs/lib/include/unistd.in.rs` already
  includes `sys_types.in.rs` (31) and declares `pub type ssize_t = c_long;` (34), so a second copy
  there would be `E0428` in every unit that includes both - the same tree-wide class 3.444's own
  mis-placed attribute produced (74 units).  A unit that needs `ssize_t` includes `unistd.in.rs`.
* `stdlib.rs`'s `exit` - the "no mirror carries it yet" class; P-90's seventh half (3.449) is what
  keeps the 26 in-unit declarations across a dozen files lawful, and moving them into a mirror is one
  deliberate move-back round, not a row this file can add on its own.

## Slice 2 (2026-10-08, status 4.104 - the integrator sets the round number): C 913-1039

**What this round is.**  The wait-object support's *declarations*, which C puts after `alarm` and
before its first function: the C 914-962 overview comment (the manual 32-thread pool, why the system
pool is not used, and the 2048-object ceiling), C 964-971's four `WFO_*` constants, C 973-978's two
`typedef`s with their two `extern` objects, C 981-1034's three records (`wait_objects_context`,
`wait_objects_pool`, `wait_objects_info`), and C 1036-1039's two statics.  Port: 1,550 -> 1,705
lines.  Every item keeps its field-by-field C comment, and the two records' arrays carry C's own
bounds (`wocs[32]`, `hs[64]`).  The next row's first item is `wait_objects_thread` (C 1051), whose
own C 1041-1050 comment goes with it.

**Evidence ① - the arm reading, and what it does *not* prove (the round's most important
finding).**  `cargo run -q -p xtask -- probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"'
--cfg 'feature="have_langinfo_codeset"'` → **exit 1, 37 errors, 0 anchored in this unit**, and HEAD's
revision (materialized as a sibling file of the same directory, same arm set) reads **the same 37**
- the diff of the two normalized outputs is **0 lines**, so this round moves the reading in neither
direction.  **But the 37 errors are 27 `E0428` + 9 `E0425` + 1 `compile_error!`: every one is a
*name-resolution* error, which means rustc never reaches type-checking in this probe.**  Measured,
three injected controls: a deliberate `E0308` in a **`pub const`** initialiser of this slice does not
appear; the same `E0308` inside a **slice-1 function body** (`alarm`) does not appear either; a
`compile_error!` under the *same* `#[cfg (feature = "windowsnt")]` **does** fire.  So at this arm set
the reading is a **parse-and-expand** reading: it proves the text is parsed, that the gate is
evaluated (the `compile_error!` firing is the proof the items are reached), and that no *resolution*
error is anchored here - it does **not** prove that any body in this file type-checks.  That is
recorded as the lane's owed item 8 below, because it is the one thing standing between this file and
the evidence the haiku lane could produce.

**Evidence ② - the frozen reading is unchanged too.**  The frozen configuration compiles none of
this file (every item is behind `windowsnt`), so `build-tu src/w32proc.c` still writes the 336-byte
P-08 object; this round did not run it (slice 1's Evidence ① is the standing reading, and nothing in
this round touches the include graph).

**Deviations (this slice's additions).**

34. **`PCWSTR` and `HRESULT` stand at the use site** (P-72 rule 4, and P-88 ①'s `UNVERIFIED` class
    for a `<windows.h>` name on this machine): `typedef const wchar_t *PCWSTR;` is `*const u16` and
    `typedef long HRESULT;` is `c_long`, both declared beside the two `typedef`s that spell them
    (C 976).  No mirror in this unit's graph carries either, which is why they are here rather than
    in `include/w32common.rs`.
35. **The two `typedef`s and their objects are *this* file's text, and the port keeps them here**
    (C 973-978), even though `src/systhread.rs` 717-730 already carries the same two aliases and
    objects for `systhread.c`'s own declaration of them (C's comment at C 973-974 says they "are
    already defined in w32fns.c").  Two units spelling the same alias is what C's own headers do
    once per translation unit; what a unit may *not* do is repeat a declaration a **mirror** in its
    graph carries, and `systhread.rs` is a unit (`tools/check-mirror-dupes.py` reads mirrors).
36. **`WFO_MAX_WAIT` is `w32_system_names::FD_SETSIZE`** - C's `#define WFO_MAX_WAIT FD_SETSIZE`
    (C 971) takes the name from `<winsock2.h>`, and `include/w32.rs` 94 carries it inside its
    `#[cfg (feature = "windowsnt")] pub mod w32_system_names`, which is not glob-imported, so the
    constant is spelled with that path (the `src/dosfns.rs` 800-817 convention for a module-scoped
    name).

**Owed (this slice's additions).**

8. **The w32 arm set's probe cannot type-check** (Evidence ①).  The 36 resolution errors are the
   mirror-header clashes slice 1's Evidence ② already named (`lib/include/signal.rs` against
   `nt/inc/include/ms-w32.rs` and the unit's MinGW-runtime section; `nt/inc/include/unistd.rs`
   against `lib/include/fcntl.in.rs`; `include/w32term.rs`'s own unspelled names; the two
   `compile_error!`-class rows).  **Until that class is discharged, no slice of this file can show
   more than a parse-and-expand reading**, and the controls in Evidence ① are how a reader can tell
   the two readings apart rather than trust the count.  This is the integrator's mirror round
   (P-87's scan follows it); the lane records it here because it is now the file's largest evidence
   gap.

## Slice 3 (2026-10-14, status 4.282): C 1041-1299

**What this round is.**  The wait-object *worker pair*, the first two definitions of the pool whose
declarations slice 2 landed: C 1041-1050's overview comment (what a worker thread does between
waits), C 1051-1117's `wait_objects_thread` (the `for (;;)` answer loop - the call counter, the
pool's timeout event placed at the end of the handle array, C 1076's `memcpy` of the caller's
handles, the `WaitForMultipleObjects` that starts the wait, the `wait_ready` handshake and C
1095-1114's `switch` over the *second* wait's result) and C 1123-1299's `start_wait_objects` (the
lazy `timeout_event` initialisation, the live-thread scan through `WaitForSingleObject (… , 0)`,
the 63-per-thread group arithmetic with C 1166-1209's forty-line sizing comment carried as C's text,
the thread-creation loop with its three `CreateEvent`s, `CreateThread`'s `64 * 1024`/`0x00010000`
and the optional `SetThreadDescription`, and C 1282-1297's wait-array fill with
`ResetEvent`/`SetEvent`).  Port: 1,705 -> **2,069 lines** (+364, of which 50 are the use-site
`<windows.h>` block of Deviations 37).  The next row's first item is C 1301's `stop_wait_objects`.

**Evidence ① - the frozen reading.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/w32proc.c` -> `built src/w32proc.rs -> ../build/femacs/w32proc.o`, **exit 0, 336 bytes** (one
shot; the P-08 empty object, because every item is behind `feature = "windowsnt"` and the frozen
configuration sets none - unchanged from slices 1/2, as expected).

**Evidence ② - the arm reading, and the gap it still cannot close.**  `cargo run -q -p xtask --
probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"' --cfg
'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg 'feature="have_langinfo_codeset"'`
-> **exit 1, 37 errors, 0 anchored in this unit** (`27 E0428 + 9 E0425 + 1 compile_error!`), i.e.
*the same 37 the file read before this round*: adding 364 lines moved the reading in neither
direction.  This is owed item 8's fact, re-measured, not a fresh reading: every error is a
*name-resolution* one, so rustc never reaches type-checking in this probe, and neither
`wait_objects_thread`'s nor `start_wait_objects`'s body is type-checked by it.  **The round's
positive reading is therefore not the probe**; it is Evidence ③.

**Evidence ③ - the name sweep, and the one real defect it found.**  `python3
tools/check-unit-names.py femacs/src/w32proc.rs 1703 2069` -> `25 capitalized identifiers, missing:
['Some']`, where `Some` is the prelude variant the tool's docstring records as a false positive.
**The first reading of this slice instead reported `WAIT_ABANDONED_0`**, and that was a genuine
hole: C 1108-1109 names it (the `case WAIT_ABANDONED_0:` / `+ 1` pair) and the name exists nowhere
in the tree, while its three siblings (`WAIT_OBJECT_0`, `WAIT_TIMEOUT`, `WAIT_FAILED`) exist only in
*other units* (`src/w32.rs` 13252-13254, `src/systhread.rs` 509, `lib-src/ntlib.rs` 180-184) that
this unit's include graph does not carry - so all four, plus `INFINITE`, plus the six entry points
this slice calls, were unresolvable names in this unit.  **Neither the probe (Evidence ②) nor the
token check can see that**: the probe cannot report a name, and the check compares token *sets*.  The
sweep plus a hand reading of C 1041-1299's text are the two readings that can, which is why the
round ran the sweep before, not after, deciding the slice was done.  The fix is Deviations 37.

**Evidence ④ - the `--check` reading.**  `python3 tools/extract-abi.py --check emacs/src/w32proc.c
femacs/src/w32proc.rs` -> **100 `- missing` rows, down from 102** (`missing function
wait_objects_thread` and `missing function start_wait_objects` retire; nothing else changes), plus
two problem rows this round owns: `- function wait_objects_thread does not mention break, case,
switch` (new; the macro-body-notes row of Deviations 38) and `- line count drifted 57.8%: C 4903 vs
Rust 2069` (the file's standing row, now the measured -57.8% and appended to
`docs/ports/line-drift-tolerance.tsv` at 0.59 = measured + 0.01, the `xfns` 0.41 rule; it is a
*negative* drift the not-yet-ported remainder owns, and it moves with every slice).

**Deviations (this slice's additions).**

37. **The five wait values and the six wait-event entry points stand in a use-site `<windows.h>`/
    `<winbase.h>` block** (P-88 ①'s `UNVERIFIED` class): `WAIT_OBJECT_0` 0, `WAIT_ABANDONED_0`
    0x00000080, `WAIT_TIMEOUT` 0x00000102, `WAIT_FAILED` 0xFFFFFFFF, `INFINITE` 0xFFFFFFFF,
    `<winerror.h>`'s `S_OK` `((HRESULT) 0L)`, and `WaitForMultipleObjects`, `WaitForSingleObject`,
    `CreateEvent`, `SetEvent`, `ResetEvent`, `SetLastError`.  None of them is in a mirror of this
    unit's graph (`include/w32.rs`, `include/w32term.rs`, `include/w32common.rs`, `nt/inc/*` all
    read for them), and the unit's own top `<windows.h>` section is *slice 1's* surface, so they
    stand at the use site exactly as `PCWSTR`/`HRESULT` do (Deviations 34).  C's `…A`/`…W` alias
    pair is spelled as C spells it.  Every one is **UNVERIFIED** - no Windows SDK on this machine.
38. **C's `switch` in `wait_objects_thread` becomes an `if`/`else if` chain, not a `match`**
    (PORTING.md 2.2).  C's `case WAIT_OBJECT_0: break;` continues the `for (;;)` loop, so it is the
    port's empty `{ }` arm; the `case WAIT_OBJECT_0 + 1:` arm `return`s *out* of the loop, and the
    four fallen-through labels (`WAIT_ABANDONED_0`, `WAIT_ABANDONED_0 + 1`, `WAIT_TIMEOUT`,
    `WAIT_FAILED`) are one `||` chain - a `match` would have to reach every one of those exits from
    inside its own scrutinee expression.  Neither the `switch` keyword nor the `case` labels
    survive, which is the `break, case, switch` token row this round appends to
    `docs/ports/macro-body-notes.tsv`.
39. **C's implicit conversions the port spells.**  `static int` returning `WFO_FAILED` (a
    `0xfffff` `DWORD` macro) is `WFO_FAILED as c_int`; C's `ctx->nCount + 1` handed to a `DWORD`
    parameter is `((*ctx).nCount + 1) as DWORD`; C's `L"sys_select_worker_thread"` is the
    `[u16; 26]` array plus `.as_ptr ()` (the `src/w32fns.rs` 423 `DARK_MODE_APP_NAME` device - C's
    `L"…"` has no Rust syntax); and the two "is this function pointer set" tests C writes as
    `if (is_debugger_present && …)` are `.is_some ()` / `.unwrap ()` on the `Option<fn>` spellings
    of `IsDebuggerPresent_Proc`/`SetThreadDescription_Proc` (C 975-976, slice 2's declarations).

**Owed (this slice's additions).**

9. **`S_OK` is a second `<winerror.h>` name this unit cannot check** (Deviations 37): its value is
   `((HRESULT) 0L)` on a 32-bit `HRESULT`, and the `HRESULT` the slice-2 block already declares is
   `c_long` - correct on this target and on MinGW, but measured on neither (no mingw toolchain, no
   `<winerror.h>`).  It joins the layout assertions owed for every SDK name in this file.
10. **The two bodies this round adds are outside every type reading** (owed item 8, re-measured as
   Evidence ②): `wait_objects_thread` and `start_wait_objects` are parsed, gated and swept for
   names, and their token sets are compared - and that is all the evidence this machine can produce
   for them until the mirror round discharges the `E0428` class.

## Slice 4 (2026-10-14, status 4.283): C 1301-1511

**What this round is.**  The pool's four control functions, i.e. everything between
`start_wait_objects` and `wait_for_objects`: C 1301-1347's `stop_wait_objects` (the
`timeout_event` signal, the `WaitForMultipleObjects (p->nt, p->hs, TRUE, 20)` that proves every
worker reached `wait_ready`, the timeout branch with its dead-thread scan, the `errcode` scan and
the `ResetEvent`), C 1349-1437's `end_wait_and_return` (the main thread's return value translated
into one of the three `WFO_*` answers, with C 1365-1385's abandoned arm and C 1388-1432's
main-thread/worker-thread index arithmetic), C 1439-1483's `shrink_wait_pool` (the every-64th-call
check, the last worker thread's retirement and the `call_count` reset) and C 1485-1511's
`free_wait_pool` (the `init_flag` guard, the quit-and-close loop, the exit wait and the
`timeout_event` close).  Port: 2,069 -> **2,335 lines** (+266, of which 8 are the
`WAIT_ABANDONED` block of Deviations 40).  The next row's first item is C 1513's `wait_for_objects`.

**Evidence ① - the frozen reading.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/w32proc.c` -> `built src/w32proc.rs -> ../build/femacs/w32proc.o`, **exit 0, 336 bytes** (one
shot, the same P-08 object as slices 1-3).

**Evidence ② - the arm reading (unchanged, and it is still not a type reading).**
`cargo run -q -p xtask -- probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the same figure slices 2 and 3 recorded, so these
four bodies are outside every type reading too (owed items 8/10, unchanged).

**Evidence ③ - the name sweep is clean this round, and that is the reading that matters here.**
`python3 tools/check-unit-names.py femacs/src/w32proc.rs 2070 2335` -> `17 capitalized
identifiers, missing: []` - the first *clean* sweep in this lane (slice 3's found
`WAIT_ABANDONED_0`; slices 1/2's found the pairs they recorded).  The round's own new SDK name,
`WAIT_ABANDONED` (C 1314), was added **before** the sweep, which is the order slice 3's defect
taught this lane: the probe cannot see a name, the token check compares token sets, and only the
sweep plus a hand reading of the C range can.

**Evidence ④ - the `--check` reading.**  96 `- missing` rows, down from 100: `missing function
stop_wait_objects`, `missing function end_wait_and_return`, `missing function shrink_wait_pool` and
`missing function free_wait_pool` all retire.  **No new token rows**: C's four `for` loops either
become `while` loops whose bodies keep C's `break`, or are the `for (;;)`-shaped loops the token
set already matches - so unlike slice 3 this round owes no `macro-body-notes.tsv` row.  The drift
figure moves the other way for the first time (57.8% -> **52.4%**, the port catching up), and the
stem's tolerance row is re-appended at 0.54 = measured + 0.01 rather than left with a stale number.

**Deviations (this slice's additions).**

40. **`WAIT_ABANDONED` joins the use-site `<windows.h>` block** (P-88 ①): `<winbase.h>` spells the
    base value (`#define WAIT_ABANDONED 0x00000080`) as well as the `WAIT_ABANDONED_0` slice 3
    declared, and C 1314 compares a `WaitForMultipleObjects` result against the base one.  No
    mirror in this unit's graph carries it; the value is **UNVERIFIED** like its siblings.
41. **`free_wait_pool` is defined here and declared by the mirror** (`include/w32.rs` 578, inside
    `pub mod w32proc_fn_decls`) - the first name of that shape in this file.  It is not an `E0428`
    collision: the mirror's declaration is a *module member* that this unit neither glob-imports nor
    repeats (PORTING.md 0.1-3), which is the reading `tools/check-mirror-dupes.py` verifies for
    mirrors and `stop_timer_thread`'s class does for a plain unit-level name.  C's own text is the
    definition (C 1485-1511) and its caller is `src/w32.c` 10559, whose port calls the bare name.

**Owed (this slice's additions).**

11. **The four bodies are parsed, gated, swept and token-compared - and not type-checked** (owed
    items 8/10, re-measured as Evidence ②): the mirror round that discharges the `E0428` class is
    still what this file waits on, and this round adds four more bodies to that queue.

## Slice 5 (2026-10-14, status 4.284): C 1513-1693

**What this round is.**  The file's two wait entry points, i.e. the pair every caller in the tree
reaches `w32proc.c` through: C 1513-1597's `wait_for_objects` (C's replacement for
`WaitForMultipleObjects` with `bWaitAll` removed - the `shrink_wait_pool` guard, the `nCount <= 64`
fast path, the busy-wait loop that walks 64-sized groups when `dwMilliseconds` is 0, and the
multithreaded path that hands the wait to `start_wait_objects` and translates the answer through
`end_wait_and_return`, with C 1585-1590's sixty-four-hertz comment) and C 1599-1693's
`msg_wait_for_objects` (the same shape over `MsgWaitForMultipleObjects`, where C waits on groups of
63, uses `<=` at C 1635-1636 because the call can also report `nCount` itself, and answers `nCount`
- not the group offset - when a message arrived, C 1664-1666's own note).  Port: 2,335 -> **2,567
lines** (+232, of which 16 are the `MsgWaitForMultipleObjects`/`QS_ALLINPUT` block of Deviations
42).  The next row's first item is C 1695-1727's subprocess overview comment.

**Evidence ① - the frozen reading.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/w32proc.c` -> exit 0, **336 bytes** (one shot, the same P-08 object as slices 1-4).

**Evidence ② - the arm reading (unchanged for the fifth time).**  `probe src/w32proc.rs <out>
--crate-type=lib -C panic=abort --cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg
'feature="have_setitimer"' --cfg 'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27
`E0428` + 9 `E0425` + 1 `compile_error!`), 0 anchored in this unit**.  These two bodies are outside
every type reading as well (owed items 8/10/11).

**Evidence ③ - the name sweep is clean again.**  `python3 tools/check-unit-names.py
femacs/src/w32proc.rs 2337 2567` -> `13 capitalized identifiers, missing: []`.  The round's own two
new SDK names (`MsgWaitForMultipleObjects`, `QS_ALLINPUT`) went into the use-site block before the
sweep, the order slice 3's defect established.

**Evidence ④ - the `--check` reading.**  **96 -> 94 `- missing` rows**: `missing function
wait_for_objects` and `missing function msg_wait_for_objects` retire.  No new token rows (C's two
`for` loops keep their `break`/`continue` tokens in the port's `while` forms, and C's
`int`/`DWORD` mix is a spelling the token families already fold), and the drift falls again
(52.4% -> **47.6%**), so the stem's tolerance row is re-appended at 0.49 = measured + 0.01.

**Deviations (this slice's additions).**

42. **`MsgWaitForMultipleObjects` and `QS_ALLINPUT` stand in a use-site `<windows.h>`/`<winuser.h>`
    block** (P-88 ①): `<windows.h>` pulls `<winuser.h>`, which is where both names live
    (`#define QS_ALLINPUT 0x04FF` is the OR of the five input-queue classes plus
    `QS_SENDMESSAGE`).  No mirror in this unit's graph carries either; both are **UNVERIFIED**.
43. **C's two counted `for` loops become `while` loops, and the `continue` path carries C's
    increment clause.**  C 1557/1650 spell
    `for (int i = 0, offset = 0; i <= group; i++, offset += 64)`, so C's `continue` (C 1567/1660)
    still runs `i++, offset += 64`; the port's `{ i += 1; offset += 64; continue; }` is that clause
    written at the `continue`, and the loop's own tail carries the same pair.  The busy-wait
    arithmetic itself (`rest`/`group`, `count`, `offset + res - WAIT_OBJECT_0`) is unchanged, with
    C's `int`/`DWORD` conversions spelled as `as` (Deviations 39).

**Owed (this slice's additions).**

12. **The two entry points are the file's most-called names and neither is type-checked here**
    (owed items 8/10/11, re-measured as Evidence ②): `sys_select` (C 3109, still to land) is their
    principal caller, so the mirror round that discharges the `E0428` class is what this file - and
    that caller - wait on.

## Slice 6 (2026-10-14, status 4.285): C 1695-1832

**What this round is.**  The child-process block's *opening* - the declarations every function after
it reads, and the file's longest banner comment.  C 1695-1823 is the overview of how subprocesses
and network/serial streams are supported here (the `child_procs[]` array and its `MAX_CHILDREN`
(= 32) bound, the `reader_thread`/`sys_select` handshake through `char_avail`/`char_consumed`, why
`WaitForMultipleObjects`'s 64-handle ceiling becomes Emacs's 32-child limit, the `fd_info[]`
conversion and the `_get_osfhandle`/`socket_to_fd` pair, and the `sys_kill` emulation with its
`Ctrl-C`/`Ctrl-BREAK`-then-`TerminateProcess` order); C 1825-1826 is `_P_NOWAIT`; C 1828-1830 is the
management list (`child_proc_count` and `child_procs[ MAX_CHILDREN ]`); C 1832 is the
`reader_thread` forward declaration.  Port: 2,567 -> **2,734 lines** (+167, of which 129 are the
carried banner).  The next row's first item is C 1834's `new_child`, whose own
"Find an unused process slot." comment goes with it.

**Evidence ① - the frozen reading.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/w32proc.c` -> exit 0, **336 bytes** (the same P-08 object as slices 1-5).

**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the sixth identical reading (owed items 8/10/11/12).

**Evidence ③ - the name sweep, and the trap this round's own prose hit.**  `python3
tools/check-unit-names.py femacs/src/w32proc.rs 2568 2734` first read `missing: ['PORTING', 'Rust']`
- **not** missing names but the tool's documented *nested-comment* false positive, produced by this
round's own prose: two of the new comments quoted C's one-line comments **with their `*/`**, so the
scanner's non-nesting comment rule ended the comment early and read the rest as code.  Rewriting
those two quotes as plain quoted text (no `*/`) makes the sweep read `1 capitalized identifier,
missing: []`.  **The lesson is worth the row: in this file the *ledger prose* can manufacture a
false positive, and the sweep is only trustworthy once the round's own comments are checked for an
embedded terminator.**
 
**Evidence ④ - the `--check` reading.**  **94 -> 90 `- missing` rows**, and the four that retire are
exactly this round's items: `missing macro _P_NOWAIT`, `missing constant _P_NOWAIT` (the `#define`
is both a macro row and a constant row for the check) and the two `missing file-scope object`
rows (`child_proc_count`, `child_procs`).  No new token rows; drift falls again (47.6% -> **44.2%**),
so the stem's tolerance row is re-appended at 0.45 = measured + 0.01.

**Deviations (this slice's additions).**

44. **The banner comment is carried as C's own text, verbatim, at its C position** - the same
    reading slices 1/2 give C 914-962, and the reason this ledger's range table does not split it:
    it is 129 lines of C prose that documents the block below it.  What the port does *not* do is
    copy it into the file's own header; the header keeps the pointer it already has.
45. **`child_process` and `MAX_CHILDREN` are the mirror's names, so the unit does not re-spell
    them** (PORTING.md 0.1-3): `include/w32.rs` 326-355 carries `_child_process`/`child_process`
    (C `src/w32.h` 84-110) and 242 carries `MAX_CHILDREN` (C's `MAXDESC/2`), both at that mirror's
    crate root and both reachable from this unit through its `include!` of `../include/w32.rs`.
    The two objects are statics C leaves zero-initialised, spelled `core::mem::zeroed ()` like the
    `wait_pool`/`wait_info` pair of slice 2.
46. **`_P_NOWAIT` is this unit's own constant** (C 1826).  `src/callproc.rs` 339 spells the same
    name for `callproc.c`'s `#ifdef WINDOWSNT` arm; two *units* spelling one constant is what C's
    own headers do once per translation unit (Deviations 35's rule), and this unit may not take it
    from another unit.

**Owed (this slice's additions).**

13. **The carried banner names the product** (C 1695-1823's prose says what a subprocess of this
    program is, in C's words): it is C's text at C's position, which is what P-86 ① asks for, and it
    is **not** one of the removed product-name declarations (those are the file's C 1-18 block that
    PORTING.md 0.1-19 drops).  `tools/check-declarations.py` reads this file and reports no
    forbidden declaration, which is the reading that keeps the two classes apart.

## Slice 7 (2026-10-14, status 4.286): C 1834-2115

**What this round is.**  The child-process block's five working functions - the ones the rest of the
file (and `src/w32.c`) reaches every subprocess through: C 1834-1933's `new_child` (the descending
scan for a free slot, the dead-process sweep with its three `DebPrint`s, and the
`Initialize` tail that zeroes the slot and starts the reader thread), C 1935-1998's `delete_child`
(the `fd_info` guard, the thread reap with its `#if 0` region, and the `child_proc_count` rewind),
C 2000-2011's `find_child_pid`, C 2013-2024's `release_listen_threads` and C 2026-2115's
`reader_thread` (the descriptor dispatch on `FILE_CONNECT`/`FILE_LISTEN`, the `char_avail`/
`char_consumed` handshake, and the post-loop close of a `FILE_DONT_CLOSE` descriptor).  Port: 2,734 ->
**3,193 lines** (+459, of which 74 are the unit's own `child_process` spelling of Deviations 47 and
36 the use-site SDK block).  The next row's first item is C 2117's `process_dir` comment.

**Evidence ① - the frozen reading.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/w32proc.c` -> exit 0, **336 bytes** (the same P-08 object as slices 1-6).

**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the seventh identical reading, so these five bodies
join the queue owed items 8/10/11/12/13 name (the `compile_error!` is the mirror graph's measured
`HAVE_NTGUI` `struct frame` gap, unchanged).

**Evidence ③ - the name sweep, and the two false-positive classes it read.**  `python3
tools/check-unit-names.py femacs/src/w32proc.rs 2735 3193` -> `41 capitalized identifiers, missing:
['Internal', 'InternalHigh', 'Offset', 'OffsetHigh', 'Pointer', 'Some']` - **both classes are the
tool's own documented limitations, not missing names**: the five field names are the *field*
declarations of the SDK records this round spells (`pub Internal: ULONG_PTR,` - the tool strips
`.field` accesses but not field definitions), and `Some` is the prelude variant.  Both were
confirmed by a control run on the peer unit that spelled the same records: `check-unit-names.py
femacs/src/w32.rs 11860 11920` reads the same five field names.  Nothing this round uses is
undefined.

**Evidence ④ - the `--check` reading.**  **90 -> 85 `- missing` rows**, and the five that retire are
exactly this round's definitions (`missing function new_child`, `delete_child`, `find_child_pid`,
`release_listen_threads`, `reader_thread`).  One **new token row**: `- function new_child does not
mention goto` - C's two `goto Initialize` jumps, now recorded in `docs/ports/macro-body-notes.tsv`
(Deviations 48).  Drift falls again (44.2% -> **34.9%**), so the stem's tolerance row is re-appended
at 0.36 = measured + 0.01.

**Deviations (this slice's additions).**

47. **The unit spells `w32.h` 74-113's `child_process` record and the three SDK records it embeds**
    (P-88 ①'s `UNVERIFIED` class): `PROCESS_INFORMATION`, `OVERLAPPED` (with the two named inner
    records of the `P-67` device) and `ULONG_PTR`.  The `include/w32.rs` mirror **defers** the
    record (its note 34-42) and keeps the pointers to it at C's pointer depth as `*mut c_void`,
    leaving its checklist box unchecked; this unit reads the record's *members*
    (`procinfo.hProcess`/`hThread`/`dwProcessId`), which a deferred `*mut c_void` cannot express,
    so it spells the record exactly as `src/w32.rs` 11847-11916 did when it became the record's
    first reader - same gate (`windowsnt`; the mirror's own copy is the dead `feature = "windows"`
    arm's text, so the two never coexist in one crate), C's member names/order/types kept and C's
    per-member prose not carried.  **This is the third spelling of the class in the tree** (mirror's
    deferred note, `src/w32.rs`, here), which is the integrator's mirror-round material: the record
    belongs in the mirror once a Windows measurement exists.
48. **C's two `goto Initialize` jumps become `break 'initialize` out of a labeled block**
    (PORTING.md 2.2).  The target is *forward*, into the tail both loops fall through to, so neither
    a `loop` nor a `match` can express it; the `doprnt` tail-flag is the same class.  The
    `Initialize:` label itself (C 1890) is carried as a comment at its C position, and the
    `goto`/`Initialize` tokens are the `macro-body-notes.tsv` row this round appends.
49. **`fd_info[i].cp == cp` is spelled `fd_info[i as usize].cp == cp as *mut c_void`** - the
    mirror's `filedesc.cp` is the deferred `*mut c_void` (its note 34-42 names this very member),
    and `src/w32.rs` 12615/12794 cast the other way (`... as *mut child_process`) at its own use
    sites.  The comparison is pointer-for-pointer as C's is; the two spellings are one cast apart.

**Owed (this slice's additions).**

14. **`GetExitCodeProcess` and `_close` are the two SDK names this round declares at the use site**
    (Deviations 47's block): `<winbase.h>`'s exit-code reader and the CRT's underscored `close`
    (`<io.h>`, a header with no file in the reference tree - the unit's own C 35 note records why
    `lib/include/unistd.in.rs` is not pulled in).  Both are **UNVERIFIED** here, like every Windows
    name in this file, and join the layout/behaviour assertions the mirror round owes.

## Slice 8 (2026-10-14, status 4.287): C 2117-2121

**What this round is.**  `process_dir`, the one file-scope object between the child-process block's
working functions and `create_child`: C 2117-2120's comment (why the directory is recorded here
rather than changed, and that its name is UTF-8) carried as C's own text, and C 2121's
`static char * process_dir;`.  Port: 3,193 -> **3,209 lines** (+16).  It is the reader
`create_child` (C 2165) and the writer `set_process_dir` (C 3903) that make it worth landing ahead
of them: this round removes the `missing file-scope object process_dir` row so the next slice's
`filename_to_ansi (process_dir, dir)` has a name to resolve.  The next row's first item is C 2123's
`create_child`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.

**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors, 0 anchored in this unit** - the eighth
identical reading.

**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 3194 3209` -> `0
capitalized identifiers, missing: []` (the round adds no name at all, which is the point).

**Evidence ④ - the `--check` reading.**  **85 -> 84 `- missing` rows**: `missing file-scope object
process_dir` retires.  No new token rows; drift 34.9% -> **34.6%**, inside the stem's 0.36 row
(measured + 0.01 at slice 7), so this round appends no tolerance row - the row's number is still the
measured one, not a stale one.

**Owed (this slice's additions).**

15. **`create_child` (C 2123-2205) is the next slice and the file's largest SDK surface so far**:
    its body needs `<windows.h>`'s `STARTUPINFO` (eighteen members) and `SECURITY_ATTRIBUTES`, the
    six create-process constants (`STARTF_USESTDHANDLES`, `STARTF_USESHOWWINDOW`, `SW_HIDE`,
    `CREATE_NEW_PROCESS_GROUP`, `CREATE_NEW_CONSOLE`, `CREATE_DEFAULT_ERROR_MODE`), the three
    `STD_*_HANDLE` values, `MAX_PATH`, and the entry points `CreateProcessA`, `GetStdHandle` and
    `CharNextA` - plus `xstrcasecmp` (the mirror carries it in `include/dispextern.rs`, which this
    unit does **not** include) and the three `Vw32_start_process_*` globals (who *declares* them for
    a reader in this unit, given `include/globals.rs` is not in its graph, is the question this
    round's name check must answer before the body is written).  Every one is the `P-88` ①
    `UNVERIFIED` class on this machine.

## Slice 9 (2026-10-14, status 4.288): C 2123-2205

**What this round is.**  `create_child` - the one place this file starts a process, and the round
that had to answer the two scope questions slice 8 left open: `xstrcasecmp` (the mirror carries it
in `include/dispextern.rs`, which this unit's graph does not include - this file names
`dispextern.h` nowhere) and the three `Vw32_start_process_*` globals.  Port: 3,209 -> **3,420
lines** (+211, of which 108 are the SDK block of Deviations 50).  The next row's first item is
C 2207's `register_child` comment.

**The two questions, answered from the tree (this round's first readings).**

  * **The three globals need no declaration here.**  `include/globals.rs` declares them
    (`Vw32_start_process_show_window` at its 6838, inside `pub mod variables`, glob-imported at its
    7208) and `include/lisp.rs` 690 includes that mirror - which this unit already includes at its
    C 57 line - so C 2143/2183/2186's three names resolve exactly as `src/w32.rs` 5919's
    `Vw32_downcase_file_names` does.  The same unit *defines* them at C 4809-4826, so the reader and
    the writer are one file, as in C.
  * **`xstrcasecmp` is declared at the use site with C's own citation** (`dispextern.h` 104), the
    device `src/xfns.rs` 10563 and `src/fontset.rs` 108 already use for units whose graph has no
    `dispextern.rs`; the definition is `src/font.rs` 298's / `include/dispextern.rs` 2648's.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.

**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the ninth identical reading, and the first one that
could have moved: the round adds 108 lines of SDK declarations, and a wrong name there would have
shown as an `E0428` (the class the probe *does* report).

**Evidence ③ - the name sweep, and the `#if 0` text it reads.**  `check-unit-names.py
femacs/src/w32proc.rs 3210 3420` -> `34 capitalized identifiers, missing: ['EH_Fail',
'InitializeSecurityDescriptor', 'SECURITY_DESCRIPTOR_REVISION', 'SetSecurityDescriptorDacl']` - the
four names of the C 2154-2160 `#if 0` region, i.e. the tool's documented **nested-comment** false
positive: that region carries an inner `/* … */`, so the scanner ends the comment early and reads
its tail (`goto EH_Fail`, the two `SECURITY_DESCRIPTOR` entries) as code.  The control is inside the
same round: C 2129-2131's region has **no** inner comment and its `SECURITY_DESCRIPTOR` is *not*
reported.  Both are carried as C's own text (P-51, the file's reading for a region no configuration
compiles).

**Evidence ④ - the `--check` reading.**  **84 -> 83 `- missing` rows** (`missing function
create_child` retires), plus one new token row - `function create_child does not mention goto`,
recorded in `docs/ports/macro-body-notes.tsv`.  Drift 34.6% -> **30.2%**, inside the stem's 0.36
row (the row is still the measured one, +0.01 at slice 7).

**Deviations (this slice's additions).**

50. **The round's use-site SDK block** (P-88 ①): `STARTUPINFO` (eighteen members, C 2127's record)
    and `SECURITY_ATTRIBUTES` (C 2128's), `MAX_PATH` (C 2133), `STARTF_USESTDHANDLES`,
    `STARTF_USESHOWWINDOW`, `SW_HIDE`, the three `STD_*_HANDLE` values, `CREATE_NEW_CONSOLE`,
    `CREATE_NEW_PROCESS_GROUP`, `CREATE_DEFAULT_ERROR_MODE`, and `CreateProcessA`, `GetStdHandle`,
    `CharNextA`.  No mirror in this unit's graph carries any of them; every value is
    **UNVERIFIED** (no Windows SDK on this machine), and `CreateProcessA`'s two
    `LPSECURITY_ATTRIBUTES` parameters are spelled `*mut SECURITY_ATTRIBUTES` because this round
    spells the record (the top block's `LPSECURITY_ATTRIBUTES` is the deferred `*mut c_void` and
    stays for the callers that pass one through).
51. **`xstrcasecmp` stands at the use site** (the round's second question): C's `dispextern.h` 104
    inline, declared `unsafe extern "C"` with that citation, exactly as `src/xfns.rs` 10563 and
    `src/fontset.rs` 108 declare it for units whose include graph has no `dispextern.rs`.  This
    file's C text names no `dispextern.h`, so nothing is included for it.
52. **C 2203's `DebPrint` line carries a stray `;`** (`DebPrint ((...););`) that the port's macro
    call cannot spell; the port writes the tuple C's own parentheses hold and the line's citation
    records the difference.  C's own compilations accept it, so this is a transcription note, not a
    behaviour change.

**Owed (this slice's additions).**

16. **`STARTUPINFO`'s 104 bytes and `SECURITY_ATTRIBUTES`' three members are unmeasured** (the
    class P-88 ① names): the record this round spells is read by C's `start.cb` and passed to
    `CreateProcessA`, so a Windows/mingw clang probe is what closes it - the same measurement the
    mirror's deferred `child_process` note (its 34-42) and Deviations 47 already owe.

## Slice 10 (2026-10-14, status 4.289): C 2207-2268

**What this round is.**  The pair that ties a started child to a descriptor and reaps it:
`register_child` (C 2207-2242: `find_child_pid` with C's `(DWORD)` cast, the not-found exit, the
dead `FULL_DEBUG` arm, the descriptor and `STATUS_READ_ACKNOWLEDGED`, and the `fd_info` in-use
guard) and `reap_subprocess` (C 2244-2268: the process/thread handle pair, its own dead
`FULL_DEBUG` arm, and the `delete_child` for a descriptor already closed).  Port: 3,420 -> **3503
lines** (+83).  The next row's first item is C 2270-2273's `waitpid` comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the tenth identical reading.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 3421 3503` -> `6
capitalized identifiers, missing: []` - clean, including the two carried `FULL_DEBUG` regions (the
C 2251-2255 one does hold an inner `/* … */`, but every name its tail carries - `DebPrint`,
`WaitForSingleObject`, `WAIT_OBJECT_0` - is an item the tree already has, so the documented
false-positive class does not fire this time).
**Evidence ④ - the `--check` reading.**  **83 -> 81 `- missing` rows**: `missing function
register_child` and `missing function reap_subprocess` retire.  No new token rows; drift 30.2% ->
**28.6%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

53. **The two `FULL_DEBUG` arms are carried as C's own text** (P-51, the file's reading for a region
    no configuration compiles): C 2224-2226 and C 2251-2255.  Unlike the C 2154-2160 region of slice
    9, neither hides a name this unit needs - the guard's `WaitForSingleObject`/`WAIT_OBJECT_0`/
    `DebPrint` are all live items elsewhere - so the carried text stays exactly C's and the sweep
    reads no invented name from it.

## Slice 11 (2026-10-14, status 4.290): C 2270-2449

**What this round is.**  `waitpid` - the file's longest single function, and the caller
`reap_subprocess` (slice 10) was landed for: C's Posix comment, the `pid > 0` single-child arm
(descending scan, `FILE_AT_EOF` page, the two failures), the any-child arm, the two timeouts, the
`do`/`while` over `wait_for_objects` with `maybe_quit`, the five-way answer, `GetExitCodeProcess`,
the `STILL_ACTIVE` punt and the exit-code massage, ending at the reap and return.  Port: 3,503 ->
**3729 lines** (+226).  The next row's first item is C 2451's `open_input_file`.

**The scope questions, answered from the tree before writing.**  (a) **`SIGINT` is in the graph**:
`include/syssignal.rs` 18 includes `lib/include/signal.rs`, and this unit includes
`include/syssignal.rs` at its C 62 line - so C 2432's value needs no declaration, even though the
name appears nowhere in the files the unit names directly.  (b) **`WNOHANG` has to be declared
here**: C reaches it through C 61's `"syswait.h"`, whose mirror records the dead `#ifndef`
fallbacks instead of the system header, and `nt/inc/sys/include/wait.rs` - the mirror that does
carry it - is included by no unit at all (`docs/ports/include-wait.md` 23-27).  Including that
mirror here would be an `E0428` against this unit's own `waitpid` definition (its only other item
is that prototype), so the constant stands at the use site (Deviations 54), which is the file's
usual device for a name whose header mirror is not in the graph.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  The first
`build-tu` run **failed** (two `#ifdef FULL_DEBUG` regions were pasted bare instead of wrapped in
their `/* C … */` carry); the fix is the only text change between that run and this one.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the eleventh identical reading.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 3504 3729` -> `17
capitalized identifiers, missing: []` - clean, including both carried `FULL_DEBUG` regions (neither
holds an inner `/* … */`).
**Evidence ④ - the `--check` and do/while readings.**  **81 -> 80 `- missing` rows**
(`missing function waitpid` retires), no new token rows; drift 28.6% -> **23.9%**, inside the stem's
0.36 row.  `python3 tools/check-do-while.py` -> `read 152 unit(s): 221 C do/while condition(s), 73
matched by a port if ... { break; }, 2 reviewed, 0 failure(s)` - the round's own `do`/`while` is one
of the 73 matched pairs.

**Deviations (this slice's additions).**

54. **`WNOHANG` stands at the use site** (the round's second question): `<sys/wait.h>`'s
    `#define WNOHANG 1`, declared beside the function that reads it with C's text, because the
    mirror that carries it (`nt/inc/sys/include/wait.rs`, 11) is included by no unit and its only
    other item - the `waitpid` prototype - is the definition this unit supplies.  This is the
    `_P_NOWAIT`/`xstrcasecmp`/`ULONG_PTR` device, and it is what keeps the two declarations apart.
55. **`STATUS_CONTROL_C_EXIT` stands at the use site too**: `<winbase.h>`/`<ntstatus.h>`'s
    `0xC000013A`, the exit code C 2431 compares; `nt/cmdproxy.rs` 118-120 spells the same value for
    cmdproxy.c and calls its own copy a placeholder, so the two units each carry the value C's
    headers give them (Deviations 46's rule for `_P_NOWAIT`).  **UNVERIFIED** on this machine.
56. **C's `retval <<= 8` and the two `as` conversions it needs are the port's own shift and casts**:
    C 2432's `retval = SIGINT` is `SIGINT as DWORD` (the constant is C's `int`) and C 2445's
    `*status = retval` is `retval as c_int` (C's implicit narrowing), the class Deviations 39
    records for this file.

## Slice 12 (2026-10-14, status 4.291): C 2451-2528

**What this round is.**  The memory-mapped-file helpers and the PE section lookup that
`w32_executable_type` (C 2533, the next slice) is built on: `open_input_file` (the `CreateFileA`/
`GetFileSize`/`CreateFileMapping`/`MapViewOfFile` chain and the five `file_data` writes),
`rva_to_section` (C's linker comment, the `IMAGE_FIRST_SECTION` walk and the
`max (SizeOfRawData, Misc.VirtualSize)` test) and `close_file_data` (the unmap and the
`SetFilePointer`/`SetEndOfFile` pair), plus C 2523-2528's `#ifndef` fallback.  Port: 3,729 ->
**4058 lines** (+329, of which 213 are the round's `<windows.h>`/`<winnt.h>` block).  The next row's
first item is C 2530-2532's `w32_executable_type` comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes** (each of the
three runs this round, including the two after the mirror and the constant were fixed).

**Evidence ② - the arm reading, and the mirror defect this round found and fixed.**  The first
`probe` read **40 errors (30 `E0428` + 9 `E0425` + 1 `compile_error!`)** - three more than this
lane's baseline of 37.  All three were anchored in the **mirror** (`include/w32heap.rs` 112, 114,
132: `open_input_file`, `close_file_data`, `rva_to_section` declared at that mirror's crate root
while this unit defines them), which is exactly the class P-39's device exists for and which the
2026-10-05 section of `docs/ports/include-w32heap.md` already fixed once for five other names.  This
round moved the four into a glob-imported `w32heap_file_decls` (that mirror's ledger records the
measurement) and re-read **37 (27 + 9 + 1), 0 anchored in this unit** - the lane's baseline.  **The
device is therefore verified from both sides: a top-level declaration is an `E0428` against the
unit's definition, a glob-imported one is shadowed by it.**  Note the probe *did* report these (it
is a name-resolution reading for `E0428`) even though it cannot report a *missing* name.

**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 3726 4050` -> `98
capitalized identifiers, missing: [54 names]` - **every one of the 54 is a *field* of the PE records
this round spells** (`Machine`, `NumberOfSections`, `AddressOfEntryPoint`, `Misc`, `Name`, …), i.e.
the class the tool's own docstring records (it strips `.field` accesses but not field
*definitions*), and `Some` does not appear this time because the slice has no `Option` use.  The
control is the peer unit's identical spelling (slice 7: `check-unit-names.py femacs/src/w32.rs
11860 11920` reads the five field names of `child_process` and `OVERLAPPED`).

**Evidence ④ - the `--check` reading.**  **80 -> 75 `- missing` rows** (`missing function
open_input_file`, `rva_to_section` and `close_file_data`, plus the two macro rows below).  Two token
rows appeared and were retired in the round: `macro IMAGE_NT_OPTIONAL_HDR32_MAGIC does not mention
image_nt_optional_hdr_magic` (fixed by declaring `<winnt.h>`'s generic `IMAGE_NT_OPTIONAL_HDR_MAGIC`
and spelling the 32-bit name as its alias - more faithful *and* token-clean) and `missing macro
IMAGE_OPTIONAL_HEADER32` (a one-row `docs/ports/excluded-macros.txt` entry: C's fallback renames a
*type*, and the port carries that type under C's 32-bit name).  Drift 23.9% -> **17.4%**, inside the
stem's 0.36 row.

**Deviations (this slice's additions).**

57. **The round's use-site `<windows.h>`/`<winbase.h>`/`<winnt.h>` block** (P-88 ①): the seven file
    API entry points and their eight constants, and the four PE records with C's `Misc` union named
    after its container (`IMAGE_FILE_HEADER`, `IMAGE_DATA_DIRECTORY`,
    `IMAGE_OPTIONAL_HEADER32`, `IMAGE_NT_HEADERS`, `IMAGE_SECTION_HEADER`,
    `IMAGE_SECTION_HEADER_Misc`).  `include/w32heap.rs` 63-76 declares the two functions with the
    records as `*mut c_void` (P-72 ③), so the definitions spell what they read - the
    `child_process` device (Deviations 47) - and the mirror's prototype keeps its own pointer depth.
    `IMAGE_FIRST_SECTION` is a `macro_rules!` because C's macro is a cast plus
    `FIELD_OFFSET`/`SizeOfOptionalHeader` arithmetic (`core::mem::offset_of!`; the tree's
    `SOCK_HANDLE` device).  Every record is **UNVERIFIED** on this machine.
58. **`min!`/`max!` come from `../lib/include/min-max.rs`, included under C 57's lisp.h line**
    (Deviations 57's round): C's `max` reaches this file through lisp.h's `#include "min-max.h"`, and
    `include/lisp.rs` pulls neither that mirror nor `<string.h>` - the same reading the `<string.h>`
    line above already records.  C 2503/2504's two `rva < VirtualAddress + real_size` bounds are C's
    own `DWORD_PTR` arithmetic, and `Misc.VirtualSize` is a union field read (C's anonymous union,
    named here).
59. **The mirror `include/w32heap.rs` gained a `w32heap_file_decls` module with a glob import**
    (the P-39 device, this round's Evidence ②), because the four declarations were at that mirror's
    crate root and three of the names are this unit's definitions.  The module is deliberately
    **ungated** - the declarations were ungated before the move - and `open_output_file` moved with
    its siblings so its definer's slice does not hit the same defect.  The mirror's ledger
    (`docs/ports/include-w32heap.md`) records it with both measurements; this file records it here
    because the defect was found by *this* unit's probe.

## Slice 13 (2026-10-14, status 4.292): C 2530-2674

**What this round is.**  `w32_executable_type` - the function the file's whole PE path exists for:
the ANSI-codepage note, the four `is_*_app` defaults, the extension test that recognises DOS `.com`
and recurses on `COMSPEC` for `.bat`/`.cmd`, and C 2571-2669's header walk (the `IMAGE_DOS_HEADER`
signature check, the `e_lfanew` bound, the three "this is a DOS program" answers, the 32-bit
optional-header read, the `IMAGE_NT_OPTIONAL_HDR64_MAGIC` arm, and the import-directory walk that
looks for a `cygwin`/`msys-` DLL name).  Port: 4,058 -> **4358 lines** (+300, of which 175 are the
round's `<windows.h>`/`<winnt.h>` block).  The next row's first item is C 2676's `compare_env`.

**Two earlier deviations are retired by this round, and both are recorded where they were made.**

  * **Deviation 6 (`RVA_TO_PTR` carried as C's own text) is retired**: slice 1 could not write the
    macro's body because the port had no `IMAGE_SECTION_HEADER` (the mirror spells the record
    `*mut c_void`, P-72 ③); slice 12 spelled it, so this round writes the macro out at its C
    position (C 66-69) as `macro_rules! RVA_TO_PTR` and its two users (C 2635, C 2644) call it.
    The **`missing macro RVA_TO_PTR` row retires with it** - one of the two rows this round removes.
  * **`PIMAGE_NT_HEADERS` and C's own local types**: the `--check` first read
    `function w32_executable_type does not mention goto, image_dos_header, pimage_nt_headers`; the
    two type tokens were the port's own spellings (`PIMAGE_DOS_HEADER` for C's `IMAGE_DOS_HEADER *`,
    and a bare `*mut IMAGE_NT_HEADERS` for C's `(PIMAGE_NT_HEADERS)` cast).  The round declares the
    alias and uses C's own spellings in the two locals and the cast, so the row is now
    `does not mention goto` alone - the labeled-block deviation, recorded in
    `docs/ports/macro-body-notes.tsv`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes** (each of the
three runs).
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the lane's baseline again after slice 12's mirror fix.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 4059 4358` -> `64
capitalized identifiers, missing: [64 names]` - every one a *field* of the records this round
spells (`e_magic`, `DataDirectory`, `FirstThunk`, `Subsystem`, …), the tool's documented class
(slice 7's control: `femacs/src/w32.rs` 11860-11920 reads the same class for `child_process`).
**Evidence ④ - the `--check` reading.**  **75 -> 73 `- missing` rows**: `missing function
w32_executable_type` and `missing macro RVA_TO_PTR`.  One token row, now the `goto` alone (above).
Drift 17.4% -> **11.2%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

60. **The round's use-site `<windows.h>`/`<winbase.h>`/`<winnt.h>` block** (P-88 ①):
    `IMAGE_DOS_HEADER` (64 bytes, C's eighteen members) with `PIMAGE_DOS_HEADER`,
    `IMAGE_IMPORT_DESCRIPTOR` (its anonymous first union named after the container, P-67),
    `PIMAGE_NT_HEADERS`, the four signatures (`IMAGE_DOS_SIGNATURE`, `IMAGE_NT_SIGNATURE`,
    `IMAGE_OS2_SIGNATURE`, `IMAGE_NT_OPTIONAL_HDR64_MAGIC`), `IMAGE_SUBSYSTEM_WINDOWS_GUI`,
    `IMAGE_DIRECTORY_ENTRY_IMPORT`, the SDK's `LOWORD` as a `macro_rules!`, and the 64-bit record
    `IMAGE_OPTIONAL_HEADER64` (240 bytes).  UNVERIFIED like every SDK record in this file.
61. **C 2610's `#ifdef IMAGE_NT_OPTIONAL_HDR64_MAGIC` becomes `feature =
    "image_nt_optional_hdr64_magic"`**, i.e. the file's `__MINGW64__` device (slice 1's gate
    mapping): a guard the *header* defines is a feature named after the macro, and the arm's text is
    carried under it (P-51).  A modern `<winnt.h>` defines the macro, so the feature is what a build
    that has it sets; on the frozen configuration the arm is dead, exactly as C's is.
62. **The `RVA_TO_PTR` macro is written out** (Deviation 6 retired): `macro_rules! RVA_TO_PTR` with
    C's three operands, and the pointer arithmetic C does in `DWORD_PTR`/`file_base` is the
    `as usize` sum in that macro.  `PIMAGE_NT_HEADERS` is declared beside the other PE aliases and
    C's own type spellings are used in the two locals and the cast (the token row above).
63. **The `unwind` tail is C's `close_file_data`/return pair**, reached by `break 'unwind` from
    C 2583's `goto` - the label's own line (C 2671) stays as a comment at its C position, as
    `new_child`'s `Initialize:` does (Deviations 48).

**Owed (this slice's additions).**

17. **`IMAGE_DOS_HEADER`'s 64 bytes, `IMAGE_IMPORT_DESCRIPTOR`'s 20 and `IMAGE_OPTIONAL_HEADER64`'s
    240 are unmeasured** (P-88 ①'s class): C 2585's `e_lfanew` offset and C 2603/2614's two
    optional-header casts are reads whose *layout* this machine cannot check, so a Windows/mingw
    clang probe is what closes them - the measurement Deviations 47/57/60 all owe.

## Slice 14 (2026-10-14, status 4.293): C 2676-2720

**What this round is.**  The environment pair `sys_spawnve` (C 2725, the next slice) calls:
`compare_env`, C's `qsort` comparator (the four-part loop test, the `toupper` comparison and the
three-way answer on `=`), and `merge_and_sort_env`, which copies both environment vectors into one
and sorts the result.  Port: 4,358 -> **4448 lines** (+90).  The next row's first item is C
2722-2724's `sys_spawnve` comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 37 errors (27 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the baseline.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 4359 4448` -> `2
capitalized identifiers, missing: ['Some']` - the prelude variant alone (the tool's documented
false positive; `toupper`, `qsort` and `compare_env` all resolve).
**Evidence ④ - the `--check` reading.**  **73 -> 71 `- missing` rows**: `missing function
compare_env` and `missing function merge_and_sort_env`.  No new token rows; drift 11.2% ->
**9.3%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

64. **`qsort` stands at the use site** with C's own text and the port's `__compar_fn_t` spelling
    (`Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>`); the `stdlib.rs`
    mirror does not carry it, and `src/buffer.rs` 129-134 declares the same row the same way - so
    the file follows that precedent rather than inventing a declaration.
65. **`compare_env` is `unsafe extern "C" fn` with C's `static`** (no `no_mangle`, this file's rule
    for the C helpers) - C's callback ABI is what `qsort`'s parameter needs, and C 2689's
    `str1++, str2++` is the port's two advances after the comparison.  In `merge_and_sort_env`, C
    2709/2714's `*nptr++ = *optr++` is the same order spelled out (assign, then advance both), and
    C 2710/2715's `optr - envp1` counts are the port's `offset_from` on the same two pointers.

## Slice 15 (2026-10-14, status 4.294): C 2722-3078

**What this round is.**  `sys_spawnve` - the file's spawn entry point and, at 357 C lines, its
second-longest function: the mode guard, the executable-name check with its `openp` fallback, the
ANSI conversion and `_mbspbrk` guard, the DOS-app `CMDPROXY` branch, the two quoting passes over
`argv` (with C's commented-out `#if 0` arm carried as text) and the `envp` merge, ending at the
`new_child`/`create_child` pair.  Port: 4,448 -> **4877 lines** (+429).  The next row's first item is
C 3080-3097's `sys_select` comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  The first
run **failed**: C 2897's `"\r\n\t\f '"` has no Rust spelling for `\f`, so the port writes `\x0c`
(the same byte) with the difference in the line's citation (Deviation 67).

**Evidence ② - the arm reading, and the round's second finding.**  The first `probe` read **38
errors (28 `E0428` + 9 `E0425` + 1 `compile_error!`)** - one more than the lane's baseline.  The new
`E0428` was `ENOEXEC`, and the check `lib/include/errno.rs` 112-120 answered why: that mirror now
carries `ENOEXEC`, `ESRCH` and `EFAULT` (status 3.446/3.514 moved them in, the same P-88 ⑤ route its
own note records for `E2BIG`), while slice 1 had declared `ESRCH`/`EFAULT` here **because the mirror
lacked them then** - three duplicate declarations, the last of which this round added.  The round
removes all three (the unit declares nothing for them; C 1318/1368/3073 resolve through the mirror)
and the probe now reads **35 errors (25 `E0428` + 9 `E0425` + 1 `compile_error!`), 0 anchored in
this unit** - two *fewer* than the standing baseline of 37, which is the better state and the reason
this reading is recorded rather than smoothed over.

**Evidence ③ - the name sweep, and the tool's two reading limits.**  `check-unit-names.py
femacs/src/w32proc.rs 4449 4884` -> `31 capitalized identifiers, missing: ['CRT', 'P', 'PORTING',
'SDK']`.  None of the four is an identifier: they are words of this round's own prose, read as code
by the tool's two documented limits - the block-comment scan does not nest (so the carried
`#if 0` region's inner `/* … */` re-synchronises the scan one delimiter out of phase) and the scan
is fed the *range* it is asked about (so a `/*` opening before the range is not recognised).
Measured the same way: the round's real names (`sa_stack_alloc`, `openp`, `faccessat`, `_mbspbrk`,
`GetCurrentProcessId`, `CharPrevA`, `CharPrevA`, `faccessat`, `X_OK`, `AT_FDCWD`, `AT_EACCESS`,
`ENCODE_FILE`, `SSDATA`, `lispstpcpy`, `ansi_encode_filename`, `stpcpy`, `strchr`, `sprintf`) all
resolve - the tool reports no name this unit uses and cannot find.  The tool's docstring gains the
range-boundary note this round measured (it already lists the nesting one).

**Evidence ④ - the `--check` reading.**  **71 -> 70 `- missing` rows** (`missing function
sys_spawnve`).  One token row appeared and was retired: `function sys_spawnve does not mention
alloca`, whose `macro-body-notes.tsv` row records PORTING.md P-05's device (`sa_stack_alloc`, the
`src/w32.rs` 1770-1774 reading).  Drift 9.3% -> **0.4%** (the port and C now carry nearly the same
number of lines for this range), inside the stem's 0.36 row.

**Deviations (this slice's additions).**

66. **C's four `alloca` sites are PORTING.md P-05's stack pool `sa_stack_alloc`** - the device is
    chosen and this unit calls it through `include/lisp.rs`'s declaration, exactly as `src/w32.rs`
    1770-1774 records for `w32.c`'s four sites; C's sizes are C's own
    (`strlen (cmdname) + 1`, `MAX_PATH`, `arglen`, `numenv * sizeof (char *)`).
67. **C's `\f` is Rust's `\x0c`** (C 2897's `sepchars`): Rust has no `\f` escape, and the byte is
    the same.
68. **The unit's three stale `errno` declarations are removed** (`ESRCH`, `EFAULT`, and this round's
    own `ENOEXEC`): the mirror carries all three now, so the unit's copies were `E0428`s against it.
    Slice 1's note and the ledger's owed rows that named them are superseded by this reading.
69. **The CRT/SDK names stand at the use site with C's own text**: `openp` (the `src/callproc.rs`
    98-103 device), `faccessat`, `_mbspbrk`, `GetCurrentProcessId`, `CharPrevA`; `AT_FDCWD` and
    `AT_EACCESS` are in the graph (`nt/inc/include/unistd.rs` 31, `lib/include/fcntl.in.rs` 34) and
    `X_OK` is in `nt/inc/sys/include/file.rs`, so none of those three is repeated here.
70. **C 2976-2988's commented-out arm is carried as C's own text** (P-51) inside the live `#else`
    arm's position, and C 2969-2974's commented-out `first`/`last` declarations are carried as C's
    comments.

## Slice 16 (2026-10-14, status 4.295): C 3099-3106

**What this round is.**  The three extern objects the file's longest function reads, landed ahead of
it because they are the item C puts between the overview comment and `sys_select`: C 3099's
`keyboard_handle` (`w32console.c`), C 3102's `interrupt_handle` (`w32xfns.c`) and C 3105's
`proc_buffered_char` (`process.c`).  Port: 4,877 -> **4907 lines** (+30).  The next row's first item
is C 3108's `sys_select`, with C 3080-3097's overview comment travelling with it.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  The first
run of the round **failed**: the three C one-line comments and their declarations had been spliced
raw instead of as comments/citations; the rewrite (each C comment as a Rust comment, each
declaration in its `/// C:` line) is the round's only text change between the two runs.

**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the reading slice 15's errno cleanup produced, and the
shape this round adds no name to.

**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 4901 4907` -> `1
capitalized identifier, missing: []`.

**Evidence ④ - the `--check` reading.**  **70 -> 67 `- missing` rows**, and the three that retire
are exactly this round's items (`missing file-scope object keyboard_handle`, `interrupt_handle`,
`proc_buffered_char`).  No token rows.  Drift 0.4% -> **-0.1%**: the port now carries as many lines
as the whole reference file while C 3108-4903 is still to land.

**Deviations (this slice's additions).**

71. **The three externs stand where C declares them, each under its own C comment**: C's
    `/* From w32console.c */`-style lines are carried as Rust comments at their positions and each
    declaration is cited in a `/// C:` line (this file's rule for a declaration).  The array's length
    is the mirror's `FD_SETSIZE` through the module path slice 2's `WFO_MAX_WAIT` already uses
    (Deviation 36), and its definition is `src/process.rs` 489's (whose `-1` fill is its own C's).

**Owed (this slice's additions).**

18. **The drift row now reads -0.1% and will swing positive as the remaining items land** (they are
    the file's most comment- and SDK-heavy blocks: `sys_select`, the twenty `DEFUN`s and
    `syms_of_ntproc`): re-measure and re-append the stem's tolerance row when the file is covered,
    the `w32fns` 0.35-to-0.50 precedent.  The per-function token check is unaffected and remains the
    real guard.

## Slice 17 (2026-10-14, status 4.296): C 3080-3483

**What this round is.**  `sys_select` - the file's longest function (376 C lines) and the one whose
declarations slices 16's three externs were landed for: the four function-local statics, the timeout
decode, the two `SELECT_TYPE` set copies with their `FD_ZERO`s, the `interrupt_handle` entry, the
descriptor scan with its stdin arm (`detect_input_pending`, `handle_file_notifications`) and its
child/socket arm (the `STATUS_READ_ACKNOWLEDGED` handshake), the `count_children` label with its
child scan, the wait and its five-way answer, the fairness `do`/`while` over the remaining handles,
and the noninteractive drain with the timeout recheck.  Port: 4,907 -> **5379 lines** (+472).  The
next row's first item is C 3485-3488's `w32_valid_process_id` comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - slice 15's improved baseline, unchanged.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 4908 5379` -> `33
capitalized identifiers, missing: []` - clean, including the three carried dead arms.
**Evidence ④ - the `--check`, do/while and drift readings.**  **67 -> 66 `- missing` rows**
(`missing function sys_select`), one token row (`does not mention goto`, recorded in
`docs/ports/macro-body-notes.tsv`).  `check-do-while.py`: `221 C do/while condition(s), 73 matched by
a port if ... { break; }, 0 failure(s)` - this round's fairness loop is one of the 73.  Drift 0.4%
-> **-9.7%**: with the file's comment- and SDK-heaviest function in, the port now carries ~10% *more*
lines than the whole reference (the shape `w32fns` ended at, 0.50); the stem's 0.36 row still covers
it (owed 18's re-measure is what fixes the final number).

**Deviations (this slice's additions).**

72. **The four function-local statics stay statics** - Rust allows `static mut` items inside a
    function, so C 3112-3115's `orfds`/`owfds`/`cps`/`wait_hnd`/`fdindex` are declared in place
    (the same `core::mem::zeroed ()`/`null_mut ()` initialisers this file gives every static), and
    the two set types are spelled through the mirror's nullary `SELECT_TYPE! ()` with `FD_ZERO!`,
    `FD_SET!` and `FD_ISSET!` (the mirror's own macros, in this unit's graph at their root).
73. **C 3276's `count_children:` label and C 3478's `goto count_children` are a labeled `loop`** with
    `continue 'count_children` (a *backward* jump: the `start:`/`goto start` device the tree records
    for `find_symbol_value` and friends), and C 3352-3461's `do { … } while (active < nh + nc)` is
    the port's `loop` with the negated `break` (P-95, `check-do-while.py`'s 73 matched pairs).
74. **C's three dead arms are carried as C's own text at their positions** (P-51): the
    `CHECK_INTERLOCK` block (C 3219-3248, a macro no configuration defines) and the two
    `FULL_DEBUG` blocks (C 3253-3256, C 3404-3407).
75. **`detect_input_pending` and `GetTickCount` stand at the use site** (`lisp.h` 5095's
    `bool detect_input_pending (void)` and `<winbase.h>`'s `GetTickCount`): no mirror in this unit's
    graph carries either, while `handle_file_notifications`/`drain_message_queue` are the
    `w32term.rs` mirror's and `FRAME_TERMCAP_P`/`SELECTED_FRAME` come in through `w32term.rs` ->
    `frame.rs`.  C's `ignored` parameter is kept although nothing reads it (the unit's
    `unused_variables` allow covers it).

## Slice 18 (2026-10-14, status 4.297): C 3485-3526

**What this round is.**  The three items between the file's two longest functions: C 3485-3496's
`w32_valid_process_id` (the four-way PID test), C 3498-3523's `find_child_console` (the
`EnumWindows` callback that matches a child's process id and the `tty`/`ConsoleWindowClass` class
name the `os_subtype` selects) and C 3525-3526's `DebugBreakProcess_Proc` typedef.  Port: 5,379 ->
**5454 lines** (+75).  The next row's first item is C 3528's `sys_kill`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading, and the mirror defect this round found.**  The first `probe` read
**36 errors (26 `E0428` + 9 `E0425` + 1 `compile_error!`)** - one more than the lane's baseline - and
the new `E0428` was `w32_valid_process_id`, anchored in `include/w32common.rs` 191: that mirror
declared it in a **top-level** `unsafe extern "C"` block, while this unit defines it.  The mirror's
own note 185-188 records the identical case for `w32_version_string` ("the tenth declaration to
move"), so this round is its **eleventh** P-39 move: the declaration sits in the glob-imported
`w32common_decls` now, with the mirror's note updated and `docs/ports/include-w32common.md` carrying
the ledger row.  Re-read after the move: **35 errors (25 + 9 + 1), 0 anchored in this unit** - the
baseline slice 15 produced, restored.  (Slice 12's `w32heap_file_decls` was the same device; the
probe reports this class because `E0428` *is* a name-resolution error.)
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 5380 5454` -> `16
capitalized identifiers, missing: []`.
**Evidence ④ - the `--check` reading.**  **66 -> 64 `- missing` rows**: `missing function
w32_valid_process_id` and `missing function find_child_console`.  No token rows; drift -9.7% ->
**-11.2%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

76. **`GetWindowThreadProcessId`, `GetClassName` and `LPARAM` stand at the use site** (P-88 ①):
    `<winuser.h>`'s two entry points (C 3506, C 3511) and `<windef.h>`'s `typedef LONG_PTR LPARAM`
    (pointer-sized, spelled `isize`).  `strcmp` is the `string.rs` mirror's and
    `os_subtype`/`OS_SUBTYPE_9X` the `w32common.rs` mirror's.  **`HWND` is *not* the graph's** -
    this note said `include/w32.rs` carried it, and slice 25 measured that it does not (that
    mirror names the type in prose and spells `w32.h`'s members as untyped pointers): the
    `find_child_console` signature's `HWND` is declared at slice 25's use site (Deviation 93).
77. **`include/w32common.rs` gained its eleventh P-39 move** (this round's Evidence ②): the
    `w32_valid_process_id` declaration moved from that mirror's root into `w32common_decls`, the
    module it glob-imports, for the reason its own note records for `w32_version_string`.  The
    macro `VALID_PROCESS_ID!` stays at the root (no unit defines it) and the `#ifdef WINDOWSNT`
    arm's prose is carried where the C text is.

## Slice 19 (2026-10-14, status 4.298): C 3528-3901

**What this round is.**  The file's third-longest function and the pair after it: `sys_kill` (the
negative-pid and signal-whitelist guards, the `sig == 0` probe with its two nested `switch`es on
`GetLastError`, the child lookup and the two `OpenProcess`/`EnumWindows` arms, the `SIGINT`/`SIGQUIT`
keyboard simulation with its thread attaching and the `GenerateConsoleCtrlEvent` fallback, the
`SIGTRAP` arm that resolves `DebugBreakProcess` through `get_proc_addr`, and the terminate arm whose
`os_subtype` chooses between `TerminateProcess` and `PostMessage (WM_CLOSE)`), plus
`prepare_standard_handles` and `reset_standard_handles`.  Port: 5,454 -> **5956 lines** (+502, of
which 103 are the round's SDK block).  The next row's first item is C 3903's `set_process_dir`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes** (both runs:
the SDK block alone, then the functions).
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the baseline, unchanged by 103 lines of new
declarations (the reading a wrong name there would have moved, `E0428` being name resolution).
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 5455 5956` -> `69
capitalized identifiers, missing: ['Msg', 'Some']`: `Some` is the prelude variant and `Msg` is the
*parameter name* of this round's `PostMessage` declaration - the tool's docstring names exactly that
class ("the parameter names of a declaration (C's own `Msg: UINT` in a `SendMessageW` prototype)").
Nothing this round uses is undefined.
**Evidence ④ - the `--check` reading.**  **64 -> 61 `- missing` rows** (`missing function sys_kill`,
`prepare_standard_handles`, `reset_standard_handles`), and **two token rows appeared and were
retired in the round**: `function sys_kill does not mention break, case, default, switch` (the three
`switch (err)`es, now a `macro-body-notes.tsv` row) and `function reset_standard_handles does not
mention 0` (C 3898's `handles[0]` index - the port spells `*handles.add (0)` rather than dropping the
index).  Drift -11.2% -> **-21.5%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

78. **The round's SDK block** (P-88 ①): `OpenProcess`, `EnumWindows` (with `WNDENUMPROC`),
    `MapVirtualKey`, `GetForegroundWindow`, `GetCurrentThreadId`, `AttachThreadInput`,
    `SetForegroundWindow`, `GetKeyState`, `keybd_event`, `GenerateConsoleCtrlEvent`, `PostMessage`
    (with `WPARAM`), `TerminateProcess`, `SetStdHandle` and the CRT's `_get_osfhandle`, plus the
    thirteen values they take (`PROCESS_*`, `ERROR_ACCESS_DENIED`/`ERROR_INVALID_PARAMETER`,
    `VK_CANCEL`/`VK_CONTROL`/`VK_LCONTROL`, the two `KEYEVENTF_*`, `CTRL_BREAK_EVENT`,
    `WM_CLOSE`/`WM_QUIT`).  `get_proc_addr` (the `w32common.rs` mirror), `report_file_error`
    (`lisp.rs`), `ENOTSUP` (`conf_post.rs`), `DUPLICATE_SAME_ACCESS`, `GetModuleHandle`,
    `GetCurrentProcess`, `DuplicateHandle`, the three `STD_*` values and `find_child_console` are
    all already in this unit's graph or its own text.
79. **C's `#if 1` and its inner `#if 0`** (C 3760-3800): the `#if 1` takes its arm in every
    configuration (its `#endif` is C 3800 and C 3799's `else` pairs with C 3762's `if`), so the port
    writes the live arm; the Shell-VxD note (C 3764-3779) and the `#if 0` block (C 3780-3790) are
    carried as C's comments at their positions.
80. **`EnumWindows (find_child_console, (LPARAM) cp)` is `EnumWindows (Some (find_child_console),
    cp as LPARAM)`**: C's function-pointer argument is the port's `Option<...>` wrapper (P-27) and
    C's `(LPARAM)` cast is the port's `as`.  The `DebugBreakProcess` resolution casts
    `get_proc_addr`'s `VOIDFNPTR` through `core::mem::transmute` (the file's slice-1 device for an
    SDK proc pointer).

## Slice 20 (2026-10-14, status 4.299): C 3903-3967

**What this round is.**  The writer of slice 8's `process_dir` (`set_process_dir`), the winsock
overview comment with the two `w32.c` externs C 3920-3922 declares, and the first two `DEFUN`s of
the block: `w32-has-winsock` (the `init_winsock` call, the `winsock_lib` guard with its
save/`init_system_name`/restore of `Vsystem_name`, and the `Qt`/`Qnil` answers) and
`w32-unload-winsock` (the one-line `term_winsock` body).  Port: 5,956 -> **6059 lines** (+103).  The
next row's first item is C 3969-3971's banner comment.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  The first
run **failed**: C 3921-3922's two extern lines had been spliced raw (the file's rule is C's own
comment carried plus a `/// C:` citation per declaration); the fix is the round's only text change
between the two runs.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the baseline.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 5957 6059` -> `11
capitalized identifiers, missing: ['Sw32_has_winsock', 'Sw32_unload_winsock']` - the two `S…`
statics the `DEFUN!` macro generates, the tool's documented *macro-generated names* class.  Nothing
this round uses is undefined.
**Evidence ④ - the `--check` reading.**  **61 -> 57 `- missing` rows** (`set_process_dir`,
`winsock_lib`, `term_winsock` and the `w32-has-winsock` subr row retire with the round's
definitions).  No token rows.  Drift -21.5% -> **-23.6%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

81. **The two `DEFUN!` invocations carry C's docstrings in the macro's `doc:` slot** (the
    `src/w32fns.rs` 10403 shape), with C's `Qt`/`Qnil` answers spelled `Qt ()`/`Qnil ()` and C's
    `init_winsock (!NILP (load_now))` the `!NILP (load_now) as c_int` the mirror's signature
    (`init_winsock (load_now: c_int) -> c_int`) asks for.
82. **The winsock overview comment and C 3920-3922's two externs stand at their C positions**: C's
    `/* From w32.c */` is carried as a Rust comment and each declaration in a `/// C:` line -
    `winsock_lib`/`term_winsock` are `w32.c`'s (`include/w32.rs` carries neither, while
    `init_winsock` is that mirror's 534), so this unit declares them, as its C does.
83. **`init_system_name` stands at the use site** (`src/process.rs` defines it; C's `process.h`
    chains to it) - a declaration-only row in the shape `src/emacs.rs` uses.

**Owed (this slice's additions).**

19. **Eighteen `DEFUN`s and `syms_of_ntproc` remain** (C 3969-4903, 1,003 C lines): the block is one
    invocation per subr, so the rounds that carry it can be split per `DEFUN` or in small groups;
    `syms_of_ntproc` (C 4764) is the file's last item and the defsubr/`DEFVAR` table that must name
    each of them.

## Slice 21 (2026-10-14, status 4.300): C 3969-4082

**What this round is.**  The banner comment and the next three `DEFUN`s: `w32-short-file-name`
(`Fexpand_file_name` plus the `w32_get_short_filename` guard), `w32-long-file-name` (the
two-character drive test, `w32_get_long_filename`, and the drive-only fixup) and
`w32-set-process-priority` (`find_child_pid`'s fixup, `OpenProcess`, the two `EQ` tests and
`SetPriorityClass`).  Port: 6,059 -> **6227 lines** (+168).  The next row's first item is C 4084's
`w32-application-type`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - the baseline.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 6060 6227` -> `37
capitalized identifiers, missing: ['Sw32_short_file_name', 'Sw32_long_file_name',
'Sw32_set_process_priority']` - the macro-generated `S…` statics again.
**Evidence ④ - the `--check` and docstring readings.**  **57 -> 54 `- missing` rows** (the three
subr rows).  No token rows - the docstrings are carried as literals in the invocations and the token
comparison accepts them.  `check-defvar-carriers.py`: `817 DEFVAR_* site(s), 817 with a declaration
whose type is checked, 0 failure(s)`.  Drift -23.6% -> **-27.0%**, inside the stem's 0.36 row.

**Deviations (this slice's additions).**

84. **Four `<winbase.h>` names stand at the use site** (P-88 ①): `PROCESS_SET_INFORMATION`,
    `NORMAL_PRIORITY_CLASS`, `IDLE_PRIORITY_CLASS`, `HIGH_PRIORITY_CLASS` and the
    `SetPriorityClass` entry point (`OpenProcess` is slice 19's).  Everything else the three
    `DEFUN`s call is the graph's: `w32_get_short_filename`/`w32_get_long_filename`/
    `dostounix_filename`/`Fexpand_file_name`/`build_unibyte_string`/`DECODE_FILE`/`SBYTES`/`SDATA`/
    `CHECK_STRING`/`CHECK_SYMBOL`/`CHECK_FIXNUM`/`EQ`/`MAX_UTF8_PATH`/`Qhigh`/`Qlow`.

## Slice 22 (2026-10-14, status 4.301): C 4084-4135

**What this round is.**  The `DEFUN` `w32-application-type`: the
`Fexpand_file_name`/`ENCODE_FILE`/`Fcopy_sequence`/`SSDATA`/`unixtodos_filename`/`filename_to_ansi`
chain, the `_mbspbrk` guard that rejects names outside the ANSI codepage, and the
`w32_executable_type` call whose four flags choose between `Qdos`, `Qcygwin`, `Qmsys` and
`Qw32_native` (with `Qunknown` for both failure paths).  Port: 6,227 -> **6296 lines** (+69).  The
next row's first item is C 4137's `#ifdef HAVE_LANGINFO_CODESET` arm.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit**.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 6228 6296` -> `14
capitalized identifiers, missing: ['Sw32_application_type']` - the macro-generated static again.
**Evidence ④ - the `--check` reading.**  **54 -> 53 `- missing` rows** (the subr row).  No token
rows; the docstring rides in the invocation as slice 21's do.  Drift -27.0% -> **-28.4%**.

**Deviations (this slice's additions).**

85. **The `Q…` symbols the body answers with are the `globals.rs` mirror's**, not this unit's: C's
    `syms_of_ntproc` (C 4764, the file's last item) is where the C text *defines* them, and the
    mirror already declares every one - so the port reads `Qdos ()`/`Qcygwin ()`/`Qmsys ()`/
    `Qw32_native ()`/`Qunknown ()` through that declaration, exactly as the `w32-has-winsock` DEFUN
    reads `Qt ()`/`Qnil ()`.

## Slice 23 (2026-10-14, status 4.302): C 4137-4234

**What this round is.**  The file's langinfo arm: C 4137's `#ifdef HAVE_LANGINFO_CODESET` (the
frozen table has the macro 1, so the arm is live and the port carries it under the
`have_langinfo_codeset` feature the include lines above already use), C 4139-4143's
`LOCALE_IPAPERSIZE` fallback, and `nl_langinfo` itself (the twenty-two-entry `w32item` table, the
lazily allocated and grown buffer with its `xmalloc`/`xrealloc` sites, the `CODESET` path that
either `sprintf`s a `cp`-prefixed codepage or `memmove`s the `cp` prefix in, and the `_NL_PAPER_*`
table with its `atoi` index).  Port: 6,296 -> **6502 lines** (+206).  The next row's first item is
C 4236's `w32-get-locale-info`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  The first
run **failed**: the `paper_size` initialiser had been written with C's braces instead of Rust's
brackets; the fix is the round's only text change between the two runs.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - unchanged by the round's ~30 new declarations, which
is the reading a wrong name among them would have moved.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 6297 6502` -> `33
capitalized identifiers, missing: ['LCType', 'Locale']` - the *parameter names* of this round's
`GetLocaleInfo` declaration (`decltype DWORD LCType, LCID Locale`), the class the tool's docstring
names, as slice 19's `Msg` was.
**Evidence ④ - the `--check` reading.**  **53 -> 50 `- missing` rows**: `missing function
nl_langinfo` and the two `LOCALE_IPAPERSIZE` rows (macro and constant).  No token rows - the two
tables and the `countof` spelling ride through the token comparison.  Drift -28.4% -> **-32.6%**,
inside the stem's 0.36 row.

**Deviations (this slice's additions).**

86. **The round's SDK/CRT surface stands at the use site** (P-88 ①): `LCID`/`LCTYPE`, the
    twenty-two `LOCALE_*` values (the `LOCALE_IPAPERSIZE` fallback included, which the port declares
    because it has no preprocessor), `GetThreadLocale`, `GetLocaleInfo`, `GetACP` and the CRT's
    `atoi` - all UNVERIFIED.  `nl_item`, `CODESET`, `_NL_NUM` and the two `_NL_PAPER_*` enumerators
    are the `nt/inc/include/langinfo.rs` mirror's, and `xmalloc`/`xrealloc` are `include/lisp.rs`'s.
87. **`countof` is spelled at the use site** as `macro_rules! countof { ($a:expr) => { (($a).len ())
    }; }`: gnulib's `<stdcountof.h>` mirror (`lib/include/stdcountof.rs` 55, whose body carries a
    `_gl_verify_is_array` clause this port does not need) is in no unit's graph, so C 4223's
    `countof (paper_size)` gets the array's length the same way the macro computes it.

## Slice 24 (2026-10-14, status 4.303): C 4236-4321

**What this round is.**  Three items: `w32-get-locale-info` (the `IsValidLocale` guard and the three
branches that answer from `LOCALE_SABBREVLANGNAME`, `LOCALE_SLANGUAGE` with `DECODE_SYSTEM`, or a
caller-given `LCTYPE` through `make_unibyte_string`), `w32-get-current-locale-id` (the one-line
`GetThreadLocale`) and the static `int_from_hex` (the `strchr` walk over C's digit table and its
`p - hex` arithmetic).  Port: 6,502 -> **6635 lines** (+133).  The next row's first item is C 4323's
comment with C 4325's `Vw32_valid_locale_ids`.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort --cfg
'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit**.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 6503 6635` -> `26
capitalized identifiers, missing: ['Locale', 'Sw32_get_locale_info',
'Sw32_get_current_locale_id']` - the `IsValidLocale` declaration's parameter name and the two
`DEFUN!` statics, all three in the tool's documented classes.
**Evidence ④ - the `--check` reading.**  **50 -> 47 `- missing` rows** (the two subrs and
`missing function int_from_hex`).  No token rows.  Drift -32.6% -> **-35.3%**, inside the stem's
0.36 row.

**Deviations (this slice's additions).**

88. **The round's `<winbase.h>` names stand at the use site** (P-88 ①): `IsValidLocale` and the
    two `LOCALE_*` values C 4265/4273 read (`LCID_SUPPORTED`, `LOCALE_SLANGUAGE`,
    `LOCALE_SABBREVLANGNAME`; `LCID`, `LCTYPE` and `LOCALE_USE_CP_ACP` are slice 23's).
89. **C 4309's `static char hex[] = "0123456789abcdefABCDEF";` is the tree's `*b"…"` device**
    (`src/sysdep.rs` 1104/4693's spelling), with C's terminating NUL kept and the array's length
    C's own (23); `p - hex` is the port's `offset_from`, the same pointer difference C's `unsigned
    digit` takes.
## Slice 25 (2026-10-14, status 4.304): C 4323-4571

**What this round is.**  Twelve `DEFUN`s and the four file-scope objects and callbacks they read:
the locale pair (`Vw32_valid_locale_ids` C 4325, `enum_locale_fn` C 4327-4333 and
`w32-get-valid-locale-ids` C 4335-4348, whose C 4344 call is the SDK's `EnumSystemLocales`),
`w32-get-default-locale-id` (C 4351-4362), `w32-set-current-locale` (C 4365-4384, with the
`dwWindowsThreadId` notification at C 4379-4381), the codepage pair (`Vw32_valid_codepages`
C 4389, `enum_codepage_fn` C 4391-4397 with C 4394's `atoi`, and `w32-get-valid-codepages`
C 4399-4410), the four console-codepage `DEFUN`s (C 4413-4466), `w32-get-codepage-charset`
(C 4469-4496, C 4492's `TranslateCharsetInfo`) and the three keyboard-layout `DEFUN`s
(C 4499-4571, C 4506's `alloca` as PORTING.md P-05's `sa_stack_alloc`, C 4511's
`while (--num_layouts >= 0)`, C 4551-4552's packed `HKL` and C 4560-4561's `MSG`/`GetMessage`
wait on `WM_EMACS_DONE`).  Port: 6,635 -> **7161 lines** (+526).  The next row's first item is
C 4573's comment with C 4578-4579's two statics.

**Evidence ① - the frozen reading.**  `build-tu src/w32proc.c` -> exit 0, **336 bytes**.  This
reading is the *ungated* text: the file gate removes every item of this unit in the frozen
configuration (P-86 (1)), so a missing `macro_rules!` or a wrong arm is not what it measures.
**Evidence ② - the arm reading.**  `probe src/w32proc.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"' --cfg 'feature="have_setitimer"' --cfg
'feature="have_langinfo_codeset"'` -> **exit 1, 35 errors (25 `E0428` + 9 `E0425` + 1
`compile_error!`), 0 anchored in this unit** - unchanged by the round's ~40 new declarations,
which is the reading a name the mirror graph also declares would have moved.  The round's *first*
spelling put `macro_rules! HIWORD` in the trailing SDK block and read **37: 2 `cannot find macro
HIWORD in this scope`**, both anchored in this unit - a `macro_rules!` is visible only after its
definition, so the macro moved to C 4515's use site (the C 2593 `LOWORD` shape, Deviation 92) and
the reading returned to the baseline.  Two consequences the lane should keep: macro *name*
resolution does read this unit's text (unlike the type resolution below), and the round's
appended-item experiment (a probe copy with a deliberately unknown name - in value position and in
an item signature - appended at the end) reports nothing for it, i.e. the reading is blind to name
errors past the point where the include graph's own errors abort the pass; a *missing* name is
caught by Evidence ③'s sweep, never by ②.
**Evidence ③ - the name sweep.**  `check-unit-names.py femacs/src/w32proc.rs 6637 7159` -> `92
capitalized identifiers, missing: ['Clone', 'CodePage', 'Copy', 'Flags', 'Locale', 'Some',
'Sw32_get_codepage_charset', 'Sw32_get_console_codepage', 'Sw32_get_console_output_codepage',
'Sw32_get_default_locale_id', 'Sw32_get_keyboard_layout', 'Sw32_get_valid_codepages',
'Sw32_get_valid_keyboard_layouts', 'Sw32_get_valid_locale_ids', 'Sw32_set_console_codepage',
'Sw32_set_console_output_codepage', 'Sw32_set_current_locale', 'Sw32_set_keyboard_layout']` - the
ten `DEFUN!` statics (class c), the three declaration parameter names `CodePage`/`Flags`/`Locale`
(class e) and the derive/prelude names `Clone`/`Copy`/`Some` (class f), all in the tool's
documented classes.
**Evidence ④ - the `--check` reading.**  **47 -> 31 `- missing` rows**: the sixteen this round
satisfies are the twelve new `F…` entries, the two callbacks and the two file-scope objects; the
eighteen `defsubr` rows stay until `syms_of_ntproc` (C 4764) lands, and the rest are the next
slice's (`get_lcid`, `get_lcid_callback`, `w32_compare_strings`, `syms_of_ntproc`, `lname`,
`found_lcid`, `LOCALE_NAME_MAX_LENGTH`, `_NLSCMPERROR`, `LINGUISTIC_IGNORECASE`).  One token row,
recorded in `docs/ports/macro-body-notes.tsv` this round: `Fw32_get_valid_keyboard_layouts` does
not mention `alloca` (C 4506 is P-05's `sa_stack_alloc`).  Drift 35.3% -> **46.1%** (7,161 port
lines against C's 4,903, the tool's own line): the port is now *ahead* of the reference, and the
measurement crossed the stem's 0.36 row, so this round re-appends that row at **0.47** (measured
+ 0.01, the w32fns 0.35/0.37/0.39/0.41 rule) - owed 18 is settled here.

**Deviations (this slice's additions).**

90. **The round's `<windows.h>`/`<wingdi.h>` surface stands at the use site** (P-88 (1)): the two
    enumeration callbacks and their entry points (`LPTSTR`, `LOCALE_ENUMPROCA`,
    `CODEPAGE_ENUMPROCA`, `EnumSystemLocales`, `EnumSystemCodePages`), the locale and console
    entries C 4359-4465 reads (`GetSystemDefaultLCID`, `GetUserDefaultLCID`, `SetThreadLocale`,
    `PostThreadMessage`, `GetConsoleCP`, `SetConsoleCP`, `GetConsoleOutputCP`,
    `SetConsoleOutputCP`, `IsValidCodePage`), the charset record and converter C 4480/4492 read
    (`FONTSIGNATURE`, `CHARSETINFO`, `LPCHARSETINFO`, `TCI_SRCCODEPAGE`,
    `TranslateCharsetInfo`), and the keyboard-layout entries C 4505-4567 read
    (`HKL`, `GetKeyboardLayoutList`, `GetKeyboardLayout`, `ActivateKeyboardLayout`, `POINT`,
    `MSG`, `GetMessage`, `UINT_PTR`) - all UNVERIFIED.  `Dial...` none; `LPTSTR` is the one name
    of the `<windows.h>` typedef line (C 50's section, `src/w32proc.rs` 244) that section names but
    does not declare, and this use site is what needs the TCHAR spelling.
91. **`LCID_SUPPORTED` is corrected to `0x00000002`** in the slice-23/24 SDK block (its citation
    line with it), and `CP_SUPPORTED` is declared with the same value: the SDK's two "supported"
    flags are distinct from the "installed" pair (`LCID_INSTALLED`/`CP_INSTALLED`, the
    `0x00000001` the `src/w32select.rs` 301 row carries), and C 4344/4406 pass the *supported*
    flag.  Slice 24's `0x00000001` made the two names coincide; both values remain UNVERIFIED on
    this machine (P-88 (1)), so this is a correction of a *recorded* value, not a measurement.
92. **`HIWORD` stands at C 4515's use site as a `macro_rules!`** (the C 2593 `LOWORD` precedent):
    C's `<windef.h>` macro is used at C 4515-4517 and C 4533-4534, and a Rust `macro_rules!` is
    visible only after its definition, so the definition sits above the first user (Evidence ②
    measured both the failure and the fix).  The call sites keep C's name with the port's `!`
    (`LOWORD!`/`HIWORD!`).
93. **`HWND` is declared at this use site and slice 22's Deviation 76 is corrected**: that note
    said `HWND` was "the graph's (`include/w32.rs`)", but that mirror only *names* the type in its
    prose (C 27, C 322) and spells `w32.h`'s own members as untyped pointers (P-72 (3)), so no
    file in this unit's graph declares it.  Slice 22's `find_child_console` (C 3500-3523) reads the
    name without a declaration; the record is corrected in place there and the handle is declared
    here, where C 4560's `MSG` needs it as its first member's type.
94. **The two `DEFUN` doc-comment pairs the round carries** (the C 4386-4388 and C 4323-4324
    comments, and C 4469-4477's "Note that this function is only guaranteed to work with ANSI
    codepages" paragraph) are carried as C's own text at their C positions, the same device the
    earlier slices use; the `DEFUN!` doc slots carry C's docstrings as single literals, whose
    leading `       doc: /*` run and the arglist line's two spaces are C's own columns.

