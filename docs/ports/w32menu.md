# emacs/src/w32menu.c (1850 lines) - whole file, C 1-1850

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32menu.o` sits in
> `W32_OBJ` (`emacs/src/Makefile.in` 316, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac`
> 3022-3030's `HAVE_W32` test, whose 3030 `AC_DEFINE`s `HAVE_NTGUI`), and
> `docs/baseline/frozen-build-sources.txt` lists no `w32menu.o` for this target - so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `src/w32menu.rs` (**2904 lines**, md5 `51dd2c51a9a01f0f0705f85599e262f4`), carrying
> **C 1-1850 with both arms of every `#if`**.  The reference `emacs/src/w32menu.c` is md5
> `fad9ed45bf9fac988ec5283541f8975f` (unchanged; `emacs/` is read-only).
>
> **This ledger overturns the reading it replaces.**  The previous `docs/ports/w32menu.md` (and the
> 11-line-comment unit it described) read "the frozen configuration does not compile this file ⇒
> the port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu`
> object) is unchanged and still true - it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32font.md`, `docs/ports/w32term.md` and `docs/ports/w32select.md` record for their
> own pairs.
>
> **The tree moved under this batch**: it opened at `88964fa7` and closed at `2f67237b`, and the
> mirror lanes landed rows in between (`462b12a1` took `include/w32term.rs`'s six `HWND` names to
> untyped pointers, `dca3e1c7` gave `include/w32common.rs` its two SDK records, and `526b4670` /
> `2f67237b` regenerated `docs/baseline/port-lines-pending.tsv`, which is where the stale
> `src/w32menu.c` ratchet row of Evidence ⑫ lived).  Every reading below was taken against
> `2f67237b` unless it says otherwise, and Evidence ④ measures the motion on one row.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is **no `<windows.h>`
> to compile or measure against**, no arm flip on a Windows target and no layout measurement: every
> record and value the stand-in sections declare is `UNVERIFIED` (documented, not measured) and the
> layout assertions are owed.  What the round does carry: the frozen `build-tu` reading, the
> machine-checked coverage table, the `#if` verdicts, a per-item C citation, the `probe` reading of
> the `have_ntgui` + `windowsnt` arm, a **free multi-configuration sweep** of the same materialized
> text, a **mutation test** over this unit's own lines, and one patched-mirror row that shows the
> unit's text type-checks once the mirror's one recorded bitfield gets a storage unit.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3022-3030: `HAVE_W32` → `AC_DEFINE HAVE_NTGUI`) | `feature = "have_ntgui"` on every item *and* on C 19-55's include graph | native MS Windows GUI, incl. the Cygwin GUI build |
| C 43/191/1493/1519/1579/1838 `#ifndef NTGUI_UNICODE`, C 143's `#ifdef` | `not (feature = "ntgui_unicode")` / `feature = "ntgui_unicode"` | `conf_post.h` 169-170 defines the macro only for `defined CYGWIN && defined HAVE_NTGUI` |
| C 49 `#ifndef makedev` (the `#else` arm is empty) | `not (feature = "makedev")` | the frozen macro table defines `makedev` (`docs/baseline/config/darwin-arm64.macros.tsv` 7209), so C's arm is the dead one there; both arms carried |
| C 58 `#undef HAVE_DIALOGS` | `not (feature = "have_dialogs")` is the live arm; `feature = "have_dialogs"` carries C 159/330/1055's arms | the file turns the macro off itself ("TODO: Implement native dialogs"), so those arms are dead in *every* configuration |
| C 60 `#ifndef TRUE` | `not (feature = "true")` (C's own `#define`s) / `feature = "true"` (the header's) | `TRUE` is in the frozen table; clang and the evaluator disagree on this guard (Evidence ⑬), which is why the file is not on `cond-eval-verified.txt` |
| C 83 `#ifndef MINGW_W64` | `not (feature = "mingw_w64")` (C's own typedefs) / `feature = "mingw_w64"` (the header's, declared at C 55's stand-in) | `nt/inc/ms-w32.h` 35 defines it for 64-bit MinGW-w64; mingw.org's MinGW does not |
| C 1546/1767 `#ifdef MENU_DEBUG` | `feature = "menu_debug"` | never in the frozen table (undef); both arms carried |

`tools/check-cond-eval.py src/w32menu.c` reads **`15 conditions, 0 unknown, 1 disagreements
[NOT VERIFIED]`** (Evidence ⑬): the fifteen are C 43, 49, 60, 83, 143, 159, 191, 324, 1055, 1493,
1519, 1546, 1579, 1767 and 1838.  The `probe` door set is therefore `have_ntgui` + `windowsnt`
(the latter for the mirrors this unit reaches, not for any of C's own guards), with the four
negations (`ntgui_unicode`, `makedev`, `have_dialogs`, `true`) off in the base arm because their
live arms are the negations.

## C line ranges

The rows are contiguous and ascending and cover every line of C 1-1850 (260 of them are the
reference's blank lines, which the gate does not require a row for; rows here absorb them).
`docs/port-map.md` 579 pairs `src/w32menu.c` 1,850 with `src/w32menu.rs`, and that is the
only reference file this stem has.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the reference's opening ownership/title block | dropped (PORTING.md 0.1-19) |
| 19-24 | `<config.h>` (C 19) and `signal.h`/`stdio.h`/`setjmp.h` (C 22-24) | file gate |
| 25-33 | `lisp.h`, `keyboard.h`, `frame.h`, `blockinput.h`, `buffer.h`, `coding.h`, `menu.h`, `pdumper.h` | file gate |
| 34-37 | the sys/types.h comment and `w32term.h` | file gate |
| 38-45 | the Cygwin comment and C 43-45's `#ifndef NTGUI_UNICODE` / `mbstring.h` | file gate; C 43 `not (feature = "ntgui_unicode")` |
| 46-51 | the sys/types.h comment and C 49-51's `#ifndef makedev` include | file gate; C 49 `not (feature = "makedev")` |
| 52-55 | `w32common.h` (`osinfo_cache`) and `commctrl.h` | file gate |
| 56-58 | the two comments and C 58's `#undef HAVE_DIALOGS` | file gate; that `#undef` is why C 159/330/1055 are dead in every configuration |
| 59-63 | C 60-63's `#ifndef TRUE` / `#define TRUE 1` / `#define FALSE 0` guard | both arms: `not (feature = "true")` and `feature = "true"` |
| 64-65 | `HMENU current_popup_menu;` | file gate |
| 66-81 | the three `WINAPI` proc typedefs (`GetMenuItemInfoA_Proc`, `SetMenuItemInfoA_Proc`, `MessageBoxW_Proc`) | file gate |
| 82-135 | C 83-135's `#ifndef MINGW_W64` block (the two flag typedefs, `PFTASKDIALOGCALLBACK`, `_TASKDIALOG_BUTTON`, `_TASKDIALOGCONFIG`, `TDN_CREATED`, `TDM_ENABLE_BUTTON`, `TDF_ALLOW_DIALOG_CANCELLATION`, `TD_INFORMATION_ICON`) | both arms: `not (feature = "mingw_w64")` and `feature = "mingw_w64"` |
| 136-141 | `TaskDialogIndirect_Proc` | file gate |
| 142-155 | C 143-155's `#ifdef NTGUI_UNICODE` / `#else` (the six file-scope objects) | both arms: `feature = "ntgui_unicode"` and `not (feature = "ntgui_unicode")` |
| 156-157 | `static TaskDialogIndirect_Proc task_dialog_indirect;` | file gate |
| 158-164 | C 159-164's `#ifdef HAVE_DIALOGS` / `#else` prototypes of the three static dialog helpers | both arms; the prototypes are dropped (0.1-12) and their text kept |
| 165-169 | the `utf8to16` / `fill_in_menu` / `w32_free_menu_strings` prototypes | dropped (0.1-12), text kept |
| 170-171 | `#define TASK_DIALOG_MAX_BUTTONS 10` | file gate |
| 172-186 | `task_dialog_callback` | file gate |
| 187-366 | `w32_popup_dialog`, incl. C 191-195's local typedef/static and C 324-353's two arms | file gate; C 191 `feature = "ntgui_unicode"`; C 324/330 `not (feature = "have_dialogs")` / `feature = "have_dialogs"` |
| 367-378 | `w32_activate_menubar` | file gate |
| 379-384 | the callback's comment and its forward declaration | prototype dropped (0.1-12), text kept |
| 385-471 | `menubar_selection_callback` | file gate |
| 472-474 | the `set_frame_menubar` banner comment | file gate |
| 475-712 | `set_frame_menubar` | file gate |
| 713-717 | the `initialize_frame_menubar` comment | file gate |
| 718-726 | `initialize_frame_menubar` | file gate |
| 727-730 | the `free_frame_menubar` comment | file gate |
| 731-744 | `free_frame_menubar` | file gate |
| 745-761 | the two `w32_menu_show` banner comments | file gate |
| 762-1052 | `w32_menu_show` | file gate |
| 1053-1077 | C 1055's `#ifdef HAVE_DIALOGS` and the dialog design comment | `feature = "have_dialogs"` (dead in every configuration; C 58's `#undef`) |
| 1078-1081 | `static char * button_names []` | `feature = "have_dialogs"` |
| 1082-1242 | `w32_dialog_show` | `feature = "have_dialogs"` |
| 1243-1248 | C 1243's `#else /* !HAVE_DIALOGS */` and its comment | `not (feature = "have_dialogs")` - the live arm |
| 1249-1295 | `is_simple_dialog` | `not (feature = "have_dialogs")` |
| 1296-1407 | `simple_dialog_show` | `not (feature = "have_dialogs")` |
| 1408-1411 | C 1408's `#endif` and the UTF8 comment | file gate |
| 1412-1448 | `utf8to16` | file gate |
| 1449-1656 | `add_menu_item`, incl. C 1493-1504 and C 1519-1531's `#ifndef NTGUI_UNICODE` arms and C 1546-1548's `#ifdef MENU_DEBUG` | file gate; `not (feature = "ntgui_unicode")` / `feature = "menu_debug"` |
| 1657-1685 | `fill_in_menu` | file gate |
| 1686-1690 | the help comment and `w32_menu_display_help`'s prototype | prototype dropped (0.1-12), text kept |
| 1691-1748 | `w32_menu_display_help` | file gate |
| 1749-1777 | `w32_free_submenu_strings`, incl. C 1767-1769's `#ifdef MENU_DEBUG` | file gate; `feature = "menu_debug"` |
| 1778-1796 | `w32_free_menu_strings` | file gate |
| 1797-1807 | the subr banner and `DEFUN ("menu-or-popup-active-p", …)` | file gate |
| 1808-1821 | `syms_of_w32menu` | file gate |
| 1822-1850 | the `globals_of_w32menu` banner and the function, incl. C 1838-1849's `#ifndef NTGUI_UNICODE` | file gate; `not (feature = "ntgui_unicode")` |
## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32m-lane/cargo cargo run -q -p xtask -- build-tu src/w32menu.c`
→ `built src/w32menu.rs -> ../build/femacs/w32menu.o`, **exit 0**, and `build/femacs/w32menu.o` is
**336 bytes** - the P-08 empty object, which is what every item being `#[cfg (feature =
"have_ntgui")]` produces.  Budget: `build-tu` spent **3** (runs #1 and #2 of the same command
exited 1 with syntax errors in the unit's text - six, then one - and wrote no object; only #3 is
the reading above).  The reading was confirmed on the **final revision** without spending a fourth
`build-tu`: the same compile driven directly
(`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`
on a freshly materialized root, no `--cfg`) exits 0 and writes **336 bytes**.

**② The arm reading (`xtask probe`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32m-lane/cargo cargo run -q -p xtask -- probe src/w32menu.rs /tmp/w32m-lane/final.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`
→ **exit 1, 13 error messages** (rustc's own `aborting due to 13 previous errors; 24 warnings
emitted`), and the primary anchors are:

| file | errors | what they are |
| --- | --- | --- |
| `_root.rs` (this unit's own lines) | 6 | the six `E0609`s of Deviation 6: `no field 'menubar_active' on type 'w32_output'` |
| `src/include/w32term.rs` | 6 | five `E0428` pairs (its 289 `current_popup_menu`, 338 `initialize_frame_menubar`, 1342 `w32_popup_dialog`, 1348 `syms_of_w32menu`, 1350 `globals_of_w32menu` - all declared at its crate root) + `E0425 cannot find type 'font'` at 1456 |
| `src/include/frame.rs` | 1 | that mirror's own `compile_error!` at 329 (its HAVE_NTGUI flag-run refusal) |

So **6 of the 13 are this unit's own anchors, and all six are the one mirror gap of Deviation 6**;
no error is a name this unit invents, and no mirror error is this unit's.  The 24 warnings are 23
`unused doc comment` (the `/// C:` lines on `unsafe extern "C"` blocks - the same shape the other
Windows units carry) and one `unused attribute` inside `include/frame.rs`.  Budget: `probe` spent
**4, reported as an overrun** (the brief asked for ≤3): #1 = the base arm on the revision before
the fixes of Deviations 4/5/7 (34 `error` lines), #2 = the base arm after them (13), #3 = the
mutation test below, #4 = the base arm re-taken on the final revision (**the reading above**,
because the mirror lanes landed rows - see the sweep's Cygwin row - while this batch ran).

**③ The mutation test (P-86 ⑦): the reading is not vacuous.**
One type error injected into this unit's own text - `set_frame_menubar`'s
`(*f).menu_bar_items_used = menu_items_used;` (C 615) became
`(*f).menu_bar_items_used = core::ptr::null_mut ();  // C 615  [MUTANT]` - and the **same** `probe`
command re-run: exit 1, **14** error messages against the base reading's 13, the single new one
`error[E0308]: mismatched types`, anchored in the unit (`_root.rs`, the mutated line, which the log
quotes with its `[MUTANT]` marker); the `diff` of the two logs' `error` lines is exactly
`+error[E0308]: mismatched types`.  So ②'s "6 in-unit anchors, all of them the recorded bitfield"
is a reading of the unit's own resolved text and not an artifact of the mirrors stopping rustc
early.  The mutation was reverted and the unit verified: `shasum -a 256` =
`e957651570bc4e226a87e218aca925b8e2f50b254e0ffda590cfe00059f4476f` before and after.  (That
revision is the one the mutation ran on; the polish listed in Deviations 4, 5, 10 and 14 came
after it, and ②'s reading is the later revision's.)

**④ The free multi-configuration sweep (direct `rustc` on the materialized unit; not a Windows
compile).**  `xtask probe` materializes the unit into `build/probes/w32menu/`; a `/tmp` copy of that
directory has this unit's current lines spliced back in (the same `include!` decisions the walk
made), and each configuration compiles it with
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`.
`errors` counts rustc's error *messages* (the `aborting due to N previous errors` line excluded);
`in-unit` counts those whose **primary** anchor is this unit's own text.

| configuration | exit | error messages | anchored in the unit |
| --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 (the 336-byte object of ①) |
| `have_ntgui` (the Cygwin GUI arm) | 1 | 14 | 13 |
| `+ windowsnt` (base) | 1 | **13** | **6** |
| base + `ntgui_unicode` | 1 | 13 | 6 |
| base + `mingw_w64` | 1 | 13 | 6 |
| base + `have_dialogs` | 1 | 20 | 13 |
| base + `menu_debug` | 1 | 13 | 6 |
| base + `true` | 1 | 13 | 6 |
| base + `makedev` | 1 | 13 | 6 |
| base + every sub-gate | 1 | 13 | 6 |

The `have_ntgui`-only row is C's Cygwin arm, and every in-unit anchor there is a name a mirror
gates more narrowly than C does: `include/w32term.rs`'s whole body carries `#[cfg (feature =
"windowsnt")]` while C's `w32term.h` has no `#if` of its own, so `FRAME_W32_WINDOW`,
`FRAME_OUTPUT_DATA`, `w32_output`, `one_w32_display_info` and `AppendMenuW_Proc` are absent in that
arm (Deviation 15).  **That row also measures the mirror graph moving under this batch**: on the
same unit text it read 25 errors / 24 in-unit anchors when this sweep was first run and 14/13 by
the final run, because another lane landed rows in `include/w32term.rs` in between.  The
`have_dialogs` row is Deviation 13's: 6 recorded-bitfield anchors + 6 `E0530` (Rust forbids a
parameter or `let` shadowing a `static`, where C's text has the global `menu` *and* four
parameters of that name) + the `widget` member C's header does not declare.

**⑤ The patched-mirror row (`/tmp` harness only - not the committed mirror).**  The same
materialization with one line added to the copy of `include/w32term.rs` - `pub menubar_active:
c_int,` beside the recorded bitfields' comment - re-compiled in the base arm: **exit 1, 8 error
messages, 0 of them anchored in this unit**.  So the six in-unit anchors of ② are exactly the one
mirror gap, and this unit's own text type-checks once that field has a storage unit.

**⑥ `python3 tools/extract-abi.py --check emacs/src/w32menu.c src/w32menu.rs`**: after this
round's five `docs/ports/macro-body-notes.tsv` rows (Appendix below), exactly **one** finding
remains: `FAIL line count drifted 57.0%: C 1850 vs Rust 2904` -
`docs/ports/line-drift-tolerance.tsv`'s row, which is not in this lane's write set (Owed 1).
Before them the reading was 10 findings: two `missing field _TASKDIALOGCONFIG.DUMMYUNIONNAME`/
`DUMMYUNIONNAME2` rows and the struct row (fixed in the unit, Deviation 2), and the five
`does not mention …` rows below.

**⑦ `python3 tools/check-declarations.py`**: `17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures`, `read 683 files (625 .rs 40 .kt 7 .swift 6 .s 5 .cc)` - this unit and this
ledger carry no finding.  The first run of this batch read **2 failures** (`recital x1` and
`title x1`) on this unit's opening comment, which recited the reference's opening block and quoted
its product name; both were **fixed in the unit** rather than recorded (the comment now says "the
reference's opening block", dropped per PORTING.md 0.1-19), and the reading above is the final one.

**⑧ `python3 tools/check-mirror-dupes.py`**: `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` - the baseline the other Windows units report, and no row names `w32menu`.  The first
run of this batch read **1 failure**: `src/w32menu.rs: set_buffer_internal_1 is already
declared by src/include/buffer.rs (in the include graph)`; the unit's own copy was
**removed** (Deviation 9) and the reading above is the final one.

**⑨ `python3 tools/check-citations.py`**: `676 file(s) scanned, 0 drift, 0 range, 1514 external, 36
allowed, 0 stale` - this unit's ~120 `/// C:`/`// C` citations and this ledger's are in range and
current.

**⑩ `python3 tools/check-dead-file-coverage.py`**: the machine-checked coverage reading for this ledger's own table: info docs/ports/w32menu.md: 48 range row(s) cover src/w32menu.c (1850 lines; 0 blank line(s) between blocks need no row); whole tree: 107 ledger(s) bound (a C range table), 0 failure(s)

**⑪ `python3 tools/check-ledger-ticks.py`**: `233 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 357 row(s) with no function tick - a unit with no function
at all, or one whose body the frozen configuration leaves under a false condition
(check-port-lines.py is that class's gate), 0 row(s) without a ledger` - this ledger carries no
`- [x] NAME` tick, so it is one of the 357 (P-86 ②'s empty checklist).

**⑫ `python3 tools/check-port-lines.py`**: this stem has left the "under ten percent" class (it
was one of the 96 files that reading names) and needs no new ratchet row.  Two readings, because
the shared list moved under the batch: mid-batch
`stale src/w32menu.c: no longer under the threshold` with `28 ratchet row(s), 0 new, 3 stale`, and
on the final revision `25 file(s) under ten percent, 30,844 reference line(s); 25 ratchet row(s), 0
new, 0 stale` - the stale row was removed by the `526b4670`/`2f67237b` rounds' regeneration of
`docs/baseline/port-lines-pending.tsv`, not by this lane (Owed 5).

**⑬ `python3 tools/check-cond-eval.py src/w32menu.c`**: `DISAGREE src/w32menu.c:60 'ifndef TRUE' -
clang True, evaluator False` and `src/w32menu.c: 15 conditions, 0 unknown, 1 disagreements [NOT
VERIFIED]` - the Gate-mapping rows, nothing else.  Because the file is not on
`cond-eval-verified.txt`, `extract-abi.py` reads its conditions conservatively (both arms live),
which is also what P-86 ① requires.

**⑭ the residual-resource reading**: `df -h /` reports `/dev/disk3s1s1 1.8Ti 12Gi 4.2Gi 74% /`,
and `/tmp/w32m-lane` (the lane's scratch: the probe logs, the mutation backup, the sweep harness and
its materialized copies - 26 MB) was deleted before that reading.  The readings either side of the
removal are the same to `df`'s rounding (`12Gi` used, `4.2Gi` available, `74%`), because the lane's
bytes are a rounding error against the disk; the two `build/` directories the evidence commands
write (`build/femacs/units/w32menu/`, `build/probes/w32menu/`) are the gates' own artifacts and are
left as every other lane leaves theirs.

## Deviations

1. **C 1-18 and the five forward-declaration blocks have no counterpart**: the reference's opening
   block is dropped (PORTING.md 0.1-19), and C 159-169 (the three static dialog helpers'
   prototypes, `utf8to16`, `fill_in_menu`, `w32_free_menu_strings`) and C 1690 (`w32_menu_display_help`,
   whose definition follows at C 1692) are dropped (PORTING.md 0.1-12 - a declaration plus a
   definition is `E0428` and items are order-independent).  Their text is kept as C's own text at
   its position.
2. **The in-place SDK surface** (P-88 ①): `<windows.h>` (the names stand at C 26's chain, which is
   how C reaches it: `lisp.h` → `conf_post.h` → `nt/inc/ms-w32.h` → `<windows.h>`),
   `<commctrl.h>` (C 55), `<mbstring.h>` (C 44) and `<winerror.h>`'s `SUCCEEDED` have no file in
   the reference tree, so the names *this file* uses are declared in the unit at those C
   positions, each with the SDK's own text above it and **marked UNVERIFIED** - no `<windows.h>`
   exists on this machine, so the widths are PORTING.md 1.1's fixed Windows ones and the records'
   sizes, offsets and values are the documented ones (Owed 7).  **The two anonymous-union member
   names are the one place the port names something C's preprocessor erases**: C's
   `union { HICON hMainIcon; PCWSTR pszMainIcon; } DUMMYUNIONNAME;` is anonymous where that macro
   expands empty (which is why C 291 reads `config.pszMainIcon`), and `<winnt.h>` gives it the
   compatibility spelling `u`; Rust has no anonymous unions, so the port's union members carry the
   names C's own text writes (`DUMMYUNIONNAME`, `DUMMYUNIONNAME2`, the latter via
   `DUMMYUNIONNAME2 u2`) and C 291's store reads `config.DUMMYUNIONNAME.pszMainIcon`.  C's own
   `#define`s (`TRUE`/`FALSE` of C 61-62, `TDN_CREATED`, `TDM_ENABLE_BUTTON`,
   `TDF_ALLOW_DIALOG_CANCELLATION`, `TD_INFORMATION_ICON`, `TASK_DIALOG_MAX_BUTTONS`) are the
   port's `pub const`/`macro_rules!` items with C's values.
3. **Every `#if` carries both arms** (P-86 ①): C 43/191/1493/1519/1579/1838's `NTGUI_UNICODE`,
   C 49's `makedev`, C 58's `HAVE_DIALOGS`, C 60's `TRUE`, C 83's `MINGW_W64` and C 1546/1767's
   `MENU_DEBUG` (the Gate-mapping table).  The `mingw_w64` arm of C 83-135 is declared at C 55's
   `commctrl.h` stand-in (that is where the header's own copy of the surface comes from), with the
   same names and values as C's own arm, so either arm reads one set.
4. **C's two `switch`es are the port's `match`es** (C 177 `task_dialog_callback`, C 303
   `w32_popup_dialog`), each arm's `break` being the end of the arm (PORTING.md 2.2/P-04); the
   `switch`/`case`/`default`/`break` tokens the body gate would otherwise ask for are recorded in
   `docs/ports/macro-body-notes.tsv` (Appendix).
5. **C's bare `alloca` is the port's stack pool** (`sa_stack_alloc`, PORTING.md P-05): C 208/239/262
   (`w32_popup_dialog`), C 397 (`menubar_selection_callback`), C 503-505 and C 546-549
   (`set_frame_menubar`).  C 191-195's **function-local** `MultiByteToWideChar_Proc` typedef is
   declared at file scope, because C's `#else` arm leaves the uses at C 207/209/237/240/259/263 to
   `src/w32.c` 537's file-scope `pMultiByteToWideChar` (the object `src/systhread.c` 440 declares
   `extern`), and the port's `#else` arm needs that object's type; the `#ifdef` arm's own local
   `static` is kept inside the function and shadows it exactly as C's inner declaration does.
6. **`f->output_data.w32->menubar_active` is one of the bitfields `include/w32term.rs` 393-400
   records rather than packs** (PORTING.md 1.3 / P-80 need the real compiler's byte dump before a
   storage unit may stand for a bitfield, and this record cannot be compiled on this target), so
   C's six assignments and tests (C 374, 462, 470, 487, 986, 1806) write C's own field path through
   the record C's accessor casts to - `(*( (*f).output_data.w32 as *mut w32_output )).menubar_active`,
   the `src/w32term.rs` 1553 spelling - and the field does not resolve yet: that is the six in-unit
   anchors of Evidence ②, and Evidence ⑤ shows them going to zero once the mirror gives the member
   a unit (Owed 2).  C's path is written literally rather than through a macro because the body
   token gate reads `output_data` and `w32` from the function body.
7. **Names no mirror in this graph carries are declared in the unit** (P-72 rule ④), each at a
   `/// C:` line naming its reference header, the `src/androidmenu.rs` 92-104 shape: `menu_bar_items`
   (lisp.h 5087), `record_unwind_save_match_data` (lisp.h 4998), `discard_mouse_events` (lisp.h
   5090), `safe_run_hooks` (lisp.h 5097), `quit` (lisp.h 4786), `show_help_echo` (keyboard.h 511),
   `kbd_buffer_store_help_event` (keyboard.h 515), `menu_separator_name_p` (keyboard.h 473),
   `check_window_system` (frame.h 1504), `adjust_frame_size` (frame.h 1511), `make_widget_value`,
   `free_menubar_widget_value_tree`, `digest_single_submenu` and `update_submenu_strings` (menu.h
   39-44, `menu.c`'s definitions), and - for the `have_dialogs` arm only - `widget_id_tick`
   (`src/xmenu.c` 1610-1611, "*FIXME: Move this to a .h file*"), `dialog_id`, `menu`,
   `dialog_selection_callback` and `popup_get_selection`.  Owed 4 names the mirror rows that retire
   each.
8. **The three wide string literals are arrays** (`WIDE_Question`, `WIDE_Information`, `WIDE_EMPTY`
   for C 287/290/1336/1354/1359's `L"Question"`/`L"Information"`/`L""`): a C wide literal has no
   Rust spelling (Rust's `c"..."` is a narrow `&CStr`), so the port writes the code units C's
   literal denotes and takes the array's address, which is what the literal decays to.
9. **`set_buffer_internal_1` is the mirror's, not the unit's**: `include/buffer.rs` declares it, so
   the unit's own copy was removed (`check-mirror-dupes`'s finding, Evidence ⑧) - the unit calls it
   unqualified through the mirror's glob import.
10. **Two spellings that a Rust keyword or a type rule forces**: C 886's local `type` is `type_`
    (the body token gate folds the trailing underscore, so the compared token is still `type`), and
    `TDM_ENABLE_BUTTON`'s body is `(WM_USER + 111) as UINT` while `WM_USER` stays a `c_int`
    (`include/w32term.rs` 907's `WM_EMACS_START` is `WM_USER + 1` in that type).
11. **Zero-initialised locals and records where C leaves a binding unassigned on a path that is
    read** (PORTING.md 1.5): `buttons`/`button_values`/`config` (C 212/213/281), `pos` (C 770),
    `buf` with `EVENT_INIT` right after it (C 428-430), `info` with its `memset` right after it
    (C 1617-1618, 1721-1723, 1757-1758) and `dialog_name` (C 1088).
12. **The line count**: C 1850 → Rust 2904 (57.0%).  The reasons are the ~500-line in-place SDK
    surface of Deviation 2, the per-item `/// C:` citations, the both-arms `#cfg`s and the carried
    C comment blocks; the tolerance row is Owed 1.
13. **C's `HAVE_DIALOGS` arm is transcribed but is not a configuration C can build either.**  C 58
    `#undef`s the macro, so the arm is dead in every configuration, and its text names
    `menu`/`dialog_id`/`widget_id_tick`/`dialog_selection_callback`/`popup_get_selection`, an
    `f->output_data.w32->widget` member `src/w32term.h` 309-456 does not declare, and an
    `(XEvent *) 0` cast of an X11 type this translation unit cannot reach.  The port keeps C's text
    and declares the names at a `/// C:` line; under `feature = "have_dialogs"` the arm then reads
    20 error messages, 13 in the unit, of which 6 are Deviation 6's, 1 is the undeclared member and
    6 are `E0530`s - Rust forbids the parameter/`let` names `menu` that C's other four functions use
    from shadowing the object this arm needs (Evidence ④).  It is not a working configuration in
    either language, and `## Not claimed` says so.
14. **`check-declarations`'s first reading on this unit** (`recital x1`, `title x1`) was fixed in
    the unit (Evidence ⑦); the ledger's own prose avoids the product name for the same rule.
15. **Two mirror names are narrower than C's conditions**: `include/w32term.rs` gates its whole
    body on `windowsnt` and `include/frame.rs` refuses its HAVE_NTGUI arm with a `compile_error!`,
    while C's headers have no such `#if` for this file's uses.  Those are the Cygwin arm's in-unit
    anchors (Evidence ④) and are not this lane's to fix (Owed 2).

### The `docs/ports/macro-body-notes.tsv` rows appended for this unit

```
task_dialog_callback	switch,case,break	… (Deviation 4)
w32_popup_dialog	switch,case,default,alloca,dword,lpcstr,winapi	… (Deviations 4/5: the local typedef's DWORD/LPCSTR/WINAPI tokens)
menubar_selection_callback	alloca	… (Deviation 5)
set_frame_menubar	alloca	… (Deviation 5)
w32_dialog_show	xevent	… (Deviation 13)
```

The file is shared by every lane, so the five lines were appended once, at its end, with their
reasons (append-only, as the lane's write set allows).

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32menu`, tolerance ≥ **0.570**,
   basis `total`, reason from Deviations 2-12.  That file is not in this lane's write set, so
   `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑥ records.  (Integrator.)
2. **`include/w32term.rs`** (its lane): give `w32_output`'s recorded bitfield `menubar_active`
   (C 400, recorded at the mirror's 393-400) a storage unit or a bits macro, and move the five
   crate-root declarations this unit defines - `current_popup_menu` (289),
   `initialize_frame_menubar` (338), `w32_popup_dialog` (1342), `syms_of_w32menu` (1348),
   `globals_of_w32menu` (1350) - into its `w32term_fn_decls`/`w32term_objs` modules, which is where
   that mirror already keeps the names this unit's siblings define.  Until then Evidence ② carries
   five `E0428` pairs and six `E0609`s.  Its whole body being `windowsnt`-gated, while C's
   `w32term.h` has no platform `#if`, is the Cygwin arm's in-unit anchors (Deviation 15).  (Mirror
   lane.)
3. **`include/frame.rs`** (its lane): its own `compile_error!` (329) is the HAVE_NTGUI arm's
   refusal, and its note asks for a mingw clang reading of frame.h 471's flag run; it is one of
   Evidence ②'s 13 errors.  `include/frame.rs` also lacks `check_window_system` (frame.h 1504) and
   `adjust_frame_size` (frame.h 1511), which this unit declares itself (Deviation 7).  (Mirror
   lane.)
4. **The mirror rows for the in-unit declarations of Deviation 7**: `include/lisp.rs`
   (`menu_bar_items` lisp.h 5087, `record_unwind_save_match_data` 4998, `discard_mouse_events`
   5090, `safe_run_hooks` 5097, `quit` 4786), `include/keyboard.rs` (`show_help_echo` 511,
   `kbd_buffer_store_help_event` 515, `menu_separator_name_p` 473), `include/menu.rs`
   (`make_widget_value` menu.h 43, `free_menubar_widget_value_tree` 39, `digest_single_submenu` 44,
   `update_submenu_strings` 40 - the four the mirror leaves to `menu.c`) and, for the
   `have_dialogs` arm only, `src/xmenu.c`'s `widget_id_tick` (1610-1611) and its three companions.
   Each is one declaration with the header's text; until they land, this unit carries its own
   copies.  (Integrator.)
5. **`docs/baseline/port-lines-pending.tsv`**: the `src/w32menu.c` ratchet row was **already
   removed** while this batch ran (Evidence ⑫: measured `stale` mid-batch, and the final run reads
   25 rows / 0 stale with no `w32menu` row left).  Nothing further to do - recorded so the next
   reader does not look for the row.  (Integrator: informational.)
6. **`menu.c`'s widget tree must be ported before this unit can link on a Windows target**: C 578,
   642, 797, 858, 906, 939, 950, 1115, 1152, 1173 and 1192 call `make_widget_value` (menu.c 571),
   C 608/671/692/980/1133/1147/1204 `free_menubar_widget_value_tree` (menu.c 593), C 585
   `digest_single_submenu` (menu.c 619) and C 633 `update_submenu_strings` (menu.c 846); this
   unit's `menu.rs` peer records that those five functions are **not** transcribed there
   (`src/menu.rs` 611-618), so the four externs in this unit are undefined symbols on a
   Windows link.  (Integrator: the menu.c lane.)
7. **The layout assertions for the `<windows.h>`/`<commctrl.h>` records** (`RECT`, `POINT`,
   `MENUITEMINFO`, `_TASKDIALOG_BUTTON`, `_TASKDIALOGCONFIG`, `SYSTEM_INFO`-class neighbours) and
   the measured values of the `MF_*`/`MIIM_*`/`MFT_*`/`MFS_*`/`MB_*`/`CF_*`-class constants, once
   a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's readings;
   `x86_64-w64-mingw32-gcc` is absent here).  (Integrator.)
8. **The `have_dialogs` arm's C-side defects** (Deviation 13) are the reference's own: nothing in
   this tree declares `menu`, `dialog_id`, `widget_id_tick`, `dialog_selection_callback`,
   `popup_get_selection`, `w32_output.widget` or `XEvent`.  Recorded, not fixable here.  (Upstream
   reference / the batch-F owner that ever builds that arm.)

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, GUI frame, `HMENU`, dialog or redisplay loop was involved.  There is no
  behavioural test, no differential run, no ERT and no device evidence.
- **The arm reading is not a Windows compile.**  This machine has no `windows.h` and no mingw
  toolchain, so Evidence ②/④/⑤ are `rustc` readings of the *materialized mirror graph* under a set
  of `--cfg`s.  13 error messages remain in the base arm, six of them anchored in this unit, and
  those six are named (Deviation 6); "6 in-unit anchors" does **not** mean the unit links, and it
  does not mean the arm is a working Windows configuration.  The mirrors are shared and moved
  underneath this batch (Evidence ④ measures that on the Cygwin row), so every count here is a
  reading of the graph at that moment, not a property of the reference.
- **The `<windows.h>`/`<commctrl.h>`/`<mbstring.h>` records and values stay unmeasured** (Owed 7):
  every one is `UNVERIFIED`, with the documented Windows values as its only source.
- **The `have_dialogs` arm is not a working configuration, in C or in the port** (Deviation 13).
  Its text is transcribed because P-86 ① requires both arms; nothing about it is claimed to
  compile, and nothing was run.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 1).  Nothing in this ledger claims
  the file's checklist is non-empty (P-86 ②) or that the port is complete.
