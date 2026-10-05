# emacs/src/w32proc.c (4903 lines) — first slice: C 1-912

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32proc.o` is added to
> `W32_OBJ` only by `configure.ac` 3051's `else` arm (the non-Cygwin one, after the `HAVE_W32` test
> at 3019), and `docs/baseline/frozen-build-sources.txt` lists no `w32proc.o` for this target - so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/w32proc.rs` (**1550 lines**, 66,415 bytes, md5
> `e7f0a31a5072265f75da9d7199ba66c1`), carrying **C 1-912** - P-86 ④'s first slice of this file -
> with **both arms of every `#if`**.
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
| 66-69 | `#define RVA_TO_PTR(var,section,filedata)` | **carried as C's text**: its body reads three fields of `<windows.h>`'s `IMAGE_SECTION_HEADER`, which `include/w32heap.rs` 63-76 spells `*mut c_void` (P-72 ③), so no Rust body is writable here (Deviations 6) |
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
| 913-4903 | **the remainder of the file** - the C 914-962 wait-object banner through C 4861-4869's `#if 0` and `syms_of_ntproc` (C 4764): the wait-object thread pool, `sys_spawnve` (C 2725), `sys_select` (C 3109), `sys_kill` (C 3530), the standard-handle pair, the twenty `DEFUN`s (C 3924-4730) and `syms_of_ntproc` | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
slice's own text, `RVA_TO_PTR` (carried as text, Deviations 6) and the two `WFO_*`/`LOCALE_*`-family
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
   a measured `IMAGE_SECTION_HEADER` for `RVA_TO_PTR` (C 66-69, carried as text: Deviations 6;
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
