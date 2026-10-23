# emacs/src/fontset.c (2257 lines)

> **This file is P-86's class: the frozen configuration compiles no `fontset.o`.**  `configure.ac`
> 7515-7521 adds it to `WINDOW_SYSTEM_OBJ` (`WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"`)
> only when `window_system != none`; `build/emacs-ref/src/Makefile` 303 reads
> `WINDOW_SYSTEM_OBJ=` **empty** for this target, `docs/baseline/frozen-build-sources.txt` lists
> no `src/fontset.o`, and `docs/status.md` 3.31 is the policy that made this an empty unit plus a
> note.  `extract-abi.py`'s checklist for the file is therefore empty (the frozen live-line
> filter, P-73), so P-86 ②'s substitute is what follows: the **C line-range table**, the
> `## Evidence` readings and the per-`#if` `## Deviations`.  Nothing in this ledger is a
> behaviour claim.
>
> Port: **`src/fontset.rs` carries the whole file** (C 1-2257), 2,622 lines (md5
> `b8adba1f77d972e9ec4331d7d76526d8`; the reference's own md5 is
> `e04b3ade6744baed5d88945dc6278bb4`), both arms of
> every `#if` carried.  The file gate is C's own `HAVE_WINDOW_SYSTEM` ->
> `feature = "have_window_system"`, on every item *and on C 27-42's include graph* (P-86 ⑧):
> when the frozen configuration's feature set is in force the unit compiles to an empty object
> (336 bytes, Evidence ①), which is the file's own shape in that configuration.  The only C text
> this port does *not* transcribe is C's own `#if 0` regions - C 956-967, C 1307-1332 (the arm
> C 1306-1351's `#else` replaces) and C 2104's `#ifdef ENABLE_CHECKING` **is** carried - recorded
> in Deviations 3 with their line ranges, the `src/ccl.rs` 35-40 rule.
>
> `docs/ports/include-fontset.md` is the header mirror's own ledger (48 lines, ten prototypes);
> this file includes it in C (C 37) and **cannot** in Rust - see Deviations 1, which is a finding
> for the integrator, not a port choice.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 7515-7521's `WINDOW_SYSTEM_OBJ`, i.e. `HAVE_WINDOW_SYSTEM`) | `feature = "have_window_system"` (on every item and on C 27-42's include graph) | any window-system configuration |
| C 38-40 `#ifdef HAVE_WINDOW_SYSTEM` (`#include TERM_HEADER`) | `feature = "have_window_system"` + the six-arm terminal-header chain of `src/frame.rs` 84-98 | a window-system configuration; one terminal header per platform |
| C 956-967 `#if 0` (inside `face_for_char`) | none - dead in every configuration | never |
| C 1307-1332 `#if 0` / C 1333 `#else` (inside `free_realized_fontsets`) | the `#else` arm is the file gate; the `#if 0` arm is recorded | the file's configuration |
| C 1688-1700 `#ifdef HAVE_WINDOW_SYSTEM` (inside `Fset_fontset_font`) | `feature = "have_window_system"` (the file gate's own macro; carried as a `#[cfg]` on the `if`) | any configuration compiling this file |
| C 2103-2152 `#ifdef ENABLE_CHECKING` (`dump_fontset`, `Ffontset_list_all`) | `all (feature = "have_window_system", feature = "enable_checking")` | a checking build |
| C 2254-2256 `#ifdef ENABLE_CHECKING` (the `defsubr`) | `feature = "enable_checking"` (nested in the file gate) | a checking build |
| the mirror side: `struct face` and `frame_face_accessors` | `feature = "dispextern_face_records"` (not a C condition of this file; it is the gate `include/dispextern.rs` 2153-2290 and `include/frame.rs` 1486 put the records behind, status 3.220) | the flip arm below |

`enable_checking` is not defined by `docs/baseline/config/darwin-arm64.macros.tsv` (872), which is
why the frozen reading compiles no `dump_fontset`; `have_window_system` and
`dispextern_face_records` are already registered (the tree's own table).  No new feature name is
needed for this unit - **Owed** carries what the arm had to pass and what the integrator still
has to do.

## Functions defined

`extract-abi.py`'s checklist for the file is empty (the frozen configuration compiles nothing
here), so this list is the port's own record and the ticks are verified against the unit's text
by `tools/check-ledger-ticks.py`.  The C line ranges below are the coverage evidence (P-86 ②).

- [x] set_fontset_id
- [x] set_fontset_name
- [x] set_fontset_ascii
- [x] set_fontset_base
- [x] set_fontset_frame
- [x] set_fontset_nofont_face
- [x] set_fontset_default
- [x] set_fontset_fallback
- [x] font_def_new
- [x] fontset_ref
- [x] fontset_add
- [x] fontset_compare_rfontdef
- [x] reorder_font_vector
- [x] fontset_get_font_group
- [x] fontset_find_font
- [x] fontset_font
- [x] make_fontset
- [x] fontset_name
- [x] fontset_ascii
- [x] free_face_fontset
- [x] face_for_char
- [x] font_for_char
- [x] make_fontset_for_ascii_face
- [x] fontset_pattern_regexp
- [x] fs_query_fontset
- [x] list_fontsets
- [x] free_realized_fontsets
- [x] check_fontset_name
- [x] accumulate_script_ranges
- [x] set_fontset_font
- [x] fontset_from_font
- [x] update_auto_fontset_alist
- [x] dump_fontset
- [x] syms_of_fontset
- [x] fontset_id_valid_p

(`Fquery_fontset`, `Fset_fontset_font`, `Fnew_fontset`, `Ffontset_info`, `Ffontset_font`,
`Ffontset_list` and `Ffontset_list_all` are `DEFUN!` subrs, C's own `DEFUN` form.)

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-26 | the file's licence/declaration block and the blank line after it | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 27-43 | `#include <config.h>`, `<stdio.h>`, `<stdlib.h>`, the blank, the seven file headers, the `HAVE_WINDOW_SYSTEM` `TERM_HEADER` pair, `font.h`, `pdumper.h` and the blank after them | `config.h` ungated; the mirrors under the file gate / TERM_HEADER arm (Deviations 1, 11, 12, 17) |
| 44-155 | the `/* FONTSET … */` documentation comment | file gate (text) |
| 157-171 | the variables section: `Vfontset_table`, `next_fontset_id`, `Vdefault_fontset`, the `make_fontset` prototype | file gate (the prototype has no Rust item) |
| 173-180 | `fontset_id_valid_p` | file gate |
| 182-187 | the `FONTSET_FROM_ID` macro | file gate |
| 188-253 | the seven accessor pairs (`FONTSET_ID`…`FONTSET_FALLBACK`, `set_fontset_*`) and `BASE_FONTSET_P` | file gate |
| 255-287 | `FONT_DEF_SPEC`/`FONT_DEF_ENCODING`/`FONT_DEF_REPERTORY`, the ten `RFONT_DEF_*` macros, `RFONT_DEF_NEW` | file gate (Deviation 2) |
| 290-311 | `FONTSET_REF`, `fontset_ref` | file gate |
| 313-318 | `FONTSET_SET` | file gate |
| 321-364 | `FONTSET_ADD`, `fontset_add` | file gate |
| 366-377 | `fontset_compare_rfontdef` | file gate (the qsort comparator, Deviations 8, 13) |
| 379-465 | `reorder_font_vector` (C 461-462 the `qsort` call) | file gate (Deviation 6) |
| 467-534 | `fontset_get_font_group` | file gate |
| 536-752 | `fontset_find_font` (C 739 the `found:` label; C 507-511 the `continue` skip loop) | file gate (Deviation 4) |
| 754-815 | `fontset_font` (C 763 `UNINIT`) | file gate (Deviation 10; the `uninit`/`1` note row) |
| 817-853 | `make_fontset` | file gate |
| 855-882 | `fontset_name`, `fontset_ascii` | file gate |
| 884-912 | `free_face_fontset` | file gate |
| 914-1031 | `face_for_char` (C 956-967 the `#if 0`) | file gate; the `#if 0` arm recorded |
| 1033-1076 | `font_for_char` | file gate |
| 1078-1102 | `make_fontset_for_ascii_face` | file gate |
| 1104-1112 | `Vcached_fontset_data`, `CACHED_FONTSET_NAME`, `CACHED_FONTSET_REGEX` | file gate |
| 1114-1184 | `fontset_pattern_regexp` (C 1155-1156 `USE_SAFE_ALLOCA`, C 1181-1182 `SAFE_FREE`) | file gate |
| 1186-1234 | `fs_query_fontset` | file gate (Deviation 16) |
| 1236-1261 | `DEFUN ("query-fontset", …)` | file gate |
| 1263-1296 | `list_fontsets` | file gate |
| 1298-1352 | `free_realized_fontsets` (C 1307-1332 `#if 0`, C 1333-1351 `#else`) | file gate; the `#if 0` arm recorded |
| 1354-1391 | `check_fontset_name` (C 1407-1408 the `FRAME_FONTSET` read) | file gate / TERM_HEADER arm |
| 1393-1403 | `accumulate_script_ranges` | file gate (the `map_char_table` callback, Deviation 8) |
| 1405-1456 | `set_fontset_font` (C 1420-1422 the three statement macros) | file gate (the `map_charset_chars` callback, Deviations 2, 8) |
| 1458-1459 | the `update_auto_fontset_alist` prototype | file gate (no Rust item) |
| 1461-1708 | `DEFUN ("set-fontset-font", …)` (C 1688-1700 the nested `#ifdef HAVE_WINDOW_SYSTEM`) | file gate (Deviation 9) |
| 1710-1778 | `DEFUN ("new-fontset", …)` | file gate |
| 1780-1849 | `auto_fontset_alist`, `num_auto_fontsets`, `fontset_from_font` (C 1815-1819 the `INT_STRLEN_BOUND` array) | file gate (Deviation 15) |
| 1851-1872 | `update_auto_fontset_alist` | file gate |
| 1874-2010 | `DEFUN ("fontset-info", …)` (C 1903-1910 `SAFE_ALLOCA_LISP`, C 2010 `SAFE_FREE`) | file gate (Deviation 5) |
| 2012-2081 | `DEFUN ("fontset-font", …)` | file gate |
| 2083-2100 | `DEFUN ("fontset-list", …)` | file gate |
| 2102-2152 | `#ifdef ENABLE_CHECKING`: `dump_fontset` (C 2108), `DEFUN ("fontset-list-all", …)` (C 2140) | `enable_checking` (nested in the file gate) |
| 2154-2257 | `syms_of_fontset` (C 2254-2256 the nested `#ifdef ENABLE_CHECKING`) | file gate / `enable_checking` (the nested arm) |

`docs/ports/include-fontset.md` is the header mirror's own ledger; `tools/check-dead-file-coverage.py`'s
leader rule has no sibling test, so it binds `emacs/src/fontset.h`'s mirror to *this* ledger too and requires
the header's 48 line numbers to fall inside the ranges below - they do, inside the first three rows.

The table's rows cover every non-blank line of the reference (the 39 blank lines between blocks
need no row; `tools/check-dead-file-coverage.py` reads this table as the binding evidence and
reports `0 failure(s)`).

## Evidence

**① The frozen configuration (`build-tu`).**

    cd femacs && CARGO_TARGET_DIR=/tmp/fontset-lane cargo run -q -p xtask -- build-tu src/fontset.c

-> run 1 (the first revision) **exit 1**: `error: this file contains an unclosed delimiter`
(the three `DEFUN!` bodies that ended on a plain `}` instead of `});`) - a syntax reading, not a
feature reading.  run 2 -> `built src/fontset.rs -> ../build/femacs/fontset.o`, **exit 0**; run 3
(the **final** revision, after the macro-call, cast and citation fixes) -> the same
`built src/fontset.rs -> ../build/femacs/fontset.o`, **exit 0**, and `build/femacs/fontset.o` is
**336 bytes**, md5 `dbed68837694579185377dbb0821ab62` -
the same size and the same md5 the P-86 class's other units read (`docs/ports/androidmenu.md`
Evidence ①, `docs/ports/ftfont.md` Evidence ①).  That is the expected reading and not a proof of
the new text: every item sits behind `have_window_system`, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute (P-86 ⑧).

**② The arm flip.**  The four `--cfg`s are the file gate, the checking gate of C 2103-2152,
the mirror gate that carries `struct face`/`frame_face_accessors`, and the terminal header
`TERM_HEADER` picks on this target (`nsterm.h`, whose mirror is the one this repository has for
the Darwin configuration):

    cd femacs && CARGO_TARGET_DIR=/tmp/fontset-lane cargo run -q -p xtask -- probe \
        src/fontset.rs /tmp/fontset-lane/x.rlib --crate-type=lib -C panic=abort \
        --cfg 'feature="have_window_system"' --cfg 'feature="enable_checking"' \
        --cfg 'feature="dispextern_face_records"' --cfg 'feature="have_ns"'

-> **exit 1, 10 errors, 11 warnings**, and **every one of the ten is in a mirror**:

| # | error | header | line |
| --- | --- | --- | --- |
| 4 | `E0425 cannot find function font_deferred_log` | `src/include/font.rs` | 308 (the body of the mirror's `FONT_DEFERRED_LOG!`, which this unit invokes four times) |
| 4 | `E0425 cannot find type NSArray` (`*2`) / `NSString` / `EmacsMenu` | `src/include/nsterm.rs` | 215-218 |
| 2 | `E0425 cannot find type composition` | `src/include/dispextern.rs` | 1639 (`glyph_string.cmp`) |

**0 rows are in this unit's own text**: `build/probes/fontset/_root.rs` (which *is* this unit's
text plus the mirrors' copies) is not the primary location of any of the ten.  The 11 warnings
are all mirror-side too (`unused doc comment` x10: `lib/include/stdbit.in.rs` 644,
`src/include/lisp.rs` 3939 x5, `src/include/dispextern.rs` 669, `src/include/charset.rs` 697,
`src/include/frame.rs` 1360, `src/include/nsterm.rs` 98/210/393/663/781/949; and one
`unused attribute`).

**③ How the unit got to that reading.**  Five `xtask probe` runs, in order (budget: three;
**reported rather than kept**): (1) exit 1, **145 errors** - dominated by 119 x `E0423 cannot
find function … ` (this port called its own `macro_rules!` names with C's spelling,
`FONTSET_FROM_ID (id)`, instead of `FONTSET_FROM_ID! (id)`), 9 x `E0428` (the `include/fontset.rs`
collision of Deviations 1, now avoided), 4 x `font_deferred_log` (mirror), 4 x the NS types
(mirror), 2 x `composition` (mirror), plus 5 x `E0308`/`E0277`/`E0606`/`E0005` that were this
port's own (`charset_ordered_list_tick & MOST_POSITIVE_FIXNUM`'s mixed widths, `AUTO_STRING`'s
`&str`, `make_fixnum`'s argument width, the `charset_id` constant-pattern collision); (2) exit 1,
**147 errors** (that run's `tee` destroyed the log - the count is the summary line's); (3) exit 1,
**18 errors**, all that remained being the 6 mirror classes plus 4 of this port's own; (4) exit 1,
**16 errors**; (5) exit 1, **10 errors** - the reading in ②.  Ten `rustc` runs were made directly
on the materialisation and on the unit for syntax only (the plain-toolchain reading is 11, see
⑤).

**④ The mutation test (P-86 ⑦).**  With `let _mutation: c_int = c"injected type error".as_ptr ();`
injected as `fontset_ref`'s first statement, the same materialisation compiled with the same
`rustc` (the repository toolchain, from `femacs`) returns **one more error** than the unmutated
text - `E0308: mismatched types, expected c_int, found *const c_char`, at the injected line - so
the compile does read this unit's text and ②'s "0 errors of its own" is real.  (Direct `rustc` on
`build/probes/fontset/_root.rs` reads **11** `error[` rows against the probe's 10: the extra one
is the known mirror-side `E0658 c_variadic` at `src/include/lisp.rs` 3761, which only the
unpinned stable toolchain reports - the `docs/ports/gnutls.md` 246 / `docs/ports/ftfont.md` ②
reading.)

**⑤ The item gate.**  `python3 tools/extract-abi.py --check emacs/src/fontset.c
src/fontset.rs` -> **FAIL on one row only**: `line count drifted 16.2%: C 2257 vs Rust
2622`.  Every other rule passes: all 33 functions, all 26 C macros, the seven subrs (Lisp name,
`min`/`max` and the docstring digest each), the six `defsubr` rows and the six file-scope objects
the extractor's item list names.  The five token exceptions the file needs are rows in
`docs/ports/macro-body-notes.tsv` (Deviations 19) and each names the C construct that has no Rust
spelling.  (Before this round the same command was vacuous: the 11-line port had no `fn`, so the
checklist was empty and it printed `OK` - P-86 ⑩.)

**⑥ The other four gates.**

    python3 tools/check-declarations.py       -> 17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
    python3 tools/check-mirror-dupes.py       -> 22 duplicate declaration(s), 17 ratchet row(s), 5 failure(s)
    python3 tools/check-dead-file-coverage.py -> 96 ledger(s) bound (a C range table), 0 failure(s)
    python3 tools/check-ledger-ticks.py       -> 233 ledger(s) bound (a function tick to compare), 0 failure(s)
    python3 tools/check-citations.py          -> 676 file(s) scanned, 5 drift, 0 range, 1189 external, 30 allowed, 5 stale

`check-mirror-dupes`'s failures are in `src/{doprnt,ftfont,keyboard,tparam,xfont}.rs`
(other lanes' in-flight files; the count moved from 17 to 5 while this round ran, because those
lanes were editing them); **none** names this unit in either reading.  `check-citations`' 5 drift/1 range/5
stale rows are all `src/include/lisp.rs` except the one this round fixed
(`src/fontset.rs`'s `fontset.h 88-89` -> `fontset.h 46`, the only line in that header that
declares `face_for_font`); its re-run reports **0** finding for this unit.

The readings in ①-④ were taken on the revision before the last edit of this round, which is the
`check-citations` fix of ⑥: the citation line `// fontset.h 88-89` in the use-site block became
`// fontset.h 46`, a **comment-only** change (the only line of that header that declares
`face_for_font`), so the arm's error set and the object's bytes are the same reading on both
revisions.  The `md5` below is the final one.

**⑦ The clang probe the two casts rest on.**  C's `charset_ordered_list_tick & MOST_POSITIVE_FIXNUM`
(C 462) mixes `EMACS_UINT` with `EMACS_INT`; the port's two casts follow C's usual arithmetic
conversions.  `INT_STRLEN_BOUND (ptrdiff_t)` is 20 on this target (the C expression C 1815 is
`(63 * 146 + 484) / 485 + 1` = 20), which is why the port's `temp` is
`[c_char; 13 + 20]` - the ledger records the arithmetic rather than a remembered number.

## Deviations

1. **C 37's `#include "fontset.h"` is not included.**  The mirror (`src/include/fontset.rs`)
   declares its ten entry points in **one top-level** `unsafe extern "C" { … }`, and nine of them
   are defined by this unit (C 861-1234, 1266, 1798), so `include!("include/fontset.rs")` here is
   rustc's `E0428: the name … is defined multiple times` - measured in Evidence ③'s run (1), 9
   rows.  That is P-39's class, whose fix is to put those declarations in a `pub mod` the unit
   glob-imports (`include/lisp.rs`'s `c_functions`, the `font.rs` `dispextern_externs` shape); the
   mirror is not this unit's file to edit (P-88 ① / P-72 rule 4), so the one declaration this file
   needs from that header (`face_for_font`, C 1100 and 1145, declared `fontset.h` 46) is in the
   use-site block and the finding is **Owed** 2 for the integrator.
2. **Statement macros.**  `RFONT_DEF_NEW`'s C `do { … } while (false);` is a Rust block (the
   constant condition has no spelling; `docs/ports/macro-body-notes.tsv` has the `false` row), and
   C 1420-1422's three in-function `#define`s (`SCRIPT_FROM`, `SCRIPT_TO`, `POP_SCRIPT_RANGE`) are
   `macro_rules!` inside `set_fontset_font`, which is where C defines them.
3. **`#if 0` regions are recorded, not transcribed** (the `src/ccl.rs` 35-40 rule): C 956-967
   (`face_for_char`'s `face->font` arm, disabled over bug #15138), C 1307-1332
   (`free_realized_fontsets`'s `free_realized_face` walk).  Both are dead in every configuration
   and `tools/cinventory.py`'s dead-branch map reads the literal `#if 0` as the reference's own
   dead region, so neither is in any checklist.  C's `#else` arm at C 1333 **is** carried (it is
   the live one).
4. **`fontset_find_font`'s `found:` label** (C 739) is a labelled block: the three `goto found`
   statements are `break 'found`, and the label's body is the block's follower - the shape
   PORTING.md 2.2 prescribes (`src/fringe.rs` 1179's `'found`, `src/androidmenu.rs` 619's
   `'finish`).  The `goto` token is a `macro-body-notes.tsv` row because a labelled block cannot
   spell it (the `getdelim` / `mpz_get_str` precedent).
5. **`SAFE_ALLOCA_LISP`'s out-parameter.**  C 1905 passes `realized[0]` (an array element); the
   mirror's `SAFE_ALLOCA_LISP!` takes an *ident* (`$buf = …`), so the port allocates into
   `realized0` and assigns `realized[0] = realized0` on the next line, keeping C's array and every
   later `realized[k][i]` read as pointer arithmetic.
6. **The `charset_ordered_list_tick & MOST_POSITIVE_FIXNUM` casts** (C 462): C's `&` mixes
   `EMACS_UINT` and `EMACS_INT`, so the port writes the two conversions C's usual arithmetic
   conversions imply.  Evidence ⑦.
7. **Integer-width spellings.**  C's implicit `int` -> `EMACS_INT`/`ptrdiff_t` promotions are
   spelled out where Rust needs them: the three `make_fixnum` sites that take an `int`
   (`RFONT_DEF_SET_FACE!`, `RFONT_DEF_SET_SCORE!`, `set_fontset_nofont_face`), `FONTSET_REF!`'s
   `$c as isize` (the macro feeds both a `ptrdiff_t` and an `int`), and the `as isize`/`as c_int`
   at the `CHAR_TABLE_REF`/`XFIXNUM` boundaries.  C's `bool` stays `bool`; the four
   `ascii_changed = 1` sites are `true` (a `macro-body-notes.tsv` row, the
   `Fset_standard_case_table` precedent).
8. **Two callbacks are `unsafe extern "C" fn`.**  `fontset_compare_rfontdef` (C's `qsort`
   comparator), `accumulate_script_ranges` (C 1615's `map_char_table`) and `set_fontset_font`
   (C 1653's `map_charset_chars`) are passed as function pointers, so the port gives them C's
   calling convention (the `callproc.rs` `call_process_kill` shape); no `#[unsafe(no_mangle)]`,
   because C declares all three `static` (PORTING.md 0.2).
9. **C 1688-1700's `#ifdef HAVE_WINDOW_SYSTEM`** cuts an `if`/`else` in half (C's `#endif` sits
   before `Fmodify_frame_parameters`).  The port spells it as a `#[cfg]` on the whole `if` - the
   two arms are carried, and because the file gate *is* that macro, the arm is always live in this
   unit; the comment at the site records the `#endif` boundary.
10. **C's uninitialised locals** (`fontset_font`'s `default_rfont_def UNINIT`, C 763;
    `fontset_get_font_group`'s `from1`/`to1`) start zeroed (PORTING.md 1.5); `UNINIT` is
    `conf_post.h`'s zero marker in this configuration, so its name cannot reappear - one
    `macro-body-notes.tsv` row.
11. **The include graph is gated** (P-86 ⑧): `lisp.h` … `pdumper.h` **and** the two libc mirrors
    (`<stdio.h>`, `<stdlib.h>`) and lisp.h's `<limits.h>`/`<string.h>` sit under
    `have_window_system`.  Those mirrors carry no `use` of their own (their scalars are the
    including unit's), so an ungated include would fail the frozen build with 146 `E0425`s for
    `c_int` - measured while writing this unit.
12. **`#include TERM_HEADER`'s arms** (C 39) are the six-arm chain `src/frame.rs` 84-98 uses, one
    per platform feature; the frozen table spells `TERM_HEADER` `undef` (macros.tsv 3742), so the
    whole chain is text here.
13. **Twenty use-site declarations** (P-88 ①), each with its C header and line: `qsort`,
    `nconc2`, `assoc_no_quit`, `larger_vector`, `decode_live_frame`, `check_window_system`,
    `recompute_basic_faces`, `xstrcasecmp`, `find_font_encoding`, `font_has_char`, `font_match_p`,
    `font_get_name`, `copy_font_spec`, `font_find_for_lface`, `font_open_for_lface`,
    `font_load_for_lface`, `font_open_by_spec`, `font_parse_family_registry`, `font_parse_xlfd`,
    `font_dynamic_unparse_xlfd`, `gui_set_frame_parameters_1` and `face_for_font` (Deviation 1).
    The mirrors in this include graph do not carry them yet (`include/font.rs` is a WIP slice, and
    `include/dispextern.rs` keeps `xstrcasecmp` inside `dispextern_externs`, which the unit reaches
    as `dispextern_externs::xstrcasecmp` - the `src/ftfont.rs` 1494 spelling).
14. **`INT_STRLEN_BOUND`** (C 1736's `<intprops.h>`, which lisp.h 30 reaches) is declared in the
    unit as the same bound over `size_of::<$t>` - the `src/android.rs` 110-121 shape, and the only
    spelling of that name in the tree.
15. **The `DEFVAR_LISP`/`DEFVAR_BOOL` variables** (`Vfont_encoding_charset_alist`,
    `Vuse_default_ascent`, `use_default_font_for_symbols`, `Vignore_relative_composition`,
    `Valternate_fontname_alist`, `Vfontset_alias_alist`, `Vvertical_centering_font_regexp`,
    `Votf_script_alist`) are defined here, because the `DEFVAR_*!` macros take the address of a
    real item and `include/globals.rs`'s `variables::*` rows are the extern declarations the
    crate-root definitions shadow (the `src/coding.rs` 250 shape).
16. **`charset_id` -> `charset_id_`** (C 550's parameter): `include/charset.rs` 76 has
    `pub const charset_id`, and a parameter whose name resolves to a constant is read as a
    refutable constant pattern (`E0005`, measured).  The trailing underscore folds away in the
    token check (`body_token_seq`'s `rstrip ("_")`), so C's own name is what the comparison sees.
17. **`AUTO_STRING`** takes C's string as `c"-".as_ptr ()` (the mirror's `AUTO_STR!` hands it to
    `strlen`/`make_unibyte_string`).
18. **Line count**: C 2,257 vs Rust 2,622 (**+16.2%**, +365 lines: the per-item `#[cfg]` for the
    file gate, the use-site declaration block of Deviation 13, and the `macro_rules!` bodies that
    C's one-line `#define`s do not pay for).  Owed 1.
19. **Five `docs/ports/macro-body-notes.tsv` rows** this unit needs, each with its reason:
    `RFONT_DEF_NEW`'s `false`, `fontset_find_font`'s `goto`, `fontset_font`'s `1,uninit`,
    `Fset_fontset_font`'s `1`, `syms_of_fontset`'s `doc`.  Four are established classes
    (`do`/`while (false)`; PORTING.md 2.2's labelled block; C's int passed to a `bool`; the
    `DEFVAR_*` docstring position), and `uninit` is the `cons_to_unsigned` class.

## Owed

1. `docs/ports/line-drift-tolerance.tsv`: one row, stem `fontset`, **basis `total`**, tolerance
   >= **0.17** - the port carries 16.2% more physical lines than C for the structural reasons in
   Deviation 18 (the P-86 per-item `#[cfg]`, the use-site block, the `macro_rules!` bodies).  This
   file is **not** in the lane's writable set, so the integrator adds it; with it,
   `extract-abi.py --check` for this pair is clean (Evidence ⑤'s one remaining row).
2. **Three mirror-side findings** (each is a header another lane owns; named here for the
   integrator):
   (a) `src/include/fontset.rs`: the ten prototypes sit in one top-level
   `unsafe extern "C" { … }`; the nine this unit defines need the `pub mod` + `use …::*;` shape
   (P-39 / P-21) before this unit can `include!` that mirror at C 37.  (b)
   `src/include/font.rs` 306-309: the `FONT_DEFERRED_LOG!` body names `font_deferred_log`,
   which that mirror does not declare in the scope of its includers (4 errors in Evidence ②).
   (c) `src/include/nsterm.rs` 215-218: `NSArray`, `NSString` and `EmacsMenu` are named but
   not declared (the NS arm's Objective-C surface); and `src/include/dispextern.rs` 1639:
   `glyph_string.cmp` is `*mut composition` with no `composition` in scope (P-88 ②'s
   untyped-pointer rule).

   **(a)-(d) were fixed by the integrator in status 3.526**: the ten prototypes went into the
   `fontset_decls` module the mirror glob-imports, `font_deferred_log` joined `font_log_decls`,
   and the four NS class pointers plus `glyph_string.cmp` became `*mut c_void` with their `/// C:`
   lines (P-72 rule 3).  The `fontset` arm now reads **exit 0 / 0 error** (was 10).  (a)'s payoff
   is still deferred: this unit keeps its typed use-site declaration of `face_for_font`, because
   the mirror spells `struct frame *`/`struct face *` as `*mut c_void` (P-55's degradation) and
   spelling C 37's include here would force casts at both call sites and downgrade two typed
   parameters - Deviation 13 stands until the frame/face type story lands (P-90 ①).
3. **The arm's `TERM_HEADER` choice is a per-target decision** the integrator may want to record:
   `have_ns` is the Darwin terminal header, and Evidence ② shows it also inherits finding 2(c).
   `FRAME_FONTSET` (C 1381, 1671) comes from that header and from nowhere else, so the arm cannot
   be read with the file gate alone.
4. `tools/check-port-lines.py --write-pending` and `docs/status.md` are the integrator's steps
   (P-86 ④); this ledger's `## Functions defined` list is a hand-kept record that a later
   `extract-abi.py` checklist regeneration would replace with the empty generated form - the C
   line ranges above are the coverage evidence either way.
5. **Nothing about this file's behaviour** can be checked here (see `## Not claimed`); the
   integrator's acceptance for the whole class is the frozen `build-tu` plus the arm reading.

## Not claimed

- **No behaviour.**  The frozen configuration compiles nothing from this file (the 336-byte
  object is empty), and the arm flip is a *compile*: no fontset was created, no face realized, no
  font listed, matched, opened or measured, and no char-table was touched.  No C compiler on this
  machine has ever built `src/fontset.c` either (there is no window system in the frozen
  configuration), so the port is compared **textually** (`extract-abi.py --check`, the token
  bodies, the layout assertions of the mirrors it includes) and by no execution.
- The arm flip's reading is Evidence ②: **0 errors in this unit's text, 10 in four headers**, and
  the arm needed four `--cfg`s, one of which (`dispextern_face_records`) is a tree gate and not a
  C condition of this file.  That is a compile reading, not "the class can be verified"
  (P-86 ③⑴): the Objective-C, `composition` and `font_deferred_log` rows mean the arm does **not**
  reach 0 errors, and the mutation test (Evidence ④) is what says the 0-of-its-own reading is
  real.
- The `#if 0` regions (C 956-967, C 1307-1332) are recorded text; nothing here claims they were
  ported or that they are dead for a reason this tree re-derived.
- The line-count row is a *drift* reading (Deviation 18), not a defect claim, and the five
  `macro-body-notes.tsv` rows are token exemptions (Deviation 19), not coverage.
- `check-mirror-dupes`'s 17 failures and `check-citations`'s 5 drift / 1 range / 5 stale rows are
  **other files'** (Evidence ⑥); nothing in this ledger speaks for them.
- **Budget, reported rather than kept**: `build-tu` **3** runs (budget 3; run 1 was the syntax
  failure of Evidence ① and run 3 is the final revision), `xtask probe` **5** runs (budget 3; Evidence ③ lists them and why each
  was needed - the 119-site macro-call class cost two), and about ten direct `rustc` runs on the
  unit and on the materialisation for syntax, the classification and the mutation test.  The disk
  was checked before writing (`df -h /`: 8.2 GiB available) and `/tmp/fontset-lane` and this
  round's probe materialisation (`build/probes/fontset/`) are removed at the end of the round.
