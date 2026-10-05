# emacs/nt/addpm.c (347 lines)

# ----------------------------------------------------------------------
## Decisions

- **The file is a P-86 unit end to end.**  The frozen configuration builds no `nt/` object at all
  (`docs/baseline/frozen-build-sources.txt` records no `nt/addpm` object for this target;
  `emacs/nt/Makefile.in`'s rule builds it for the Windows port only), so `extract-abi.py`'s
  checklist is *empty* (P-86 ②, clause ⑩: an empty checklist is not a reading).  The **build-level
  file gate** is therefore `feature = "windowsnt"`, on every item *and on the include graph*
  (P-86 ⑧) - which is why the frozen `build-tu` produces a 336-byte empty object - and the whole
  text is carried rather than summarised.  The three declaration-only sections (`<windows.h>`,
  `<shlobj.h>`, `<ddeml.h>`) are the reason this file is far longer than its reference; the drift is
  requested as a tolerance row in `## Owed`.
- **Four sub-gates, all carried with both arms** (P-86 ①): C 45's `#ifndef MINGW_W64` (the frozen
  side is `not (feature = "mingw_w64")`; that name is already in `femacs/xtask/Cargo.toml`, the
  `w32fns.rs` 38-40 site), C 59's `#ifndef OLD_PATHS` and C 89/C 100's `#ifdef OLD_PATHS` /
  `#else` pair (both sides `feature = "old_paths"`), and C 47's inner `#ifdef _WIN32_IE`.
  `old_paths` is **not** in `Cargo.toml` yet (Owed 1).
- **C 55-57's three SDK headers have no mirror and no SDK on this machine.**  There is no Windows
  SDK, CRT or MinGW-w64 here (`femacs/src/w32proc.rs`'s acquisition note; the same reason
  `femacs/nt/runemacs.rs` declares its own surface), so the unit declares everything it names - the
  typedefs, `MAX_PATH` and the constant table, the registry/`CoInitialize`/`MessageBox`/
  `MultiByteToWideChar`/`SHGetSpecialFolderPath` entries, `IShellLinkA`/`IPersistFile` and their two
  vtables, the two `IID`s and the class id, the DDE handles and its eight entry points - at the
  `#include` line that would have declared it (P-88 ①), each with its own `/// C:` line.
  Flat cost: the port is 957 lines against the reference's 347.  **Every
  value, every vtable member order and every record layout is UNVERIFIED** (P-88 ②: no number from
  memory); Owed 2 names the host that measures them.
- **`COBJMACROS` is why the COM calls read the way they do.**  C 53 asks the SDK headers for the C
  object macros, so the four `IShellLinkA_*` and the two `IPersistFile_*` calls are macros and the
  port keeps each as a `macro_rules!` of the same name whose body is the header's own
  `(p)->lpVtbl->Method (p, …)` expansion - the `femacs/src/w32fns.rs` 92-95 reading of the same
  macro (that file declares the same `COBJMACROS` marker, as `()`, because its C spells it as a
  comment; this file's C 53 spells it `1`, so the const is `c_int` = 1 and the value compares).
- **C 266 and C 297 hand the shell the shortcut's description.**  That string is the program's
  statement of its *own* name, so the port passes this repository's own name there (`femacs`),
  PORTING.md 0.1-19's seventh half (status 3.503); the reference's own literal stays in
  `nt/addpm.c` 266/297 and is not carried into this tree.  Every other literal of this file is C's
  bytes, `REG_ROOT`/`REG_APP_PATH` and the Start Menu folder name included (allowlisted).
- **C 74-76's `DdeCommand` macro reaches the caller's `conversation`.**  A `macro_rules!` body
  cannot name its caller's locals (macro hygiene), so that local is an explicit second parameter and
  each of the three call sites passes it - the same limitation PORTING.md's P-22 note records for
  `marker.c`'s `CONSIDER` (Deviation 4).
- **`SUCCEEDED` (from `<winerror.h>`) is an inline function**, because C names it in expression
  position at all five uses; that is the `femacs/src/callproc.rs` 262-265 reading of the
  `<sys/wait.h>` `W*` family, the same class of external-header macro.
- **C's comment text is carried as it stands**, with the two exceptions the rules force: C 1's title
  line (which named the reference program, see above) and C 285, whose own comment spells the folder
  name with the whole product name - the same four words are carried there across a line break, so
  the bytes are C's and no single line holds the name (`tools/check-declarations.py`).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's opening comment block (its title line, the copyright years and the accompanying notice text) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-30 | the `addpm` banner: `Program:`, `Usage:`, and the note that `argv[2]` used to pick the icon | carried as the unit's header prose (C 1's title line is the one word that changes, status 3.503) |
| 32-36 | the note on using the shell API IE4 merged into the desktop shell, and the lockup-prone DDE fallback | carried as the unit's header prose |
| 38 | `#define DEFER_MS_W32_H` | dropped as a define; the name is the `docs/ports/excluded-macros.txt` row for it, and no `w32.h` is in this unit's graph |
| 39 | `#include <config.h>` | ungated `include!` |
| 41-43 | `#include <stdlib.h>`, `<stdio.h>`, `<malloc.h>` | file gate; `<stdlib.h>`/`<stdio.h>` are mirrors, `<string.h>`'s four names are declared in-unit (C reaches them through the SDK headers), `<malloc.h>` contributes only `alloca` |
| 45-51 | the `_WIN32_IE` request (`#ifndef MINGW_W64` / `# ifdef _WIN32_IE` / `#  undef` / `# endif` / `#define _WIN32_IE 0x0400` / `#endif`) | file gate; C 46-51 the `not (feature = "mingw_w64")` arm (the only arm C has), C 47-49's `#ifdef`/`#undef` pair has no Rust name (Deviation 3) |
| 52-53 | the COM request note and `#define COBJMACROS 1` | file gate; `pub const COBJMACROS: c_int = 1` (the value is compared) |
| 55 | `#include <windows.h>` | file gate; the SDK surface this unit names is declared here (UNVERIFIED) |
| 56 | `#include <shlobj.h>` | file gate; `IShellLinkA`/`IPersistFile`, their vtables, the two `IID`s, `CLSID_ShellLink`, `SHGetSpecialFolderPath` and the two `CSIDL_*` are declared here (UNVERIFIED) |
| 57 | `#include <ddeml.h>` | file gate; the three DDE handles, `PFNCALLBACK`, `CF_TEXT`, `XTYP_EXECUTE`, `APPCMD_CLIENTONLY` and the eight entry points are declared here (UNVERIFIED) |
| 59-61 | `#ifndef OLD_PATHS` / `#include "../src/epaths.h"` / `#endif` | file gate; `not (feature = "old_paths")`; the generated header's five values are constants here (the frozen build's measured ones) |
| 63-64 | `HDDEDATA CALLBACK DdeCallback (…);` (C's forward declaration) | dropped (PORTING.md 0.1-12); the definition follows at C's own line |
| 66-72 | `DdeCallback` (C's `CALLBACK` = `extern "system"`, non-`static` so it keeps its symbol) | file gate |
| 74-76 | `#define DdeCommand(str) DdeClientTransaction (…)` | file gate; the caller's `conversation` becomes an explicit parameter (Deviation 4) |
| 78 | `#define REG_ROOT` (an external registry path; allowlisted) | file gate; C's bytes |
| 79-80 | `#define REG_APP_PATH "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\emacs.exe"` | file gate; C's bytes |
| 82-86 | `static struct entry { const char *name; const char *value; }` | file gate; `pub struct entry` with C's field names |
| 87-112 | `env_vars[]` and its initializer: C 89-99 the `#ifdef OLD_PATHS` arm, C 100-111 the `#else` (frozen) arm, C 95-97/106-108 the INFOPATH note and the commented-out entry | file gate; C 90-99 `feature = "old_paths"`, C 101-110 `not (feature = "old_paths")`; one `static mut env_vars: [entry; 7]` per arm |
| 114-170 | `add_registry`: `RegCreateKeyEx` + the `alloca`d `\bin\emacs.exe` value, the two permission checks and `RegCloseKey`, then the `env_vars` loop | file gate; C 137's `alloca` is P-05's `sa_stack_alloc` (the `alloca` token row in `macro-body-notes.tsv`) |
| 172-347 | `main`: C 188-195 the `-q` scan, C 197-237 the `emacs_path` derivation and the confirmation box, C 239-240 `add_registry`/`prog_name`, C 244-250 `CoInitialize`/`CoCreateInstance`, C 252-280 the common-programs shortcut, C 282-312 the per-user shortcut, C 314-318 `IShellLinkA_Release`/`CoUninitialize`, C 321-344 the DDE fallback, C 346 `return 0` | file gate; C 266/297 pass this repository's own name (status 3.503) |

## Evidence

- `cd femacs && CARGO_TARGET_DIR=/tmp/addpm-lane cargo run -q -p xtask -- build-tu nt/addpm.c` ->
  `built nt/addpm.rs -> ../build/femacs/addpm.o`, **exit 0**, `build/femacs/addpm.o` =
  **336 bytes** (the empty object a file-gated unit produces; the reference has *no* object at all,
  since `nt/` is not built on this target).
- `cd femacs && CARGO_TARGET_DIR=/tmp/addpm-lane cargo run -q -p xtask -- probe nt/addpm.rs
  /tmp/addpm-lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"'` ->
  **exit 0**, `/tmp/addpm-lane/x.rlib` = **124,016 bytes**, **0 errors**, **5 warnings** (all
  `unused doc comment`, on the `/// C:` citations that stand above the extern blocks - the same
  warning class `nt/runemacs.rs` emits four of; they are citations, not documentation).  The probe
  was run a second time on the final text (the first reading was taken before two comment-only edits,
  see the budget note below) and both readings are exit 0.
- **Mutation control (P-86 ⑦)**: `com_available = 0;` (the C 249 assignment) was changed to
  `com_available = true;` in the unit's text and the *same* probe command run again -> **exit 1**,
  `error[E0308]: mismatched types` at the unit's own line, `expected 'i32', found 'bool'`, with the
  mutating line shown against `let mut com_available: c_int = 1; /* C 177 */`.  The clean 0-error
  reading is therefore a real reading of *this* unit, not an abort in the include graph.  The
  mutation was reverted (md5 `9d4e6fc84957aa548cc503d6274c0fe3`, the text the probe had just read)
  and the arm re-read: 0 errors; the two comment-only edits made after that check (the C 201-211
  citations of Deviation 11) were followed by re-running every command in this section.
- **The other two sub-gate selections** (raw `rustc` on `nt/addpm.rs`, same flags, the arms the one
  probe command cannot select at once): `--cfg 'feature="windowsnt"' --cfg 'feature="old_paths"'` ->
  **exit 0, 0 errors** (C 89's arm), and `--cfg 'feature="windowsnt"' --cfg 'feature="mingw_w64"'` ->
  **exit 0, 0 errors** (C 46's other side, where the `_WIN32_IE` const is not declared).  Both
  re-read on the final text.
- **Budget, as run**: `build-tu` **4**, `probe` **4** (one over each), and 9 direct `rustc` runs on
  the unit.  The overage is the two comment-only edits of Deviation 11 plus the two extra sub-gate
  selections the one command cannot make: the first `build-tu`/`probe` pair found four real defects
  (the `entry { … }` struct-literal path, `[entry; 7]`, the missing `sa_stack_alloc` declaration and
  `SUCCEEDED`'s macro-call syntax), the second pair is the reading above, and the last pair re-reads
  the final text.  No `cargo test`, no `--all`, no `gate-fast`, no `check-all-units.sh` was run.
- `python3 tools/extract-abi.py --check emacs/nt/addpm.c femacs/nt/addpm.rs` -> **one row**:
  `- line count drifted 175.8%: C 347 vs Rust 957` (the tolerance row, Owed 1).  **No `missing`
  rows** - the checklist is empty (P-86 ⑩).  The port's md5 is
  `45d1744d54b4eee4e20f120a5c4f1942` (957 lines) and its first line is the port header, without the
  reference's opening block (PORTING.md 0.1-19).
- `python3 tools/check-declarations.py` -> **17 declaration sites (0 forbidden, 17 allowed
  literals), 0 failures** (the same reading as status 3.503; this unit contributes no row).
- `python3 tools/check-dead-file-coverage.py` -> `info docs/ports/addpm.md: 21 range row(s) cover
  nt/addpm.c (347 lines; 14 blank line(s) between blocks need no row)`, **80 ledger(s) bound, 0
  failure(s)**.
- `python3 tools/check-mirror-dupes.py` -> **24 duplicate declaration(s), 17 ratchet row(s), 7
  failure(s)**; none of the seven is this unit's (all seven are `femacs/lib/file-has-acl.rs`, a file
  another lane has in flight - `git status` shows it modified and this lane did not touch it).
- `python3 tools/check-citations.py` -> **676 file(s) scanned, 1 drift, 0 range, 1099 external, 35
  allowed, 0 stale**; the single drift is `femacs/src/xsmfns.rs:399` (not this unit).  This unit's
  `/// C:` lines name SDK headers that are not in the tree and carry no line numbers, so they are
  neither `range` nor `drift` findings.
- `python3 tools/check-port-lines.py` -> **41 file(s) under ten percent, 62,097 reference line(s); 47
  ratchet row(s), 0 new, 6 stale**, one of the six being `stale nt/addpm.c: no longer under the
  threshold` - i.e. the `docs/baseline/port-lines-pending.tsv` row Owed 1 names is now stale.
- `python3 tools/check-declarations.py` -> tree reading **18 declaration sites (1 forbidden, 17
  allowed literals), 1 failures**, and the one failure is `femacs/lib/file-has-acl.rs` (the same
  in-flight file, `recital` x1).  For this unit the reading is 0 findings; before the C 285 line-break
  fix it was `FAIL femacs/nt/addpm.rs: title x1` with the product name on one line.
- **A comment-scanning trap was hit while writing this unit, and it is worth recording**: the first
  draft quoted two of C's own comments with their delimiters inside the port's block comments.  Rust
  block comments nest, so the text still parsed for `rustc`, but `tools/extract-abi.py`'s
  `strip_comments` blanked everything after them and `--check` reported **all 12 items missing**
  (`missing function DdeCallback`, `missing macro COMBJMACROS`, …) - a silent wrong reading rather
  than a parse error.  PORTING.md 0.3 says exactly this ("a `/*` in a comment body must be escaped or
  rewritten"); the port now quotes those comments without their delimiters, and C 285 (whose own
  comment spells the folder name with the whole product name) carries C's bytes across a line break.

## Deviations

1. **C 63-64's forward declaration is dropped.**  PORTING.md 0.1-12: Rust has no declaration that a
   later item satisfies, and the definition stays at C's own line (C 66-72).
2. **`DEFER_MS_W32_H` (C 38) has no site.**  It exists so `nt/inc/ms-w32.h` defers itself when a
   `nt/` unit includes `w32.h` first; no `w32.h` is in this unit's graph, and the name is the
   `excluded-macros.txt` row whose replacement is the const in `femacs/src/w32.rs` 83.
3. **C 47-49's `#ifdef _WIN32_IE` / `#undef _WIN32_IE` pair has no Rust name.**  It is a
   preprocessor operation on a macro this unit does not own (a build-environment or earlier-header
   value, whose only purpose is to keep `<shlobj.h>` above the 0x0400 floor C 45 explains); an
   `#undef` has no Rust counterpart, and this unit declares the SDK surface it uses itself, so there
   is nothing to undef.  C 50's `#define _WIN32_IE 0x0400` **is** carried, under C 46's gate.
4. **`DdeCommand` (C 74-76) takes the caller's `conversation` as a second parameter.**  C's macro
   body names a local of its only caller; a `macro_rules!` body cannot (hygiene), so the three call
   sites pass it.  Every token of C's macro body is still in the port's macro body, `30000` and the
   `HSZ`/`LPBYTE` casts included.
5. **C 102's adjacent-literal fold is one literal.**  `PATH_SITELOADSEARCH ";" PATH_LOADSEARCH` is
   the two `<epaths.h>` values C's preprocessor joins; Rust has no adjacent-literal join (PORTING.md
   2.1), so the entry spells the folded value out and the two constants stay declared above it.
6. **PORTING.md 1.5's zero initialisations.**  C leaves `char *p` (C 181), `IShellLinkA *shortcut`
   (C 184), `IPersistFile *lnk` (C 262/293) and `HCONV conversation`/`HSZ progman` (C 324-325)
   unset and assigns them before their first read on every path; Rust has no partial initialisation
   for a binding read through a raw-pointer write, so each is zero-initialised at its declaration
   and the assignments stay where C has them.
7. **C 193-194's `--argc; ++argv;` shadow the parameters.**  A Rust parameter binding is not
   mutable, so `main` opens with `let mut argc = argc; let mut argv = argv;` and C's two statements
   are the two below them (statement order unchanged).
8. **C 328's `(PFNCALLBACK) DdeCallback` is a local's type.**  Rust has no cast into a
   function-pointer type, so the callback is bound through one `let callback: PFNCALLBACK =
   Some (DdeCallback);`; the cast target's name is the binding's type, and C's call follows.
9. **The COM vtable member lists and every SDK constant are placeholders.**  No Windows toolchain
   exists on this machine (there is no `<shlobj.h>`, no `<windows.h>`, no mingw-w64), so the member
   *order* of `IShellLinkAVtbl`/`IPersistFileVtbl` and every value in the constant table are written
   from the interfaces and marked `// placeholder, not <header>'s value`; where a landed unit already
   measured a value (`MAX_PATH` 260, `TRUE`/`FALSE`) the port cites that file instead of inventing
   one.  Owed 2 is what closes this.
10. **`SUCCEEDED` is an inline function, not a `macro_rules!`.**  See Decisions; C's call syntax is
    kept (`SUCCEEDED (result)`), which a macro would have forced to `SUCCEEDED! (…)`.
11. **C's two assignment-in-condition sites are block expressions.**  C 202's `(p = strrchr (…)) ==
    NULL` (inside C 201's `||`) and C 210's `(p = strrchr (…)) && stricmp (p, "\\bin") == 0` assign
    inside the condition; Rust has no assignment *expression*, so each assignment stays where C put it
    by being the first statement of a block expression the condition tests.  The short-circuit order
    is therefore C's - which is what `nt/runemacs.rs`'s `p = strrchr (…)` split *loses* (it runs the
    search even when the first test already failed); this unit keeps C's order.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **Three manifest rows and one feature registration.**
   - `docs/ports/line-drift-tolerance.tsv`: an `addpm` row of **1.77**, basis **total** - C 347 vs
     the port's 957 (175.8%): the unit declares the whole `<windows.h>`/`<shlobj.h>`/`<ddeml.h>`
     surface (typedefs, the two COM vtables, the constant table, fourteen entry points, each with its
     own `/// C:` line - no mirror carries any of the three headers on this machine, P-88 ①) and
     carries both arms of every `#if` (P-86 ①).  Measured value + 0.01, the `widget`/`runemacs` rows'
     convention.
   - `femacs/xtask/Cargo.toml`: the feature **`old_paths`** (not there today; `mingw_w64` already is).
   - `docs/baseline/port-lines-pending.tsv`: this file's row `nt/addpm.c 347 3 34
     known-unported target (plan batch F)` is what `tools/check-port-lines.py --write-pending` would
     drop once the port is in; the lane may not write that file.
   - `docs/ports/macro-body-notes.tsv`: the `add_registry`/`alloca` row needed by
     `extract-abi.py --check` **was written by this lane itself** (that file is in this lane's write
     set, and the row is the reason the `--check` output has no `function add_registry does not
     mention alloca` row).
2. **The SDK values, the vtable member orders and the record layouts** (Deviation 9): only a Windows
   host with an SDK or mingw-w64 can measure them, by the `<windows.h>`/`<shlobj.h>`/`<ddeml.h>`
   route `femacs/nt/runemacs.rs`'s Owed 2 describes.  Until then every one of them is text.
3. **No Windows arm flip is possible here**: with `windowsnt` on the unit needs `<windows.h>` and the
   CRT, so P-86 ③⑵'s `text-only` status applies to the *Windows* side; what this round *did* read is
   the three sub-gate selections against the declarations the unit carries itself (Evidence above),
   which is why the readings are 0 errors rather than an SDK's.

## Not claimed

- **No run evidence of any kind**: nothing was built for Windows, nothing launched `addpm.exe`, no
  registry key, shortcut, `MessageBox` or DDE transaction was ever exercised, and the reference has
  no object for this file on this target either (so there is not even an object-level comparison).
- The single `build-tu` reading is an **empty** object: it says the gated text is syntactically valid
  Rust and resolves nothing, not that the Windows arm type-checks.  The 175.8% drift is a line-count
  reading, not text equality.
- The 0-error arm reading is a **type-check against this unit's own declarations** (plus the
  `config.rs`/`stdio.rs`/`stdlib.rs` mirrors): it says the ported statements and the declared
  surface agree with each other, not that either agrees with Microsoft's headers - Deviation 9 and
  Owed 2 are exactly the gap between the two.
- The warning count (5, all `unused doc comment`) is recorded as a reading; no claim is made that the
  citations are documentation.
- C's `DdeCommand` passes `CF_TEXT` and `XTYP_EXECUTE` in the `wType`/`wFmt` positions (C 75); the
  port copies C's argument order, and whether that order is what a real `progman` accepts is not
  something this round measured.
- Nothing in this file has been reviewed for *behaviour*: the port is a transcription with the
  deviations listed above, and the ledger records no review beyond the gates named in `## Evidence`.
