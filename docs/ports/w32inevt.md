# emacs/src/w32inevt.c (862 lines) — whole file, C 1-862

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32inevt.o` is added to
> `W32_OBJ` by `configure.ac` 3051 — the `else` arm of the Cygwin split inside the `HAVE_W32` test
> (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` — and `docs/baseline/frozen-build-sources.txt`
> lists no `w32inevt.o` for this target, so `tools/extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this ledger is that clause's substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `src/w32inevt.rs` (**1493 lines**,
> md5 `1ae106d3cc5f06097ffb003bdd77c4df`), carrying **C 1-862 with both arms of every `#if`**.  The
> reference `emacs/src/w32inevt.c` is md5 `faa680644b22c03590fdf67e6a126906` (unchanged; `emacs/` is
> read-only) and its `docs/port-map.md` 577 row pairs `src/w32inevt.c` 862 with
> `src/w32inevt.rs` (the port's own count in that table is the integrator's regeneration —
> Owed 7).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w32inevt.md` (and the
> 16-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object)
> is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32console.md`, `docs/ports/w32menu.md` and `docs/ports/w32term.md` record for their
> own pairs.
>
> **The file gate is `windowsnt`, and the arm set is `windowsnt` + `have_ntgui` +
> `have_window_system`.**  `HAVE_NTGUI` (3024) is on for *both* the Cygwin and the native Windows GUI
> build, while `w32inevt.o` joins `W32_OBJ` only in the non-Cygwin arm (3051) — the build whose
> `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The units of that same arm already landed with
> `windowsnt` as their file gate (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`,
> `src/w32console.rs`), so this one does too; `have_ntgui` and `have_window_system` are set in the
> arm reading because the mirrors this unit reaches (`w32term.rs`, `frame.rs`, `dispextern.rs`)
> spell their platform arms with them.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wincon.h`), so there is **no
> `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
> measurement: every record, every value and every prototype of the `<windows.h>` stand-in section is
> `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are owed (Owed
> 4).  What the round does carry is the frozen `build-tu` reading, the machine-checked range table,
> the `#if` verdicts, a per-item C citation, the per-file error census of the three-cfg arm *and* of
> the same arm with `have_w32notify` on, and a **mutation test** over this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3022-3051; the native-Windows `else` of the Cygwin split) | `feature = "windowsnt"` (on every item *and* on C 26-44's include graph) | the native MS Windows GUI build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not the Cygwin GUI build |
| C 30 `#ifndef MOUSE_MOVED` and C 33 `#ifndef MOUSE_HWHEELED` | the file gate; "define it if the SDK has not", so the constant carries C's value with **no** `cfg` (the `include/w32term.rs` reading of its nine guards) | the SDK in either case: both are `<wincon.h>`'s, so the guard body is the live definition |
| C 120 `#if 0` (the `is_dead_key` helper) | dead in every configuration; C's text is kept as a comment | never |
| C 644 `#if HAVE_W32NOTIFY` / C 733 `#else` / C 739 `#endif` | `feature = "have_w32notify"` for the first arm, `not (feature = "have_w32notify")` for the second; **both arms carried** | `HAVE_W32NOTIFY` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1675, so the `#else` arm (`handle_file_notifications` returning 0) is the frozen one |

`tools/check-cond-eval.py src/w32inevt.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ④): C 30, 33, 120 and 644 — those four rows are the whole conditional text of
the file (the `#endifs` at 32/35/130/739 and the `#else` at 733 are not conditions).  `HAVE_NTGUI`,
`WINDOWSNT` and `HAVE_W32NOTIFY` are all `undef` in the frozen macro table, and C's own two macros
(`MOUSE_MOVED`, `MOUSE_HWHEELED`) are carried as constants of the same names (C 30-35).

## C line ranges

`docs/port-map.md` pairs **two** references with this ledger — 577 `src/w32inevt.c` (862 lines) and
578 `src/w32inevt.h` (32 lines, the mirror's header) — and `tools/check-dead-file-coverage.py` binds
both rows here, so the table below is **contiguous** (each row spans its block *and* the blank
separator that follows it, as `docs/ports/msdos.md`'s and `docs/ports/w32term.md`'s rows do for the
same reason).  The "C" column is `src/w32inevt.c`'s numbering; the header's 32 lines fall inside it.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-25 | the two authors and the note that the file came from ntkbd.c | carried as C's own comment |
| 26-29 | `#include <config.h>`, `<stdio.h>`, `<windows.h>` | file gate; `<windows.h>` is the in-place stand-in section (P-88 ①, UNVERIFIED) |
| 30-36 | `#ifndef MOUSE_MOVED` / `#define MOUSE_MOVED 1` / `#endif` and the `MOUSE_HWHEELED 8` twin | file gate (guards carried as constants); `MOUSE_WHEELED`/`DOUBLE_CLICK` join them (Deviations 4) |
| 37-45 | `lisp.h`, `keyboard.h`, `frame.h`, `blockinput.h`, `termchar.h`, `w32term.h`, `w32inevt.h`, `w32common.h` | file gate |
| 46-56 | `keyboard_handle` (C 47), the last-mouse-motion banner (C 49-51), `map_keypad_keys`/`w32_key_to_modifier` (C 53-55) | file gate; the four `extern` declarations are dropped (0.1-12) and re-declared at their use sites (Deviations 2, 12) |
| 57-64 | `EVENT_QUEUE_SIZE`, `event_queue`, `queue_ptr`, `queue_end`, `dbcs_lead` | file gate |
| 65-73 | `w32_read_console_input` (C's `static inline`) | file gate |
| 74-77 | `faked_key` (`extern` + definition) | file gate; the declaration is dropped, the definition keeps C's linkage |
| 78-105 | `fill_queue` | file gate |
| 106-115 | `get_frame` | file gate |
| 116-136 | the modifier-key banner, C 120-130's `#if 0` `is_dead_key`, the codepage banner | file gate; C 120-130 is dead text kept as a comment |
| 137-410 | `key_event` | file gate; C 177-274's `switch`/`goto` is the flag rewrite (Deviations 5) |
| 411-436 | `w32_console_mouse_position` | file gate |
| 437-451 | `mouse_moved_to` | file gate |
| 452-469 | the console button-order comment, `NUM_TRANSLATED_MOUSE_BUTTONS`, `emacs_button_translation` | file gate |
| 470-615 | `do_mouse_event` | file gate; C 487-613's `switch` is a `match` (Deviations 5) |
| 616-624 | `resize_event` | file gate |
| 625-643 | `maybe_generate_resize_event` | file gate |
| 644-740 | `handle_file_notifications`, C 644's `#if HAVE_W32NOTIFY` arm and C 733's `#else` twin | `feature = "have_w32notify"` / `not (feature = "have_w32notify")` |
| 741-772 | the console-input overview comment | carried as C's own comment |
| 773-862 | `w32_console_read_socket` | file gate; C 810-847's `switch` is a `match` (Deviations 5) |

## Evidence` readings and the `## Deviations`.  Port: `src/w32inevt.rs` (**1493 lines**,
> md5 `1ae106d3cc5f06097ffb003bdd77c4df`), carrying **C 1-862 with both arms of every `#if`**.  The
> reference `emacs/src/w32inevt.c` is md5 `faa680644b22c03590fdf67e6a126906` (unchanged; `emacs/` is
> read-only) and its `docs/port-map.md` 577 row pairs `src/w32inevt.c` 862 with
> `src/w32inevt.rs` (the port's own count in that table is the integrator's regeneration —
> Owed 7).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w32inevt.md` (and the
> 16-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object)
> is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32console.md`, `docs/ports/w32menu.md` and `docs/ports/w32term.md` record for their
> own pairs.
>
> **The file gate is `windowsnt`, and the arm set is `windowsnt` + `have_ntgui` +
> `have_window_system`.**  `HAVE_NTGUI` (3024) is on for *both* the Cygwin and the native Windows GUI
> build, while `w32inevt.o` joins `W32_OBJ` only in the non-Cygwin arm (3051) — the build whose
> `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The units of that same arm already landed with
> `windowsnt` as their file gate (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`,
> `src/w32console.rs`), so this one does too; `have_ntgui` and `have_window_system` are set in the
> arm reading because the mirrors this unit reaches (`w32term.rs`, `frame.rs`, `dispextern.rs`)
> spell their platform arms with them.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wincon.h`), so there is **no
> `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
> measurement: every record, every value and every prototype of the `<windows.h>` stand-in section is
> `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are owed (Owed
> 4).  What the round does carry is the frozen `build-tu` reading, the machine-checked range table,
> the `#if` verdicts, a per-item C citation, the per-file error census of the three-cfg arm *and* of
> the same arm with `have_w32notify` on, and a **mutation test** over this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3022-3051; the native-Windows `else` of the Cygwin split) | `feature = "windowsnt"` (on every item *and* on C 26-44's include graph) | the native MS Windows GUI build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not the Cygwin GUI build |
| C 30 `#ifndef MOUSE_MOVED` and C 33 `#ifndef MOUSE_HWHEELED` | the file gate; "define it if the SDK has not", so the constant carries C's value with **no** `cfg` (the `include/w32term.rs` reading of its nine guards) | the SDK in either case: both are `<wincon.h>`'s, so the guard body is the live definition |
| C 120 `#if 0` (the `is_dead_key` helper) | dead in every configuration; C's text is kept as a comment | never |
| C 644 `#if HAVE_W32NOTIFY` / C 733 `#else` / C 739 `#endif` | `feature = "have_w32notify"` for the first arm, `not (feature = "have_w32notify")` for the second; **both arms carried** | `HAVE_W32NOTIFY` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1675, so the `#else` arm (`handle_file_notifications` returning 0) is the frozen one |

`tools/check-cond-eval.py src/w32inevt.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ④): C 30, 33, 120 and 644 — those four rows are the whole conditional text of
the file (the `#endifs` at 32/35/130/739 and the `#else` at 733 are not conditions).  `HAVE_NTGUI`,
`WINDOWSNT` and `HAVE_W32NOTIFY` are all `undef` in the frozen macro table, and C's own two macros
(`MOUSE_MOVED`, `MOUSE_HWHEELED`) are carried as constants of the same names (C 30-35).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-23 | the two authors and the note that the file came from ntkbd.c | carried as C's own comment |
| 26-28 | `#include <config.h>`, `<stdio.h>`, `<windows.h>` | file gate; `<windows.h>` is the in-place stand-in section (P-88 ①, UNVERIFIED) |
| 30-35 | `#ifndef MOUSE_MOVED` / `#define MOUSE_MOVED 1` / `#endif` and the `MOUSE_HWHEELED 8` twin | file gate (guards carried as constants); `MOUSE_WHEELED`/`DOUBLE_CLICK` join them (Deviations 4) |
| 37-44 | `lisp.h`, `keyboard.h`, `frame.h`, `blockinput.h`, `termchar.h`, `w32term.h`, `w32inevt.h`, `w32common.h` | file gate |
| 46-55 | `keyboard_handle` (C 47), the last-mouse-motion banner (C 49-51), `map_keypad_keys`/`w32_key_to_modifier` (C 53-55) | file gate; the four `extern` declarations are dropped (0.1-12) and re-declared at their use sites (Deviations 2, 12) |
| 57-63 | `EVENT_QUEUE_SIZE`, `event_queue`, `queue_ptr`, `queue_end`, `dbcs_lead` | file gate |
| 65-72 | `w32_read_console_input` (C's `static inline`) | file gate |
| 74-76 | `faked_key` (`extern` + definition) | file gate; the declaration is dropped, the definition keeps C's linkage |
| 78-104 | `fill_queue` | file gate |
| 106-114 | `get_frame` | file gate |
| 116-134 | the modifier-key banner, C 120-130's `#if 0` `is_dead_key`, the codepage banner | file gate; C 120-130 is dead text kept as a comment |
| 137-409 | `key_event` | file gate; C 177-274's `switch`/`goto` is the flag rewrite (Deviations 5) |
| 411-435 | `w32_console_mouse_position` | file gate |
| 437-450 | `mouse_moved_to` | file gate |
| 452-468 | the console button-order comment, `NUM_TRANSLATED_MOUSE_BUTTONS`, `emacs_button_translation` | file gate |
| 470-614 | `do_mouse_event` | file gate; C 487-613's `switch` is a `match` (Deviations 5) |
| 616-623 | `resize_event` | file gate |
| 625-642 | `maybe_generate_resize_event` | file gate |
| 644-739 | `handle_file_notifications`, C 644's `#if HAVE_W32NOTIFY` arm and C 733's `#else` twin | `feature = "have_w32notify"` / `not (feature = "have_w32notify")` |
| 741-771 | the console-input overview comment | carried as C's own comment |
| 773-862 | `w32_console_read_socket` | file gate; C 810-847's `switch` is a `match` (Deviations 5) |

Between-row lines: 24-25, 29, 36, 45, 56, 64, 73, 77, 105, 115, 135-136, 410, 436, 451, 469, 615, 624,
643, 740 and 772 are blank, which the coverage gate leaves out (its own blank-line rule).

## Evidence

**① The frozen configuration (`build-tu`), before and after.**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32i-lane/target cargo run -q -p xtask -- build-tu
src/w32inevt.c` → `built src/w32inevt.rs -> ../build/femacs/w32inevt.o`, **exit 0**, and
`build/femacs/w32inevt.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` — the *same*
reading the inherited 16-line unit took before the port was written (**five of this round's six runs
read 336 bytes and that md5**, including the runs after every edit, because every item and every
`include!` sits behind the file gate: P-86 ⑧ predicts exactly this, and an ungated `include!` would
break this compile at once).  The sixth run is the one that failed: the first arm-free compile of the
*written* unit read `E0425: cannot find type c_int` on two `const`s that were not yet gated (fixed by
gating them) — what that door measures on a P-86 file is that the gated text still parses, and a
gate-less item is outside it.

**② The arm flip, as a per-file error census (P-86 ③⑴/⑦).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32i-lane/target cargo run -q -p xtask -- probe src/w32inevt.rs
/tmp/w32i-lane/final.rlib --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"'
--cfg 'feature="have_ntgui"' --cfg 'feature="have_window_system"'` → **exit 1, 9 errors, 30
warnings**, and **not one of the errors is anchored in this unit's lines**:

| file the error is anchored in | count | errors |
| --- | ---: | --- |
| `include/w32term.rs` | 5 | `E0425 cannot find type` — `RECT` (its 1421/1422/1428), `HDC` (1436), `COLORREF` (1445) |
| `include/w32inevt.rs` | 2 | `E0428 the name ... is defined multiple times` — `w32_console_read_socket` (its 27) and `w32_console_mouse_position` (28), both names **this unit defines** |
| `include/w32term.rs` | 1 | `E0428` — `handle_file_notifications` (its 1121), the third name this unit defines |
| `include/frame.rs` | 1 | that mirror's deliberate `have_ntgui` `compile_error!` ("the HAVE_NTGUI arm of `struct frame`'s flag run is unmeasured on this machine") |

So the unit's own text contributes **0 errors** to that reading; the three `E0428`s are the class
`docs/ports/w32console.md` Owed 1 already records (a *mirror* file is the primary span of each, and
the fix is the `include/w32.rs` module + glob-import shape), and the six other errors belong to the
mirrors' own owed lists.  Owed 1/2 name the rows.

**②a The arm the three `--cfg`s leave out, measured.**  The same command with
`--cfg 'feature="have_w32notify"'` added — i.e. C 644's arm, the one carrying the statements
this port transcribes into `enter_crit!`/`leave_crit!`, the notification unlink, the
`FILE_NOTIFY_INFORMATION` walk and the three Lisp helpers — reads **exit 1, 9 errors, 30 warnings**:
the *same nine*, in the same four mirror rows.  The notify arm therefore adds no finding of its own
(the `c_variadic`-style toolchain surprises are what ③'s note records; `src/w32proc.rs`'s
`CRITICAL_SECTION` copy is not on this unit's graph, so the stand-in declares it, Deviations 3).

**③ The "0 in the unit" reading is not vacuous, measured two ways.**
*(a) The door's own earlier readings.*  The same `probe` door reported **61 errors** for the first
draft, **52 of them anchored in the unit's own lines** (twelve `cannot find macro input_event_bits`,
four `cannot find macro frame_bits`, twenty-five `E0609 no field …` — twenty-four on
`*mut input_event` and one on the `mouse_highlight` chain — nine `E0308 mismatched types`, one
`E0408` and one `E0606`), then **13** (four left, all `frame_bits`), then **9** with none — so the
door demonstrably type-checks this unit's bodies, and the last reading's zero is a measurement
rather than a phase that never ran (P-86 ⑦'s trap).
*(b) A mutation test on the materialized text.*  `build/femacs/units/w32inevt/` (from `build-tu`)
was copied to `/tmp/w32i-lane/base` and `/tmp/w32i-lane/mut`, and one line was injected at the top of
`key_event`'s body in the copy: `let _mutant: c_int = "w32inevt_arm_probe_no_such_type";`.  Both
copies were compiled with the **pinned toolchain** (`rust-toolchain.toml` →
`nightly-2026-10-01-aarch64-apple-darwin`) and the same four `rustc` flags as the door (edition
2024, `-O`, `--crate-type=lib`, `-C panic=abort`) under the three cfgs: **base 9 errors / 30
warnings**, **mutant 10 errors / 30 warnings**, the extra one `E0308 mismatched types` anchored at
the injected line in `_root.rs` (its line 669) — one more error than the base and none elsewhere.  *(Toolchain
note: a bare `rustc` from `PATH` is stable 1.98.1 here and reads **10** errors on the same text, the
extra one `E0658 c_variadic` on `include/lisp.rs` 3763; the doors and this harness use the pinned
nightly, which is the reading the tables above quote.)*

**④ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/w32inevt.c` →
`src/w32inevt.c: 4 conditions, 0 unknown, 0 disagreements [VERIFIED]` (C 30, 33, 120, 644), which is
why the frozen macro table decides this file's arms and why C 644's `#else` is the frozen one.

**⑤ The text gates.**  `python3 tools/extract-abi.py --check emacs/src/w32inevt.c
src/w32inevt.rs` → **one** finding and no other: `line count drifted 73.2%: C 862 vs Rust
1493`.  The three findings the first run carried were the three `switch`-body token rows of
`docs/ports/macro-body-notes.tsv` this round appended (Deviations 5 records why; the file is
append-only).  `python3 tools/check-declarations.py` → `0 failures` (17 allowed literals);
`python3 tools/check-mirror-dupes.py` → `17 ratchet row(s), 0 failure(s)` (the seven use-site
declarations of Deviations 12 add no hit: no mirror on this graph carries any of them);
`python3 tools/check-citations.py` → `0 drift, 0 range` (1645 external, 36 allowed);
`python3 tools/check-dead-file-coverage.py` → `0 failure(s)`; `python3 tools/check-ledger-ticks.py`
→ `0 failure(s)` (this ledger ticks no function: the file's checklist is the empty P-86 ② one).

**⑥ What the file is.**  C 139-409's `key_event` translates a console `KEY_EVENT_RECORD` into an
`input_event` (modifier/lock-key handling, the AltGr guard, the ASCII/Unicode/DBCS-codepage
fallbacks); C 470-614's `do_mouse_event` and C 616-642's two resize helpers do the same for
`MOUSE_EVENT_RECORD` and `WINDOW_BUFFER_SIZE_RECORD`; C 644-739's `handle_file_notifications`
drains the `HAVE_W32NOTIFY` set list; and C 773-862's `w32_console_read_socket` is the reader
`sys_select` wakes.  Nothing of it was executed anywhere (see `## Not claimed`).

## Deviations

1. **C 1-18's opening ownership/title block is dropped** (PORTING.md 0.1-19: the title names the
   reference product); the port's first line is the revision line, and C 19-23's two-author comment
   is carried as C's own words.
2. **C 46-55's four `extern` declarations are dropped (0.1-12) and the names that need a
   declaration stand in the use-site section**: `keyboard_handle` (C 47, defined by
   `w32console.c` 78) and `map_keypad_keys` (C 54, defined by `w32fns.c`).  C 55's
   `w32_key_to_modifier` is dropped with **no** use site: C declares it and never calls it, which is
   the 0.1-12 rule for a forward declaration.  C 75's `extern int faked_key;` is this unit's own
   declaration of C 76's definition, dropped with the definition following in C's order.
3. **C 28's `<windows.h>` is an in-place stand-in section** (P-88 ①/P-90 ④): the eleven scalar
   typedefs (including `wchar_t`, the Windows 16-bit one the tree spells `c_ushort` —
   `include/w32.rs` 31's reading of the same type), the nine pointer typedefs plus `ULONG_PTR`,
   the thirteen records/unions (`COORD`, `SMALL_RECT`, `KEY_EVENT_RECORD` and its named `uChar` union,
   `MOUSE_EVENT_RECORD`, `WINDOW_BUFFER_SIZE_RECORD`, `MENU_EVENT_RECORD`, `FOCUS_EVENT_RECORD`,
   `INPUT_RECORD` and its named `Event` union, `CONSOLE_SCREEN_BUFFER_INFO`,
   `FILE_NOTIFY_INFORMATION`, `CRITICAL_SECTION`), the fourteen entry points and the thirty
   constants this file spells (the section's twenty-six and the four of C 30-35/C 546/C 571 that
   Deviations 4 records).  **Every one is UNVERIFIED** (no SDK on this machine) and Owed 4
   lists the layout assertions.  `CRITICAL_SECTION`, `LPCRITICAL_SECTION` and the
   `EnterCriticalSection`/`LeaveCriticalSection` pair are here because C 666/675's
   `enter_crit ()`/`leave_crit ()` expand through `include/w32term.rs` 1028-1044's pair.
4. **C 30-35's two `#ifndef` guards carry the constant with C's value and no `cfg`** (the "define it
   if the SDK has not" reading `include/w32term.rs` records for its own nine), and the two
   `<wincon.h>` `dwEventFlags` values the guards do **not** cover — C 546's `MOUSE_WHEELED` and
   C 571's `DOUBLE_CLICK` — stand in the same section so the two switch labels have their values
   (`DOUBLE_CLICK`'s case is the `_` arm, so that constant is carried without a Rust use).
5. **C's two `switch`es and the `goto` into `disable_lock_key`.**  C 177-274's switch is a `match`
   (Rust's `match` needs the `_ => {}` C's `switch` does not) and the three
   `goto disable_lock_key` jumps (C 224/229/234) are the `disable_lock_key` flag whose body C
   236-252 keeps once — no path falls *into* the label in C (the three lock-key cases jump and
   `VK_SCROLL` returns otherwise), so the body sits after the match with the same three entry
   states.  C 487-613's `switch (flags)` is a `match` whose `_` arm is C's
   `case DOUBLE_CLICK: default:` pair, and C 584's `for (i = 0; mask; i++, mask <<= 1)` is a `while`
   with `mask.wrapping_shl (1)` (PORTING.md 1.1: C's unsigned shift written explicitly).  The three
   rows of `docs/ports/macro-body-notes.tsv` appended this round record exactly the tokens those
   rewrites remove (`switch`, `case`, `default`, `goto`, `break`, the two ignored event labels and
   `true`); every other token of the three bodies is still compared.
6. **C's `emacs_ev->field` accesses go through the pointer** (`(*emacs_ev).field`): a Rust raw
   pointer has no field access, so all twenty-four of C 328-407's and C 558-610's member writes
   spell the dereference.  `input_event`'s two bit fields are the per-unit `input_event_bits!` macro
   (the `src/msdos.rs` 3424 copy — `macro_rules!` is textual and `src/keyboard.rs`'s copy belongs to
   that unit), `f->mouse_moved` is the per-unit `set_frame_flag1!`/`mouse_moved!` pair (the
   `src/msdos.rs` 681-697 shape; `include/frame.rs` 520-530 carries the shift but no accessor), and
   `mouse_face_hidden` is a third per-unit accessor (the `src/msdos.rs` 3434-3444 shape, over
   `include/dispextern.rs` 2127's `bits` and its `MOUSE_FACE_HIDDEN` mask).
7. **The fixnum setters take `EMACS_INT`**: `XSETINT!`/`XSETFASTINT!` expand to
   `make_fixnum`/`make_fixed_natnum`, whose parameter is `EMACS_INT`, so C's `int`/`SHORT` arguments
   carry `as EMACS_INT` (the `src/window.rs` 3449 reading) at C 430-431, 564-565 and 608-609.
8. **`offsetof`/`sizeof`/`wchar_t`.**  C 682's `offsetof (FILE_NOTIFY_INFORMATION, FileName)
   + sizeof (wchar_t)` is `core::mem::offset_of! (FILE_NOTIFY_INFORMATION, FileName)
   + core::mem::size_of::<wchar_t> ()`, and C 355's `wchar_t code` is the stand-in's `wchar_t`
   (`c_ushort`; Deviations 3).
9. **C 644-739's `DWORD` arithmetic keeps C's wrapping.**  C 723's
   `info_size -= fni->NextEntryOffset` is `wrapping_sub` (PORTING.md 1.1), and C 695/714-716's two
   consecutive `inev.arg` writes (`Fcons (…)` then `list3 (…)`) are kept as the two statements C
   has, in C's order.
10. **`lispy_function_keys` is declared with 256 slots at its use site.**  `keyboard.h` 525 declares
    an incomplete array and `src/keyboard.c` 5513's `HAVE_NTGUI` arm defines 256 of them, so the port
    declares those 256 rather than an incomplete array C 291 could not index (the `src/xwidget.rs`
    1233-1240 row for the X11 arm).  C 291's index is a `WORD`, so C reads past the definition for a
    code above 255; the port's `as usize` index is a bounds check instead of that read — the one
    place this unit's semantics is deliberately *not* C's UB (Owed 3 names the definition's missing
    `#[unsafe(no_mangle)]`).
11. **The two-hop cast at C 821-822** (`terminal->display_info.tty->mouse_highlight`): the union
    member is the mirror's untyped `*mut c_void` (P-72 rule 3), so the cast to
    `*mut tty_display_info` is explicit and C's grouping is written with the parentheses Rust's
    operator precedence needs.
12. **Seven readers no mirror on this unit's graph carries are declared at their use sites**
    (P-66/P-72 rule 4): `keyboard_handle`, `map_keypad_keys`, `change_frame_size` (`dispextern.h`
    3924), `clear_mouse_face` (`dispextern.h` 3691), `note_mouse_highlight` (`dispextern.h` 3688),
    `gen_help_event` (`keyboard.h` 513-514) and `lispy_function_keys` (`keyboard.h` 525).  Each keeps
    C's prototype in a `/// C:` line, and Owed 2 names the mirror rows that would retire them.
13. **C's comments are carried as prose** (C 19-23, 106-114, 116-118, 132-138, 137-138, 145/167-169/
    176/182-184/242-243/258-260/264-268/286/297-302/311-314/321-323/337/344-345/351-354/359-363/373/
    388/399/411/437/441/452-462/480-481/484/504/509-512/535-536/542-543/549-554/573-576/580-581/631/
    637-638/652-653/663-665/688-691/704-705/726/741-771/789-791/800-803/854-856), and C 120-130's
    `#if 0` text stands inside a comment (the `src/cm.rs` 86 reading).  C's own indentation and
    blank-line shape are not reproduced line for line, which is most of the 73.2% the drift row
    measures.
14. **C's `NULL`/`0`/`false` spellings follow the mirror's types**: `ns = NULL` is
    `std::ptr::null_mut ()`, C 507's `window_from_coordinates (f, mx, my, 0, 0, 0, 0)` is the
    mirror's `*mut c_void` frame plus `std::ptr::null_mut (), false, false, false` (its `part`
    parameter is a pointer and its three `bool`s), and C 692's `!NILP (obj) && CONSP (obj)` becomes
    `! NILP (obj) && CONSP (obj)` (the mirrors' own names).

## Owed

1. **Three mirror rows, one per `E0428` in Evidence ②.**  `include/w32inevt.rs` 27/28 declares
   `w32_console_read_socket` and `w32_console_mouse_position` in a **top-level**
   `unsafe extern "C"` block, and `include/w32term.rs` 1121 declares `handle_file_notifications` the
   same way; each is a name this unit *defines*, and the established fix is the one
   `include/w32.rs` already uses (`w32_get_internal_run_time` and `term_timers`/`init_timers`: the
   declaration moves into a module the mirror glob-imports, so the unit's definition shadows it).
   `include/w32inevt.rs` 19's `w32_console_unicode_input` is that same class for `src/w32console.rs`
   (which defines it) and is already `docs/ports/w32console.md` Owed 1.  Not this lane's write set.
2. **Six mirror rows for Deviations 12's use-site declarations**: `keyboard.h` 513-514
   (`gen_help_event`) and 525 (`lispy_function_keys`) → `include/keyboard.rs`; `dispextern.h` 3688/
   3691/3924 (`note_mouse_highlight`, `clear_mouse_face`, `change_frame_size`) →
   `include/dispextern.rs` (`dispextern_late_decls`, whose own 4039 comment records why those names
   could not land yet); and `keyboard_handle`/`map_keypad_keys` have no header declaration of their
   own in the reference (C repeats the `extern` in each `w32*.c`), so a shared home in
   `include/w32common.rs` or `include/w32term.rs` is the integrator's call.
3. **`src/keyboard.rs`'s `lispy_function_keys` definition is `static mut` with no
   `#[unsafe(no_mangle)]`** (its 5690 and 5957 arms), while C 5513's definition has external linkage
   and this unit reads it — a real Windows link needs that symbol (the same row `src/xwidget.rs`
   Owed 3 records).  Not this lane's file.
4. **The layout assertions and the measured values of the `<windows.h>` records and constants**
   (`COORD`, `SMALL_RECT`, the two `KEY_EVENT_RECORD` unions, `MOUSE_EVENT_RECORD`,
   `WINDOW_BUFFER_SIZE_RECORD`, `MENU_EVENT_RECORD`, `FOCUS_EVENT_RECORD`, `INPUT_RECORD`,
   `FILE_NOTIFY_INFORMATION`, `CONSOLE_SCREEN_BUFFER_INFO`, `CRITICAL_SECTION` and the thirty values), once a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's
   readings; `x86_64-w64-mingw32-gcc` is absent here).
5. **The five `E0425`s in `include/w32term.rs`** (`RECT` at 1421/1422/1428, `HDC` at 1436,
   `COLORREF` at 1445) and **`include/frame.rs`'s deliberate `have_ntgui` `compile_error!`** are the
   six other errors of Evidence ②; they belong with those two mirrors' existing owed lists, not to
   this file.
6. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32inevt`, tolerance ≥ **0.74**,
   basis `total`, reason from Deviations 3-14 (the stand-in section, the per-item gate and `/// C:`
   citations, the two per-unit accessor families and the comment prose).  That file is not in this
   lane's write set, so `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑤
   records.
7. **`docs/baseline/port-lines-pending.tsv`** (the stale `src/w32inevt.c` ratchet row),
   **`docs/port-map.md`** (the port's 16-line count), **`docs/baseline/feature-coverage.tsv`** and
   **`docs/status.md`**: the integrator's regeneration and running-log entry for this revision (the
   `docs/ports/w32console.md` Owed 8/9 rows).

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte object, unchanged by every edit), so this is a *text* port: nothing here
  has been executed, and no Windows target, console handle, `INPUT_RECORD`, `COORD`, codepage or
  event loop was involved.  There is no behavioural test, no differential run, no ERT and no device
  evidence.
- **The arm reading is not a Windows compile.**  This machine has no `<windows.h>`, so Evidence ② is
  a `rustc` reading of the *materialized mirror graph* under three `--cfg`s (plus ②a's fourth); all
  nine of its errors are in mirror copies and "0 in the unit" means only that no error's primary span
  is one of this unit's lines (Evidence ③ shows the reading is not vacuous) — **not** that the unit
  links, and not that the arm is a working Windows configuration.
- **The `<windows.h>` records and values stay unmeasured** (Owed 4): every one is `UNVERIFIED`,
  with the documented SDK text as its only source.
- **The `lispy_function_keys` bounds difference is a deliberate deviation** (Deviations 10), and
  `src/keyboard.rs`'s symbol is not linkable yet (Owed 3).
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 6).  Nothing in this ledger claims the
  file's checklist is non-empty (P-86 ②) or that the port is complete.
