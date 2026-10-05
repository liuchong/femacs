# emacs/src/w32font.c (3079 lines) — whole file, C 1-3079

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32font.o` sits in
> `W32_OBJ` (`emacs/src/Makefile.in` 316-319, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac` 3030
> under the `HAVE_W32` test whose 3024 `AC_DEFINE`s `HAVE_NTGUI`), and
> `docs/baseline/frozen-build-sources.txt` lists no `w32font.o` for this target — so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `femacs/src/w32font.rs` (**3967 lines**, md5 `b0c655d37fa9b2ebd20c807c4f8af48b`), carrying
> **C 1-3079 with both arms of every `#if`**.  The reference `emacs/src/w32font.c` is md5
> `50838ad7c768e97ef32b07eea3f2a907` (unchanged; `emacs/` is read-only).
>
> **This ledger overturns the reading it replaces.**  The previous `docs/ports/w32font.md` (and the
> 11-line-comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu`
> object) is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32select.md` and `docs/ports/w32term.md` record for their own pairs.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`), so there is **no `<windows.h>`
> to compile or measure against**, no arm flip on a Windows target and no layout measurement: every
> record and value the stand-in sections declare is `UNVERIFIED` (documented, not measured) and the
> layout assertions are owed.  What the round does carry: the frozen `build-tu` reading, the
> machine-checked coverage table, the `#if` verdicts, a per-item C citation, the `probe` reading of
> the `windowsnt` arm (0 errors anchored in this unit), a **free multi-configuration sweep** of the
> same materialized text, and a **mutation test** over this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, i.e. `HAVE_W32` → `HAVE_NTGUI`) | `feature = "have_ntgui"` (on every item *and* on C 19-36's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 31-34 `#ifdef WINDOWSNT` / `w32common.h` + `w32.h` | `all (feature = "have_ntgui", feature = "windowsnt")`; the `#else` arm is empty | the non-Cygwin (MinGW) Windows build |
| C 42-44 `#ifndef CLEARTYPE_QUALITY` / `#define … 5` | `not (feature = "cleartype_quality")` for C's arm; the SDK arm (`feature = "cleartype_quality"`) carries the same value | the frozen table has no such macro ⇒ C's own `#define` |
| C 45-47 `#ifndef CLEARTYPE_NATURAL_QUALITY` / `#define … 6` | `not (feature = "cleartype_natural_quality")` / `feature = "cleartype_natural_quality"` | as above |
| C 51-53 `#ifndef VIETNAMESE_CHARSET` / `#define … 163` | `not (feature = "vietnamese_charset")` / `feature = "vietnamese_charset"` | as above |
| C 54-56 `#ifndef JOHAB_CHARSET` / `#define … 130` | `not (feature = "johab_charset")` / `feature = "johab_charset"` | as above |
| C 105-218 `#ifdef WINDOWSNT` (the four `g_b_init_*` flags, the four `WINAPI` proc typedefs, `w32_load_unicows_or_gdi32` and the four wrappers) | `all (feature = "have_ntgui", feature = "windowsnt")` | the non-Cygwin Windows build |
| C 220-230 `#else /* Cygwin */` (the four `#define`s that call the SDK directly) | `all (feature = "have_ntgui", not (feature = "windowsnt"))` | the Cygwin GUI build |
| C 371-401 `#if 0` (the script bitmap body of `w32font_has_char`) | none: dead text in every configuration, carried as C's own text inside a comment (the `cm.rs` 86 reading) | never |
| C 1583-1585 `#ifndef WINDOWSNT` / `#define _strlwr strlwr` | `all (feature = "have_ntgui", not (feature = "windowsnt"))`; the `windowsnt` arm declares the SDK's `_strlwr` | the Cygwin GUI build |
| C 3073-3078 `#ifdef WINDOWSNT` (the four `g_b_init_* = 0` assignments) | `all (feature = "have_ntgui", feature = "windowsnt")`; the Cygwin arm's text is the empty `#else` | the non-Cygwin Windows build |

`tools/check-cond-eval.py src/w32font.c` reads **`9 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** — the nine rows above (C 31, 42, 45, 51, 54, 105, 371, 1583, 3073) and nothing else,
so this is the whole conditional text of the file.  The `probe` door set is therefore
`have_ntgui` + `windowsnt` (+ `have_window_system`, see Owed 1), with the other four features off
because their live arms are the negations.

## C line ranges

The rows are contiguous and ascending and cover every non-blank line of C 1-3079; the lines between
two rows (C 18, 37, 41, 48, 57, 69, 80, 104, …) are the reference's blank lines, which the gate does
not require a row for.  `docs/port-map.md` 570-571 lists two targets for this stem
(`src/w32font.c` 3,079 and `src/w32font.h` 120, the latter bound to `femacs/src/include/w32font.rs`):
the table starts at C 1 and ends past the header's last line, so both readings clear (the
`docs/ports/w32heap.md` 574-575 shape; there is no `src-w32font.md`).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19) |
| 19-37 | the include graph: `config.h` (C 19), `<windows.h>` (C 20), `<stdio.h>`, `<math.h>`, `<ctype.h>`, `<commdlg.h>`, `lisp.h`, `w32term.h`, `frame.h`, `coding.h`, `w32font.h`, C 31-34's `WINDOWSNT` pair, `pdumper.h` | file gate; C 32-33 `all (have_ntgui, windowsnt)` |
| 38-41 | the Cleartype design comment | file gate |
| 42-48 | C 42-44's `CLEARTYPE_QUALITY` guard and C 45-47's `CLEARTYPE_NATURAL_QUALITY` guard | file gate; each `not (feature = …)` |
| 49-57 | the MSVC-charset comment and C 51-56's `VIETNAMESE_CHARSET`/`JOHAB_CHARSET` guards | file gate; each `not (feature = …)` |
| 58-69 | the seven file-scope forward declarations | dropped (PORTING.md 0.1-12), text kept as `C:` lines |
| 70-80 | the three `CALLBACK ALIGN_STACK` callback declarations | same |
| 81-100 | `struct font_callback_data` | file gate |
| 101-104 | the `list_all_matching_fonts` prototype | dropped |
| 105-231 | the `#ifdef WINDOWSNT`/`#else /* Cygwin */` block: the four flags, the four proc typedefs, `w32_load_unicows_or_gdi32` and the four wrappers | file gate; C 105-218 `windowsnt`, C 220-230 `not (windowsnt)` |
| 232-244 | `intern_font_name` | file gate |
| 245-254 | `w32font_get_cache` | file gate |
| 255-265 | `w32font_list` | file gate |
| 266-277 | `w32font_match` | file gate |
| 278-309 | `w32font_list_family` | file gate |
| 310-333 | `w32font_open` | file gate |
| 334-362 | `w32font_close` | file gate |
| 363-406 | `w32font_has_char`, incl. C 371-401's `#if 0` | file gate; the `#if 0` region carried as C's text |
| 407-428 | `w32font_encode_char` | file gate |
| 429-621 | `w32font_text_extents` | file gate |
| 622-627 | the draw-method banner comment | file gate |
| 628-741 | `w32font_draw` | file gate |
| 742-816 | the six optional-method design comments (`free_entity`, `prepare_face`, `done_face`, `get_bitmap`, `free_bitmap`, `anchor_point`, `otf_capability`, `otf_drive`) | carried as C's own text |
| 817-903 | the "Notes about the way fonts are found on MS-Windows" design comment | carried (an extract; see Deviations 8) |
| 904-987 | `w32font_list_internal` | file gate |
| 988-1042 | `w32font_match_internal` | file gate |
| 1043-1173 | `w32font_open_internal` | file gate |
| 1174-1192 | `add_font_name_to_list` | file gate |
| 1193-1196 | `w32_decode_weight`/`w32_encode_weight` prototypes | dropped |
| 1197-1307 | `w32_enumfont_pattern_entity` | file gate |
| 1308-1325 | `w32_generic_family` | file gate |
| 1326-1345 | `logfonts_match` | file gate |
| 1346-1349 | the three `CSB_*` macros | file gate |
| 1350-1545 | `font_matches_spec` | file gate |
| 1546-1582 | `w32font_coverage_ok` and its four `SUBRANGE1_*` masks | file gate |
| 1583-1586 | the `_strlwr` guard | C 1583-1585 `not (windowsnt)`; the `windowsnt` arm declares the SDK's `_strlwr` |
| 1587-1631 | `check_face_name` | file gate |
| 1632-1774 | `add_font_entity_to_list` | file gate |
| 1775-1788 | `add_one_font_entity_to_list` | file gate |
| 1789-1870 | `x_to_w32_charset` | file gate |
| 1871-1885 | `registry_to_w32_charset` | file gate |
| 1886-2079 | `w32_to_x_charset` | file gate |
| 2080-2093 | `w32_registry` | file gate |
| 2094-2108 | `w32_decode_weight` | file gate |
| 2109-2125 | `w32_encode_weight` | file gate |
| 2126-2140 | `w32_to_fc_weight` | file gate |
| 2141-2288 | `fill_in_logfont` | file gate |
| 2289-2331 | `list_all_matching_fonts` | file gate |
| 2332-2358 | `lispy_antialias_type` | file gate |
| 2359-2374 | `w32_antialias_type` | file gate |
| 2375-2600 | `font_supported_scripts` (the `SUBRANGE`/`MASK_ANY` macros, their invocations and C 2592-2593's `#undef`s) | file gate |
| 2601-2673 | `w32font_full_name` | file gate |
| 2674-2712 | `logfont_to_fcname` | file gate |
| 2713-2751 | `compute_metrics` | file gate |
| 2752-2818 | the `x-select-font` `DEFUN` | file gate |
| 2819-2829 | `w32font_booleans` / `w32font_non_booleans` | file gate |
| 2830-2835 | `w32font_filter_properties` | file gate |
| 2836-2870 | `w32font_driver` | file gate |
| 2871-3063 | the `syms_of_w32font_for_pdumper` prototype (C 2871, dropped) and `syms_of_w32font` (C 2873-3062, incl. C 3017-3035's `DEFVAR_LISP` docstring) | file gate |
| 3064-3069 | `syms_of_w32font_for_pdumper` | file gate |
| 3070-3079 | `globals_of_w32font` | file gate; C 3073-3078 `windowsnt` |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32f-lane cargo run -q -p xtask -- build-tu src/w32font.c`
→ `built src/w32font.rs -> ../build/femacs/w32font.o`, **exit 0**, and `build/femacs/w32font.o` is
**336 bytes** - the P-08 empty object, which is what every item being `#[cfg (feature =
"have_ntgui")]` produces.  Budget: `build-tu` spent **3** (the first two runs of the same command
exited 1 with syntax errors in the unit's text and wrote no object - the text was still being
written; only the third is the reading above).  The reading was confirmed on the **final
revision** without spending a fourth `build-tu`: the same compile driven directly
(`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`
on the materialized `_root.rs`, no `--cfg`) exits 0 and writes **336 bytes**.

**② The arm reading (`xtask probe`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32f-lane cargo run -q -p xtask -- probe src/w32font.rs /tmp/w32f-lane/base2.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="have_window_system"'`
→ **exit 1, 21 error lines, and 0 of them anchored in this unit** (`_root.rs`: 0 - P-86 ⑦'s
by-file reading).  By file: **`src/include/w32font.rs` 10** (the `E0428` pairs at its 151, 152, 154,
156, 158, 159, 160, 163, 166, 176 - the ten functions this unit defines, which that mirror declares
at crate root; the unit's definitions are the secondary anchors), **`src/include/w32term.rs` 4**
(`HWND` at 1406, 1409, 1411, 1451 - a name that mirror's own note leaves to its includers),
**`src/include/w32common.rs` 2** (`SYSTEM_INFO` 111, `OSVERSIONINFO` 113 - that mirror's note
defers them), **`lib/include/signal.rs` 2** (`sigset_t`/`__sigbits` at 148/156), **`src/include/frame.rs` 1**
(the `have_ntgui` `compile_error!` at 329 - that mirror's own refusal note).  The reading was **re-taken on the
final revision** (`feature = "have_ntgui"`, `windowsnt`, `have_window_system`; sha256
`12ef2fd79327fb0567b8078a01dcc89ebf072e8f9b7a42899be740b24d641c7e`) and the `diff` of the two
logs' `error` lines is **empty**: 21 error lines, 0 in the unit, the same by-file split (the
`#[unsafe(no_mangle)]` Evidence ⑤ required on `w32font_get_cache` did not move it).  Budget:
`probe` spent **5** (reported honestly, two over the ≤3 asked for): 1 = the first
`have_ntgui`-only reading of the file while the body still had defects, 2 = the pre-fix
`have_ntgui`+`windowsnt`+`have_window_system` reading (142 errors, 128 anchored in the unit - the
115 pointer-indexing ones and the rest), 3 = the reading above, 4 = the mutation test below, 5 =
the re-take on the final revision.

**③ The free multi-configuration sweep (direct `rustc` on the materialized root; not a
Windows-target compile).**  `xtask probe` materializes the unit into `build/probes/w32font/`; a copy
of that directory in `/tmp/w32f-lane` was compiled once per configuration with
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`,
and the errors are counted by primary `-->` anchor.

| configuration | exit | `error` lines | anchored in the unit |
| --- | --- | --- | --- |
| `have_ntgui` | 1 | 17 | 15 |
| `+ have_window_system` | 1 | 6 | 4 |
| `+ windowsnt + have_window_system` (base) | 1 | 20 | **0** |

The `have_ntgui`-only rows are the Cygwin arm, and every in-unit anchor there is a name a mirror
gates more narrowly than C does: the ten `struct font` members and `glyph_string_bits!` that
`include/dispextern.rs` carries only under `have_window_system` (C's GUI builds always have
`HAVE_WINDOW_SYSTEM`), and the four `FRAME_W32_WINDOW`/`FRAME_FONT` uses that `include/w32term.rs`
gates on `windowsnt` while C's `w32term.h` defines them for **both** arms of the Windows build (the
first four `#include`s of that mirror's own note).  With the base set the unit's lines are clean.
The frozen row of that table is Evidence ① (0 errors, 336-byte object).

**④ The mutation test (P-86 ⑦): the reading is not vacuous.**  One type error injected into this
unit's own text - `w32font_open`'s `(*w32_font).glyph_idx = 0;` became
`(*w32_font).glyph_idx = std::ptr::null_mut ();` - and the **same** `probe` command re-run: exit 1,
**22** error lines against the base reading's 21, the single new one
`error[E0308]: mismatched types`, anchored in the unit (`_root.rs:1044`, the mutated line); the
`diff` of the two logs' `error` lines is exactly `+error[E0308]: mismatched types`.  The mutation
was reverted and the unit verified: `shasum -a 256 femacs/src/w32font.rs` =
`6a19666d498c83467094e832047dfe53359ea288530b4166c6bf433fa0b279ae` before and after.  (That
revision is the one the mutation test ran on; the only later change is the
`#[unsafe(no_mangle)]` this round's own Evidence ⑤ finding required on `w32font_get_cache`, and the
final revision's sha256 is `12ef2fd79327fb0567b8078a01dcc89ebf072e8f9b7a42899be740b24d641c7e`.)  So ②'s
reading certifies the unit's own lines type-check under that arm (not merely resolve) and that no
item of this unit collides.

**⑤ `python3 tools/extract-abi.py --check emacs/src/w32font.c femacs/src/w32font.rs`**: before this
round's `docs/ports/macro-body-notes.tsv` rows, `FAIL` with 17 findings (16 `does not
mention …` rows plus `function w32font_get_cache is not static in C but has no
#[unsafe(no_mangle)] in the port`) and the line-drift row; the 16 note rows **are written** (that
file is in this lane's write set, appended at its end) and the `no_mangle` one was fixed in the
unit, so the re-run leaves exactly **one** finding:
`FAIL line count drifted 28.8%: C 3079 vs Rust 3967` - `docs/ports/line-drift-tolerance.tsv`'s row
(not in this lane's write set; Owed 2).

**⑥ `python3 tools/check-cond-eval.py src/w32font.c`**: `9 conditions, 0 unknown, 0 disagreements
[VERIFIED]` - the Gate-mapping rows, nothing else.

**⑦ `python3 tools/check-declarations.py`**: `17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures`, `read 683 files (625 .rs 40 .kt 7 .swift 6 .s 5 .cc)` - this unit and this
ledger carry no finding.

**⑧ `python3 tools/check-mirror-dupes.py`**: `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` - unchanged (the same three numbers the last landed Windows unit reported), and no row
names `w32font`: this unit's only top-level names are the ten definitions the `include/w32font.rs`
mirror already *declares* (definitions are not what this gate reports), and everything else is
`static`/private.

**⑨ `python3 tools/check-citations.py`**: `676 file(s) scanned, 0 drift, 0 range, 1512 external, 36
allowed, 0 stale` - this unit's ~150 `/// C:`/`// C` citations and this ledger's are all in range
and current.

**⑩ `python3 tools/check-dead-file-coverage.py`**: `101 ledger(s) bound (a C range table), 0
failure(s)`.  This ledger's own rows: `61 range row(s) cover src/w32font.c (3079 lines; 0 blank
line(s) between blocks need no row)` **and** `61 range row(s) cover src/w32font.h (120 lines; 0
blank line(s) between blocks need no row)` - both readings clear, because the table's ranges tile
C 1-3079 including the blank separator lines (`docs/ports/w32select.md`'s 1-22 row is the precedent
for absorbing them).

**⑪ `python3 tools/check-ledger-ticks.py`**: `233 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 357 row(s) with no function tick …` - this ledger carries no
`- [x] NAME` tick, so it is one of the 357 (P-86 ②'s empty checklist).

**⑫ `python3 tools/check-port-lines.py`**: `28 file(s) under ten percent, 38,276 reference
line(s); 28 ratchet row(s), 0 new, 0 stale` - this stem has left the "under ten percent" class (it
was one of the 96 files that reading names), and it needs no new ratchet row.

**⑬ the residual-resource reading**: `df -h /` reports `/dev/disk3s1s1 1.8Ti 12Gi 4.5Gi 73% /` (the
scratch removal freed the last of the lane's bytes; the reading before it was `12Gi 5.4Gi 69%`) -
and `/tmp/w32f-lane` (the scratch: the probe logs, the mutation backup and the `/tmp` copies of the
materialized unit) was deleted after this ledger was written.

## Deviations

1. **C 1-17 and the five forward-declaration blocks have no counterpart**: the opening
   ownership/title block is dropped (PORTING.md 0.1-19), and C 58-68, C 70-79, C 101-103,
   C 1193-1194 and C 2871 are dropped (PORTING.md 0.1-12 - a declaration plus a definition is
   `E0428` and items are order-independent).  Their text is kept as `C:` lines at its position.
2. **The `<windows.h>` stand-in section at C 20 and the `<commdlg.h>` one at C 24** (P-88 ①): the
   reference tree has no file for either header, so the names this file uses stand at C's own
   `#include` line (no file is added under `femacs/src/include/`), each with the SDK's own text
   above it.  The spellings repeat `src/w32term.rs`'s and `src/w32select.rs`'s sections, so a
   Windows translation unit sees one set of names.  **Nothing was measured** - the scalars, the
   `LOGFONT`/`TEXTMETRICW`/`NEWTEXTMETRICW`/`ENUMLOGFONTEX`/`FONTSIGNATURE`/`GLYPHMETRICS`/`MAT2`/
   `TEXTMETRIC`/`CHOOSEFONT` records, the charset/quality/precision/pitch/weight/`CSB_*`/
   `SUBRANGE1_*` values and the `CF_*` flags are the documented Windows ones, **UNVERIFIED**, and
   the layout assertions are owed (Owed 4).  `DWORD_PTR`-class names are other mirrors' and are
   deliberately not repeated.
3. **The four "define it if the SDK has not" guards carry both arms** (C 42-44, 45-47, 51-53,
   54-56): C's `#define` arm under `not (feature = "<name>")`, the SDK arm under
   `feature = "<name>"` - both with the same value, so a target that defines the name reads the
   same constants (the `docs/ports/w32select.md` `CF_DIBV5` reading of the same guard class).
4. **C 371-401 is `#if 0`** (the commented-out script bitmap body of `w32font_has_char`): dead in
   every configuration, so the port keeps C's text inside a comment (the `cm.rs` 86 reading).  The
   Rust block-comment nesting rule is why the carriers of C comment text in this unit are
   `//`-prefixed line comments rather than `/* … */` blocks.
5. **`struct frame *` is `*mut c_void` in the driver callbacks** (`w32font_get_cache`,
   `w32font_list`, `w32font_match`, `w32font_list_family`, `w32font_open`): the mirror's
   `font_driver` members are typed that way (P-72 ③), and every one of these functions casts back
   at its first statement; the `src/androidfont.rs` 561 reading is the precedent.  The four other
   members keep C's `struct frame *` because the mirror's type is not part of the record.
6. **`glyph->type` and `glyph_string->padding_p` are the mirror's packed bit members**, so C's
   field reads are the port's `glyph_bits! (type, …)` / `glyph_string_bits! (padding_p, …)`
   (`include/dispextern.rs` 553-590 and 1828-1854; the `src/w32term.rs` 1733 spelling of the same
   two macros).  The `glyphless` sub-structure's own `method`/`for_no_font`/`len`/`ch` members are
   read through this unit's `glyphless_bits!`, whose four arms are the mirror's shift/mask
   constants.
7. **`s->clip` is the mirror's `[c_int; 2]`** (dispextern.h 1497's `NativeRectangle` is a *type*
   macro whose Windows answer is `RECT`, `include/w32gui.rs` 168), so C 641/653's two
   `CreateRectRgnIndirect (s->clip)` / `(s->clip + 1)` calls cast to `*const RECT` - the two casts
   are the port's, and the mirror's note records why the field is not a `RECT` yet.
8. **The "Notes about the way fonts are found on MS-Windows" comment (C 817-902) is carried as an
   extract**: the two `…` elisions are the paragraphs C's own text repeats from `fontset.el`'s
   documentation; the load-bearing sentences (USB/CSB, the two heuristics, the tofu paragraph, the
   hyphen paragraph and the URL) are verbatim.
9. **`alloca` is the port's stack pool** (`sa_stack_alloc`, PORTING.md P-05) at C 541, C 724,
   C 1088, C 1113/1118 and C 1804; **`UNINIT`** (C 438) is the macro `include/conf_post.rs`
   records as excluded, and C's `= {0,}` arm it stands for is the port's null pointer.
10. **C's two `switch`es are the port's `match`es**: `w32_to_x_charset`'s nineteen `case` labels
    (C 1905-1969) are guard arms (`_ if fncharset == ANSI_CHARSET as c_int => …`) because a Rust
    pattern cannot name a `BYTE` constant against an `int` scrutinee (the `src/coding.rs` 3790
    shape), and `lispy_antialias_type`'s four (C 2336-2352) are direct patterns.  C's `for` loops
    whose bodies `break` (`w32font_text_extents` C 453/542, `w32font_close` C 352,
    `w32font_list_family` C 302, `w32font_draw` C 702/724) are `while` loops with the increment at
    the end of the body, so a `break` skips it exactly as C's does.
11. **Sixteen token-level differences are recorded in `docs/ports/macro-body-notes.tsv`** (the rows
    appended this round): the two macro-head parameter names, the `(FONTENUMPROC)` casts, the
    `alloca`/`UNINIT` and `switch`/`case`/`break` spellings, C's `0`/`1` as the port's
    `false`/`true`, and the `DEFVAR_LISP` `doc:` label.
12. **Four names are declared in this unit because no mirror in this graph carries them**
    (`intern_driver`, `memq_no_quit`, the six `font.h` font-* functions, `decode_window_system_frame`)
    plus `atoi`/`strstr`/`strncpy`/`strnicmp` and the `strlwr`/`_strlwr` pair, each at a `/// C:`
    line with the reference's own text (the `docs/ports/w32select.md` Deviation 4 shape; Owed 3).
13. **Zero-initialised locals where C leaves a binding unassigned on a path that returns**
    (PORTING.md 0.1-13): `w32_to_x_charset`'s `charset_type`, `compute_metrics`' `width`,
    `w32font_text_extents`' `w32_font`-adjacent `f`, and the five `core::mem::zeroed ()` records
    whose C declaration has a `memset` right after it.
14. **The line count**: C 3079 → Rust 3967 (28.8%).  The reasons are the per-item `/// C:` citations,
    the two stand-in sections, the both-arms `#cfg`s, the wrapped `if`/`while` conditions and the
    `C:`-carried comment blocks; the tolerance row is Owed 2.

## Owed

1. **The Windows round that flips this arm** must register this file's arm set as
   `have_ntgui` + `windowsnt` **+ `have_window_system`** (C's `HAVE_NTGUI` implies
   `HAVE_WINDOW_SYSTEM`; the `have_ntgui`-only reading of Evidence ③ is missing the eight
   `struct font` members and `glyph_string_bits!`), and it lifts ②'s by-file list.
2. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32font`, tolerance ≥ **0.288**,
   basis `total`, reason from Deviations 2-14.  That file is not in this lane's write set, so
   `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑤ records.
3. **The mirror rows for the in-unit declarations of Deviation 12**: `include/lisp.rs`
   (`intern_driver` lisp.h 4729, `memq_no_quit` lisp.h 4286), `include/font.rs`
   (`font_make_entity` font.h 842, `font_make_object` font.h 846, `font_intern_prop` font.h 885,
   `font_put_extra` font.h 913, `font_filter_properties` font.h 927, `register_font_driver`
   font.h 896), `include/frame.rs` (`decode_window_system_frame`), `lib/include/stdlib.rs` (`atoi`)
   and `lib/include/string.rs` (`strstr`, `strncpy`, `strnicmp`, `strlwr`, `_strlwr`).  Each is one
   declaration with the header's text; until they land, this unit carries its own copies.
4. **The layout assertions for the `<windows.h>`/`<commdlg.h>` records** (`LOGFONT`, `TEXTMETRICW`,
   `NEWTEXTMETRICW`, `NEWTEXTMETRICEX`, `ENUMLOGFONTEX`, `FONTSIGNATURE`, `GLYPHMETRICS`, `MAT2`,
   `TEXTMETRIC`, `POINT`/`POINTL`/`SIZE`/`RECT`, `CHOOSEFONT`, `font_callback_data`) and the
   measured values of the charset/quality/`CSB_*`/`CF_*` constants, once a Windows or mingw
   toolchain exists (`xtask layout-asserts` reads a C compiler's readings; `x86_64-w64-mingw32-gcc`
   is absent here).
5. **Two mirror gates are narrower than C's conditions and are the Cygwin arm's four in-unit
   anchors** (Evidence ③): `include/w32term.rs`'s `FRAME_W32_WINDOW`/`FRAME_FONT` are gated
   `windowsnt` while C's `w32term.h` 42-46 defines them unconditionally, and `include/frame.rs`
   refuses the whole arm with a `compile_error!` (its own note asks for a mingw clang reading of
   frame.h 471's flag run).  Neither is this lane's to fix.
6. **This ledger is bound to two references** (`src/w32font.c` and `src/w32font.h`, since the
   `include/` target falls back to the bare stem): the table covers both, but the header's own
   ledger is `docs/ports/include-w32font.md`, which this lane did not touch.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, GUI frame, `HDC`, font driver or redisplay loop was involved.  There is no
  behavioural test, no differential run, no ERT and no device evidence.
- **The arm reading is not a Windows compile.**  This machine has no `windows.h` and no mingw
  toolchain, so Evidence ②/③ are `rustc` readings of the *materialized mirror graph* under three
  `--cfg`s; 21 errors remain in it, all anchored in mirror copies, and "0 in the unit" means that
  no error's primary anchor is one of this unit's lines - not that the unit *links*, and not that
  the arm is a working Windows configuration.
- **The `<windows.h>`/`<commdlg.h>` records and values stay unmeasured** (Owed 4): every one is
  `UNVERIFIED`, with the documented-sibling-file precedent as its only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 2).  Nothing in this ledger claims
  the file's checklist is non-empty (P-86 ②) or that the port is complete.
