# emacs/src/w32select.c (1370 lines) — whole file, C 1-1370

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32select.o` is added to
> `W32_OBJ` by `emacs/src/Makefile.in` 483 (the non-Cygwin MinGW arm of `configure.ac` 3051's
> `HAVE_W32` split - the build whose `nt/inc/ms-w32.h` `#define`s `WINDOWSNT`), and
> `docs/baseline/frozen-build-sources.txt` lists no `w32select.o` for this target - so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/w32select.rs` (**2,003 lines**, 97,816 bytes, md5
> `26589f8c063e5c4274d4f375643a84f3`), carrying **C 1-1370 with both arms of every `#if`**.  The
> reference `emacs/src/w32select.c` is md5 `67bfa757dcb7cfd3bdbf97aa342fa776` (unchanged; `emacs/` is
> read-only).
>
> **This ledger overturns the reading it replaces.**  The previous `docs/ports/w32select.md` (and the
> 3-line-comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (P-08, `docs/status.md` 3.31).  P-86 ①
> replaces that conclusion with *transcribe the text, both arms, no stub*; what the earlier note
> measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object) is unchanged
> and still true.  The stub file was `femacs/src/w32select.rs`'s 3-line head plus the two carried
> comments (`docs/port-map.md` 583 still prints the 1,996-line reading of an earlier edit of this
> batch).
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no Homebrew
> `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is **no `<windows.h>` to compile
> or measure against**, no arm flip on a Windows target and no layout measurement: every record and
> value this file's stand-in section declares is `UNVERIFIED` (documented, not measured), and the
> layout assertions are owed.  What the batch *does* carry: the frozen `build-tu` reading, the
> machine-checked coverage table, the four `#if` verdicts, a per-item C citation, the `probe` reading
> of the `windowsnt` arm (whose findings are all in other files except the one mirror collision
> `## Findings` 2 names), and an arm-reach pre-flight over this unit's own text.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`'s MinGW arm ⇔ `nt/inc/ms-w32.h`'s `WINDOWSNT`) | `feature = "windowsnt"` (on every item *and* on the include graph) | the non-Cygwin Windows (MinGW) build |
| C 80 `#ifndef CF_DIBV5` | the C `#define` arm: `all (feature = "windowsnt", not (feature = "cf_dibv5"))`; the SDK arm carries the same two values | the frozen table has no `CF_DIBV5` ⇒ C's own arm |
| C 93 `#ifdef CYGWIN` (the two extra headers and `#define _memccpy memccpy`) | `feature = "cygwin"` | a Cygwin GUI build (the frozen table has `CYGWIN` `undef`) |
| C 144 `#if TRACE` (`ONTRACE`) | `feature = "trace"` / its negation | `TRACE` is defined nowhere in the tree or the frozen table ⇒ C's `#else` (the empty arm) |
| C 828 `#ifdef WINDOWSNT` (the GDI+ half of `convert_dibv5_to_png`) | `all (feature = "windowsnt", …)`; the `#else` arm (C 862-864) is `not (feature = "windowsnt")` | the file gate itself; the `#else` is dead on this build |

`tools/check-cond-eval.py src/w32select.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** - those four and nothing else - so the four rows above are the whole conditional text
of the file.  The `probe` door set is therefore that one live value per row
(`--cfg 'feature="windowsnt"'`, with `cf_dibv5`/`cygwin`/`trace` *off* because their live arms are the
negations).  The file is *not* on `docs/baseline/config/cond-eval-verified.txt`, so `extract-abi.py`
resolves no `#if` here (P-66); `--check` is nevertheless not vacuous (P-86 ⑩), see Evidence ④.

## C line ranges

The rows are contiguous and ascending and cover every non-blank line of C 1-1370: the lines between
two rows (C 22, 64, 73-74, 98, 117, 143, 149, 209, …) are the reference's blank lines, which the gate
does not require a row for.  `docs/port-map.md` 583-584 lists two targets for this stem
(`src/w32select.c` 1,370 and `src/w32select.h` 28, the latter bound to
`femacs/src/include/w32select.rs`): the table starts at C 1 and ends past the header's last line, so
both readings clear (the `docs/ports/w32heap.md` 574-575 shape; there is no `src-w32select.md`).

| C | block | arm |
| --- | --- | --- |
| 1-22 | the file's opening ownership/title block (C 22 is the blank line after it; the row reaches it because the header bound to this ledger, `src/w32select.h`, has `#include <windows.h>` on its line 22) | dropped (PORTING.md 0.1-19) |
| 23-63 | "Notes on usage of selection-coding-system and next-selection-coding-system on MS Windows" | carried as C's own text |
| 65-72 | "Ideas for further directions" | carried as C's own text |
| 73-97 | the include graph: C 75 `config.h`; C 76-79 `<windows.h>`/`<wingdi.h>`/`<wtypes.h>`/`<gdiplus.h>`; C 80-84 the `CF_DIBV5` block; C 85 `lisp.h`; C 86 `w32common.h`; C 87 `w32term.h`; C 88 `w32select.h`; C 89 `blockinput.h`; C 90 `coding.h`; C 91 `w32gdiplus.h`; C 93-97 the `CYGWIN` arm | file gate; C 80-84 `not (feature = "cf_dibv5")`; C 93-97 `feature = "cygwin"` |
| 99-116 | C's forward declarations of this file's 15 functions | dropped (PORTING.md 0.1-12; the C lines are recorded as `C:` text) |
| 118-142 | the pseudo-constants (`DEFAULT_LCID`, `ANSICP`/`OEMCP`, `QANSICP`/`QOEMCP`), `clipboard_owner`, `modifying_clipboard`, the configured and current transfer state | file gate |
| 144-148 | the `TRACE`/`ONTRACE` pair | `feature = "trace"` and its negation |
| 150-208 | `convert_to_handle_as_ascii` (C 151-153's comment included) | file gate |
| 210-248 | `convert_to_handle_as_coded` (C 210-213's comment included) | file gate |
| 250-296 | `render` | file gate |
| 298-329 | `render_locale` | file gate |
| 331-390 | `render_all` (C 331-333 and the C 355-381 format note included) | file gate |
| 392-413 | `run_protected` | file gate |
| 415-422 | `lisp_error_handler` | file gate |
| 424-460 | `owner_callback` | file gate |
| 462-475 | `create_owner` | file gate |
| 477-488 | `term_w32select` (C 477's comment included) | file gate |
| 490-568 | `setup_config` | file gate |
| 570-597 | `enum_locale_callback` | file gate |
| 599-617 | `cp_from_locale` | file gate |
| 619-628 | `coding_from_cp` | file gate |
| 630-656 | `validate_coding_system` | file gate |
| 658-677 | `setup_windows_coding_system` | file gate |
| 679-798 | `DEFUN w32-set-clipboard-data` | file gate |
| 800-822 | `stdfmt_name[]` (C 800-802's comment included) | file gate |
| 824-865 | `convert_dibv5_to_png` (C 824's comment included); C 828's `#ifdef WINDOWSNT` arm inside it | file gate; C 828 `feature = "windowsnt"`, C 862-864 its negation |
| 867-878 | `get_clipboard_format_name` | file gate |
| 880-963 | `DEFUN w32--get-clipboard-data-media` | file gate |
| 965-1180 | `DEFUN w32-get-clipboard-data` | file gate |
| 1182-1226 | `DEFUN w32-selection-exists-p` (C 1182's comment included) | file gate |
| 1228-1310 | `DEFUN w32-selection-targets` (C 1228's comment included) | file gate |
| 1312-1343 | `syms_of_w32select` (C 1312-1314's comment included) | file gate |
| 1345-1370 | `globals_of_w32select` (C 1345-1347's comment included) | file gate |

## Evidence

① **Frozen `build-tu`** (`cd femacs && CARGO_TARGET_DIR=/tmp/w32sel-lane cargo run -q -p xtask --
build-tu src/w32select.c`): one fail (two Rust syntax errors the braceless-C spellings produced -
`else match` and `if (…) Fsignal (…)`, both fixed to the block forms C's statements need), then
`built src/w32select.rs -> ../build/femacs/w32select.o`, **exit 0, 336 bytes** - the P-08 empty
object, which is what every item being `#[cfg (feature = "windowsnt")]` produces.  (Note the frozen
compile *parses* the whole unit: the syntax errors were caught with every item gated off.)  Budget:
`build-tu` spent **3** (within the ≤3 asked for).

② **The arm reading** (`cargo run -q -p xtask -- probe src/w32select.rs /tmp/w32sel-lane/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="windowsnt"'`): **exit 1, 97 errors, and 0 of them
anchored in this unit** (`_root.rs`: 0 - P-86 ⑦'s by-file reading).  By file: `src/include/w32term.rs`
53 (`HCURSOR` 18, `COLORREF` 7, `Emacs_GC` 5, `RECT` 5, `MSG`/`font`/`Emacs_Color` 7, 13 `unused doc
comment` warnings - names that mirror's own head note leaves to its includers, P-87 ③),
`lib/include/signal.rs` 13 (E0428 against `nt/inc/include/ms-w32.rs` - the `<signal.h>` a `windowsnt`
unit reaches comes from `include/lisp.rs` → `include/thread.rs`, the `docs/ports/w32proc.md` Evidence
② class), `src/include/buffer.rs` 8 (`STRING_CHAR`, `UNIBYTE_TO_CHAR`, `string_char_and_length` -
`character.h` names no mirror in its graph carries), `src/include/termhooks.rs` 5 (`Time`,
`Emacs_Color`), `src/include/frame.rs` 4 (`text_cursor_kinds`), **`src/include/w32select.rs` 3 (the
three entry points this unit defines - Findings 2)**, `src/include/w32gdiplus.rs` 3 (`VOID` and the
record names its typedef list uses), `nt/inc/include/ms-w32.rs` 2, `lib/include/errno.rs` 2
(`ENOTSUP`/`EOPNOTSUPP`), `src/include/w32common.rs` 2 (`SYSTEM_INFO`/`OSVERSIONINFO`, which that
mirror defers by its own note).  Budget: `probe` spent **3** (within the ≤3 asked for): 1 = the first
reading, 2 = the reading above (after the `## Findings` 3 workaround), 3 = the mutation test below.

③ **The mutation test (P-86 ⑦)** - the reading in ② is a *name-resolution* reading of this unit's
text, not a type-check.  Two mutations of this unit's own text, both compiled with the door set of ②:
(a) a **type** mutation (`cp_from_locale`'s `let mut cp: UINT;` → `let mut cp: *mut c_void;`) leaves
the output **byte-identical** (97 errors, 0 in the unit: `diff` of the two logs' error lines is
empty), so the unit's function bodies are not type-checked; (b) a **name** mutation of the same kind
of position (`cfg_codepage = ANSICP;` → `ANSICP_XX;`) *is* reported (+1 error, `cannot find value
ANSICP_XX`, in the unit's body).  So ②'s reading certifies that every name this unit's text uses
resolves inside this graph - including names used inside function bodies and inside `DEFUN!` bodies -
and that no item of this unit collides; it does **not** certify that the unit type-checks.  The
type-check of the bodies is what the include graph's own resolution errors stop (the `w32heap`/`w32proc`
reading of the same class).

④ **`python3 tools/extract-abi.py --check emacs/src/w32select.c femacs/src/w32select.rs`**: before the
`## Owed` 1 rows, `FAIL` with 9 findings: `does not mention` **8** (`convert_to_handle_as_ascii` `0`;
`render` `break,case,switch`; `run_protected` `0`; `owner_callback` `break,case,switch`;
`setup_windows_coding_system` `common_flags,mode`; `syms_of_w32select` `doc`;
`Fw32_set_clipboard_data` `done,error,goto`; `Fw32_get_clipboard_data` `goto`) and `line count drifted
46.2%: C 1370 vs Rust 2003` 1.  The 8 are `docs/ports/macro-body-notes.tsv` rows; **they are written
(in this batch's write set, appended at the end of that file)** and the re-run leaves exactly the
line-drift row, which is `docs/ports/line-drift-tolerance.tsv`'s and is owed (that file is not in this
lane's write set).

⑤ **`python3 tools/check-cond-eval.py src/w32select.c`**: `4 conditions, 0 unknown, 0 disagreements
[VERIFIED]` - the Gate-mapping rows, nothing else.

⑥ **`python3 tools/check-declarations.py`**: `17 declaration sites (0 forbidden, 17 allowed literals),
0 failures`, `read 683 files` - the unit's first line is the `// femacs: port of …` form and no
comment it carries states anything about this repository (the carried C comments say "Emacs", which
is not one of the gate's patterns; the two C comments that say "MS Windows" and "xselect.c" are the
reference's own text).

⑦ **`python3 tools/check-mirror-dupes.py`**: `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` - unchanged: this unit's declarations are in no mirror's top level except the three the
mirror *does* carry, and those three are definitions (which the gate does not report).

⑧ **`python3 tools/check-citations.py`**: `676 file(s) scanned, 1 drift, 0 range, 1160 external, 35
allowed, 0 stale` - the one drift is `femacs/src/xfont.rs:388` (another lane's file), so this unit's
citations (`/// C:`/`// C:` lines, ~150 of them) are all in range and current.

⑨ **`python3 tools/check-dead-file-coverage.py`**: `33 ledger(s) bound (a C range table), 0
failure(s)`, this file's own row being `info docs/ports/w32select.md: 33 range row(s) cover
src/w32select.c (1370 lines; 32 blank line(s) between blocks need no row)`.

⑩ **`python3 tools/check-port-lines.py`**: `37 file(s) under ten percent, 57,503 reference
line(s); 37 ratchet row(s), 0 new, 0 stale` - this file is **not** in
`docs/baseline/port-lines-pending.tsv` (146% of its reference), so no ratchet row applies.

⑪ **The unit**: `femacs/src/w32select.rs` 2,003 lines, 97,816 bytes, md5
`26589f8c063e5c4274d4f375643a84f3`; the reference `emacs/src/w32select.c` 1,370 lines, md5
`67bfa757dcb7cfd3bdbf97aa342fa776`.

⑫ **The arm-reach pre-flight** (this lane's stub set: a `/tmp` copy of `femacs/{src,lib,nt}` with
`include/w32select.rs`'s three declarations moved into a glob-imported `pub module` - the one mirror
patch Findings 2 records - and a `/tmp` materializer that expands the unit's `include!` graph once per
file): **the unit's own text has 0 findings**, and every finding (118 of them) is in another file, the
same set ② lists.  A **type** mutation of the unit is invisible here too; a **name** mutation is
reported - the same resolution-only boundary as ③, measured twice.

## Findings

1. **The reference translation unit calls `filename_to_utf16` (C 856) with no visible prototype.**
   The only declaration is `w32.h` 209, and neither C 85's `lisp.h` chain (`comp.h`, `thread.h`,
   `globals.h`) nor C 87's `w32term.h` chain (`w32gui.h`/`frame.h`/`atimer.h`, `w32common.h` 24)
   reaches `w32.h` - so on a C99-or-later compiler this is an implicit declaration (a hard error on
   GCC 14+; a warning before).  **Attribution: the reference side.**  The port cannot be "one-to-one"
   and prototype-less at once, so it declares the function at its use position with the header's own
   text (Deviation 4) and marks it as a finding for whoever owns the reference-side question.  The
   same class, not acted on here: C's own `#include <string.h>`/`<stdlib.h>` come only through
   `lisp.h` 30 and `ms-w32.h` 410/271.
2. **`femacs/src/include/w32select.rs` declares the three entry points at crate root** (`syms_of_…`,
   `globals_of_…`, `term_w32select`), and this unit *defines* all three - the P-39/E0428 shape
   (`femacs/src/include/w32gui.rs` 88-108 is the fixed form: `pub mod …_decls` + `use …::*;`).
   Measured: in the real `probe` the three errors are attributed to that mirror, and with the mirror
   patched in ⑫'s copy the unit's own text has no findings.  **Attribution: the mirror** (its head
   note's "no unit `include!`s this mirror" is stale too - this unit does, at C 88).  Unfixed here:
   the mirror is outside this lane's write set; the fix is one module wrapper in that file.
3. **`xtask`'s `materialize` leaves a degraded gated `include!` edge's `#[cfg]` attached to the next
   item** (`femacs/xtask/src/main.rs` 1861-1864: P-89 ③'s "expands at its ungated reach" branch emits
   `// [xtask dedupe] …` for the edge and leaves the attribute line that preceded it in place, so the
   attribute ANDs onto whatever item follows).  Measured: with C 94-95 spelled as the faithful
   `#[cfg (feature = "cygwin")] include!(…)` pair, this unit's own `unsafe extern` block became
   `cygwin`-gated and four declarations vanished - `probe` printed four `E0425`s **in this unit's
   text** (`_memccpy` twice, `strstr`, `cmd_error_internal`, `internal_condition_case_1`), all four an
   artifact of the transformation rather than of the unit (the generated `_root.rs` shows the two
   dangling `#[cfg (feature = "cygwin")]` lines with nothing between them).  **Attribution: the
   tool**, exposed by this file (it is the first unit with a gated edge whose target also has an
   ungated reach).  Worked around by Deviation 3; the tool fix is either to consume the attribute line
   with the edge or to emit the degraded edge as a self-contained cfg'd item.
4. **The include graph's own Windows arm has never been compiled** (②'s by-file list: `w32term.rs`'s
   `HCURSOR`/`COLORREF`/`RECT`/`MSG`/`font`/`Emacs_GC`/`Emacs_Color`, `buffer.rs`'s `STRING_CHAR`
   family, `termhooks.rs`'s `Time`/`Emacs_Color`, `frame.rs`'s `text_cursor_kinds`, the four
   `E0428` pairs `signal.rs`/`errno.rs`/`ms-w32.rs`/`config.rs`, `w32gdiplus.rs`'s `VOID` and record
   names, `w32common.rs`'s `SYSTEM_INFO`/`OSVERSIONINFO`).  **Attribution: those mirrors** (each
   mirror's own note predicts some of them); the Windows round that flips this arm lifts all of them
   (Owed 3).  This unit's own stand-in section already supplies the `<windows.h>` names *its* text
   reaches.

## Deviations

1. **C 1-21 and C 99-116 have no counterpart**: the opening ownership/title block is dropped
   (PORTING.md 0.1-19) and the fifteen forward declarations are dropped (PORTING.md 0.1-12 - a
   declaration plus a definition is `E0428` and items are order-independent).  C 99-116's own text is
   kept as `C:` lines at its position.
2. **The `<windows.h>`/`<wingdi.h>`/`<wtypes.h>`/`<gdiplus.h>` stand-in section** (C 76-79): the
   reference tree has no file for any of the four, so the names this file uses stand at C's own
   `#include` line (P-88 ①, no file added under `femacs/src/include/`), each with the SDK's own text
   above it.  The spellings repeat `src/w32fns.rs`'s and `src/w32proc.rs`'s sections, so a `windowsnt`
   translation unit sees one set of names.  **Nothing was measured** - `DWORD`, `LONG`, `BOOL`,
   `SIZE_T`, `WPARAM`, the handle typedefs, `MAX_PATH`, the `CF_*`/`WM_*`/`LOCALE_*`/`GMEM_*` values,
   `WNDCLASS`, `GUID`/`CLSID`, `BITMAPV5HEADER`/`RGBQUAD`/`CIEXYZTRIPLE`, the GDI+ `GpStatus`/`Ok`/
   `RotateFlipType`/`PixelFormat32bppARGB` values and the five opaque GDI+ records are the documented
   Windows/GDI+ ones, **UNVERIFIED**, and the layout assertions are owed.  `DWORD_PTR` is
   `include/w32common.rs`'s and is deliberately not repeated (a second declaration is `E0428`).
3. **C 93-97's two `include!` edges are carried as C's own lines** (`// C 94: #include <string.h>`,
   `// C 95: #include <stdio.h>`) rather than as `include!` edges: their targets are already in the
   graph at C 85, and the faithful gated spelling trips Findings 3 (four spurious `E0425`s in this
   unit's text, measured).  The arm is dead in every configuration this tree builds and its content is
   the two duplicate includes; the `_memccpy` rename it installs is kept as the C line and in
   Deviation 4's note.
4. **Six names are declared in-unit because no mirror in this graph carries them** (each with the
   declaring header's own text; Owed 2 names the mirror rows): `internal_condition_case_1` and
   `cmd_error_internal` (lisp.h 4806/5099; `include/lisp.rs` has neither), `strstr` (`<string.h>`;
   `lib/include/string.rs` has `memcpy`/`memset`/`memcmp`/`strlen`/`strcmp`/`strcpy`/`memchr` but not
   this one), `_memccpy` (the runtime's own name; C's Cygwin arm renames it to `memccpy`, which no
   mirror carries either), `setup_coding_system` (coding.h 686; `include/coding.rs` has
   `encode_coding_object`/`decode_coding_object` but not this one) and `filename_to_utf16` (Findings
   1).  The `unsafe extern` blocks that hold them have `//` rather than `///` comments: rustdoc
   documents no extern block and the `///` spelling prints `unused doc comment` for each (the same
   warnings `include/w32term.rs` carries).
5. **C 154-208's `next[0]` is the port's `*next`** and C 197's `dst[coding.produced] =
   dst[coding.produced+1] = '\0';` is two stores: Rust writes the indexed store with `offset (…)`, and
   a chained assignment has no Rust form.  Both are recorded in `docs/ports/macro-body-notes.tsv`
   (the `convert_to_handle_as_ascii` row).
6. **C's two `switch`es are two `match`es** (`render` C 266-279, `owner_callback` C 428-457) with the
   `case` order kept and a `_ => {}` arm for C's implicit `default`; `WM_DESTROY`'s `break` is the
   empty arm that falls through to C 459's call (`macro-body-notes.tsv`'s `render`/`owner_callback`
   rows, PORTING.md 2.2).
7. **C's five `goto`s are two labelled blocks**: `Fw32_set_clipboard_data`'s `'flow` (whose error
   block sits at the end of the block so the `!ok` fall-through reaches it, C 777-779) and
   `Fw32_get_clipboard_data`'s `'done`/`'closeclip` nesting (C 990/1009/1016/1019), each `break`
   being C's jump.  Recorded in `macro-body-notes.tsv`.
8. **`setup_windows_coding_system`'s two bitfield writers** (C 676-677) are two masked writes into
   the mirror's `bits: u32` storage unit (`include/coding.rs` 244-268 has the shifts/masks and the
   readers; the unit `src/coding.rs`'s `coding_bits_set` is another unit's private helper and is not in
   this graph).  Recorded in `macro-body-notes.tsv`.
9. **Zero-initialised locals** (PORTING.md 1.5): `convert_to_handle_as_coded`'s `struct
   coding_system coding` (C 218), `w32-get-clipboard-data`'s `coding` (C 1046),
   `convert_dibv5_to_png`'s `CLSID clsid_png` (C 829), `create_owner`'s `WNDCLASS wc` (C 466) and
   `coding_from_cp`'s `char buffer[30]` (C 621) become `core::mem::zeroed ()`/`[0; 30]`; the file-scope
   `static`s (C 120-142) are zeroed by the same rule and a `Lisp_Object` is `lisp_h_Qnil ()`.
10. **`render` is `unsafe extern "C" fn`** (C 250): C passes its address to `run_protected` and to
    `internal_condition_case_1`, so PORTING.md 1.4's C-ABI rule applies (measured: the `unsafe fn`
    spelling is `E0308` - "expected `unsafe extern "C" fn`, found fn item" - which is the one real
    unit-side type error this batch fixed).
11. **`run_protected`'s `int owfi` is a `bool`** (C 399/405-410): `thread.h` 184's member is a `bool`
    and `waiting_for_input` is its macro (C stores 0/1), so the port copies the flag as a `bool` and
    writes `false`/`owfi` back; `0` is therefore absent from the body (the `macro-body-notes.tsv`
    `run_protected` row).
12. **`Fw32__get_clipboard_data_media`'s `static char name[256]`** is the function-static C declares
    (C 921) and `temp_file_in` is re-bound (`let mut … = temp_file_in`) because C 907 reassigns the
    parameter (the `src/androidselect.rs` 277 shape).
13. **`Fw32_selection_targets`'s `char fmt_name[256]` (C 1293) is hoisted out of the `else` arm** to
    the top of the loop body: C 1298 stores its address in `name` and C 1300 uses it after the arm's
    block closes, while a Rust local's storage ends with its block.  Same bytes, same loop, one scope
    level up.
14. **`render`'s C 284 trace and `render_all`'s C 337 trace** are `ONTRACE!` invocations whose
    argument text is C's `fprintf (stderr, …)` - the dead `trace` arm, carried as C's text (it is not
    compiled in any configuration this tree builds: `TRACE` is defined nowhere).
15. **`stdfmt_name` is a `[&CStr; 18]`** rather than C's `const char *[]`: the port's spelling of a C
    string literal is `c"…"` (a `&CStr`) and a use site takes `.as_ptr ()`; the bytes are C's own.
    `CLASSNAME` in `create_owner` is the same spelling (`c"Emacs Clipboard"`), C's `char[]`.
16. **`convert_dibv5_to_png`'s GDI+ calls go through the mirror's name macros**:
    `include/w32gdiplus.rs` 291-386 carries C 95-141's `#undef`/`#define` pairs as
    `macro_rules! GdipCreateBitmapFromScan0!` etc. (each yielding the `fn_*` object C's `#define`
    names), so a call site is `(GdipCreateBitmapFromScan0! ().unwrap ()) (…)` - C's own `#define` plus
    the port's nullable function pointer.  The `GpBitmap **` argument and the `GpImage *` parameter
    agree because both handle names are `*mut c_void` (Deviation 2).
17. **`bmi->bV5SizeImage / bmi->bV5Height`** (C 836) spells C's usual arithmetic conversion (the `LONG`
    operand becomes unsigned): the port writes `((*bmi).bV5Height as DWORD)`.
18. **`(void) ignored;`** (C 693, C 978) is `let _ = ignored;`.
19. **`QANSICP`/`QOEMCP`** are this file's own `static mut Lisp_Object`s (C 122) - not the generated
    `globals.rs` set - while `QCLIPBOARD`, `Qutf_16le_dos`, `Qraw_text`, `Qerror` and the rest come
    from `include/globals.rs` as the `Q…()` functions C's `DEFSYM`s name (`Qerror` is therefore
    `Qerror ()`, measured as `E0308` before the fix).  `DEFSYM!` itself is the empty macro the mirror
    records (lisp.h 2394).
20. **`syms_of_w32select`/`globals_of_w32select`/`term_w32select` are `#[unsafe(no_mangle)] pub unsafe
    extern "C" fn`** (PORTING.md 0.2/P-52: C's non-`static` definitions), which is what collides with
    the mirror (Findings 2).

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32select`, tolerance ≥ 0.462,
   basis `total`, reason from Deviations 1-20 (the stand-in sections, the per-item `/// C:` citations,
   the dropped forward declarations, the four in-unit declarations, the labelled blocks' extra
   `break` lines).  That file is not in this lane's write set, so `extract-abi.py --check` stays `FAIL`
   with exactly the one row Evidence ④ records.  (`docs/ports/macro-body-notes.tsv`'s eight rows are
   **written** in this batch, per that file's "add rows only" permission.)
2. **The mirror rows for the six names of Deviation 4**: `include/lisp.rs` (`internal_condition_case_1`,
   `cmd_error_internal` - lisp.h 4806/5099), `lib/include/string.rs` (`strstr`, `<string.h>`;
   `_memccpy`'s runtime name has no header in the tree), `include/coding.rs` (`setup_coding_system`,
   coding.h 686), `include/w32.rs` (`filename_to_utf16`, w32.h 209) and `include/w32.h`-side
   `MAX_PATH` (Deviation 2's stand-in).  Each row is one declaration with the header's text; until they
   land, this unit carries its own copies.
3. **`femacs/src/include/w32select.rs`'s three crate-root declarations** (Findings 2): move them into
   `pub mod w32select_decls { use super::*; … } use w32select_decls::*;` - the `include/w32gui.rs`
   88-108 form - and refresh its head note (this unit *does* `include!` it, at C 88).  With that patch
   applied in a `/tmp` copy, this unit's own text has no findings (Evidence ⑫).
4. **The Windows round that flips this arm**: it lifts ②'s by-file list (the four `E0428` pairs, the
   names `w32term.rs`/`buffer.rs`/`termhooks.rs`/`frame.rs`/`w32gdiplus.rs` leave to their includers,
   `w32common.rs`'s two deferred records), measures every record Deviation 2 marks `UNVERIFIED`
   (`sizeof (WNDCLASS)`, `sizeof (BITMAPV5HEADER)` and the five `bV5*` offsets it reads, `sizeof
   (CLSID)`, `MAX_PATH`, the `CF_*`/`GMEM_*`/`WM_*`/`LOCALE_*`/`CMP_*` values, and the GDI+
   `RotateNoneFlipY`/`PixelFormat32bppARGB` values), runs the file on a real clipboard, and re-runs the
   mutation test of Evidence ③ to upgrade the reading from name resolution to a type-check.
5. **Findings 1 and 3** are not this lane's to fix (the reference TU's prototype-less call; the
   `xtask` attribute-swallowing).  Both are recorded with their measurement so the integrator can route
   them.

## Not claimed

- **The frozen configuration compiles nothing here** and the 336-byte object is the empty object; the
  port is not "built" by any reading of this tree.
- **The arm reading is a name-resolution reading of this unit's text, not a compile of this unit**:
  Evidence ② shows 0 findings in the unit, and Evidence ③'s two mutations show that this reading
  covers the unit's names (including inside bodies) but not its types; the graph's own resolution
  errors are what stop the type-checker.
- **No Windows compile, no link, no run, no runtime behaviour**: the clipboard handling, the locale
  enumeration, the delayed rendering, the CRLF conversion and the DIBV5→PNG path are transcribed, not
  exercised; no Windows binary was produced.
- **Every record/value the stand-in section declares is UNVERIFIED** (documented, not measured), and no
  `sizeof`/`offsetof` assertion exists for any of them.
- **Nothing about the reference's behaviour is claimed from this ledger**: the C line-range table, the
  `#if` verdicts, the gate readings and the mutation test are the whole evidence.
- **The three entry points' `E0428` is not "the unit's error"**: with the mirror patched in a `/tmp`
  copy the unit's text is clean (Evidence ⑫); the finding is the mirror's crate-root spelling.
