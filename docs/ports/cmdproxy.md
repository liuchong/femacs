# emacs/nt/cmdproxy.c (965 lines) — whole file, C 1-965

> **One of P-86's class: the frozen configuration compiles nothing here.**  `cmdproxy` is built by
> `emacs/nt/Makefile.in` for the Windows port only, and `docs/baseline/frozen-build-sources.txt`
> lists no `nt/cmdproxy` object for this target, so `extract-abi.py`'s checklist for the file is
> empty (P-86 clause 2) and this ledger is that clause's substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `nt/cmdproxy.rs` (**1337 lines**,
> md5 `9e43f0e5422ef69aaf915a876ac7a24e`), carrying **C 1-965 with both arms of every `#if`**.  The
> reference `emacs/nt/cmdproxy.c` is md5 `4674005cf012355838a3f42034f5d151` (unchanged; `emacs/` is
> read-only).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/cmdproxy.md` (and the
> 3-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 clause 1 replaces that conclusion with *transcribe the text, both arms, no stub*;
> what the earlier note measured (not in the frozen source list, empty checklist, 336-byte
> `build-tu` object) is unchanged and still true — it is the *conclusion* that is withdrawn, exactly
> as `docs/ports/runemacs.md`, `docs/ports/addpm.md` and `docs/ports/w32console.md` record for their
> own pairs.
>
> **Acquisition: this piece is `text-only` (P-86 clause 3(ii)).**  The machine has no Windows and no
> mingw toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is **no `<windows.h>`
> to compile or measure against**, no arm flip on a Windows target and no layout measurement: every
> record and every value of the `<windows.h>` stand-in section is `UNVERIFIED` (the documented SDK
> text, not a measurement) and the layout assertions are owed.  What the round does carry is the
> frozen `build-tu` reading, the machine-checked coverage table, the `#if` verdicts, a per-block C
> citation, the `probe` reading of the arm under this machine's `rustc`, and a **mutation test** over
> this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`emacs/nt/Makefile.in`'s `OTHERS`; the Windows port) | `feature = "windowsnt"` (on every item *and* on C 35-43's include graph) | the native MS Windows build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not this target |
| C 49-51 `#ifdef _UCRT` / `# define _snprintf snprintf` / `#endif` | `feature = "ucrt"` for the `snprintf` arm and the `use` alias, `not (feature = "ucrt")` for C 55's `_snprintf` declaration; both arms carried | `_UCRT` is a compiler-provided MSVC/UCRT macro and is not in `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen side is the `not (ucrt)` arm; `ucrt` is not registered in `xtask/Cargo.toml` yet (Owed 1) |
| C 64-68 `#if __GNUC__ + (__GNUC_MINOR__ >= 4) >= 5` / `… __attribute__((noreturn));` / `#else` / `…;` / `#endif` | **no Rust name** (both arms are declarations and are dropped, PORTING.md 0.1-12); the consequence is `fail`'s `-> !` | the frozen table has `__GNUC__` 4 / `__GNUC_MINOR__` 2 (macros.tsv 6264/6259), so the `#else` arm is the live one; Deviation 3 |
| C 497-510 `#if 0` | **no gate**: C's own preprocessor never compiles it; the port keeps C's text inside a comment (the `src/cm.rs` 86 reading) | never |
| C 543-550 `#if 0` | as above | never |
| C 800-809 `#if 0` (`#else` at C 810 is the live arm, C 811-813) | as above for the dead arm; C 811-813 is plain code | never / always |

`tools/check-cond-eval.py nt/cmdproxy.c` reads **`5 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ⑥): C 49, C 64, C 497, C 543 and C 800 — those five rows are the whole
conditional text of the file, and the file is on `docs/baseline/config/cond-eval-verified.txt`
(line 183), so the frozen macro table decides every arm.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-28 | the file's opening block: its title line, the copyright years, the two paragraphs describing what the proxy does, and the accompanying notice text that follows them | dropped (the port opens with the `// femacs: port of …` line; PORTING.md 0.1-19) |
| 30 | `#define DEFER_MS_W32_H` | dropped as a define: the name is the row `docs/ports/excluded-macros.txt` carries for it, and no `w32.h` is in this unit's graph (the `nt/runemacs.rs` 43-46 shape) |
| 31-34 | the note that the stdio-consolesafe `*printf` redefinitions are not wanted, then `#define OMIT_CONSOLESAFE 1` | file gate; `pub const OMIT_CONSOLESAFE: c_int = 1` (the `nt/addpm.rs` 52-53 shape for the same kind of marker; `src/conf_post.h` 400 is its only reader) |
| 35 | `#include <config.h>` | ungated `include!` (`../src/include/config.rs`) |
| 37 | `#include <windows.h>` | file gate; the SDK surface this unit names is declared here (UNVERIFIED, Deviation 2) |
| 39-43 | `#include <stdarg.h>`, `<malloc.h>`, `<stdlib.h>`, `<string.h>`, `<ctype.h>` | file gate; `<stdarg.h>` and `<malloc.h>` contribute `VaList` and `alloca` (`sa_stack_alloc`, P-05), the other three are mirrors; Deviation 4 |
| 45-51 | the UCRT note, then `#ifdef _UCRT` / `# define _snprintf snprintf` / `#endif` | file gate / `feature = "ucrt"`; the `snprintf` arm plus the `use … as _snprintf` alias, and C 55's declaration on the other arm (Deviation 5) |
| 53-55 | the note that `<stdio.h>` is deliberately not included, then `extern int _snprintf (char *buffer, size_t count, const char *format, ...);` | file gate; the declaration is carried at C 55's position under `not (feature = "ucrt")` |
| 57 | the `Mock C library routines` banner | carried as C's own comment |
| 59 | the note that the routines exist to minimize the executable size | carried as C's own comment |
| 61-62 | `#define stdout GetStdHandle (STD_OUTPUT_HANDLE)` / `#define stderr GetStdHandle (STD_ERROR_HANDLE)` | file gate; same-named `macro_rules!` (PORTING.md 2.1) |
| 64-68 | `#if __GNUC__ + (__GNUC_MINOR__ >= 4) >= 5` / `void fail (const char *, ...) __attribute__((noreturn));` / `#else` / `void fail (const char *, ...);` / `#endif` | dropped (both arms are declarations, PORTING.md 0.1-12); recorded as one comment block and as `fail`'s `-> !` (Deviation 3) |
| 69-72 | the four declarations `vfprintf`/`fprintf`/`printf`/`warn` | dropped (same rule); the five definitions follow in C's order |
| 74-82 | `vfprintf` | file gate |
| 84-95 | `fprintf` | file gate; the `va_list`/`va_start`/`va_end` tokens are the appended `macro-body-notes.tsv` row (Deviation 6) |
| 97-108 | `printf` | file gate; same row |
| 110-120 | `fail` | file gate; `-> !` is C 65's noreturn attribute (Deviation 3) |
| 122-130 | `warn` | file gate; the same `va_*` row |
| 132 | the banner `/*******…*******/` | carried as C's own comment |
| 134-147 | `canon_filename` | file gate |
| 149-154 | `skip_space` | file gate |
| 156-161 | `skip_nonspace` | file gate |
| 163-166 | the `escape_char` note and `static const int escape_char = '\\';` | file gate; `const escape_char: c_int` |
| 168-269 | `get_next_token` (C 168's own comment is its first line) | file gate |
| 271-291 | `batch_file_p` (C 271's comment) | file gate |
| 293-343 | `search_dir` (C 293-294's comment) | file gate |
| 345-415 | `make_absolute` (C 345-347's comment); C 402's `min` is `<windows.h>`'s `<windef.h>` macro and C 389's `alloca` is `sa_stack_alloc` | file gate (Deviation 6, 7) |
| 417-493 | `try_dequote_cmdline` (C 417-420's comment); C 433-437's anonymous enum, C 429's `alloca`, C 441-485's three `switch`es | file gate (Deviations 6, 8, 9) |
| 495 | the banner `/*****************************************************************/` | carried as C's own comment |
| 497-510 | `#if 0` / `_argv` / `_argc` / the commented-out `setup_argv` definition / `#endif` | no gate: C's preprocessor never compiles it, so the port keeps C's text inside a comment |
| 512-519 | the note that the child-process information is global, `static PROCESS_INFORMATION child;`, `static int interactive = TRUE;`, and `BOOL console_event_handler (DWORD);` | file gate; the two statics are `static mut` with C's zero initialiser / `TRUE`, the prototype is dropped (PORTING.md 0.1-12) |
| 521-553 | `console_event_handler`; C 543-550 the `#if 0` `default:` arm | file gate / `#if 0` as a comment; Deviation 10 |
| 555-599 | `spawn` (C 555-556's comment) | file gate; the two `core::mem::zeroed ()` locals and three use-site casts (Deviations 7, 11) |
| 601-612 | `get_env_size` (C 601's comment) | file gate |
| 614 | the `Main program` banner | carried as C's own comment |
| 616-965 | `main`: C 633's handler cast, C 635-664 the working directory/`GetACP` fix-up, C 669-735 the `argv[0]`/`modname` comparison with its basename fallback, C 747-798 the switch scan, C 800-814 the `#if 0`/`#else` pair, C 820-842 the direct-execution attempt, C 844-945 the `pass_to_shell` block, C 947-965 the tail | file gate; C 800-809 dead as a comment, C 811-813 live; Deviations 7, 8, 12, 13 |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/cmdp-lane/target cargo run -q -p xtask -- build-tu nt/cmdproxy.c`
→ `built nt/cmdproxy.rs -> ../build/femacs/cmdproxy.o`, **exit 0**, and `build/femacs/cmdproxy.o` is
**336 bytes** — an empty object, the expected reading (every item sits behind the file gate, so under
the frozen feature set `rustc` compiles only the header comment and the `#![allow]`).  This door
still measures the unit's text: cfg-stripping happens *after* parsing, so a syntax error inside a
`#[cfg]`-ed-out item fails it.

**② The arm flip (P-86 clause 3(i); the file gate plus its sub-gates).**
`cd femacs && CARGO_TARGET_DIR=/tmp/cmdp-lane/target cargo run -q -p xtask -- probe nt/cmdproxy.rs
/tmp/cmdp-lane/cmdproxy.rlib --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"' --cfg
'feature="have_ntgui"' --cfg 'feature="have_window_system"'` → the **first** run read **exit 1, 6
errors**, classified **by file** (P-86 clause 7):

| anchor | count | what |
| --- | --- | --- |
| the `lib/include/stdlib.rs` copy | 3 | `E0425 c_long` / `c_double` / `c_ulong` at that mirror's 14/16/37 — the mirror spells its three string-to-number entries with names it takes from the unit that `include!`s it, and this unit's `use` line did not carry them (the `nt/addpm.rs` 40 reading) |
| `_root.rs` (this unit) | 3 | `E0308` × 3: `strlen`'s `usize` into `WriteFile`'s `DWORD` (C 81), `envblock`'s `*mut c_char` into `CreateProcess`'s `LPVOID` (C 583-584), and `Some (console_event_handler)`'s `extern "C"` into `PHANDLER_ROUTINE`'s `extern "system"` (C 633) |

All six were this unit's own text (the mirror's three are the includer's missing `use`), so all six
were fixed here — no finding was left for a mirror.  The re-read on the fixed text is **exit 0, 0
errors, 4 warnings, 94,280-byte rlib**.  The 4 warnings are all `unused doc comment`, on the
`/// C:` citation blocks that stand above the four `unsafe extern` blocks — the same class
`nt/addpm.rs` emits five of ("they are citations, not documentation").

**③ The mutation test: the "0 in the unit" reading is not vacuous.**  P-86 clause 7's trap is a graph
that aborts before type-checking the unit.  A copy of the materialized root in `/tmp` with one
injected type error inside `get_env_size` (`let _mutation: c_int =
"cmdproxy_arm_probe_no_such_type";`) reads **one more error, anchored at `_root.rs` and naming the
mutant** (1 error / 4 warnings vs the base reading's 0): `rustc` does reach this unit's bodies, and
the base 0 is a real type-check.  (The first attempt ran the *stable* toolchain by hand and reported
6 × `E0658` for `c_variadic`; re-run through `rustup run nightly-2026-10-01`, the pin, the reading
above is the comparable one.)

**④ `extract-abi --check`.**  `python3 tools/extract-abi.py --check emacs/nt/cmdproxy.c
nt/cmdproxy.rs` → **one** finding left: `line count drifted 38.5%: C 965 vs Rust 1337`.  The
six findings the first run carried were the body-token rows for the C-variadic `*printf` family and
for `alloca`/`switch`, appended by this lane to `docs/ports/macro-body-notes.tsv` (Deviation 6;
that file is append-only, and seven rows were added).  The drift row is the integrator's
(`docs/ports/line-drift-tolerance.tsv` is not in this lane's write set) — Owed 2.

**⑤ The `#if` verdicts.**  `python3 tools/check-cond-eval.py nt/cmdproxy.c` → `5 conditions, 0
unknown, 0 disagreements [VERIFIED]` (C 49, C 64, C 497, C 543, C 800).

**⑥ The coverage table.**  `python3 tools/check-dead-file-coverage.py` → `0 failure(s)` (the
reading below, after the table above was written).

## Deviations

1. **C's forward declarations are dropped** (PORTING.md 0.1-12): C 64-68's `fail` pair, C 69-72's
   `vfprintf`/`fprintf`/`printf`/`warn`, and C 519's `console_event_handler`.  Each definition
   appears once, in C's order.
2. **C 37's `<windows.h>` is an in-place stand-in section** (P-88 clause 1/2, P-90 clause 4): the
   nine scalar typedefs, `PHANDLER_ROUTINE`, the three records (`STARTUPINFO`,
   `SECURITY_ATTRIBUTES`, `PROCESS_INFORMATION` — copied from `nt/runemacs.rs` 157-227, the
   same SDK header and the same ANSI arm), the ten constants and the nineteen entry points this file
   spells.  **Every one is UNVERIFIED**: no Windows SDK, CRT or MinGW-w64 exists on this machine, so
   every value that is not a citation of a landed unit (`MAX_PATH` 260, `TRUE`/`FALSE`, `DWORD`,
   `BOOL`, `WORD`) is written from the documented SDK text and marked
   `// placeholder, not <header>'s value`.  `WinDef.h`'s `min` and `WINUSERAPI`'s `WINAPIV`
   `wvsprintfA` (cdecl, so its own `unsafe extern "C"` block) are the two members that are not
   `WINAPI`.
3. **C 64-68's `__GNUC__` pair has no Rust site and one consequence.**  Both arms are declarations
   and are dropped (0.1-12), so the condition cannot be a `#[cfg]`; what C 65's
   `__attribute__((noreturn))` says is spelled on the definition instead: `fail` is `-> !` because
   its body's `exit (-1)` is a non-returning exit (PORTING.md's attribute table maps the noreturn
   family to the never type, the `src/eval.rs` `error`/`verror` shape).  The frozen table's
   `__GNUC__` 4 / `__GNUC_MINOR__` 2 make the `#else` arm the live one for this configuration.
4. **C 39-43's five headers.**  `<stdarg.h>` and `<malloc.h>` have no mirror: the first contributes
   `core::ffi::VaList` (PORTING.md 2.4 — `va_start`/`va_end` have no counterpart, the `...`
   parameter *is* the list) and the second only `alloca`, which is P-05's `sa_stack_alloc`
   (`src/alloc.rs` 1039) — the one deliberate class of port-only exports, declared once here
   as `nt/runemacs.rs` 320-325 does.  `<stdlib.h>`, `<string.h>` and `<ctype.h>` are mirrors; the
   eight names they do not carry (`atoi`, `exit`, `strdup`, `strcat`, `strpbrk`, `strncpy`,
   `stricmp`, `strnicmp`) are declared at their use sites (P-88 clause 1).  `stdlib.rs` also spells
   three signatures with `c_long`/`c_double`/`c_ulong`, which an `include!`d mirror takes from its
   includer, so this unit's `use` line carries all seven `c_*` names (Evidence ②).
5. **C 49-51's `#ifdef _UCRT` may not become an unconditional alias.**  Both arms are carried; C's
   `# define _snprintf snprintf` is rendered as `#[cfg (feature = "ucrt")] use self::snprintf as
   _snprintf;`, so the eleven call sites in `main` keep C's own `_snprintf` spelling on both arms
   (`use` is the port's spelling of a name alias; a `macro_rules!` would have forced `!` at every
   site and changed all eleven statements).  `ucrt` is **not** in `xtask/Cargo.toml` —
   Owed 1.  The `ucrt` arm is compiled by no reading of this round (the mandated `probe` command
   sets `windowsnt`/`have_ntgui`/`have_window_system` only), so it is text that the frozen door
   parses and the arm door strips.
6. **Seven rows were appended to `docs/ports/macro-body-notes.tsv`** (append-only; no line of the
   file was rewritten): `fprintf`/`printf`/`warn` for `va_list`/`va_start`/`va_end` (P-04, the
   `error`/`fatal` rows' shape), `make_absolute`/`main` for `alloca` (P-05, the AVAIL_ALLOCA row),
   and `try_dequote_cmdline`/`console_event_handler` for `switch`/`case`/`default` (PORTING.md 2.2,
   the `lib/fcntl.c` row).  The rows record the residual they create: with `switch`/`case` exempted,
   a *deleted* arm of those two switches is invisible to the token check.
7. **C's `size_t`/`int` and pointer mixes are written out at the use site** (PORTING.md 1.5, P-72
   clause 3, the `nt/runemacs.rs` 546 shape): `strlen`'s `usize` into `WriteFile`'s `DWORD`,
   `envblock` into `CreateProcess`'s `LPVOID`, `retcode` into `GetExitCodeProcess`'s `*mut DWORD`,
   `bufsize` into `SearchPath`'s `DWORD`, and the `(DWORD *)` cast C 590 makes.  C's two chained
   assignments (`remlen = maxlen = …`, C 881/926; `buf = p = alloca (…)`, C 883; `cmdline = p =
   alloca (…)`, C 928) are each split into their two assignments in C's evaluation order, because
   Rust has no assignment expression.
8. **C's `switch`es are the port's `match`es** (PORTING.md 2.2): C 441-485's `switch (state)` with
   its two inner `switch (c)`es, and C 524's `switch (event)`.  C's character labels (`case '"'`,
   `case '^'`, the nine metacharacters) cannot be Rust patterns, so those two matches read
   `match c as u8` with byte-literal arms (the `src/lread.rs` 4098 cast-scrutinee precedent — the
   `src/android.rs` 1409 note is about the other way out, a named constant for a *name* C has); the
   outer `switch (state)` matches C 433-437's anonymous enum, which the port names `State` while
   keeping C's three member names (the `src/sort.rs` 659 `enum Label` shape).  C's two `switch`es
   have no `default`, so `console_event_handler`'s has the empty `_ => {}` arm (the
   `src/w32console.rs` 591-616 reading of the same shape).
9. **`console_event_handler` is `extern "system"`, not `extern "C"`.**  C's own declaration and
   definition (C 519/521) carry no `WINAPI`, but C 633's cast `(PHANDLER_ROUTINE)
   console_event_handler` asserts the callback type, and Rust has no cast into a function-pointer
   type (`nt/addpm.rs` 66-72's reading of the same cast), so the definition must carry the
   convention the pointer type has.  The port's definition is therefore the one place the two C
   spellings are reconciled; on a 32-bit Windows target that is the difference between `__cdecl`
   and `__stdcall`, and the port follows the type C's cast names.
10. **C's two `#if 0` regions inside functions are comments** (P-08, the `src/cm.rs` 86 and
    `src/w32uniscribe.rs` 1060-1065 readings): C 497-510's `_argv`/`_argc`/`setup_argv` block, C
    543-550's `default:` arm, and C 800-809's argument-collecting loop.  C 810's `#else` arm
    (C 811-813) is live and is plain code; C 544's `cmdproxy: received %d event` string is inside
    the dead arm and stays in the reference only.
11. **Locals C leaves uninitialised are zero-initialised** (PORTING.md 1.5) and each one is named in
    a comment at its declaration: `bytes_written`, `dummy`, `i`, `p`/`path`/`fname`, `c`, `s`,
    `pdot`/`qdot`, `need_shell`, `cmdline`, `progname`, `envsize`, `pass_through_args`,
    `num_pass_through_args`, `status`, `args`, `maxlen`/`remlen`/`run_command_dot_com`/`buf`, and
    `rc` (which `spawn` writes through `&mut rc`).  `sec_attrs` and `start` are
    `core::mem::zeroed ()` like the two `nt/runemacs.rs` 56-58 records.
12. **C 633's cast is a binding.**  `SetConsoleCtrlHandler ((PHANDLER_ROUTINE) console_event_handler,
    TRUE)` becomes one extra statement, `let handler: PHANDLER_ROUTINE = Some (console_event_handler);`
    (Deviation 9; the `nt/addpm.rs` deviation-8 shape).
13. **C 844's label and C 959's backward `goto`** are the `'pass_to_shell: loop` at C 844 and its
    `continue` (PORTING.md 2.2: a backward `goto` must not become an early `return`, and the loop's
    tail keeps C 962-964's `fail` + `return 0`), and C 652-658's `*progname = '\0'` /
    `*progname = '\\'` pair keeps C's order around the `SetCurrentDirectory` call.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **One feature registration**: `xtask/Cargo.toml` has no `ucrt`, which C 49's
   `#ifdef _UCRT` needs (the `old_paths` row the `nt/addpm.md` round owes, same class).  Until it is
   registered, `rustc` warns `unexpected_cfgs` for that arm.
2. **One tolerance row**: `docs/ports/line-drift-tolerance.tsv` needs a `cmdproxy` row of **0.39**,
   basis **total** — C 965 vs the port's 1337 (138.5%; the measured value + 0.01, the `addpm`/
   `runemacs` rows' convention).  The unit is long because it declares its whole `<windows.h>`/CRT
   surface (typedefs, three records, ten constants, nineteen entry points, each with its own `/// C:`
   line — no mirror carries `<windows.h>` on this machine, P-88 clause 1) and carries both arms of
   every `#if` (P-86 clause 1).  That file is not in this lane's write set, so `extract-abi.py
   --check` stays `FAIL` with exactly the one row Evidence ④ records.
3. **`docs/baseline/port-lines-pending.tsv`**: this file's row (`nt/cmdproxy.c 965 3 96
   known-unported target (plan batch F)`) is stale now that the port is in; regenerate with
   `tools/check-port-lines.py --write-pending`.  Not this lane's write set.
4. **`docs/port-map.md` line 299's third column** (the ratchet's `3`) is stale for the same reason.
5. **The layout assertions and the measured values for the three records and the ten constants**,
   once a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's readings;
   `x86_64-w64-mingw32-gcc` is absent here).  Until then every one of them is text (Deviation 2).
6. **`docs/status.md` and `docs/baseline/feature-coverage.tsv`**: the integrator's regeneration and
   running-log entry for this revision (the `docs/ports/w32console.md` Owed 9 row).

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, console handle, child process or command shell was ever involved.  There is
  no behavioural test, no differential run, no ERT and no device evidence.  In particular the `/c`
  quoting arithmetic of `main` (C 866-944) is transcribed as text, not exercised.
- **The arm reading is not a Windows compile.**  This machine has no `<windows.h>`, so Evidence ② is
  a `rustc` reading of the *materialized mirror graph* under three `--cfg`s; it says the ported
  statements and the unit's own declared surface agree with each other, **not** that either agrees
  with Microsoft's headers, not that the `ucrt` arm compiles, and not that the unit links.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left and it is the integrator's tolerance row (Owed 2).  Nothing in this ledger claims the
  file's checklist is non-empty (P-86 clause 2) or that the port is complete.
- The warning count (4, all `unused doc comment`) is recorded as a reading; no claim is made that the
  citations are documentation.
- The `## Deviations` 6 rows record a residual: with `switch`/`case`/`default` exempted from the
  token check, a deleted arm of those two switches would be invisible to it.  The arms are compared
  by reading, not by a gate.
