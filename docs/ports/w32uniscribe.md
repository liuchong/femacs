# emacs/src/w32uniscribe.c (1628 lines) — whole file, C 1-1628

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32uniscribe.o` sits in
> `W32_OBJ` (`emacs/src/Makefile.in` 317-319, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac` 3031
> under the `HAVE_W32` test whose 3024 `AC_DEFINE`s `HAVE_NTGUI`), and
> `docs/baseline/frozen-build-sources.txt` lists no `w32uniscribe.o` for this target — so
> `extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `femacs/src/w32uniscribe.rs` (**2766 lines**, md5 `cc0449a0282d263b49e00840dd4f106d`, sha256
> `967fde4731fac8760dca4e5aaa7c44a06eca1b48ab2e5e411a84f848cbb65dbf`), carrying **C 1-1628 with both
> arms of every `#if`**.  The reference `emacs/src/w32uniscribe.c` is md5
> `744023a043e526b50e8d0b083e4c88c1` (unchanged; `emacs/` is read-only).
>
> **The arm set is `have_ntgui` + `windowsnt` + `have_window_system` (+ `harfbuzz` for the
> HarfBuzz half)**, the same set the landed sibling `docs/ports/w32font.md` registers, plus the
> fourth feature this file's `#ifdef HAVE_HARFBUZZ` needs.  P-86 ①'s reading applies: transcribe
> the text, both arms, no stub; what the earlier P-08 reading measured (not in the frozen source
> list, empty checklist, 336-byte `build-tu` object) is unchanged and still true — it is the
> *conclusion* that is withdrawn.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`usp10.h`), so there is **no
> `<windows.h>` and no `<usp10.h>` to compile or measure against**, no arm flip on a Windows target
> and no layout measurement: every record and value the three stand-in sections declare is
> `UNVERIFIED` (documented, not measured) and the layout assertions are owed.  What the round does
> carry: the frozen `build-tu` reading, the machine-checked coverage table, the `#if` verdicts, a
> per-block C citation, the `probe` reading of the `have_ntgui`+`windowsnt`+`have_window_system`+
> `harfbuzz` arm (0 errors anchored in this unit), a **free multi-configuration sweep** of the same
> materialized text, and a **mutation test** over this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, i.e. `HAVE_W32` → `HAVE_NTGUI`) | `feature = "have_ntgui"` (on every item *and* on C 19-45's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 33-36 `#ifdef HAVE_HARFBUZZ` / `# include <hb.h>` / `# include <hb-ot.h>` / `#endif` | `all (feature = "have_ntgui", feature = "harfbuzz")`; the `#else` arm is empty | the HarfBuzz configurations |
| C 54-88 `#ifdef HAVE_HARFBUZZ` (the ten `DEF_DLL_FN` declarations and C 78-87's ten renaming `#define`s) | `all (feature = "have_ntgui", feature = "harfbuzz")` | as above |
| C 191-195 `#ifdef HAVE_HARFBUZZ` inside `uniscribe_open` (the `if`/`else` pair), `#endif` before C 196 | `feature = "harfbuzz"` for the pair; `not (feature = "harfbuzz")` for C 196, which is the other arm's only statement | both arms carried |
| C 207-214 `#ifdef HAVE_HARFBUZZ` inside `uniscribe_close`, `#endif` before C 215 | `feature = "harfbuzz"`; the non-HarfBuzz arm reaches C 215-216 alone | both arms carried |
| C 1060-1065 `#if 0` (the `DFLT`-fallback arm of the script search) | none: dead text in every configuration, carried as C's own words | never |
| C 1241-1463 `#ifdef HAVE_HARFBUZZ` (the eight HarfBuzz functions) | `all (feature = "have_ntgui", feature = "harfbuzz")` | the HarfBuzz configurations |
| C 1288-1292 `#if GNUC_PREREQ (4, 3, 0)` / `#else` / `#endif` around one initialiser | `include/conf_post.rs`'s `GNUC_PREREQ!` (an `if`/`else` pair, both arms); the two arms are `__builtin_bswap32` → `swap_bytes` and gnulib's `bswap_32` | decided at compile time |
| C 1513-1529 `#ifdef HAVE_HARFBUZZ` (`load_harfbuzz_funcs`) | `all (feature = "have_ntgui", feature = "harfbuzz")` | as above |
| C 1541-1573 `#ifdef WINDOWSNT` / `#else /* Cygwin */` (the `usp10.dll` load and the six pointer stores) | `feature = "windowsnt"` / `not (feature = "windowsnt")` | both arms carried |
| C 1592-1627 `#ifdef HAVE_HARFBUZZ` (the HarfBuzz registration), incl. C 1599-1603's inner `#ifdef WINDOWSNT` / `#else /* CYGWIN */` | `all (feature = "have_ntgui", feature = "harfbuzz")`, the inner pair on `windowsnt` | both arms carried |

`tools/check-cond-eval.py src/w32uniscribe.c` reads **`11 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** — the rows above (C 33, 54, 191, 207, 1060, 1241, 1288, 1513, 1541, 1592, 1599) and
nothing else, so this is the whole conditional text of the file.  The `probe` door set is therefore
`have_ntgui` + `windowsnt` (+ `have_window_system`, + `harfbuzz`), with the other arms named above.

## C line ranges

The rows are contiguous and ascending and cover every line of C 1-1628; the reference's blank lines
and `^L` page breaks are absorbed into the neighbouring rows (the `docs/ports/w32select.md` 1-22
precedent, and the reading `docs/ports/w32font.md` records).  `docs/port-map.md` lists this stem
once (`src/w32uniscribe.c` 1,628 — there is no `src/w32uniscribe.h`), so one reference is bound.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19) |
| 19-21 | the blank lines and `#include <config.h>` | file gate |
| 22-30 | the API-version comment, C 29-30's `#undef`/`#define _WIN32_WINNT 0x0600` | file gate; C's override carried as the module-scoped `const` (Deviation 3) |
| 31-32 | C 31's `#include <windows.h>` and C 32's `#include <usp10.h>` stand-in sections | file gate; every record/value `UNVERIFIED` (P-88 ①) |
| 33-37 | C 33-36's `#ifdef HAVE_HARFBUZZ` `<hb.h>`/`<hb-ot.h>` pair | file gate; C 34-35 `all (have_ntgui, harfbuzz)` |
| 38-45 | the eight reference mirrors (`lisp.h`, `w32term.h`, `frame.h`, `composite.h`, `font.h`, `w32font.h`, `pdumper.h`, `w32common.h`) | file gate |
| 46-48 | `extern int uniscribe_available;` (dropped, 0.1-12) and its definition | file gate |
| 49-53 | the `EnumFontFamiliesEx` callback's forward declaration (dropped) | file gate |
| 54-88 | C 54's `#ifdef HAVE_HARFBUZZ`: `harfbuzz_font_driver`, `harfbuzz_available`, the ten `DEF_DLL_FN`s, the ten `hb_x` → `fn_hb_x` `#define`s | `all (have_ntgui, harfbuzz)` |
| 89-91 | the `otf_features` forward declaration (dropped) | file gate |
| 92-116 | the six Uniscribe function-pointer globals | file gate |
| 117-126 | the banner comment and `uniscribe_list` | file gate |
| 127-134 | `uniscribe_match` | file gate |
| 135-162 | `uniscribe_list_family` | file gate |
| 163-199 | `uniscribe_open` (incl. C 191-195's `#ifdef HAVE_HARFBUZZ` pair) | file gate; C 192-195 `harfbuzz`, C 196 the other arm |
| 200-221 | `uniscribe_close` (incl. C 207-214's arm) | file gate; as above |
| 222-243 | the `uniscribe_otf_capability` design comment | file gate |
| 244-272 | `uniscribe_otf_capability` | file gate |
| 273-290 | the `uniscribe_shape` design comment | file gate |
| 291-603 | `uniscribe_shape` (C's five `alloca` requests, the RTL adjustment loop) | file gate |
| 604-607 | the `uniscribe_encode_char` banner comment | file gate |
| 608-708 | `uniscribe_encode_char` | file gate |
| 709-731 | the "Shared with w32font / Unused" comment block | carried as C's own text (nested-comment rewrite) |
| 732-736 | the callback banner comment | file gate |
| 737-768 | `add_opentype_font_name_to_list` | file gate |
| 769-775 | the ":otf property handling" comment | file gate |
| 776-804 | the four `:otf` macros (`OTF_TAG`, `OTF_INT16_VAL`, `OTF_DWORDTAG_VAL`, `OTF_TAG_VAL`) and `SNAME` | file gate; the `goto` label is a macro parameter (Deviation 6) |
| 805-820 | the three Vista proc typedefs, their three `static` pointers and `uniscribe_new_apis` | file gate |
| 821-826 | the `uniscribe_check_features` comment | file gate |
| 827-871 | `uniscribe_check_features` | file gate |
| 872-878 | the `uniscribe_check_otf_1` comment | file gate |
| 879-967 | `uniscribe_check_otf_1` (C 963's `no_support` label) | file gate |
| 968-971 | the `uniscribe_check_otf` comment | file gate |
| 972-1152 | `uniscribe_check_otf`, incl. C 1060-1065's `#if 0` and C 1143-1145's three labels | file gate; the `#if 0` region carried as C's words |
| 1153-1239 | `otf_features` (C 1237's `font_table_error` label) | file gate |
| 1240-1463 | C 1241's `#ifdef HAVE_HARFBUZZ`: `w32hb_list`, `w32hb_match`, `free_cb`, `w32hb_get_font_table` (incl. C 1288-1292's `GNUC_PREREQ` pair), `w32hb_get_font`, `w32hb_encode_char`, `w32hb_begin_font`, `w32hb_get_variation_glyphs` | `all (have_ntgui, harfbuzz)` |
| 1464-1467 | the three `#undef`s (`OTF_INT16_VAL`, `OTF_TAG_VAL`, `OTF_TAG`) | carried as C's own text (Deviation 13) |
| 1468-1499 | `uniscribe_font_driver` | file gate |
| 1500-1505 | the "called at every startup" note and the two `syms_of_*` prototypes | file gate; C 1505 dropped (0.1-12) |
| 1506-1511 | `syms_of_w32uniscribe` | file gate |
| 1512-1529 | `load_harfbuzz_funcs`, C 1513's `#ifdef HAVE_HARFBUZZ` | `all (have_ntgui, harfbuzz)` |
| 1530-1628 | `syms_of_w32uniscribe_for_pdumper`, incl. C 1541-1573's `WINDOWSNT`/Cygwin pair, C 1592's `#ifdef HAVE_HARFBUZZ` and C 1599-1603's inner pair | file gate; C 1542-1561 `windowsnt`, C 1563-1572 `not (windowsnt)`; C 1593-1626 `harfbuzz`, its inner pair on `windowsnt` |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32u-lane cargo run -q -p xtask -- build-tu src/w32uniscribe.c`
→ the first run exited 1 with `error[E0758]: unterminated block comment` in the unit's own text and
wrote no object (the text was still being written); the second ran clean:
`built src/w32uniscribe.rs -> ../build/femacs/w32uniscribe.o`, **exit 0**, and
`build/femacs/w32uniscribe.o` is **336 bytes** — the P-08 empty object, which is what every item
being `#[cfg (feature = "have_ntgui")]` produces.  A third run **on the final revision** (after
every edit, `CARGO_TARGET_DIR=/tmp/w32u-lane2`) reads the same: exit 0,
`built src/w32uniscribe.rs -> ../build/femacs/w32uniscribe.o`, **336 bytes** — and that run's
materialized `_root.rs` is the final text (0 occurrences of the mutation of ④).  Budget: `build-tu`
spent **3**.

**② The arm reading (`xtask probe`), on the final revision.**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32u-lane cargo run -q -p xtask -- probe src/w32uniscribe.rs
/tmp/w32u-lane/base2.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"'
--cfg 'feature="windowsnt"' --cfg 'feature="have_window_system"' --cfg 'feature="harfbuzz"'`
→ **exit 1, 1 error line, and 0 of them anchored in this unit** (`_root.rs`: 0 — P-86 ⑦'s by-file
reading; 25 warnings).  The one error is a **mirror's own refusal**:
`build/probes/w32uniscribe/_Users_liu_projects_src_github_com_liuchong_femacs_femacs_src_include_frame_rs.rs:329:1`,
`src/include/frame.rs`'s deliberate `compile_error!` ("the HAVE_NTGUI arm of `struct frame`'s flag
run is unmeasured on this machine … a Windows/mingw clang probe has to measure it") — the same row
`docs/ports/w32font.md` Owed 5 records, and not this lane's to fix.  The same reading was taken
once on the pre-fix text (23 warnings, the same single mirror error) — the by-file split did not
move.  Budget: `probe` spent **5** (reported honestly, two over the ≤3 asked for): 1 and 2 = the
base arm on the pre-fix text (the first invocation's output was truncated by the shell's `tail`, so
it was taken again with the log captured), 3 = the mutation test on that revision (④ below), 4 = the
base arm above on the **final** revision, 5 = the mutation test on the final revision.

**③ The free multi-configuration sweep (direct `rustc` on the materialized root; not a Windows-target
compile).**  `xtask build-tu` materializes the unit into `build/femacs/units/w32uniscribe/`; the
script `/tmp/w32u-lane/sweep.py` re-rendered `_root.rs` from the unit's final text (only the copies
are fixed) and compiled it once per configuration with
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort
-C opt-level=0`, counting errors by primary `-->` anchor.

| configuration | exit | `error` lines | in this unit | in a mirror copy |
| --- | --- | --- | --- | --- |
| `have_ntgui` | 1 | 1 | **0** | 1 (`src/include/frame.rs` 329) |
| `+ windowsnt` | 1 | 1 | **0** | 1 (same) |
| `+ have_window_system` | 1 | 1 | **0** | 1 (same) |
| `+ windowsnt + have_window_system` | 1 | 1 | **0** | 1 (same) |
| `+ have_window_system + harfbuzz` | 1 | 1 | **0** | 1 (same) |
| `+ windowsnt + have_window_system + harfbuzz` (base) | 1 | 1 | **0** | 1 (same) |
| none (frozen) | **0** | 0 | 0 | 0 |

So *every* arm of the file's four conditions type-checks with no error anchored in this unit, and the
frozen row is Evidence ①.  **Toolchain note**: run with the *default* stable toolchain the same
configurations report one extra mirror error, `src/include/lisp.rs` 3761's
`E0658 use of unstable library feature c_variadic`; `femacs/rust-toolchain.toml` pins
`nightly-2026-10-01`, where that feature is stable, and the sweep above uses the pinned toolchain
(the reading `docs/ports/w32font.md`'s Evidence records for its own sweep).

**④ The mutation test (P-86 ⑦): the reading is not vacuous.**  One type error injected into this
unit's own text — `free_cb`'s body got a `let _mutation: *mut u8 = ptr;` line before its `free
(ptr);` — and the **same** `probe` command re-run on the **final** revision: exit 1, **2** error
lines against ②'s 1, the single new one `error[E0308]: mismatched types`, anchored in the unit's own
text (`build/probes/w32uniscribe/_root.rs:2303:28`, and the unit's own line 2303 is that line — the
materialized root *is* the unit's text).  The mutation was reverted and the unit verified:
`shasum -a 256 femacs/src/w32uniscribe.rs` = `967fde47…cbb65dbf` before and after, and
`grep -c _mutation` is 0.  (The same mutation was run once on the pre-fix revision, where it landed
at `_root.rs:2289:28` with the same single `E0308`.)  So ②'s reading certifies the unit's own lines
type-check under that arm (not merely resolve) and that no item of this unit collides.

**⑤ `python3 tools/extract-abi.py --check emacs/src/w32uniscribe.c femacs/src/w32uniscribe.rs`**:
before this round's `docs/ports/macro-body-notes.tsv` rows it was `FAIL` with 17 findings (16
`does not mention …`/`family` rows plus `constant _WIN32_WINNT: C 0x0600 vs Rust 0x500`) and the
line-drift row; the eight note rows **are written** (that file is in this lane's write set, appended
at its end: `OTF_INT16_VAL`, `OTF_DWORDTAG_VAL`, `OTF_TAG_VAL`, `uniscribe_list_family`,
`uniscribe_shape`, `uniscribe_encode_char`, `uniscribe_check_otf_1`, `uniscribe_check_otf`) and the
`_WIN32_WINNT` override, the `(SCRIPT_CACHE)` casts, the three `*_Proc` cast targets and the
`wchar_t` → `c_ushort` spelling were fixed in the unit, so the re-run leaves exactly **one** finding:
`FAIL line count drifted 69.9%: C 1628 vs Rust 2766` — `docs/ports/line-drift-tolerance.tsv`'s row
(not in this lane's write set; Owed 2).

**⑥ `python3 tools/check-cond-eval.py src/w32uniscribe.c`**: `11 conditions, 0 unknown, 0
disagreements [VERIFIED]` — the Gate-mapping rows, nothing else.

**⑦ `python3 tools/check-declarations.py`**: the last reading taken while this ledger was written is
`17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`, `read 683 files (625 .rs
40 .kt 7 .swift 6 .s 5 .cc)` — **neither this unit nor this ledger carries a finding**.  (Two earlier
runs of the same gate, minutes apart, read `18 declaration sites (1 forbidden, …), 1 failures` with
the one failure in `femacs/oldXMenu/insque.rs`: other lanes are landing files into this shared
working tree, which is why ⑦-⑫ name the readings' *identity* as well as their numbers.)

**⑧ `python3 tools/check-mirror-dupes.py`**: the last reading is `18 duplicate declaration(s), 17
ratchet row(s), 1 failure(s)`, the one failure being `femacs/src/gmalloc.rs: free is already declared
by femacs/lib/include/stdlib.rs` — **no row names `w32uniscribe`** (checked; the gate's own output
mentions this stem 0 times): this unit's only top-level names are the driver, the two counters and
`uniscribe_check_otf`, whose declaration `include/w32font.rs` 176 already keeps in a glob-imported
module (definitions are not what this gate reports), and everything else is `static`/private.  Two
earlier runs of the same gate read `21 duplicate declaration(s) … 4 failure(s)` (all in
`femacs/src/gmalloc.rs`) and `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)` — the
same other lane's file, in flux (see ⑦'s note).

**⑨ `python3 tools/check-citations.py`**: `676 file(s) scanned, 0 drift, 0 range, 1522 external, 36
allowed, 0 stale` — this unit's ~230 `/// C:`/`// C` citations and this ledger's are all in range and
current; the external count is 10 above the last landed Windows unit's 1512, and those are this
unit's citations of the three SDK surfaces (`<windows.h>`, `<usp10.h>`, `<hb.h>`).

**⑩ `python3 tools/check-dead-file-coverage.py`**: exit 0, `116 ledger(s) bound (a C range table), 0
failure(s)` — this ledger's own row is
`info docs/ports/w32uniscribe.md: 42 range row(s) cover src/w32uniscribe.c (1628 lines; 0 blank
line(s) between blocks need no row)`, i.e. the table above tiles C 1-1628 (its blank lines and `^L`
page breaks are absorbed, which is why the gate needs no row for any of them).

**⑪ `python3 tools/check-ledger-ticks.py`**: `233 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 357 row(s) with no function tick …` — this ledger carries no
`- [x] NAME` tick, so it is one of the 357 (P-86 ②'s empty checklist).

**⑫ `python3 tools/check-port-lines.py`**: exit 0; `21 file(s) under ten percent, 23,883 reference
line(s); 25 ratchet row(s), 0 new, 4 stale` — one of the four stale rows is
`docs/baseline/port-lines-pending.tsv` 29's `src/w32uniscribe.c`, which this stem has left (Owed 7;
the ratchet is not in this lane's write set).

**⑬ the residual-resource reading**: `df -h /` reports `/dev/disk3s1s1 1.8Ti 12Gi 4.4Gi 73% /`
after the scratch removal (the readings on either side of the lane's runs were `12Gi 4.4Gi 74%` and
`12Gi 4.4Gi 73%` — the lane's own footprint was the `build/` objects and the `/tmp` logs), and
`/tmp/w32u-lane` — the
sweep script `sweep.py`, the `mkroot.py` re-render helper, the probe logs, the mutation backups and
the `/tmp` copies of the materialized unit — was deleted after this ledger was written.  The two
`build/` directories the sanctioned tools wrote (`build/femacs/units/w32uniscribe/` and
`build/probes/w32uniscribe/`, plus `build/femacs/w32uniscribe.o`) stay, as every landed unit's
readings leave them.

## Deviations

1. **The opening ownership/title block (C 1-18) is dropped** (PORTING.md 0.1-19); the port's first
   line is the revision line `// femacs: port of src/w32uniscribe.c @ fd7afced403.`
2. **Three stand-in sections** (C 31's `<windows.h>`, C 32's `<usp10.h>`, C 33-36's
   `<hb.h>`/`<hb-ot.h>`) declare, in the unit, every type, value and entry point this file reads from
   a header the reference tree does not carry (P-88 ①).  **Every one is `UNVERIFIED`** (no Windows
   SDK on this machine); the two bitfield runs (`SCRIPT_ANALYSIS`, `SCRIPT_VISATTR`) are P-80's
   packed-unit-plus-named-accessors spelling and **no layout assertion is written**.
3. **C 29-30's `#undef _WIN32_WINNT` / `#define _WIN32_WINNT 0x0600`** is an SDK-macro override:
   the port carries it as the module-scoped `const _WIN32_WINNT = 0x0600` that the unit's `use`
   brings into scope, because the crate-root name is `include/conf_post.rs` 191's own C
   `#define _WIN32_WINNT 0x500` (two same-named `const`s in one scope are `E0428`); nothing in the
   port reads the name.
4. **The include graph is inside the file gate** (the eight mirrors plus
   `../lib/include/{stdcountof,stdlib,byteswap.in}.rs` and `include/conf_post.rs`), which is what
   keeps the frozen object empty (P-86 ⑧; Evidence ①).  C's own edges to `<stdcountof.h>` (lisp.h
   28, C 885's `countof`), `<string.h>` (lisp.h 30, C 143's `memset`/C 786's `memcpy`) and the SDK's
   `<stdlib.h>` (C 1275/C 1301's `free`/`malloc`) are spelled where the port needs them.
5. **Forward declarations dropped** (0.1-12), their text kept as `/// C:` lines: C 47
   (`uniscribe_available`), C 51-53 (the callback), C 91 (`otf_features`), C 1505
   (`syms_of_w32uniscribe_for_pdumper`).
6. **`goto` and labels**: `uniscribe_check_otf_1`'s three `goto no_support` (C 897/920/932/941/949/
   957) are `break 'no_support` out of the labelled block carrying C 889-961, with C 963-966's
   cleanup after it; `uniscribe_check_otf`'s `goto done` (C 1012) and `goto no_support` (C 1042/
   1070/1096/1105/1137) and `otf_features`'s `goto font_table_error` are `break 'font_table_error`
   (C 1143-1145's three adjacent labels share one cleanup), and **the three OTF macros take the
   label as a `$lbl:lifetime` parameter** because their C bodies jump to it.  Eight rows in
   `docs/ports/macro-body-notes.tsv` record the tokens Rust cannot carry.
7. **`alloca` → the stack pool** `sa_stack_alloc` (PORTING.md P-05/3.2): five sites in
   `uniscribe_shape` (C 314, 348-352) and one in `uniscribe_encode_char` (C 643).
8. **C 78-87's ten `#define hb_x fn_hb_x`** have no Rust counterpart: the ten lines are carried as
   C's text and every call site is spelled the way C's preprocessor leaves it, `(fn_hb_x.unwrap ())
   (…)` under `harfbuzz` (the `src/hbfont.rs` 19 reading).
9. **The `:otf` macros**: C's `STR[i]` is the port's `offset (i)`; the implicit conversion C's
   `GetFontData` call makes on the offset is an explicit `$offset as DWORD`; `OTF_TAG`'s result is an
   `int` in C, so each of its six use sites converts to the SDK type it lands in (`as OPENTYPE_TAG` /
   `as DWORD`) exactly as C's implicit conversion does.
10. **C's chained assignment** (`max_glyphs = nchars = LGSTRING_GLYPH_LEN (lgstring);`, C 312) is two
    statements, because Rust's assignment yields `()`; C 1288-1292's one declaration with two arms is
    a binding plus the two-arm selection (`if GNUC_PREREQ! (4, 3, 0) { … } else { … }`).
11. **C's casts that name a type only the cast needs** are spelled as Rust has to: the
    `(FONTENUMPROC) add_opentype_font_name_to_list` argument is the port's `Some (…)` (its note row),
    `(LPARAM) &list` is `(&raw mut list) as LPARAM`, and the `(SCRIPT_CACHE) &(…->cache)` pair is
    `(&raw mut …) as *mut SCRIPT_CACHE` (the parameter's own `SCRIPT_CACHE *` type).
12. **`wchar_t` is MinGW's 16-bit `wchar_t`** and is spelled `c_ushort` (the `wchar_t` row of
    `docs/ports/token-aliases.tsv`; the `src/sound.rs` 1649 reading) — C 299's `chars` and C 615's
    `ch[2]`.  C's integer promotions of the `unsigned short` offsets are explicit `as c_int` casts,
    and `+=` between two `unsigned short`s (C 1073/1099/1177/1187/1214) is `wrapping_add`, which is
    C's truncating assignment.
13. **C 1465-1467's three `#undef`s** are carried as C's own text: Rust has no `#undef`, and the
    three `macro_rules!` stop being used at C 1239.
14. **Three `font_driver` slots are `transmute`d**, because the mirror's declaration and the record's
    member spell one pointer differently: `w32font_get_cache` (`*mut frame` in
    `include/w32font.rs`'s module vs the slot's `*mut c_void`), `w32font_draw` (that mirror's
    `*mut c_void` stand-in vs the slot's `*mut glyph_string`), and `begin_hb_font`
    (`*mut hb_font_t` vs the mirror's `*mut c_void` result).
15. **C's struct copy and its `(void *)` casts**: `harfbuzz_font_driver = uniscribe_font_driver;`
    (C 1613) is `std::ptr::read (&raw const …)`, and the nine
    `(T) get_proc_addr (uniscribe, "…")` stores are `transmute::<VOIDFNPTR, T>` (the spelling
    `DEF_DLL_FN`/`LOAD_DLL_FN` themselves use).
16. **Zero-initialised locals where C leaves a binding unassigned** before a pointer or an address is
    taken (PORTING.md 0.1-13): `context`, `old_font`, `items`, `glyphs`/`clusters`/`attributes`/
    `advances`/`offsets`, `ftags`, the `tags`/`farray`/`ch`/`script`/`lang`/`feature` arrays,
    `char_metric`, `overall_metrics`, `font_match_pattern`, the many `let mut i: c_int = 0;`
    counters and the `check_font`/`uniscribe` handles.  C 143's `memset` is kept as itself.
17. **C 1060-1065's `#if 0`** and **C 710-731's "Shared with w32font / Unused" comment** are carried
    as C's own words with `/ *` and `* /` for the delimiters, because Rust's block comments nest
    (PORTING.md 0.3); the `#if 0` arm is dead text in every configuration (the `cm.rs` 86 reading).
18. **`__builtin_bswap32` is Rust's `swap_bytes`** (PORTING.md 2.6) and C's `#else` arm is gnulib's
    `bswap_32` (`lib/include/byteswap.in.rs`); both arms of `GNUC_PREREQ (4, 3, 0)` are carried.
19. **The line count**: C 1628 → Rust 2766 (**69.9%**).  The reasons are the per-block `/// C:`
    citations, the three stand-in sections, the both-arms `#[cfg]`s, the wrapped conditions and the
    C-carried comment blocks; the tolerance row is Owed 2.

## Owed

1. **The Windows round that flips this arm** must register this file's arm set as `have_ntgui` +
   `windowsnt` + `have_window_system` **+ `harfbuzz`** (C's `HAVE_NTGUI` implies
   `HAVE_WINDOW_SYSTEM`, and the file's HarfBuzz half is a second, independent condition), and it
   lifts ③'s by-file list.
2. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32uniscribe`, tolerance ≥
   **0.699**, basis `total`, reason from Deviations 2-19.  That file is not in this lane's write set,
   so `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑤ records.
3. **The mirror rows for the two in-unit declarations** of Deviation 5's class: `include/lisp.rs`
   (`memq_no_quit`, lisp.h 4286) and `include/font.rs` (`font_make_object` font.h 846,
   `register_font_driver` font.h 896).  Each is one declaration with the header's text; until they
   land, this unit carries its own copies (P-90 ⑦).
4. **The layout assertions for the `<windows.h>`/`<usp10.h>` records** (`LOGFONT`, `ENUMLOGFONTEX`,
   `NEWTEXTMETRICW`/`NEWTEXTMETRICEX`/`FONTSIGNATURE`, `TEXTMETRICW`, `POINT`/`SIZE`/`RECT`,
   `SCRIPT_CONTROL`/`SCRIPT_STATE`/`SCRIPT_ANALYSIS`/`SCRIPT_ITEM`/`SCRIPT_VISATTR`, `GOFFSET`,
   `ABC`) and the measured values of `E_OUTOFMEMORY`/`E_PENDING`/`OUT_OUTLINE_PRECIS`/
   `TRUETYPE_FONTTYPE`/`ETO_GLYPH_INDEX`/`NTMFLAGS_OPENTYPE`, once a Windows or mingw toolchain
   exists (`xtask layout-asserts` reads a C compiler's readings; `x86_64-w64-mingw32-gcc` is absent
   here).  The two `SCRIPT_*` bitfield runs owe a real offset/width measurement as well.
5. **A dedicated `<hb.h>`/`<hb-ot.h>` mirror** (`femacs/src/include/hb.rs`, P-90 ①): the surface has
   more than one consumer in the reference (`src/ftfont.c`, `src/sfntfont.c`, this file), so this
   unit's in-unit stand-in section should move there and be deleted here.  `src/hbfont.rs`'s own
   Owed 2 is the same row.
6. **`include/frame.rs`'s deliberate `have_ntgui` `compile_error!`** (its own note asks for a
   Windows/mingw clang reading of frame.h 471's flag run) is the one error the arm reading still
   reports (Evidence ②); it is not this lane's to fix.
7. **`docs/baseline/port-lines-pending.tsv`**: regenerate with `tools/check-port-lines.py
   --write-pending` — line 29's `src/w32uniscribe.c` row is stale now that the stem is above the
   ten-percent threshold (Evidence ⑫).  Not this lane's write set.
8. **`docs/baseline/feature-coverage.tsv` and `docs/status.md`**: the integrator's regeneration and
   running-log entry for this revision (the `docs/ports/w32font.md` Owed 10/11 rows).
9. **The gates that still fail in this shared working tree fail for reasons that are not this
   unit's**: `tools/check-declarations.py`'s `femacs/oldXMenu/insque.rs` row and
   `tools/check-mirror-dupes.py`'s `femacs/src/gmalloc.rs` row, both another lane's work in flux
   (Evidence ⑦/⑧).  Named here so the next reader does not attribute them to this file.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, GUI frame, `HDC`, font driver, Uniscribe cache or redisplay loop was
  involved.  There is no behavioural test, no differential run, no ERT and no device evidence.
- **The arm reading is not a Windows compile.**  This machine has no `windows.h`/`usp10.h` and no
  mingw toolchain, so Evidence ②/③/④ are `rustc` readings of the *materialized mirror graph* under
  four `--cfg`s; one error remains in it, anchored in a mirror copy, and "0 in the unit" means that
  no error's primary anchor is one of this unit's lines — not that the unit *links*, and not that
  the arm is a working Windows configuration.
- **The `<windows.h>`/`<usp10.h>`/`<hb.h>` records and values stay unmeasured** (Owed 4/5): every one
  is `UNVERIFIED`, with the documented-sibling-file precedent as its only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 2).  Nothing in this ledger claims the
  file's checklist is non-empty (P-86 ②) or that the port is complete.
