# emacs/nt/runemacs.c (270 lines)

# ----------------------------------------------------------------------
## Decisions

- **The file is a P-86 unit end to end.**  The frozen configuration builds no `nt/` object at all
  (`docs/baseline/frozen-build-sources.txt` records no `nt/runemacs` object for this target;
  `emacs/nt/Makefile.in`'s `OTHERS` builds it only for the Windows port), so `extract-abi.py`'s
  checklist is *empty* (P-86 ②, clause ⑩: an empty checklist is not a reading).  The **build-level
  file gate** is therefore `feature = "windowsnt"`, on every item *and on the include graph*
  (P-86 ⑧) - which is why the frozen `build-tu` produces a 336-byte empty object - and the whole
  text is carried rather than summarised.
- **P-86 ① supersedes this file's earlier note-only treatment.**  The previous ledger recorded the
  port as "an empty unit with this note" under docs/status.md 3.31 (`docs/status.md` 3.285's ratchet
  listed the file as `known-unported target (plan batch F)`); P-86 (2026-10-05, "规则，立即生效")
  rules that for this class the arms are carried, and this file's 270 lines are now transcribed in
  C's order.
- **C 94's `#ifdef CHOOSE_NEWEST_EXE` is `feature = "choose_newest_exe"`.**  C 41's `#define` is
  *commented out*, so the frozen side is C 124's `#else` arm; both arms are carried (P-86 ①), and
  the feature is registered in `femacs/xtask/Cargo.toml`.
- **C 46's `<windows.h>` and C 48's `<malloc.h>` have no mirror** - there is no Windows SDK and no
  MSVC CRT on this machine (`femacs/src/w32proc.rs`'s acquisition note: no `windows.h`/`winnt.h`
  anywhere, no mingw-w64) - so the unit declares the whole surface it uses at C 46's position
  (P-88 ①, the `src/w32*.rs` practice): the type/handle typedefs, `STARTUPINFO`/
  `SECURITY_ATTRIBUTES`/`PROCESS_INFORMATION`/`OSVERSIONINFO`/`FILETIME`/`WIN32_FIND_DATA`, the
  `STARTF_*`/`SW_*`/priority-class/`MB_*`/`IDOK`/`INFINITE` constants and the fourteen entry points,
  each with its own `/// C:` line.  **Every constant value is a placeholder** and the record layouts
  are unmeasured (P-88 ②: no numbers from memory) - see Owed 2.
- `alloca` (C 78/99) is P-05's `sa_stack_alloc` (`femacs/src/alloc.rs` 1039); C's `max` (C 82) is
  `<windows.h>`'s `<windef.h>` macro and `ZeroMemory` (C 250) the SDK's `memset` spelling, so the
  unit defines both macros with the SDK's own `#define` quoted (the `src/w32.rs` 325-328 shape).
- **A gap in `tools/extract-abi.py` was found here and fixed (status 3.468).**  The
  "non-`static` C function must carry `#[unsafe(no_mangle)]`" rule's `defined_name_after_attribute`
  accepted only `extern "C"`; `WinMain` is the first non-static **WINAPI** definition in the tree,
  which the port spells `extern "system"` (PORTING.md §5.1), so the rule reported a correct
  definition as missing its attribute.  The pattern now takes `extern "C"`/`"system"`/`"stdcall"`
  (negative control: dropping the attribute brings the row back).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (`/* Run Emacs...` + the GPL text) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 21-26 | the "Simple program to start Emacs with its console window hidden" note | carried as the port's header prose |
| 28-39 | the `CHOOSE_NEWEST_EXE` explanation (what the hack is for, and that a newer `emacs*.exe` gets picked) | carried as the port's header prose + the feature's comment in `Cargo.toml` |
| 41 | `/* #define CHOOSE_NEWEST_EXE */` (commented out) | dropped as a define; the *arm* is `feature = "choose_newest_exe"` |
| 43 | `#define DEFER_MS_W32_H` | dropped (Deviation 1) |
| 44 | `#include <config.h>` | ungated `include!` |
| 46-48 | `#include <windows.h>` / `<string.h>` / `<malloc.h>` | file gate; `<windows.h>`'s surface is declared here (UNVERIFIED), `<string.h>` is `lib/include/string.rs`, `<malloc.h>` contributes only `alloca` |
| 50-51 | `static void set_user_model_id (void);` and `static int ensure_unicows_dll (void);` | dropped (PORTING.md 0.1-12, forward declarations); the definitions follow `WinMain` |
| 53-203 | `WinMain` (`int WINAPI`): the `ensure_unicows_dll`/`set_user_model_id` calls, `GetModuleFileName`, the `alloca`d command line, the `nt`→`src` fix-up, both `CHOOSE_NEWEST_EXE` arms, the `-wait`/`-high`/`-low` scan, the `--iconic`/`--maximized` pass-through, the `emacs_dir` export, `CreateProcess` and the wait, and C 200-202's `error:` label | file gate; C 94-123 `choose_newest_exe`, C 124-125 `not (choose_newest_exe)` |
| 205-231 | `set_user_model_id`: `LoadLibrary ("shell32.dll")`, `GetProcAddress ("SetCurrentProcessExplicitAppUserModelID")`, the AppUserModelID call (C 206's own wide-string literal is the reference's id; the port carries its own, status 3.503) and `FreeLibrary` | file gate |
| 233-270 | `ensure_unicows_dll`: `ZeroMemory`+`GetVersionEx`, the `VER_PLATFORM_WIN32_WINDOWS` arm, `LoadLibrary ("Unicows.dll")`, the `MessageBox` and its one-arm `switch`, `FreeLibrary`, and the two `return 1`s | file gate |

## Evidence

- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-nl cargo run -q -p xtask -- build-tu nt/runemacs.c` ->
  `built nt/runemacs.rs -> ../build/femacs/runemacs.o`, **exit 0**, `build/femacs/runemacs.o` =
  **336 bytes** (the empty object a file-gated unit produces; the reference has *no* object at all,
  since `nt/` is not built on this target).
- `python3 tools/extract-abi.py --check emacs/nt/runemacs.c femacs/nt/runemacs.rs` -> two rows:
  `- function ensure_unicows_dll does not mention case, default, switch` (owed to
  `docs/ports/macro-body-notes.tsv`, written in 3.468) and
  `- line count drifted 141.5%: C 270 vs Rust 652` (the tolerance row, 3.468).  **No `missing`
  rows**: the checklist is empty (P-86 ⑩).
- `python3 tools/check-dead-file-coverage.py` -> `docs/ports/runemacs.md: N range row(s) cover
  nt/runemacs.c (270 lines; …)`, 0 failures (this ledger is bound by its `## C line ranges`
  section).
- `python3 tools/check-port-lines.py` -> the file's ratchet row (`nt/runemacs.c 270 3 27`) is gone
  from `docs/baseline/port-lines-pending.tsv` (removed in 3.468, exactly what `--write-pending`
  would do for this file), and the reading moves `69 files / 88,245 lines` -> `68 / 87,975`.
- The negative control for the tool fix: removing `#[unsafe(no_mangle)]` from `WinMain` makes
  `--check` print `function WinMain is not static in C but has no #[unsafe(no_mangle)]` again
  (measured on the pre-3.468 rule and re-measured after the patch).

## Deviations

1. **`DEFER_MS_W32_H` (C 43) is dropped.**  It exists so that `w32.h` (which several `nt/` units
   include before `<windows.h>`) does not pull `<windows.h>` in twice; this unit includes
   `<windows.h>` itself and no `w32.h` is in its graph, so the guard has no site.
2. **C 50-51's forward declarations are dropped** (PORTING.md 0.1-12) - the two `static` helpers are
   defined below `WinMain`, which is enough in Rust.
3. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `STARTUPINFO start`,
   `SECURITY_ATTRIBUTES sec_attrs` and `PROCESS_INFORMATION child` (C leaves them alone until C 137's
   `ZeroMemory`, C 152-154's field-by-field fill and `CreateProcess`'s out-parameter write),
   `WIN32_FIND_DATA wfd`, `OSVERSIONINFO os_ver` (C 250's `ZeroMemory` follows) and `HANDLE fh`.
4. **`*p++ = '\\';` (C 121) and `while (*++cmdline == ' ')` (C 145) are two statements each** - Rust
   has no postfix/prefix increment (the `src/sfnt.rs` reading of the same idioms; the second one is
   a `loop` whose first statement is the step, the test following it).
5. **C 108-122's `do { … } while (FindNextFile (fh, &wfd));` is a `loop` with the test at the end**
   (PORTING.md 2.2), so the body runs once before the first `FindNextFile` exactly as C's does.
6. **C 190's `error:` label is a labelled block** (`'error: { … }` with `break 'error` for each
   `goto`, PORTING.md 2.2); the block's tail is C 200-202's `MessageBox` and `return 1`.
7. **C 206's AppUserModelID is a `[u16; 7]` of code units** (`femacs`) with the trailing `0x0000` -
   the port carries its own name, not the reference's (C's own literal is in `nt/runemacs.c` 206;
   status 3.503), and Rust has no wide-string literal (the `src/w32fns.rs` 391 shape).
8. **`static const char iconic_opt[]`/`maximized_opt[]` (C 65) are `&[u8]` slices without the NUL**,
   with `+ 1` where C's `sizeof` counts the terminator (the `xg_set_widget_bg` `format` reading),
   cast to `*const c_char` at the `strcat` calls.
9. **`GetProcAddress`'s `FARPROC` is `transmute`d to the C 197 function-pointer type** (C's
   `(void *)` cast; PORTING.md's function-pointer rule, the `src/w32.rs` `..._Proc` aliases).
10. **The SDK values and record layouts are placeholders.**  No Windows SDK/CRT/toolchain exists on
    this machine, so every constant is marked `// placeholder, not <header>'s value`, no
    `size_of`/`offset_of` assertion is written, and the whole section is `UNVERIFIED` (P-88 ②; the
    `src/w32*.rs` units' own ledgers record the same).  Where a landed unit already measured a value
    (`MAX_PATH` 260, `TRUE`/`FALSE`, `INVALID_HANDLE_VALUE`, `VER_PLATFORM_WIN32_WINDOWS`) the port
    cites that file and line instead of inventing one.
11. **`memset`/`strcat`/`stricmp`/`sa_stack_alloc` are declared at their use sites** (P-88 ①):
    `lib/include/string.rs` carries neither `memset` nor `strcat`, `stricmp` is the MSVC CRT's name
    for a case-insensitive compare, and `sa_stack_alloc` is the port's own allocator.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **The two manifest rows this unit needs** - the `ensure_unicows_dll` token row in
   `docs/ports/macro-body-notes.tsv` and this file's tolerance row in
   `docs/ports/line-drift-tolerance.tsv` (141.5%: the whole `<windows.h>` surface plus C's comment
   blocks; basis `total`).  ✅ **Both written in 3.468**, together with the `choose_newest_exe`
   feature registration and the removal of this file's `docs/baseline/port-lines-pending.tsv` row.
2. **The SDK values and record layouts** (Deviation 10): only a Windows host/toolchain can measure
   them - the fourteen entry points' calling conventions and the six records' sizes/offsets are
   unverified text today.  A machine with mingw-w64 or an MSVC SDK closes this by the same
   `spikes/inline-probe.sh` route the Unix hosts use.
3. **No arm flip is possible here**: with `windowsnt` on, the unit needs `<windows.h>` and the CRT,
   so the P-86 ③⑵ `text-only` status applies - the *frozen* object (336 bytes) and `rustc`'s
   syntax/name resolution are the whole reading.

## Not claimed

- **No run evidence of any kind**: nothing was built for Windows, nothing launched `emacs.exe`, no
  `CreateProcess`/`MessageBox`/`GetVersionEx` was executed, and the reference has no object for this
  file on this target either (so there is not even an object-level comparison).
- The single `build-tu` reading is an **empty** object: it says the gated text is syntactically
  valid Rust and resolves nothing, not that the Windows arm type-checks.  The 141.5% drift is a
  line-count reading, not text equality.
- The placeholders in Deviation 10 mean the port would misbehave on a real Windows host if any value
  is wrong; that is recorded as Owed 2 rather than hidden.
- The `git checkout --` accident during this unit's writing (the file was reverted to its 3-line
  stub and rebuilt from `build/femacs/units/runemacs/_root.rs`, xtask's materialised copy of the
  text that had just been compiled) is recorded in `docs/status.md` 3.468 with the evidence that the
  recovery is faithful: 652 lines, the same 336-byte object, and the same two `--check` rows.

## 2026-10-05: the AppUserModelID is the port's own name (3.503)

C 206 hands the Windows shell the *reference's* AppUserModelID; it is the id this program's windows
are grouped by, so the seventh half of PORTING.md 0.1-19 makes it this repository's own name
(`USER_MODEL_ID`, a `[u16; 7]` of the port's name plus C's `0x0000`, where C had a `[u16; 10]`).
C's own literal stays in `nt/runemacs.c` 206 and the deviation is item 7 of `## Deviations`.
Readings: `extract-abi --check emacs/nt/runemacs.c femacs/nt/runemacs.rs` -> **OK**, with the
`runemacs` row in `docs/ports/line-drift-tolerance.tsv` moved `1.43 -> 1.46` (C 270 vs the port's
659 = 144.1%; the unit grew 7 lines here, all of it prose around the constant).

## 2026-10-05: the `windowsnt` arm type-checks now (3.509)

This unit's ledger carried "cannot flip the arm" in Owed 3, so nothing had ever read its Windows
side.  The `nt/addpm.rs` lane's flip of this same file read **three** errors; fixing them and
re-reading found four more that the first three had masked.  All seven are this unit's own text,
and the arm now reads **`exit 0`, 0 error, 86,456-byte rlib**:

1. `memset` was declared here *and* by `lib/include/string.rs` 24, which this unit includes at
   C 47 - `E0428`.  The local copy is gone; the mirror's is the one C's `ZeroMemory`/`memset`
   calls resolve to.
2. `if !ensure_unicows_dll ()` - C's `!` is on an `int`-returning helper, and Rust's `!` is
   `bool`-only (`E0308`); the port writes `ensure_unicows_dll () == 0`, which is C's value.
3. `memset (&raw mut start, …)` needed C's implicit `struct STARTUPINFO *` → `void *` as
   `.cast ()` (P-72 rule 3, use-site).
4. `cmdline` is C 53-54's fourth parameter and C 127-146 **reassigns** it (`-wait`/`-high`/`-low`
   scan), so the binding is `mut` - four `E0384`s without it.

Evidence: `rustc --edition 2024 --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"
nt/runemacs.rs` -> exit 0, 0 error, 86,456 bytes; the mutation reading (P-86 (7)) injects
`let _mutation: c_int = c"injected".as_ptr ();` into `WinMain` and the same command reports
`1 x E0308` at that line, so the zero is a live reading; the frozen arm is unchanged
(`build-tu nt/runemacs.c` -> 336-byte empty object) and `extract-abi --check` stays **OK**
(the unit is 660 lines; the `runemacs` drift row of 1.46 still covers 144.4%).
