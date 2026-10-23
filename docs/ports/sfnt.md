# emacs/src/sfnt.c (21,215 lines) — slices 1-9: C 1-6306, C 6312-6557 and C 8161-10224 plus C 10225-10663, C 13918-14554 and C 14555-16447

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2645 adds `sfnt.o` to `ANDROID_OBJ` only inside the `XCONFIGURE = android`
> branch (the `REALLY_ANDROID` one, 2620-2654) — `android.o` is added one branch earlier, at
> 2598, which is why `src/android.c`'s file gate is `have_android` alone and this one is the
> pair — and `docs/baseline/frozen-build-sources.txt` lists no `sfnt.o`.  What follows is P-86
> ②'s **C line-range table** for these slices, the `## Evidence` readings and the
>
> **The file is COMPLETE as of slice 24 (2026-10-06): every line of C 1-21215 is transcribed**, the
> `#ifdef TEST` programme and `main` included.  The range table's last row and `## Evidence` ⑲
> carry that reading, and `docs/ports/line-drift-tolerance.tsv`'s newest `sfnt` row records what
> the slice cost (C 21,215 vs Rust 22,955, 8.2%).  "Complete" is a **text** statement - no gate
> compiles the TEST arm, so nothing here has been type-checked or run; see "Not claimed".
> `**⑤ Slice 10's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the standing empty object - every item of this
unit is behind the file gate the frozen configuration leaves false, so build-tu is a syntax reading
only).  **The arm flip is where the text is type-checked**: `probe src/sfnt.rs … --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'` reads **8 errors at HEAD** and **1
after this slice** - the 1 is Owed 25's named `E0425: cannot find function sfnt_interpret_run`
(C 9832's call to C 11396), and the 7 that left are **five `E0425`s for `time_t`/`timespec` in
`lib/include/sys_time.rs` 44/90, `lib/include/sys_select.rs` 56 and `src/include/thread.rs`
302/320, which this slice's `<time.h>`-mirror edge fixes** (Deviation 78 (a)), plus nothing else -
the slice itself adds **no** error in either direction.  `python3 tools/extract-abi.py --check
emacs/src/sfnt.c src/sfnt.rs` -> **175 rows before, 165 after**: the five functions and the
five macros above left the list (31 -> 26 `missing function`, 143 -> 138 `missing macro`), the
line-count row stays (13,649 Rust lines against C's 21,215 is **35.7%**, the whole-file drift the
C 11103-21215 remainder and the TEST programme make; slice 9 read 37.9%), and one token row was
needed (`sfnt_interpret_iup_1 goto,touched,wraparound`).  The other gates: `check-declarations.py`
**17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale** (this turn also moved
`citations-allowlist.tsv`'s `dispextern.rs` row from 617 to 620 - the row the previous round's
`GC*` edit had left stale, see the status entry); `check-dead-file-coverage.py` **145 ledger(s)
bound (a C range table), 0 failure(s)**; `check-port-lines.py` **0 files under ten percent, 0
ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**,
0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0
disagreements**.

**⑥ Slice 11's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - the file gate is false
here, so this is a syntax reading).  **The arm flip**: `probe src/sfnt.rs … --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'` reads **1 error at HEAD and 1 after
this slice** - the same Owed 25 `E0425: cannot find function sfnt_interpret_run` on both sides, so
this slice adds no error at all (HEAD's five `time_t`/`timespec` `E0425`s were retired by slice 10's
`<time.h>` edge, Evidence ⑤).  Its own one compile defect was an `E0308` at `_root.rs` 13849 in the
first run: C's `sfnt_fword projection` is assigned the `sfnt_f26dot6` `DUAL_PROJECT` returns, and the
port needed C's narrowing written out (`as sfnt_fword`, Deviation 79 (b)).
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **162 rows** (165
before): `missing function` **26 -> 23** (this slice's three), `missing macro` 138 unchanged, the
line-count row 13,649 -> **14,000** Rust lines against C's 21,215, **34.0%** (was 35.7%).  Two token
rows were needed: `sfnt_interpret_iup_1`'s list gained `wraparound_1` (slice 10's reshape spells the
three entries `'untouched`/`'skip33`/`'body`, so C's own label names are what the row has to carry)
and `sfnt_project_zp1_zp0_org goto` was added (Deviation 79 (a)).  The other gates:
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the range
table's last two rows now split at C 11395 and still tile C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0
stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a ledger;
`check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.

**⑦ Slice 12's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes** (the empty object; the file gate is false, so this is a syntax reading).  **The arm flip**:
`probe src/sfnt.rs … --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` reads
**1 error at HEAD and 1 after this slice** - the same Owed 25 `E0425: cannot find function
sfnt_interpret_run`, so the twelve new functions add no error.  The first drafts needed three rounds
(24 -> 3 -> 1 errors), all of them C's implicit conversions written out and one naming trap:
`sfnt_glyph` is the port's `sfnt_glyph_index` (C's scalar typedef; `struct sfnt_glyph` owns the
other name - the `E0605`/`E0308` pair at C 13765/13775), `sfnt_table_directory`'s `num_tables` is
`uint16_t` and its `offset`/`length` are `uint32_t` (four sites `as`-converted where C promotes),
`base_character_value` is `uint16_t` and the return type is `uint32_t` (one `as`), and six bindings
C fills through a pointer (`num_ranges`, `num_mappings`, `template`, and the three locals the
`bsearch` calls read) are zero-initialised (Deviation 10's class, four `E0381`s).
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **150 rows** (162
before): `missing function` **23 -> 11**, `missing macro` 138 unchanged, the line-count row 14,000 ->
**14,645** Rust lines against C's 21,215, **31.0%** (was 34.0%).  Three rows the first drafts earned
went away with the code rather than with a note: two `does not mention mappings`/`ranges` rows (the
`bsearch` bases now spell C's `uvs->mappings`/`uvs->ranges`) and one `family context` row (the two
`sizeof`-over-a-place sites C 13628/13631 now keep the `context` family token - Deviation 17's
ruling); the one token row this slice does owe is `sfnt_create_uvs_context goto` (C 13685's `bail:`
and its five `goto`s).  The other gates: `check-declarations.py` **17 declaration sites (0
forbidden, 17 allowed literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift,
0 range, 2029 external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound
(a C range table), 0 failure(s)** (the remainder row now splits at C 13369 and the table still tiles
C 1-21215, with C 13916-13917 as its own nothing-to-port row);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0
stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a ledger;
`check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.

**⑧ Slice 13's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes** (the empty object; a syntax reading).  **The arm flip**: `probe src/sfnt.rs … --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'` reads **1 error at HEAD and 1 after
this slice** - the same Owed 25 `E0425: cannot find function sfnt_interpret_run` - and this slice's
four functions needed **no** fix at all: the first run was clean, which the ledger records as a
reading rather than as silence (the twin's idioms came from the ported `sfnt_transform_coordinates`,
whose body is C 2551-2618's, and the builder's from `sfnt_build_glyph_outline`).
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **146 rows** (150
before): `missing function` **11 -> 7**, `missing macro` 138 unchanged, the line-count row 14,645 ->
**14,938** Rust lines against C's 21,215, **29.6%** (was 31.0%).  Two token rows were needed and
both are the established classes: `sfnt_decompose_instructed_outline goto` (C 12164's `fail:` and
its two `goto`s) and `sfnt_transform_f26dot6 double` (the six `f64` locals and the two `1.0`
products - the `sfnt_transform_coordinates` row's class).  The other gates:
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the remainder
row now splits into five and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0
stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a ledger;
`check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.

**⑨ Slice 14's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - the file gate is false
here, so this is a syntax reading).  **The arm flip**: `probe src/sfnt.rs … --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'` reads **1 error at HEAD and 0 after
this slice** - HEAD's error is the Owed 25 `E0425: cannot find function sfnt_interpret_run` (C
9832's call inside `sfnt_interpret_call`), and this slice *is* the definition that names it; the
0-error arm produces an object (**9,219,424 bytes** at `/tmp/sfnt14/x.rlib`, 4 warnings, none of
them about this file's text), which nothing loads or runs.  The slice's five `probe` runs are (i)
**6 `E0308`s** - C's implicit conversions written out (the three `DELTAC*` calls' `operand2 as
c_uchar` and the three `DELTAP*` calls' `argn as c_uchar`, against C 9538/9705's `unsigned char
operand`), (ii) **1 `E0425: cannot find macro IP`** - the 146-macro transcription had missed C
8007-8010, the `IP ()` definition that sits between `SHPIX` (C 7985-8005) and `MSIRP` (C
8012-8022), (iii) **8 `E0381`s** - the four `&raw mut` out-parameters Rust cannot prove initialised
(`SPVTL`'s and `SFVTL`'s `vector`, `SDPVTL`'s `vector`, `SHPIX`'s `dx`/`dy`; each of those four
macros is expanded twice, Deviation 10's class), (iv) **0 errors** after the `IP` macro landed, and
(v) **0 errors** - the delivered revision after the header comment was rewritten (a comments-only
edit: the reading is repeated rather than carried over, which is why the count is five).
The macro set is checked by name too: the C 6550-8160 window holds **142** distinct `#define NAME
(…)` names and the port's in-function `macro_rules!`s hold the same **142** (a two-way `comm` is
empty in both directions; C defines `POP`/`POP_UNCHECKED` twice inside that window, once per TEST
arm).  `python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **7 rows** (146
before): `missing function` **7 -> 6** (the six callers of `sfnt_interpret_run`:
`sfnt_interpret_font_program`, `sfnt_interpret_control_value_program`,
`sfnt_interpret_simple_glyph` and the three compound-glyph interpreters), `missing macro` **138 ->
0** (the last one, `TEST_STATIC`, is the `docs/ports/excluded-macros.txt` row Owed 1 has asked for
since slice 1: its Rust form is the file's `cfg_attr`), and the line-count row **14,938 -> 16,872**
Rust lines against C's 21,215, **20.5%** (was 29.6%).  The same run *before* this slice's notes
land reads **24 rows**, and the difference is the 16 the notes retire: the 15 instruction macros
whose C bodies carry `goto`/labels and `sfnt_interpret_run` itself for C 11420's
`switch`/`case`/`default`, each one now a row of `docs/ports/macro-body-notes.tsv`.  The other gates: `check-declarations.py` **17
declaration sites (0 forbidden, 17 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the
remainder row now splits at C 11983 and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑩ Slice 15's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - a syntax reading).
**The arm flip reads 0 errors at HEAD and 0 after this slice**: slice 14 closed the flip's last
`E0425`, so this slice cannot move that reading and does not claim to.  What it adds is measured two
other ways.  (i) **The object, symbol by symbol**: `probe src/sfnt.rs … --emit obj` produces a
**90,168-byte** Mach-O object whose `nm -g` defines **`_sfnt_interpret_control_value_program`** and
**`_sfnt_interpret_font_program`** - the two C symbols, i.e. the `cfg_attr`'s `no_mangle` doing
what C's `TEST_STATIC` does outside the TEST arm - and whose `nm -u` shows the three plain
`setjmp`/`longjmp` references the unit now needs: `_setjmp` (Mach-O's underscore for the C name
`setjmp`, this slice's), `_longjmp` (C's `longjmp`, C 6556's) and `__setjmp` (the C name `_setjmp`,
the lisp.rs mirror's `sys_setjmp`).  That is where the fresh C measurement is used:
a three-line programme (`int f (jmp_buf b) { return setjmp (b); }` and the same for `longjmp`)
prints `_setjmp`/`_longjmp`, so C's plain `setjmp` binds the C name `setjmp`.  (ii)
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **5 rows** (7
before): `missing function` **6 -> 4** (`sfnt_interpret_simple_glyph` and the three compound-glyph
interpreters), no `missing macro` row, and the line-count row **16,872 -> 16,987** Rust lines
against C's 21,215, **19.9%** (was 20.5%).  All four `probe` runs are **clean** - 0 errors - and
they are (i) the new text's first run, (ii) the `--emit obj` variant whose `nm` readings are above,
(iii) the HEAD baseline (0 errors, which is what slice 14's delivery reads), and (iv) the delivered
revision after the header comment was rewritten (a comments-only edit: the reading is repeated
rather than carried over).
The other gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029
external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range
table), 0 failure(s)**; `check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s);
0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**,
0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0
disagreements**.
**⑪ Slice 16's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - a syntax reading).  The
arm flip reads **0 errors at HEAD and 0 after this slice** (slice 14 closed its last `E0425`), so
the two other compiler readings are what carry this slice.  (i) **The object**: `probe src/sfnt.rs
… --emit obj` -> exit 0, 0 errors, **91,816 bytes**, and `nm -g` defines all three C symbols this
file's `cfg_attr` now produces - `_sfnt_interpret_control_value_program`,
`_sfnt_interpret_font_program` and **`_sfnt_interpret_simple_glyph`** (C 12304's `TEST_STATIC`).
(ii) `python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **4 rows** (5
before): `missing function` **4 -> 3** (the three compound-glyph interpreters), no `missing macro`,
no `does not mention` row, and the line-count row **17,235 -> 17,238** Rust lines against C's
21,215, **18.7%** (was 18.8% before the `ckd_mul` spelling below).  The slice's six `probe` runs are
(i) the first draft, which read **2 `E0381`s** - C 12313/12314's `phantom_point_1_y`/
`phantom_point_2_y`, the two out-parameters `sfnt_compute_phantom_points` writes, zero-initialised
here (Deviation 10's class) -, (ii) `--emit obj` after that fix (the `nm` reading above), (iii) the
same after the file header was rewritten (a comments-only edit; the reading is repeated rather than
carried over), (iv) the HEAD baseline, (v) after C 12324/12330's two `ckd_mul`s were spelled with
C's multiplicand inside the call (Deviation 84 (d)), and (vi) after the notes row for `alloca`
landed.  The other gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029
external, 36 allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range
table), 0 failure(s)**; `check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s);
0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**,
0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0
disagreements**.
**⑫ Slice 17's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - a syntax reading).  The
arm flip reads **0 errors at HEAD and 0 after this slice** (slice 14 closed its last `E0425`), and
the `--emit obj` reading carries one fact worth writing down: the object is **91,816 bytes, the
same size slice 16's was**, because `sfnt_interpret_compound_glyph_2` is C's plain `static` and
nothing in this port calls it yet (its only caller is C 12794's `sfnt_interpret_compound_glyph_1`,
the next slice), so rustc drops the dead function from the object altogether.  The slice's compiler
reading is therefore the **clean first run** (0 errors), repeated on the delivered revision after
the header comment was rewritten.  `python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> **3 rows** (4 before): `missing function` **3 -> 2** (the two remaining
compound-glyph interpreters), no `missing macro`, no `does not mention` row (this slice's `alloca`
becomes a `docs/ports/macro-body-notes.tsv` row, the `sfnt_interpret_simple_glyph` row's twin), and
the line-count row **17,238 -> 17,470** Rust lines against C's 21,215, **17.7%** (was 18.7%).  The
other gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0
failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36
allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0
failure(s)** (the compound row now splits at C 12739 and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑬ Slice 18's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - a syntax reading).  The
arm flip reads **0 errors at HEAD and 0 after this slice**, and the `--emit obj` reading is this
slice's headline: the object grows from slice 17's **91,816 bytes to 98,448**, and `nm -g` now
defines **four** C symbols - this slice's `_sfnt_interpret_compound_glyph` (C 13229's
`TEST_STATIC`), joined by `_sfnt_interpret_control_value_program`, `_sfnt_interpret_font_program`
and `_sfnt_interpret_simple_glyph`.  That growth is the measured consequence of the group becoming
reachable: `sfnt_interpret_compound_glyph_1` and `_2` were dead text until this slice's exported
wrapper called them, which is why slice 17's object had stayed at slice 16's size.
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **1 row** (3
before) - **the line-count row alone**: `missing function` **2 -> 0** (both remaining interpreters
landed), no `missing macro`, no `does not mention` row, and the line-count row **17,470 -> 18,147**
Rust lines against C's 21,215, **14.5%** (was 17.7%).  The whole non-TEST half of `src/sfnt.c` is
therefore transcribed; the only C left in the unit is C 16448-21215's `#ifdef TEST` programme
(Owed 21).  The slice's first run read **2 errors** - an `E0308` (`sfnt_lookup_glyph_metrics` takes
`sfnt_glyph_index`, a `u32`, where `component->glyph_index` is a `uint16_t`: `as sfnt_glyph_index`)
and an `E0381` (the wrapper's stack `context`, whose C `memset` is the port's `write_bytes`, so the
local is `std::mem::zeroed ()` at its declaration - the `sfnt_decompose_glyph` 4102/4152 shape) -
and the delivered revision, the `--emit obj` run above and the HEAD baseline are all clean.  The
other gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0
failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36
allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0
failure(s)** (the compound row now splits at C 13369 and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑭ Slice 19's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (still the empty object - a syntax reading).  The
arm flip reads **0 errors at HEAD and 0 after this slice**, and **the object is 98,448 bytes on
both sides**: every line this slice adds is behind `feature = "test"`, which the probe's cfg set
does not carry, so the frozen arm is provably untouched - the same kind of measurement slice 17
showed from the other side.  `python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> **1 row**, the line-count row, as at slice 18 (the TEST arm's items are
`#ifdef TEST` dead text for that tool), so this slice moves the line count alone: 17,470 -> **18,428**
Rust lines against C's 21,215, **13.1%** (was 14.5%).  Two measurements are new.  (i) **The `test`
flip is not a working configuration today**: adding `--cfg 'feature="test"'` to the probe's two cfgs
reads **3,101 errors**, and the overwhelming majority are *not* this unit's text - 2,507 error lines
in `include/sfnt.rs`, 572 in `unistd.in.rs`, 465 in `stdbit.in.rs`, 80 in `fcntl.in.rs` and so on,
all of the `cannot find type c_int`/`c_void`/`size_t` and `cannot find macro eassert` families -
because the mirrors' own gates assume the frozen shape.  The flip therefore needs a tree-wide pass
before any per-item judgement, which is exactly why `xtask`'s `FLIPS` table carries no `test` entry.
(ii) That same run found **one** defect in this slice's own text: C 16596's prototype written out is
`error: free function without a body`, because Rust has no function prototype.  The delivered
revision drops it and says why in the text; after that fix the frozen arm is clean, and the
`test`-flipped run's errors name no other line of the new block.  The other gates:
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the last row
now splits at C 16704, so the table prints **68 rows** for this file and still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑮ Slice 20's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading).  The arm
flip reads **0 errors at HEAD and 0 after this slice**, and the object is **98,448 bytes on both
sides** again: this slice's text is behind `feature = "test"` too, so the frozen arm is untouched
for the same measured reason as slice 19.  `python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> **1 row**, the line-count row: 17,470 -> **18,672** Rust lines against C's
21,215, **12.0%** (was 13.1%).  **The correction this slice carries is the reading worth keeping**:
slice 19 said the X11 names "have no mirror in this tree", and that was stale.
`include/xlib.rs` *is* the tree's mirror of the X11 SDK's `<X11/Xlib.h>` (`have_x_windows`-gated,
with the measured constants in its `x11_constants` module and a measured **192-byte** `XEvent` at
its 1181), and this machine does have the headers - `/opt/homebrew/include/X11/Xlib.h`, plus
`extensions/Xrender.h` as a symlink to libxrender 0.9.12 - which `ls` and a `clang` run **with that
include path** confirm (slice 19's probe omitted the path and reported the header missing).  What
does remain missing is measurable name by name, and Owed 30 lists it with the header line each name
would cite; the mirror's own header records the method.  This slice also adds C 57's edge
(`include!("include/xlib.rs")` beside the TEST gate - the mirror's items stay
`have_x_windows`-gated, so the arm's X11 text resolves only where both features are on), and C
16769/16770's two `alloca`s go to the stack pool (`sa_stack_alloc`, PORTING.md P-05).  The other
gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0
failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36
allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0
failure(s)** (the test-programme rows now split at C 16889 and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑯ Slice 21's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading).  The arm flip
reads **0 errors at HEAD and 0 after this slice**, and the object is **98,448 bytes on both sides**
again: the whole slice is behind `feature = "test"`, so the frozen arm is untouched (the same
measurement slices 19-20 recorded).  `python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> **1 row**, the line-count row: 18,672 -> **20,073** Rust lines against C's
21,215, **5.4%** (was 12.0%), i.e. this slice alone closed more than half of the drift the unit came
in with.  **Two verifications are new, and both were run against C's own text rather than by
eye.**  (i) **The four rounding arrays**: the C region (18002-18081) was extracted verbatim with its
three initialiser macros and compiled here (`clang /tmp/cnt/t2.c`), which printed `54 48 9 8` bytes
for `sfnt_sround_instructions`/`sfnt_sround_values`/`sfnt_s45round_instructions`/
`sfnt_s45round_values` and then every byte of the two instruction arrays; the port's literals were
evaluated element by element and compared - **54/54 and 9/9 identical**, and the four element counts
match.  That measurement also *caught two errors in the first draft*: `SFNT_ROUND_VALUE` expands to
**three** elements (`0xb0, value, 0x68`), so the two arrays are 54 and 9, not the 42 and 7 the draft
declared - the counts were corrected from the clang reading, and the two `countof` fields with them.
(ii) **All 81 `_test_args` objects**: a script parsed both sides (C's `static struct … = { … };`
bodies, expanding the `(uint32_t []) { … }` compound literals and evaluating the same arithmetic in
both languages) and compared name, values, element count, trap flag and IP for every object:
**79 objects compared field by field, 0 mismatches**, the other two (`sround_test_args`,
`s45round_test_args`) being the ones whose stack is a named array - covered by (i).  The slice's own
`probe` runs are the frozen one on the arriving text, the same after the header comment was
rewritten, and the HEAD baseline: all three 0 errors.  The other gates: `check-declarations.py`
**17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the
test-programme rows now split at C 18182 and the table still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a
ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0 unknown, 0 disagreements**.
**⑰ Slice 22's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading).  The arm flip
reads **0 errors at HEAD and 0 after this slice**, and the object is 98,448 bytes on both sides (the
slice is behind `feature = "test"`).  **`python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> `OK`** - the unit's checker now reports **zero rows**, for the first time in
its life: every function, `DEFUN`, macro, type, struct field, file-scope object and constant the
tool compares is present, and the line count is inside tolerance (20,073 -> **21,327** Rust lines
against C's 21,215, **+0.5%** - the port is now slightly *longer* than C, where slice 21 read
5.4% short).  The only line the tool still prints about this unit is its `(1 C macros have a
recorded Rust counterpart: TEST_STATIC)` note from the exclusion row slice 14 landed.  **The
transcription of `src/sfnt.c` is therefore complete in the checker's terms** - which is a statement
about the text, not about behaviour (see "Not claimed").  Two measurements back the table up.
(i) **`all_tests[]` has 94 entries, not 166**: the ledger had carried "166 `_test_args` objects"
since an early slice, and slices 19-21 repeated it; counting C's own text settles both numbers -
`_test_args` objects are **81** (slice 21 transcribed all of them, and its field-by-field
comparison covered 79 + 2) and the table entries are **94** (`grep`'s four independent counters -
entry openers, closers, `check` fields and name strings - all read 94).  The wrong number is
corrected here and in the range table, and the correction is the reading: nothing about the port
changed, a claim did.  (ii) **Every entry was compared against C a second time**: a script parsed
C's 94 entries and the port's 94 entries and compared name, instruction bytes, argument and check
function - **93 byte-identical, 1 in the `const`-block form C's sized compound literal needs
(`(unsigned char [103]) { 0x40, 101, }`), and the two named-array entries' `num_instructions` are
slice 21's measured 54 and 9**.  One defect did surface on the way, in the *new* text: C 18321/
18345's `- (signed char) 32` / `31` written out verbatim is a parse error in Rust
(`expected … found char`, measured on the frozen probe - which parses even `#[cfg]`-disabled text),
so both are written `(-(32 as c_schar)) as c_uchar` / `(-(31 as c_schar)) as c_uchar`; the first
frozen run after the append read 4 errors and the delivered revision reads 0.  The other gates:
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the table now
splits at C 19651 and still tiles C 1-21215); `check-port-lines.py` **0 file(s) under ten percent, 0
reference line(s); 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s)
bound, 0 failure(s)**, 0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0
unknown, 0 disagreements**.
**⑱ Slice 23's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading).  The arm flip
reads **0 errors at HEAD and 0 after this slice**, and the object is **98,448 bytes on both sides**
(the slice is behind `feature = "test"`).  `python3 tools/extract-abi.py --check emacs/src/sfnt.c
src/sfnt.rs` -> **`OK`** again, holding at 21,888 Rust lines against C's 21,215 (**+3.2%**,
where slice 22 read +0.5%), so the checker's two readings - completeness and the line-count
tolerance - are both still inside their bounds while the file's last block is written.  What the
frozen probe verifies here is *syntax only*: the two chunks parsed clean on their **first** run
(unlike slice 22, whose first run read 4 errors), because every name in this block is either C's own
X11 spelling or a mirror name this unit already had, `waitpid` excepted - it is declared at the use
site (P-88 ①'s `sysdep.rs` 948 shape) and that declaration is what the frozen run's parser accepted.
The blocks' own text has no type-checking reading and does not claim one (`feature = "test"` is
carried by no gate; flipping it reads 3,101 errors, the bulk in the mirrors - Evidence ⑭).  The
other gates: `check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0
failures**; `check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36
allowed, 0 stale**; `check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0
failure(s)** (the table now splits at C 20202 and still tiles C 1-21215);
`check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new,
0 stale** (the +3.2% file ratio is inside the tolerance the ratchet records); `check-ledger-ticks.py`
**238 ledger(s) bound, 0 failure(s)**, 0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c`
**31 conditions, 0 unknown, 0 disagreements**.
**⑲ Slice 24's readings.**  `cargo run -q -p xtask -- build-tu src/sfnt.c` -> exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading).  The arm flip
reads **0 errors at HEAD and 0 after this slice**, and the object is **98,448 bytes on both
sides** (the whole slice is behind `feature = "test"`), so 1,061 Rust lines leave the frozen arm
the same size.  The md5 is a *source-sensitive* reading and is recorded that way: the two texts
compared before this revision's header paragraph both read
`b77d7449a2faa1ff45c0e6220e240172`; after the header paragraph the same 98,448 bytes read
`dfceb47e2fbe93d50977a7647557fb83`, and **one added comment line moves it again**
(`31033a291beca5a6195c8136c22d81aa`, measured twice, deterministic for a fixed text) - the object
carries something content-dependent even though **no `__DWARF` section exists** (measured with
`otool -l`) and no `sfnt.rs:` string is in it (`strings`), so the *cause* is not claimed here and
the **size** is what a cross-revision comparison should use.  What this reading carries is the slice's
*own* syntax, and it is worth keeping because of how it failed: the first run read **6 errors and
every one of them was this unit's own text** - five parse errors where C writes a one-statement
`if`/`else` body without braces (`if (data[i]) fprintf (…)`, C 20451-20453; the two `if (axis_name)
fprintf (…)`, C 20545-20547 and C 20565-20567; the two `if (!hhea || !maxp) hmtx = …` sites,
C 20625-20626 and C 20742-20743) and one line left in C's own spelling (`if (fvar->instance_count >
1)`, C 20571, whose `->` is not Rust).  The frozen probe parses `#[cfg]`-disabled text - which is
what caught all six, and it is the only checker that reads this arm at all.  Each body got the
braces Rust requires and the arrow became `(*fvar)`, after which both runs are clean.  `python3
tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **`OK`**, with the unit
**complete**: Rust 21,894 -> **22,955** lines against C's 21,215 = **+8.2%**, past the rule's 5%,
so this slice appends the unit's one `docs/ports/line-drift-tolerance.tsv` row (**0.09**, the
first *and* last one for it, since the file is finished); the row carries the measured composition
of the 1,740-line excess - 2,352 comment-only lines, 2,487 lines carrying a `// C NNNN` citation,
518 `#[cfg]` attribute lines, 24 `include!` edges - and the two shapes behind the rest (C's
`for (i = 0; i < n; ++i)` heads cost three Rust lines, and every braces-less C body gained its
braces).  The completeness half of that reading is the stronger one and it is exact and narrow:
every function, `DEFUN`, macro, type, struct field, file-scope object and constant the tool
compares is present and equal, which now includes the slice's two file-scope functions
(`sfnt_test_uvs`, `main`), the two object-like `#define`s C 20473-20474 (answered as `const`s -
`macro_body_of`'s documented const shape) and the two `#define` pairs inside `main` (`GG`/`FG`,
`LB`/`RB`, answered as in-block `macro_rules!`).  It still knows nothing about the `#ifdef TEST`
*text* it drops and nothing about behaviour.  The other gates: `check-declarations.py` **17
declaration sites (0 forbidden, 17 allowed literals), 0 failures**; `check-citations.py` **676
file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound (a C range table), 0 failure(s)** (the table's
last row is transcribed and still tiles C 1-21215); `check-port-lines.py` **0 file(s) under ten
percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale** (the +8.2% file ratio is inside
the tolerance this slice's row records); `check-ledger-ticks.py` **238 ledger(s) bound, 0
failure(s)**, 0 row(s) without a ledger; `check-cond-eval.py src/sfnt.c` **31 conditions, 0
unknown, 0 disagreements**.
## Deviations`.  Port: `src/sfnt.rs` (13,174 lines, `grep -c ''`; `extract-abi.py`'s
> own count, 13,174, is the same newline count), both arms of every `#if` carried — except the
> macros C 6559-8159, whose Rust home is inside the functions that call them (P-57; the range
> table's own row says which).
>
> **This file's ledger replaces the previous revision's conclusion** ("the file is not in the
> frozen build, so the port is the empty unit plus a note"), exactly as `docs/ports/android.md`
> and `docs/ports/androidselect.md` did: P-86 ① rules that a three-line header comment is *not
> started*, not *done*.
>
> **One measurement differs from `src/android.c`'s, and it changes what the tool can say.**
> `src/sfnt.c` **is** in `docs/baseline/config/cond-eval-verified.txt` (353, together with
> `src/sfnt.h`/`src/sfntfont.c`/`src/sfntfont.h`), so `tools/extract-abi.py` evaluates its `#if`s
> and builds a real inventory — 421 declarations.  The tool prints its "not compiled by the
> frozen configuration … nothing in this file is required here" note and *clears* that inventory
> only when the port has no `fn` at all (`tools/extract-abi.py` 2225-2227); the 11-line
> placeholder had none, which is why `--check` read `OK` on it.  Both slices do, so `--check` is
> **not vacuous here**: its reading (Evidence ③) is a live coverage statement about the 21,215
> C lines and counts out what the remaining slices owe.  P-86 ②'s table below is therefore not a
> substitute for a checklist that the tool cannot produce (the `android.c`/`image.c` situation) —
> it is this ledger's account of what the slices *do* carry, next to the tool's account of what
> the file still owes.
>
> **The slices are C 1-6306, C 6312-6557, C 8161-10224, plus slice 7's two out-of-order ranges
> (C 10225-10663 and C 13918-14554).**  Slice 9 is the two ranges slice 6's stop and slice 7's
> first range leave standing (they close C 6307-10224 apart from the C 6559-8159 macros, which
> belong to `sfnt_interpret_run`'s slice - see the paragraph above and the table's middle row).
> Slice 1 is C 1-1112 (through the `}` of `sfnt_read_cmap_table`
> at C 1112): the include graph and the TEST harness (C 20-139), `xaddmalloc`/`xaddrealloc`/
> `MIN`/`MAX` (C 141-160), the file's own note (C 162-191), `sfnt_table_names` (C 195-219), the
> two swap helpers and their macros (C 221-238), `sfnt_read_table_directory` (C 248-323),
> `sfnt_find_table` (C 328-341), the six cmap subtable readers
> `sfnt_read_cmap_format_0`/`_2`/`_4`/`_6`/`_8`/`_12` (C 350-841), `sfnt_read_24` (C 845-849),
> `sfnt_read_cmap_format_14` (C 858-932), `sfnt_read_cmap_table_1` (C 939-1011) and
> `sfnt_read_cmap_table` (C 1020-1112).  Slice 2 is C 1113-2407: the glyph lookup routines
> (C 1114-1465: the six per-format lookups, `sfnt_bsearch_above` with its two comparators, and
> the `TEST_STATIC` dispatcher `sfnt_lookup_glyph`), the table readers (C 1467-1906:
> `sfnt_read_head_table`, `sfnt_read_hhea_table`, both `loca` forms, `sfnt_read_maxp_table`,
> `sfnt_read_glyf_table` and the `HAVE_MMAP` pair `sfnt_map_glyf_table`/`sfnt_unmap_glyf_table`),
> and the two glyph outline readers `sfnt_read_simple_glyph`/`sfnt_read_compound_glyph`
> (C 1908-2407).  Slice 3 is C 2408-3594: `sfnt_read_glyph` (C 2413-2525) and `sfnt_free_glyph`
> (C 2529-2538), then the whole **glyph outline decomposition** - `sfnt_transform_coordinates`
> (C 2550-2618), `struct sfnt_compound_glyph_context` (C 2620-2647),
> `sfnt_expand_compound_glyph_context` (C 2657-2723), `sfnt_round_fixed` (C 2728-2736),
> `sfnt_decompose_compound_glyph` (C 2753-3145), `sfnt_lerp_half` (C 3150-3156),
> `sfnt_decompose_glyph_1` (C 3166-3293), `sfnt_decompose_glyph_2` (C 3306-3433) and the
> dispatcher `sfnt_decompose_glyph` (C 3470-3594).  Slice 4 is C 3595-4543: the record
> `struct sfnt_build_glyph_outline_context` (C 3596-3606) and the `build_outline_context` object
> (C 3610) the checker still listed, then the whole **outline builder** - `sfnt_build_append`
> (C 3616-3674), the fixed-point helpers `sfnt_multiply_divide`/`_rounded`/`_signed` (C 3678-3714),
> `sfnt_mul_fixed`/`_round` (C 3719-3733), `sfnt_move_to_and_build`/`sfnt_line_to_and_build`
> (C 3738-3767), `sfnt_div_fixed`/`sfnt_ceil_fixed`/`sfnt_floor_fixed` (C 3772-3792),
> `sfnt_curve_is_flat`/`sfnt_curve_to_and_build_1`/`sfnt_curve_to_and_build` (C 3798-3879) and
> `sfnt_build_glyph_outline` (C 3902-3971) - and the whole *"ragged"* (fast) **rasterizer**:
> `sfnt_poly_coverage` (C 3983-3993), `sfnt_poly_grid_ceil` (C 3998-4003), the anonymous enum's
> `SFNT_POLY_ALIGNMENT` (C 4005-4008), `sfnt_prepare_raster` (C 4015-4038), the
> `sfnt_edge_proc`/`sfnt_span_proc` typedefs (C 4040-4042), `sfnt_step_edge` (C 4047-4052),
> `sfnt_build_outline_edges` (C 4059-4167), `sfnt_edge_sort` (C 4175-4194), `sfnt_poly_edges`
> (C 4220-4296), `sfnt_saturate_short` (C 4301-4308), `sfnt_fill_span` (C 4313-4435),
> `sfnt_poly_span` (C 4442-4489), `sfnt_raster_span` (C 4498-4503), `sfnt_raster_edge`
> (C 4508-4514) and `sfnt_raster_glyph_outline` (C 4519-4543).  Slice 5 is C 4544-5451: the
> three wrapping-arithmetic macros `sfnt_add`/`sfnt_sub`/`sfnt_mul` (C 4547-4554), the "Exact
> coverage scaler." note (C 4558-4647), the records `struct sfnt_step` (C 4651-4662),
> `struct sfnt_step_raster` (C 4667-4677), the anonymous enum's `SFNT_BLOCK_STEPS` (C 4679-4682),
> `struct sfnt_step_chunk` (C 4687-4697) and `struct sfnt_fedge` (C 4704-4717) with the
> `sfnt_fedge_proc` typedef (C 4719-4720), `sfnt_build_outline_fedges` (C 4726-4826), the
> `sfnt_step_raster_proc` typedef (C 4828), `sfnt_insert_raster_step` (C 4834-4883; C 4857's
> `goto found`), the whole **exact rasterizer** `sfnt_poly_edges_exact` (C 4896-5303, with C 4959's
> `APPEND_STEP`, C 5011's `TRAPEZOID_AREA`, C 5083's `TRIANGLE_AREA` and C 4984's `goto next`),
> `sfnt_compute_fill` (C 5309-5316), `sfnt_poly_set_steps` (C 5321-5355, the `FALLTHROUGH`
> ladder), `sfnt_poly_steps` (C 5360-5396), `sfnt_raster_steps` (C 5400-5404),
> `sfnt_raster_edges_exact` (C 5409-5419) and the `TEST_STATIC` `sfnt_raster_glyph_outline_exact`
> (C 5425-5451).  Slice 6 is C 5455-6306: the "Glyph metrics computation." banner with
> `sfnt_read_hmtx_table` (C 5468-5532), `sfnt_lookup_glyph_metrics` (C 5541-5575),
> `sfnt_scale_metrics` (C 5581-5592) and `sfnt_get_scale` (C 5597-5612); the "Font style parsing."
> banner with `sfnt_read_name_table` (C 5623-5768) and `sfnt_find_name` (C 5774-5792);
> `sfnt_read_meta_table` (C 5799-5916) and `sfnt_find_metadata` (C 5922-5939); the "TrueType
> collection format support." banner with `sfnt_read_ttc_header` (C 5950-6032); the "TrueType
> hinting support." note (C 6036-6075); the "Functions for reading tables used by the TrueType
> interpreter." banner with `sfnt_read_cvt_table` (C 6086-6131), `sfnt_read_fpgm_table`
> (C 6137-6179) and `sfnt_read_prep_table` (C 6185-6227); and the "Interpreter execution
> environment." banner with the eight 26.6 fixed-point helpers (C 6236-6306: `sfnt_div_f26dot6`,
> `sfnt_mul_f26dot6`, `sfnt_mul_f26dot6_round`, `sfnt_mul_f2dot14`, `sfnt_mul_f26dot6_fixed`,
> `sfnt_floor_f26dot6`, `sfnt_ceil_f26dot6` and `sfnt_round_f26dot6`).  Slice 7 is **two ranges,
> chosen by P-91 ①'s dependency closure and written at their own C positions (P-91 ②)** rather
> than by walking C's order - which is what P-91 was written for, after slice 6 hit the wall: (a)
> C 10225-10663, the **graphics-state validation family** `sfnt_round_none` ... `sfnt_round_super45`
> (C 10230-10330), `sfnt_project_onto_x_axis_vector`/`_y_axis_vector` (C 10338-10356),
> `sfnt_dot_fix_14` (C 10360-10376), `sfnt_project_onto_any_vector` /
> `sfnt_dual_project_onto_any_vector` (C 10384-10410), `sfnt_move_x`/`sfnt_move_y`/`sfnt_move`
> (C 10417-10521), `sfnt_short_frac_dot` (C 10526-10530), `sfnt_validate_gs` (C 10537-10633) and
> `sfnt_set_freedom_vector`/`sfnt_set_projection_vector` (C 10639-10663) - the definition C 6316
> was waiting for, whose every callee was already in the file (`sfnt_round_f26dot6` C 6298,
> `sfnt_floor_f26dot6` C 6281, `sfnt_ceil_f26dot6` C 6289 from slice 6, and
> `sfnt_multiply_divide_signed` C 3696-3714 from slice 5); and (b) C 13918-14554, the
> **glyph-variation readers** `sfnt_read_fvar_table` (C 13940-14155), `sfnt_read_gvar_table`
> (C 14164-14318), `sfnt_read_avar_table` (C 14327-14443) and `sfnt_read_packed_points`
> (C 14460-14554), whose callees (`sfnt_find_table`, the `xmalloc` family, `read`, `lseek`,
> `memcpy`, `sfnt_swap16!`/`sfnt_swap32!`, `SFNT_ENDOF!`, `sa_stack_alloc`) are all in the file too.
> **Neither range needed a P-91 ④ allowance**: no ported call site in this slice names a `static`
> whose definition is still missing.  **Slice 9's first half is C 6312-6557** -
> `sfnt_init_graphics_state` (C 6316-6344), `sfnt_make_interpreter` (C 6358-6522, now *unblocked*:
> C 6308-6310's forward declaration stays dropped per PORTING.md 0.1-12 and its callee
> `sfnt_validate_gs` is in the file at C 10538's text), the `sfnt_interpreter_run_context` enum
> (C 6533-6541) and `sfnt_interpret_trap` (C 6550-6557) - and **its second half is C 8161-10224**,
> the interpreter-helpers block, which closes the C 6307-10224 section except for the macros.
> Between the two halves stand C 6559-8159, the **interpreter macros**: slice 9 does **not** port
> that region at its own C position, because a `macro_rules!` resolves locals *and labels* at its
> **definition** site and every one of those bodies names the caller's `interpreter` (or
> `is_prep`, or `sfnt_interpret_run`'s `skip_step`/`next_instruction`/`loopcall_begin`/
> `deltac*_start`/`deltap*_start` labels).  P-57's rule - `src/coding.rs` and `src/ccl.rs`'s shape
> - is one in-function copy per using function, and the C 11396 function that uses almost all of
> them is the next slice; the four that this range's functions use (`TRAP`, `POP`, `PROJECT`,
> `DUAL_PROJECT`) are defined inside each caller, and the range table's middle row carries the
> ruling.  What still gates `sfnt_make_interpreter` is Owed 12's second half (C 6427-6431's
> `#ifdef TEST` hooks, whose three members the mirror deliberately does not declare).
>
> **Slice 8 is C 14555-16447**, the rest of the glyph-variation group and the file's **last
> non-TEST block**: the packed-delta reader `sfnt_read_packed_deltas` (C 14562-14618), the `cvar`
> reader `sfnt_read_cvar_table` (C 14626-14950), the three blend helpers `sfnt_init_blend`
> (C 14959-14975), `sfnt_free_blend` (C 14979-14983) and `sfnt_normalize_blend` (C 14988-15074),
> the `gvar` glyph header record `sfnt_gvar_glyph_header` (C 15078-15088, defined in the `.c`,
> not in `sfnt.h`), `sfnt_compute_tuple_scale` (C 15096-15174), the three interpolators
> `sfnt_infer_deltas_2` (C 15184-15356), `sfnt_infer_deltas_1` (C 15361-15406) and
> `sfnt_infer_deltas` (C 15434-15457), the two appliers `sfnt_vary_simple_glyph` (C 15488-15851)
> and `sfnt_vary_compound_glyph` (C 15861-16212), `sfnt_vary_interpreter` (C 16221-16277) and the
> two metadata readers `sfnt_read_OS_2_table` (C 16292-16381) and `sfnt_read_post_table`
> (C 16398-16444).  The range **stops at C 16447** because C 16448's `#ifdef TEST` opens the test
> harness and runs to C 21215 — measured: that `#ifdef TEST` matches the file's last `#endif` at
> C 21215 — so C 16448-21215 is the remainder, and no gate type-checks it (the arm flip's feature
> set carries no `test`, so every `sfnt_test_*`/`sfnt_check_*` item there would be parse-only
> text).  **The next live definition after this range is C 16450's `struct sfnt_test_dcontext`**,
> i.e. the TEST programme.  The table's last two rows are this range and "the remainder"; slice
> 9 replaces the single **C 6307-10224** row with three (C 6307-6557, C 6558-8159 and
> C 8160-10224), so the table still tiles C 1-21215 contiguously.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 22-39's include graph) | a *real* Android configuration |
| C 41 `#ifdef HAVE_MMAP` … C 43 (`#include <sys/mman.h>`) | **no gate**: `HAVE_MMAP` is 1 on this target (`docs/baseline/config/darwin-arm64.macros.tsv` 1469), so the include is live in every configuration.  Slice 2 is the first text here to *name* what the include provides, so the unit now carries C's four prototypes and the five constants in a `#[cfg (file gate)]` block at C 41's site (the P-88 ① shape; Owed 4 records that `docs/baseline/feature-coverage.tsv` has no `have_mmap` row) | always, on this target |
| C 45 `#if defined __GNUC__ && !defined __clang__` … C 47 (`#pragma GCC diagnostic`) | **dropped**: a diagnostic pragma has no Rust counterpart (the `-Wstringop-overflow` class) | never |
| C 49 `#ifdef TEST` … C 134 (the harness: `memory_full_up`, the six allocators, `TEST_STATIC`, `eassert`, `AVOID`) | `all (file gate, feature = "test")` — `xtask/Cargo.toml` 564 registers C's `TEST` as `test` | a test build |
| C 136 `#else` … C 139 (`#define TEST_STATIC` + `#include "lisp.h"`) | `all (file gate, not (feature = "test"))` | the frozen-shaped configuration |
| C 128 / C 137 (`#define TEST_STATIC static` / `#define TEST_STATIC`) | `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]` on the eleven readers C exports through it (slice 1's `sfnt_read_table_directory`/`sfnt_read_cmap_table`; slice 2's `sfnt_lookup_glyph`, `sfnt_read_head_table`, `sfnt_read_hhea_table`, `sfnt_read_loca_table_short`, `sfnt_read_loca_table_long`, `sfnt_read_maxp_table`, `sfnt_read_glyf_table`) — a construct PORTING.md does not yet cover, see Deviations 1 and Owed 1 | linkage only; the item is on the file gate either way |
| C 224 / C 232 `#ifndef WORDS_BIGENDIAN` (the two swap helpers' bodies) | `not (feature = "words_bigendian")` (`xtask/Cargo.toml` 614); the other arm is C's empty body — the big-endian byte order needs no swap, so it is not a branch the port invents | little-endian |
| C 1827 `#if defined HAVE_MMAP && !defined TEST` … C 1906 `#endif` | `all (file gate, not (feature = "test"))`: `HAVE_MMAP` is 1 on this target and no `have_mmap` feature is registered, so the port carries that half live (C 41's ruling, above) and gates C's `!defined TEST` half.  **C's `#if` has no `#else`**, so the other arm is empty text | a real Android non-TEST configuration |
| C 1871 `#ifdef HAVE_POSIX_MADVISE` … C 1874 `#elif defined HAVE_MADVISE` … C 1876 `#endif` | `not (feature = "no_have_posix_madvise")` for the live `posix_madvise` arm; `feature = "no_have_posix_madvise"` + `cfg! (feature = "have_madvise")` for the `madvise` arm (the `pdumper.rs` 5281-5284 shape; `docs/baseline/feature-coverage.tsv` 241/436 register both names).  The third arm — neither macro — has no text in C | `posix_madvise`: always on this target (`HAVE_POSIX_MADVISE` is 1, 1492; `HAVE_MADVISE` is 1 too, 1445) |
| C 4547-4554's three file-scope `#define`s (`sfnt_add`/`sfnt_sub`/`sfnt_mul`), C 4959's `APPEND_STEP`, C 5011's `TRAPEZOID_AREA` and C 5083's `TRIANGLE_AREA` (the last three inside `sfnt_poly_edges_exact`), and their `#undef`s (C 5262, C 5265-5266) | `macro_rules!` at the same scope: file scope for the first three, the enclosing block for the three function-local ones — Rust's macros are textually scoped, so the `#undef`s have no text (Deviation 35) | every configuration (C's macros are unconditional) |
| C 4322 `#ifndef NDEBUG` … C 4429 (`sfnt_fill_span`'s `row_end` and its four `eassert`s) | `feature = "no_ndebug"` — the frozen configuration **defines** `NDEBUG` (`docs/baseline/config/darwin-arm64.macros.tsv` 2892), so this is dead text and P-51's rule names the feature after the non-frozen side (the `no_signbit`/`no_rint` shape); C's `#else` is empty, so there is no second arm.  The name is **not registered** yet — Owed 8 | a configuration with `NDEBUG` undefined |
| C 1837 / C 1891 (`sfnt_map_glyf_table`/`sfnt_unmap_glyf_table` are not `TEST_STATIC`: sfnt.h declares them `extern` at C 1490-1491) | plain `#[unsafe(no_mangle)] pub unsafe extern "C"` under the file gate + `not (feature = "test")` — no `cfg_attr` | a real Android non-TEST configuration |
| C 16448 `#ifdef TEST` … C 21215 (the file's **last** `#endif`; the whole test programme) | `all (file gate, feature = "test")` — the same `TEST` name C 49 already uses.  **C 16448-16889 is transcribed (slices 19-20)**; C 16890-21215 is the remainder, and Owed 21 + Owed 30 record the X11 surface (`include/xlib.rs` **is** the `<X11/Xlib.h>` mirror - slice 19's "no mirror" note was stale; the names still missing from it, and the absent Xrender mirror, are Owed 30's list) | nothing, today (no gate sets `test`; Evidence ⑭ measures what flipping it costs) |
| C 34's `#include <stdlib.h>` / C 2451's `abort ()` and slice 4's `abs` (C 3712-3713, C 3812, C 4134) | the file gate; the unit carries C's two prototypes in use-site blocks at C 34's site (`lib/include/stdlib.rs` has `malloc` and friends but neither name, the `androidvfs.rs` 228-234 shape) | a real Android configuration |

`not_android_stubify` is the feature the tree registered for C's `HAVE_ANDROID && !defined
ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` 640, first used by `alloc.rs` 6428);
`androidselect.rs`'s gate is the same pair, and `docs/ports/androidselect.md` is where that
decision is argued.  C 22-39's graph is gated as a whole (P-86 ⑧): under the frozen feature set
rustc compiles no include from it, which is what ① reads as the 336-byte empty object.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-22 | the blank and `#include "sfnt.h"` | file gate |
| 23-39 | the file's own include list (C 24 `<attribute.h>`, C 25 `<byteswap.h>`, C 26 `<fcntl.h>`, C 27 `<intprops.h>`, C 28 `<inttypes.h>`, C 29 `<stdbit.h>`, C 30 `<stdckdint.h>`, C 31 `<stdcountof.h>`, C 32 `<stdint.h>`, C 33 `<stdio.h>`, C 34 `<stdlib.h>`, C 35 `<string.h>`, C 36 `<unistd.h>`, C 37 `<setjmp.h>`, C 38 `<errno.h>`, C 39 `<alloca.h>`) | file gate |
| 40-47 | C 41-43's `#ifdef HAVE_MMAP`/`<sys/mman.h>`/`#endif` and C 45-47's `__GNUC__`-not-`__clang__` `#pragma` block | live unconditionally (slice 2 adds the unit's use-site declarations here) / dropped |
| 48-139 | the `#ifdef TEST` harness (C 51-56's own includes, C 58-64 `memory_full_up`, C 66-77 `xmalloc`, C 79-90 `xcalloc`, C 92-96 `xzalloc`, C 98-105 `xnmalloc`, C 107-118 `xrealloc`, C 120-124 `xfree`, C 126-134 `TEST_STATIC`/`eassert`/`AVOID`, C 136-139 the `#else` arm) | `test` / `not test` (both carried) |
| 140-157 | `xaddmalloc` (C 141-148), `xaddrealloc` (C 150-157) | file gate |
| 158-160 | the blank and `MIN`/`MAX` (C 159-160, whose bodies are minmax.h's) | file gate |
| 161-191 | the file's own note: what the scaler covers, the TrueType Reference Manual, and why Microsoft's extended scaler differs | file gate |
| 192-219 | `sfnt_table_names` (C 197-219): all 18 slots designated, `SFNT_TABLE_CMAP` … `SFNT_TABLE_POST` | file gate |
| 220-234 | `sfnt_swap16_1` (C 221-226) and `sfnt_swap32_1` (C 229-234) | file gate / `not words_bigendian` |
| 235-238 | `sfnt_swap16` and `sfnt_swap32` (C 237-238) | file gate |
| 239-323 | `sfnt_read_table_directory` (C 248-323; C 253-255 `SFNT_ENDOF`'s use, C 262-271 the TTC probe that returns `(struct sfnt_offset_subtable *) -1`, C 284-288 the second `-1` return, C 292-320 the table-directory realloc and the 4-swap loop) | file gate / `TEST_STATIC` |
| 324-341 | `sfnt_find_table` (C 328-341) | file gate |
| 342-345 | the form feed and the "Character mapping routines." banner | file gate |
| 346-379 | `sfnt_read_cmap_format_0` (C 350-379) | file gate |
| 380-483 | `sfnt_read_cmap_format_2` (C 384-483; C 390-398 the `SFNT_ENDOF` minimum, C 411-437 the 256-entry `sub_header_keys` scan that computes `nsub`, C 442-470 the variable-length read and the `subheaders`/`glyph_index_array` split) | file gate |
| 484-589 | `sfnt_read_cmap_format_4` (C 488-589; C 498-499 the `SFNT_ENDOF` minimum, C 533-548 the `variable_size` arithmetic and the short-table bail, C 558-565 the five data pointers, C 571 the one `reserved_pad` swap, C 573-585 the two swap loops) | file gate |
| 590-659 | `sfnt_read_cmap_format_6` (C 594-659; the `uint16_t i` loop at C 654 is C's own counter type) | file gate |
| 660-750 | `sfnt_read_cmap_format_8` (C 664-750; C 674 reads the 32-bit length before the header, C 711/717 the `ckd_mul`/`ckd_add` pair) | file gate |
| 751-841 | `sfnt_read_cmap_format_12` (C 755-841; C 779 `xaddmalloc`, C 802/808 the `ckd_mul`/`ckd_add` pair) | file gate |
| 842-849 | `sfnt_read_24` (C 845-849; `bytes[0] << 16u` and its `8u` twin) | file gate |
| 850-932 | `sfnt_read_cmap_format_14` (C 858-932; C 887-888 the `ckd_mul (…) || ckd_add (…)`, C 918-923 the two `memcpy`s of the unaligned offsets) | file gate |
| 933-1011 | `sfnt_read_cmap_table_1` (C 939-1011; C 946 the `ckd_add` of the two offsets, C 952-968 the format-14 special case that reads no `length`, C 973-1010 the seven-way `switch`) | file gate |
| 1012-1112 | `sfnt_read_cmap_table` (C 1020-1112; C 1060-1061 the `subtables` array, C 1089 the `*data` array whose `sizeof *data` is a *pointer's* size, C 1096 the `(void *) -1` IO-error test, C 1101-1106 the partial-free loop) | file gate / `TEST_STATIC` |
| 1113-1465 | the glyph lookup routines: `sfnt_lookup_glyph_0` (C 1117-1125), `sfnt_lookup_glyph_2` (C 1131-1181; C 1157-1171 the `slice` walk whose bounds test compares against `glyph_index_array + num_glyphs`), `sfnt_bsearch_above` (C 1186-1220), `sfnt_compare_uint16` (C 1225-1229), `sfnt_lookup_glyph_4_1` (C 1235-1262), `sfnt_lookup_glyph_4` (C 1267-1298; C 1284-1288 the `segment_address` difference *before* the null test), `sfnt_lookup_glyph_6` (C 1303-1313), `sfnt_compare_char` (C 1318-1326), `sfnt_lookup_glyph_8` (C 1331-1375; C 1345-1360 the `> 64` binary search, C 1363-1373 the linear fallback), `sfnt_lookup_glyph_12` (C 1380-1424) and the six-arm dispatcher `sfnt_lookup_glyph` (C 1433-1465) | file gate / `TEST_STATIC` on the dispatcher only |
| 1466-1541 | the form feed and the "Header reading routines." banner, then `sfnt_read_head_table` (C 1476-1541; C 1500-1503 the version test between the two swap runs, C 1507-1512 the magic test) | file gate / `TEST_STATIC` |
| 1542-1607 | `sfnt_read_hhea_table` (C 1548-1607; C 1563 the `directory->length != sizeof *hhea` gate, C 1590-1604 the fifteen field swaps — `caret_offset` is *not* among them in C and is not swapped here either) | file gate / `TEST_STATIC` |
| 1608-1652 | `sfnt_read_loca_table_short` (C 1614-1652; C 1637-1639 the `xaddmalloc` + `(loca + 1)` split + `length / 2`) | file gate / `TEST_STATIC` |
| 1653-1697 | `sfnt_read_loca_table_long` (C 1659-1697; the same three lines with `length / 4` and 32-bit swaps) | file gate / `TEST_STATIC` |
| 1698-1781 | `sfnt_read_maxp_table` (C 1705-1781; C 1763 the `SFNT_ENDOF (struct sfnt_maxp_table, num_glyphs, uint16_t)` minimum, C 1768 the `MIN` of the directory length and `sizeof *maxp`, C 1774 the version-1.0 size test, C 1779-1780 the 0.5/1.0 fork) | file gate / `TEST_STATIC` |
| 1782-1825 | the form feed and the "Glyph outlining generation." banner, then `sfnt_read_glyf_table` (C 1792-1825) | file gate / `TEST_STATIC` |
| 1826-1906 | C 1827's `#if defined HAVE_MMAP && !defined TEST` … C 1906's `#endif /* HAVE_MMAP */`: `sfnt_map_glyf_table` (C 1837-1885, with C 1871-1876's `#ifdef HAVE_POSIX_MADVISE`/`#elif defined HAVE_MADVISE`) and `sfnt_unmap_glyf_table` (C 1891-1904) | file gate / `not test`; `no_have_posix_madvise` inside C 1871 |
| 1907-2169 | `sfnt_read_simple_glyph` (C 1912-2169; C 1970-1974 the `ckd_mul`/`ckd_add` chain that ends in `memory_full_up`, C 2004-2034 the REPEAT_FLAG loop, C 2047-2085 the X vectors, C 2089-2126 the Y vectors) | file gate |
| 2170-2407 | `sfnt_read_compound_glyph` (C 2175-2407; C 2188-2226 the first sizing pass, C 2234 the C quirk `data += sizeof data + instruction_length`, C 2290-2304 the two `ARG_1_AND_2_ARE_WORDS` arms, C 2308-2350 the three scale arms, C 2395-2403 the instruction/`NULL` fork) | file gate |
| 2408-2525 | the blank at C 2408, C 2409-2411's comment, then `sfnt_read_glyph` (C 2413-2525; C 2433/2437 the two `loca` bounds tests `glyph_code + 1 >= …->num_offsets`, C 2436/2438 the short form's `offsets[…] * 2`, C 2449 the `abort ()` when neither table was given, C 2455-2474 the empty-glyph branch, C 2477 the `SFNT_ENDOF (struct sfnt_glyph, ymax, sfnt_fword)` minimum, C 2485-2493 the `memcpy` and the five `sfnt_swap16`s, C 2502/2518 the two `sfnt_read_*_glyph` calls at `offset + SFNT_ENDOF (…)`) | file gate / `TEST_STATIC` |
| 2526-2538 | `sfnt_free_glyph` (C 2529-2538, whose three `xfree`s are C 2535-2537) | file gate / `TEST_STATIC` |
| 2539-2618 | the form feed, the `/* Glyph outline decomposition.  */` banner, then `sfnt_transform_coordinates` (C 2550-2618; C 2561/2566/2572 the three scale arms `WE_HAVE_A_SCALE`/`WE_HAVE_AN_X_AND_Y_SCALE`/`WE_HAVE_A_TWO_BY_TWO`, C 2581-2583 the no-scale arm's `x[i] += x_off, y[i] += y_off` comma statement, C 2608-2616 the affine matrix loop with C's two `double → sfnt_fixed` conversion points) | file gate |
| 2619-2647 | `struct sfnt_compound_glyph_context` (C 2620-2647; the two `sfnt_fixed *`/`unsigned char *`/`size_t *` arrays and the six `sfnt_f26dot6` phantom points) | file gate |
| 2648-2723 | `sfnt_expand_compound_glyph_context` (C 2657-2723; C 2669/2673 the two `ckd_add`s, C 2677/2683/2706/2709 the four `ckd_mul`s, C 2693-2699 the three `xrealloc`s, C 2700-2702/2719-2721 the two `base - count` pointer folds) | file gate |
| 2724-2736 | `sfnt_round_fixed` (C 2728-2736; the `number += (1 << 15)` and the `number & ~0xffff` mask) | file gate |
| 2737-3145 | `sfnt_decompose_compound_glyph` (C 2753-3145; C 2768's `size_t point UNINIT, point2 UNINIT, index`, C 2783/2810 the `base_index`/`contour_start` reads, C 2793 the `ARGS_ARE_XY_VALUES` fork with its four `argument1.b`/`d` sign-extension arms, C 2801/2807 the `SCALED_COMPONENT_OFFSET` transform call, C 2833-2872 the point-anchor arm's two `goto`s and its `skip_computation:` label, C 2896-2908 the phantom-point offsets, C 2950-2960 the `sfnt_expand_compound_glyph_context` call, C 2963-2974 the three `x_base`/`y_base`/`flags_base` copies, C 3036-3144 the compound-subglyph arm with `error_in_defer_offsets:`/`skip_computation_from_defer_offsets:` and the trailing `sfnt_transform_coordinates`) | file gate |
| 3146-3156 | `sfnt_lerp_half` (C 3150-3156, the two midpoint interpolations) | file gate |
| 3157-3293 | `sfnt_decompose_glyph_1` (C 3166-3293; C 3186-3210 the three start-point arms including the `last -= 1; here -= 1` pair whose `size_t` wrap the port keeps, C 3227-3241 the on-curve walk with its `continue`, C 3247-3276 the off-curve inner loop with `goto continue_loop`, C 3279-3289 the `goto exit` and the `exit:` tail) | file gate |
| 3294-3433 | `sfnt_decompose_glyph_2` (C 3306-3433, the same body over `sfnt_fixed`/`sfnt_f26dot6` instead of `sfnt_fword`) | file gate |
| 3434-3594 | `sfnt_decompose_glyph` (C 3470-3594; C 3479-3512 the simple-glyph walk, C 3516 the `memset (&context, 0, sizeof context)`, C 3526 the `compound_metrics` save, C 3538-3556 the two `goto`s, C 3581/3588 the `early:`/`fail:` tails with their four `xfree`s each) | file gate |
| 3595-4543 | the blank at C 3595, then `struct sfnt_build_glyph_outline_context` (C 3596-3606) and the `build_outline_context` object (C 3610); `sfnt_build_append` (C 3616-3674; C 3645's `ckd_mul`, C 3647's `xaddrealloc`, C 3662-3663 the two chained bounding-box assignments); `sfnt_multiply_divide`/`_rounded`/`_signed` (C 3678-3714), `sfnt_mul_fixed`/`_round` (C 3719-3733), `sfnt_move_to_and_build`/`sfnt_line_to_and_build` (C 3738-3767), `sfnt_div_fixed`/`sfnt_ceil_fixed`/`sfnt_floor_fixed` (C 3772-3792; C 3783/3791's octal masks), `sfnt_curve_is_flat` (C 3798-3815; C 3812's `abs`), `sfnt_curve_to_and_build_1` (C 3822-3854; the three `sfnt_lerp_half` calls), `sfnt_curve_to_and_build` (C 3860-3879) and `sfnt_build_glyph_outline` (C 3902-3971; C 3918's two `sizeof`s, C 3942-3943 the two distortion terms, C 3946-3951 the `Some (…)` callback pair); the form feed and the "Glyph rasterization." banner, `sfnt_poly_coverage` (C 3983-3993), `sfnt_poly_grid_ceil` (C 3998-4003), the anonymous enum's `SFNT_POLY_ALIGNMENT` (C 4005-4008), `sfnt_prepare_raster` (C 4015-4038), the `sfnt_edge_proc`/`sfnt_span_proc` typedefs (C 4040-4042), `sfnt_step_edge` (C 4047-4052), `sfnt_build_outline_edges` (C 4059-4167; C 4069-4072 the `alloca`/`xnmalloc` fork, C 4134's `abs`, the three `continue`s at C 4092/4096/4144), `sfnt_edge_sort` (C 4175-4194), `sfnt_poly_edges` (C 4220-4296; the three pointer-to-pointer walks and the `(a = *prev) && (n = a->next)` condition), `sfnt_saturate_short` (C 4301-4308), `sfnt_fill_span` (C 4313-4435; C 4322-4324's `#ifndef NDEBUG` declaration and the four `eassert`s at C 4367/4385/4414/4428, C 4342's coverage subscript, C 4400/4418/4433 the three `*start++`), `sfnt_poly_span` (C 4442-4489; C 4452's and C 4476's chained assignments), the "Main entry point for outline rasterization." banner and `sfnt_raster_span` (C 4498-4503), `sfnt_raster_edge` (C 4508-4514) and `sfnt_raster_glyph_outline` (C 4519-4543; C 4530-4532 the `ckd` pair, C 4535's `(data + 1)` cell pointer) | file gate / `TEST_STATIC` on `sfnt_build_glyph_outline`, `sfnt_prepare_raster` and `sfnt_raster_glyph_outline`; `feature = "no_ndebug"` inside C 4322 |
| 4544-5451 | the blank at C 4544 and C 4547-4554's three macros; the "Exact coverage scaler." note (C 4558-4647); `struct sfnt_step` (C 4651-4662), `struct sfnt_step_raster` (C 4667-4677), the anonymous enum's `SFNT_BLOCK_STEPS` (C 4679-4682), `struct sfnt_step_chunk` (C 4687-4697), `struct sfnt_fedge` (C 4704-4717) and `typedef void (*sfnt_fedge_proc)` (C 4719-4720); `sfnt_build_outline_fedges` (C 4726-4826; C 4735-4739 the `alloca`/`xnmalloc` fork, C 4796-4798 `dx`/`dy`, C 4806 the `step_y = dx ? … : 0` ternary, C 4810-4816 the five `sfnt_fixed_float` stores); `typedef void (*sfnt_step_raster_proc)` (C 4828); `sfnt_insert_raster_step` (C 4834-4883; C 4849-4860 the sorted-list search with `goto found`, C 4862-4873 the chunk allocation, C 4875 the `nused++` subscript); `sfnt_poly_edges_exact` (C 4896-5303; C 4912 the `MAX`/`sfnt_floor_fixed` floor, C 4920 the `xcalloc`, C 4922 the scanline loop, C 4927-4947 the unsorted sweep, C 4953-5285 the active-edge loop with C 4959's `APPEND_STEP`, C 5011's `TRAPEZOID_AREA` and C 5083's `TRIANGLE_AREA`, C 5019-5071 the one-pixel case, C 5090-5260 the two multi-pixel cases, C 5262/5265-5266 the three `#undef`s, C 5273-5274 the scanline alignment, C 5275-5284 the `next:` tail, C 5292 the `proc` call, C 5296-5302 the chunk-freeing walk); `sfnt_compute_fill` (C 5309-5316); `sfnt_poly_set_steps` (C 5321-5355; the seven-armed `FALLTHROUGH` ladder); `sfnt_poly_steps` (C 5360-5396; C 5376 the two-increment `for` head, C 5380-5391 the step walk); `sfnt_raster_steps` (C 5400-5404); `sfnt_raster_edges_exact` (C 5409-5419) and `sfnt_raster_glyph_outline_exact` (C 5425-5451; C 5436-5438 the `ckd` pair, C 5439 the `xcalloc`, C 5443 the cell pointer) | file gate / `TEST_STATIC` on `sfnt_raster_glyph_outline_exact` |
| 5452-6306 | the blank at C 5452, C 5453's form feed and the "Glyph metrics computation." banner; `sfnt_read_hmtx_table` (C 5468-5532; C 5486-5490 the `MAX`-based size, C 5503 `xaddmalloc`, C 5515-5517 the two data pointers, C 5521-5528 the two swap loops), `sfnt_lookup_glyph_metrics` (C 5541-5575; C 5551/5557 the long/short entry fork), `sfnt_scale_metrics` (C 5581-5592) and `sfnt_get_scale` (C 5597-5612); the "Font style parsing." banner with `sfnt_read_name_table` (C 5623-5768; C 5644 `SFNT_ENDOF`, C 5662-5665 the three header swaps, C 5698-5706 the six record swaps, C 5710-5732 the string-offset extension and its `ckd` pair, C 5755-5764 the bounds validation) and `sfnt_find_name` (C 5774-5792); `sfnt_read_meta_table` (C 5799-5916; C 5819 `SFNT_ENDOF`, C 5843 the version test, C 5855-5858 the three-condition `ckd` gate, C 5888 the `memcpy`, C 5900-5911 the per-map offsets and their `ckd_add`) and `sfnt_find_metadata` (C 5922-5939); the "TrueType collection format support." banner with `sfnt_read_ttc_header` (C 5950-6032; C 5978 the tag test, C 5987-5994 the offsets array, C 6009 the version-2.0 fork, C 6015-6017 the `SFNT_ENDOF`/`offsetof` difference); the "TrueType hinting support." note (C 6036-6075); the "Functions for reading tables used by the TrueType interpreter." banner with `sfnt_read_cvt_table` (C 6086-6131; C 6106 the `ckd_add`, C 6114-6115 the element count and values pointer, C 6126-6127 the swap loop), `sfnt_read_fpgm_table` (C 6137-6179) and `sfnt_read_prep_table` (C 6185-6227); and the "Interpreter execution environment." banner with `sfnt_div_f26dot6` (C 6236-6240), `sfnt_mul_f26dot6` (C 6245-6249), `sfnt_mul_f26dot6_round` (C 6255-6259), `sfnt_mul_f2dot14` (C 6264-6268), `sfnt_mul_f26dot6_fixed` (C 6273-6277), `sfnt_floor_f26dot6` (C 6281-6285), `sfnt_ceil_f26dot6` (C 6289-6293) and `sfnt_round_f26dot6` (C 6298-6306) | file gate / `TEST_STATIC` on the ten readers (`sfnt_scale_metrics`, `sfnt_get_scale` and `sfnt_find_metadata` are C's `MAYBE_UNUSED TEST_STATIC`, Deviation 43) |
| 6307-6557 | **slice 9's first half**: C 6308-6309's `/* Needed by sfnt_init_graphics_state.  */` and C 6310's `static void sfnt_validate_gs (struct sfnt_graphics_state *);` (the forward declaration PORTING.md 0.1-12 deletes - Rust has no prototype for a `static`, and the definition is in this unit at C 10538's text from slice 7), `sfnt_init_graphics_state` (C 6316-6344; the 24 field stores and the `sfnt_validate_gs (&state)` call), `sfnt_make_interpreter` (C 6358-6522; C 6368's CFF `version != 0x00010000` bail, the six `ckd_add`s and the `ckd_mul` pair that size the record, C 6404's `alignof`, C 6425's `xmalloc`, C 6449-6470's pointer arithmetic into that one allocation, C 6490's `memset`, C 6495-6498's CVT load, C 6495-6518's scale/ppem/axis setup), the record-less enum `sfnt_interpreter_run_context` (C 6533-6541, whose `SFNT_RUN_CONTEXT_TEST` member sits behind C 6538's `#ifdef TEST`) and `sfnt_interpret_trap` (C 6550-6557; C 6549's `static AVOID` is the port's `-> !` and C 6556's `longjmp` the tree's first *plain* one) | file gate / `TEST_STATIC` on `sfnt_make_interpreter`; C 6427-6431's `#ifdef TEST` hook block is recorded, not carried (the mirror has no such members - Owed 23) |
| 6558-8159 | **the blank at C 6558 and the interpreter macros C 6559-8159 - *not ported here***: `STACKSIZE` (C 6559-6560), `TRAP` (C 6562-6563), `MOVE` (C 6565-6566), `CHECK_STACK_ELEMENTS`/`CHECK_STACK_AVAILABLE`/`CHECK_PREP` (C 6568-6589), the live `#ifndef TEST` stack macros `POP`/`POP_UNCHECKED` (C 6597-6602), `LOOK` (C 6620-6623), `PUSH`/`PUSH_UNCHECKED` (C 6627-6641) and `PUSH2_UNCHECKED` (C 6663-6667), and the ~130 instruction macros `SRP0` (C 6671) … `NOT_IMPLEMENTED` (C 8152-8159) | not ported in slice 9: **their Rust home is P-57's in-function copy in `sfnt_interpret_run`** (C 11396, the next slice) and in the functions that use them here, because a `macro_rules!` resolves locals *and labels* at its definition site and every body names the caller's `interpreter` / `is_prep` / `sfnt_interpret_run`'s labels.  The four this slice's functions use (`TRAP`, `POP`, `PROJECT`, `DUAL_PROJECT`) are defined inside each caller, and C 6595/6625 and C 6643/6661's TEST arms name the `pop_hook`/`push_hook` members the mirror omits (Owed 23) |
| 8160-10224 | **slice 9's second half**: the blank at C 8160 and the interpreter-helpers block C 8161-10224 - `sfnt_scale_by_freedom_vector` (C 8162-8171), `sfnt_interpret_utp` (C 8180-8223; the `mask`/touched-flag computation), `sfnt_save_projection_vector`/`sfnt_save_freedom_vector` (C 8233-8255), the three zone addressees `sfnt_address_zp2`/`_zp1`/`_zp0` (C 8266-8423, each with its `goto next`/`next:` pair over two optional out-parameters) and `sfnt_store_zp2` (C 8432-8458), the `#if 0` block C 8460-8487 around `sfnt_line_to_standard_form`, the three `sfnt_check_zp*` (C 8492-8534), the five movers `sfnt_move_zp0`/`_zp1`/`_zp2`/`_glyph_zone`/`_twilight_zone` (C 8544-8634, all through `state.move`), `sfnt_direct_move_zp2` (C 8646-8676), `sfnt_project_vector`/`sfnt_dual_project_vector` (C 8682-8696), `sfnt_interpret_fliprgoff`/`_fliprgon`/`_flippt` (C 8702-8768), `sfnt_interpret_scfs` (C 8778-8792), `sfnt_round_symmetric` (C 8798-8811), `sfnt_interpret_miap` (C 8829-8888), `sfnt_interpret_alignrp_1`/`sfnt_interpret_alignrp`/`sfnt_interpret_alignpts` (C 8894-8964), `sfnt_interpret_isect` (C 8974-9120; C 8983-8990, C 8992-9034 and C 9065-9071 are `#if 0` arms and the two `goto degenerate_case` sites a labelled block), `sfnt_sqrt_fixed` (C 9125-9155, whose `do … while (--count)` is a `loop`), `sfnt_normalize_vector` (C 9161-9213; C 9172's `fail:` label sits *inside* C's first `if` and C 9208's `goto fail` enters it - Deviation 66), `sfnt_line_to_vector` (C 9231-9270), `sfnt_measure_distance` (C 9280-9301), `sfnt_interpret_msirp` (C 9313-9361), `sfnt_interpret_ip` (C 9369-9527; the `zone->simple` original-coordinate fork), `sfnt_deltac` (C 9538-9656; the two `switch`es), `sfnt_interpret_mdap` (C 9669-9694), `sfnt_deltap` (C 9705-9815), C 9817-9819's `sfnt_interpret_run` forward declaration (dropped per 0.1-12; its definition is C 11396 and C 9832's `sfnt_interpret_call` is the one site P-91 ④ names), `sfnt_interpret_call` (C 9832-9860), `sfnt_set_srounding_state` (C 9871-9932), `sfnt_skip_code` (C 9938-9989), `sfnt_interpret_unimplemented` (C 9996-10019), `sfnt_interpret_fdef` (C 10025-10074) and `_idef` (C 10080-10130), `sfnt_interpret_if` (C 10138-10185) and `sfnt_interpret_else` (C 10194-10224) | file gate; no `TEST_STATIC` (every definition here is C's `static`), and no use-site declaration beyond `longjmp` at C 37's site (Deviation 69) |
| 10225-10663 | the **graphics-state validation family**, placed at its own C position (P-91 ②): `sfnt_round_none` (C 10230-10234), `sfnt_round_to_grid` (C 10239-10243), `sfnt_round_to_double_grid` (C 10248-10253; C 10246's `020`/`037` octal pair), `sfnt_round_down_to_grid` (C 10258-10263), `sfnt_round_up_to_grid` (C 10268-10273), `sfnt_round_to_half_grid` (C 10278-10283), `sfnt_round_super` (C 10288-10305; the two `sfnt_add` sites and the `phase` ternary), `sfnt_round_super45` (C 10311-10330), `sfnt_project_onto_x_axis_vector` (C 10338-10343), `sfnt_project_onto_y_axis_vector` (C 10351-10356), `sfnt_dot_fix_14` (C 10360-10376; the compound-literal casts and the straddling shift), `sfnt_project_onto_any_vector` (C 10384-10391), `sfnt_dual_project_onto_any_vector` (C 10403-10410), `sfnt_move_x` (C 10417-10430), `sfnt_move_y` (C 10437-10450), `sfnt_move` (C 10456-10521; `vector_dot_product`, the two `versor` tests and the two `sfnt_multiply_divide_signed` calls), `sfnt_short_frac_dot` (C 10526-10530), `sfnt_validate_gs` (C 10537-10633; the eight-arm `switch` on `round_state`, the two `0o40000` projection tests, the `vector_dot_product` computation and the three `move` arms) and `sfnt_set_freedom_vector` (C 10639-10647) / `sfnt_set_projection_vector` (C 10653-10663) | file gate |
| 10664-11101 | the shift and interpolate instructions: C 10665-10667's comment with `sfnt_interpret_shz` (C 10669-10699), C 10701-10703's with `sfnt_interpret_shc` (C 10705-10780), C 10782-10788's with `sfnt_interpret_shp` (C 10790-10825), C 10827-10845's four point-access macros with C 10847-10947's `IUP_SINGLE_PAIR ()`, and C 10949-10952's with `sfnt_interpret_iup_1` (C 10955-11046, whose four `goto`s are three labelled blocks - Deviation 78 (c)) and `sfnt_interpret_iup` (C 11058-11101) | file gate; the five macros are in-function copies (P-57) - **transcribed** (slice 10) |
| 11103-11395 | C 11103-11110's comment with `sfnt_interpret_mirp` (C 11113-11235), C 11237-11255's with `sfnt_project_zp1_zp0_org` (C 11257-11293, whose C 11275 `goto project_normally` is a labelled block) and C 11295-11302's with `sfnt_interpret_mdrp` (C 11305-11382) | file gate - **transcribed** (slice 11) |
| 11396-11983 | `sfnt_interpret_run` (C 11396-11983, C's `static`; C 11384-11394's comment belongs to it) **and the C 6559-8159 instruction macros, which are its in-function `macro_rules!` copies** (P-57: C's file-scope macros name this caller's `interpreter`, `is_prep` and labels - 142 names, `comm`-checked both ways), C 11420's `switch (opcode)` as the port's `match` (146 arms plus C 11949's `default:`) | file gate - **transcribed** (slice 14) |
| 11984-12115 | the two programme entries: C 11984's blank, C 11985-11994's comment and the `TEST_STATIC` `sfnt_interpret_font_program` (C 11995-12011), then C 12012's blank, C 12013-12021's comment and the `TEST_STATIC` `sfnt_interpret_control_value_program` (C 12023-12082, whose C 12048-12059 comment and C 12061-12077 graphics-state reversion block are carried whole), and finally C 12083's blank, C 12084's form feed and C 12086-12115's two comment blocks (the "Glyph hinting." banner and the instructed-outline-decomposition note that belongs to C 12116's function, transcribed by slice 13) | file gate; `TEST_STATIC` on both entries - **transcribed** (slice 15) |
| 12116-12295 | the instructed-outline helpers: `sfnt_decompose_instructed_outline` (C 12117-12166, whose two `goto fail`s are one labelled block), the `TEST_STATIC` builder `sfnt_build_instructed_outline` (C 12176-12243) and `sfnt_compute_phantom_points` (C 12258-12295) | file gate; `TEST_STATIC` on the builder - **transcribed** (slice 13) |
| 12296-12494 | C 12296's blank, C 12297-12303's comment and the `TEST_STATIC` `sfnt_interpret_simple_glyph` (C 12304-12484: the `ckd_mul`/`ckd_add` zone sizing, the `alloca`/`xmalloc` fork, the two point-loading loops with their flags, the phantom points, the glyph program with its `setjmp`, and the one-`xmalloc` `sfnt_instructed_outline` with its four `memcpy`s) | file gate; `TEST_STATIC` - **transcribed** (slice 16) |
| 12495-12566 | `sfnt_transform_f26dot6` (C 12496-12566), the 26.6 twin of the ported `sfnt_transform_coordinates` (C 2551-2618, `sfnt_transform_f26dot6`'s three callers are in the compound-glyph interpreters) | file gate - **transcribed** (slice 13) |
| 12567-12739 | C 12567's blank, C 12568-12588's comment and the `static` `sfnt_interpret_compound_glyph_2` (C 12590-12739: the `ckd_mul`/`ckd_add` zone sizing, the `alloca`/`xmalloc` fork, the two `volatile` locals, the contour renumbering and the two coordinate-copy loops, S1/S2 into the zone's original x array, the compound program with its `setjmp`, and the phantom points written back through `x_base`/`y_base`) | file gate - **transcribed** (slice 17) |
| 12740-13369 | the last two compound-glyph interpreters: C 12741-12748's comment and `sfnt_interpret_compound_glyph_1` (C 12750-13201, the component loop with its two recursive paths), then C 13203-13228's comment with the `TEST_STATIC` `sfnt_interpret_compound_glyph` (C 13229-13321, the stack context, the outline sizing and the four `xfree` guards), and C 13322-13369's blank lines, form feed and the UVS comment block (C 13326-13369, which belongs to C 13374's reader) | file gate; `TEST_STATIC` on the wrapper - **transcribed** (slice 18) |
| 13370-13915 | the variation-selector and table-mapping block, taken **out of order** (the two UVS readers `sfnt_read_default_uvs_table` (C 13375-13438) and `sfnt_read_nondefault_uvs_table` (C 13445-13512), `sfnt_compare_table_offsets` (C 13517-13530), `sfnt_create_uvs_context` (C 13543-13701) and `sfnt_free_uvs_context` (C 13706-13717), the two comparators `sfnt_compare_uvs_mapping` (C 13723-13737) / `sfnt_compare_unicode_value_range` (C 13742-13757), `sfnt_variation_glyph_for_char` (C 13765-13776), `sfnt_is_character_default` (C 13782-13791), and C 13795's `#if defined HAVE_MMAP && !defined TEST` arm with `sfnt_map_table` (C 13815-13856) / `sfnt_unmap_table` (C 13862-13865) plus C 13871's `#ifndef TEST` arm with `sfnt_read_table` (C 13880-13915)) | file gate; `TEST_STATIC` on the four readers, and `not (feature = "test")` on the three functions C puts inside `#ifndef TEST` - **transcribed** (slice 12) |
| 13916-13917 | C 13915's `}` and the blank after it (the glyph-variation block starts at C 13918) | nothing to port |
| 13918-14554 | the **glyph-variation readers**: C 13919's form feed (dropped) and the "Glyph variations." banner (C 13921-13933); `sfnt_read_fvar_table` (C 13940-14155; C 13959 the `SFNT_ENDOF` minimum, C 14009-14015 the two `ckd_mul`/`ckd_add` pairs, C 14045-14046 the `alignof (struct sfnt_instance)` pad, C 14064-14068 the two derived pointers, C 14096-14098 the `alloca`/`xmalloc` fork, C 14100-14123 the per-instance read and `coords` walk, C 14133-14139 the PostScript-name arm, C 14143/14150 the two `xfree (buffer)` guards); `sfnt_read_gvar_table` (C 14164-14318; C 14220-14225 the long/short offset fork, C 14239-14241 the `ckd_add` pair, C 14271-14272 the `global_coords` pointer, C 14276/14300 the two `ckd_add`+`lseek` pairs, C 14293-14296 the `glyph_variation_data` pointer); `sfnt_read_avar_table` (C 14327-14443; C 14373-14377 the rest-of-table read, C 14396-14399 the two `ckd_add`s, C 14404 the bounds test, C 14414-14418 the realloc and the `correspondences` split, C 14420-14432 the second pass) and `sfnt_read_packed_points` (C 14460-14554; C 14473/14493 the two `data >= end` bails, C 14483's `(uint16_t *) -1`, C 14488-14497 the two-byte count, C 14519/14524-14525 and C 14534/14539-14540 the two unpacking walks) | file gate / `TEST_STATIC` on the three table readers |
| 14555-16447 | **slice 8, the file's last non-TEST block**: C 14556-14561's comment and `sfnt_read_packed_deltas` (C 14562-14618; C 14573's `data >= end` bail, C 14584's `*data++` control byte, C 14593/14602 the two more bails, C 14596-14597's two-byte value, C 14612's write-back); the `cvar` reader `sfnt_read_cvar_table` (C 14626-14950; C 14649-14650 the `SFNT_ENDOF` minimum, C 14682's `& 0x0fff`, C 14683-14684 the signed `data_offset`, C 14692-14704 the three pointer initialisers and the zeroed `size`, C 14706-14717 the shared-point read, C 14720-14771 the size-computation walk with its five `ckd_add`s, C 14777-14780 the realloc and the two derived pointers, C 14787-14934 the second pass and its four `sfnt_read_packed_points`/`sfnt_read_packed_deltas` calls plus the four `cvt->num_elements` clamps, C 14937/14943 the two `(void *) -1` guards); the three blend helpers `sfnt_init_blend` (C 14959-14975; C 14972's `axis_count * sizeof *coords * 2` and the two pointers), `sfnt_free_blend` (C 14979-14983) and `sfnt_normalize_blend` (C 14988-15074; C 14998-15036 the three-way normalize with its two `sfnt_div_fixed` pairs, C 15041-15073 the avar walk and its `sfnt_multiply_divide_signed`); the record `struct sfnt_gvar_glyph_header` (C 15078-15088); `sfnt_compute_tuple_scale` (C 15096-15174; C 15103's two `UNINIT`, C 15107's `0200000`, C 15146-15148 the `MIN`/`MAX` fit test, C 15163/15167 the two `sfnt_multiply_divide` calls); the three interpolators `sfnt_infer_deltas_2` (C 15184-15356; C 15195's `j != pair_end` walk, C 15200-15208 the contour wrap, C 15262's `goto consider_y`, C 15334's `goto next`), `sfnt_infer_deltas_1` (C 15361-15406; C 15368's two `SIZE_MAX`, C 15380's `goto next`, C 15397's trailing pair) and `sfnt_infer_deltas` (C 15434-15457; C 15442-15456 the contour walk and its three-way bounds return); the two appliers `sfnt_vary_simple_glyph` (C 15488-15851; C 15518-15521 the offset fork, C 15530's `sizeof header` test, C 15559-15573 the shared-point read, C 15580-15587 the `alloca`/`xnmalloc` fork, C 15615-15663 the coordinate and intermediate reads, C 15668-15673 the scale test, C 15704-15813 the `switch (point_count)` with its `UINT16_MAX` and `default` arms, the touched/original arrays and the `sfnt_infer_deltas` call) and `sfnt_vary_compound_glyph` (C 15861-16212; the same walk against `num_components`, C 16152-16176 the two `flags` tests and the `argument1`/`argument2` writes); `sfnt_vary_interpreter` (C 16221-16277; C 16252-16254 the `ndeltas` ternary, C 16259's `points ? points[j] : j`, C 16261's `index > cvt_size` test, C 16271's `cvt[i] += delta`); and the two metadata readers `sfnt_read_OS_2_table` (C 16292-16381; C 16313-16317 the `minimum` sum with its `offsetof`, C 16331-16333 the read up to `panose`, C 16361-16364 the second read, the 23 swaps) and `sfnt_read_post_table` (C 16398-16444; C 16416's `sizeof *post` gate, the nine swaps) | file gate / `TEST_STATIC` on the fourteen public definitions |
| 16448-16704 | C 16448's `#ifdef TEST` (whose matching `#endif` is C 21215, the file's last line) and its first block: `struct sfnt_test_dcontext` (C 16450-16460), the `sfnt_test_max` object (C 16463), the eight callbacks `sfnt_test_move_to`/`_line_to`/`_curve_to`/`_get_glyph`/`_free_glyph`/`_get_metrics`/`_span`/`_edge_ignore` (C 16465-16595) and the X11 debugger's state with `sfnt_test_edges`/`sfnt_debug_edges`/`sfnt_test_edge` (C 16596-16704; C 57-58's `<X11/Xlib.h>` mirror is `include/xlib.rs` and C 57's edge is added by slice 20, whose `sfnt_x_raster` needs `<X11/extensions/Xrender.h>`, which has no mirror - Owed 30) | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 19) |
| 16705-16889 | the raster test: `sfnt_x_raster` (C 16704-16861, the XRender viewer - the `XOpenDisplay`/`XRenderQueryExtension` prologue, the `goto depth_found` loop, the two `alloca`d arrays, the fifteen `XImage` field writes, the picture pair and the event loop) and `sfnt_test_raster` (C 16863-16884, the ASCII printer that forks the viewer when `SFNT_X` is set); C 57's `include!` edge to `include/xlib.rs` lands here too | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 20) |
| 16890-18182 | the interpreter test programme's fixtures, records, checks and argument objects: C 16890-16941's four fixtures, `sfnt_make_test_interpreter` (C 16943-16950), `struct sfnt_interpreter_test` with `sfnt_run_interpreter_test` (C 16952-16979), `struct sfnt_generic_test_args`/`struct sfnt_rounding_test_args` with the twenty `sfnt_check_*` functions (C 16981-17423) and **all 81 `_test_args` objects** (C 17425-18181, two of them carrying C's three initialiser macros and the four measured arrays) | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 21) |
| 18183-19651 | `all_tests[]` itself - **94 entries** (measured four ways, Evidence ⑰ (i); the ledger's earlier "166" was wrong and is corrected), each an instruction byte array (91 compound literals, one sized literal and two named arrays), a `num_instructions`, an `arg` pointer or `NULL`, and a check function | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 22) |
| 19652-20202 | C 19652's blank line, C 19655's `/* Instruction debugger.  */` banner and the debugger itself: `sfnt_setup_debugger` (C 19657-19692), `sfnt_name_instruction` with its 256-name table (C 19694-19972), `sfnt_draw_debugger` (C 19974-20025), `sfnt_run_hook` (C 20027-20069), the two `exec_prep`/`exec_fpgm` objects (C 20071-20072), `sfnt_identify_instruction` (C 20074-20105), `test_raster_glyph_outline` (C 20109), `sfnt_verbose` (C 20112-20177) and `sfnt_push_hook`/`sfnt_pop_hook` (C 20179-20199) | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 23) |
| 20203-21215 | C 20203's blank line, `sfnt_test_uvs` (C 20203-20300) and `main` (C 20301-21213): the CMAP table read and its subtable prints, `--check-variation-selectors`, the two `#define FANCY_PPEM`/`EASY_PPEM` (C 20473-20474), the fifteen table readers and their prints (`head`/`hhea`/`glyf`/`maxp`/`name`/`meta`/`cvt`/`fpgm`/`prep`/`fvar`/`gvar`/`avar`/`OS_2`/`post`/`cvar`), the FVAR axis/instance walk with the two `goto free_lab` arms (C 20576/20579) and the instance selection, the `memset`/`sfnt_init_blend`/`sfnt_normalize_blend` block, the `fancy` benchmark (the interpreter, the two `alloca`d arrays, the 7-way `exit`, `sfnt_x_raster`), the meta/HEAD/HHEA prints, the second interpreter block with the `GG`/`FG`/`LB`/`RB` macros, and the `while (true)` query loop with `sfnt_decompose_glyph`, the outline/edge/raster timings and the `sfnt_interpret_simple_glyph`/`sfnt_interpret_compound_glyph` fork; ends with the `free_lab:` tail's twenty-four `xfree`s and `return 0`, at the file's last C line | `all (file gate, feature = "test")`; **no gate compiles it** - **transcribed** (slice 24) |

The seventy-two rows above tile C 1-21215 **contiguously** (blank lines included) rather than
skipping the blanks between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`src/include/sfnt.rs` (this file's header's mirror) to this same `sfnt.md` through its
bare-stem fallback, so a table with gaps at blanks would leave `src/sfnt.h`'s non-blank lines
uncovered and fail (the `android.md`/`w32term.md` shape).  The last row reaches C 21215, which is
what the gate requires of the final row.

**The file is complete with slice 24.**  `sfnt_test_uvs` (C 20203-20300) and `main` (C 20301-21213)
are transcribed, so every line of C 1-21215 now has a Rust counterpart (the `#endif` at C 21215 is
the preprocessor's, not Rust's - the `feature = "test"` gate on the forty-one items is what carries
C 16448's `#ifdef TEST`).  What that statement is worth is written in `## Evidence` ⑲ and in
"Not claimed": it is a **text** reading - the unit's `--check` reads `OK`, and no gate compiles the
TEST arm, so nothing here has been type-checked or run.  The line-count cost of the slice is the
one `docs/ports/line-drift-tolerance.tsv`'s newest `sfnt` row records (C 21,215 vs Rust 22,955,
8.2%, with the composition of the excess measured there).

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnt-lane/cargo cargo run -q -p xtask -- build-tu src/sfnt.c`
→ `built src/sfnt.rs -> ../build/femacs/sfnt.o`, **exit 0**, and `build/femacs/sfnt.o` is
**336 bytes** — an empty object, measured on the final revision (and on each of the earlier
ones: the bytes are the same because the frozen feature set compiles none of them).  That is the expected
reading and **not** a proof of the new text: every item sits behind the file gate, so under the
frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute.  Two
further `build-tu` runs are part of the same evidence: slice 1's first run reported its syntax
errors (`unclosed delimiter` ×2, `expected {, found header` ×1), and slice 2's first run
reported exactly two (`\`<\` is interpreted as a start of generic arguments for \`size_t\``, at
`_root.rs` 2033 and 2268 — a cast run against a following `<`, now parenthesised; Deviation 12).
Slice 3's one run is the clean reading above (exit 0, 336 bytes): its errors were all of the
*type* class and surfaced in the arm flip instead, which is where the compiler can see them.
**Slice 4 needed two runs, and its first was a *syntax* reading of this unit's own new text** -
the class only the frozen build can see, because it parses every item and compiles none:
`struct fields are separated by \`,\`` ×2 (`_root.rs` 4135, 4138 - C 3599/3602's `;` after a
struct field), `Rust has no postfix increment operator` ×1 (`_root.rs` 4758 - C 4151's `edge++`)
and \`<\` is interpreted as a start of generic arguments for \`c_int\`` ×2 (`_root.rs` 5089,
5105 - C 4473/4487's `raster->height << 16` directly after a cast).  The second run is the clean
reading above.  **Slice 5 needed three runs, two of them syntax readings of its own new text**:
`unexpected closing delimiter: \`}\`` ×1 (`_root.rs` 6022, against the `while` at 5648 - the
port's `'next:` block had one brace too many where C 5265-5266's `#undef`s stand) and
`struct fields are separated by \`,\`` ×8 (`_root.rs` 5299, 5303, 5317, 5320, 5339, 5342,
5358, 5361 - C's `;` after the four new records' fields).  The third run is the clean reading
above.  **Slice 6 needed three runs**: the first a syntax reading (`\`<\` is interpreted as a
start of generic arguments for \`size_t\`` ×2, at `_root.rs` 6460 and 6656 - C 5647's and C
5822's `directory->length < required`, Deviation 12's class, fixed with the two parens the port's
other sites already carry), the second the clean reading above, and the **third a re-confirmation
of the delivered revision** after the `sfnt_glyph_index` signature fix (Deviation 42) - exit 0 and
336 bytes again, on the md5 this ledger's header records.
**Slice 7's `build-tu` reading is the clean one above**: **three runs of three**, every one exit
0 and **336 bytes** - the first parses the two new ranges as they are spliced in, the second
re-measures the revision after the file's header comment was rewritten, the third the final one
after three of this slice's own `/* … */` notes were corrected.  Both of those edits are comment
text outside every gate (they cannot move this reading or the arm reading's result); the runs are
there so the md5 this ledger records is the md5 a reading measured.  Nothing in this slice's own text is a syntax reading: the classes slices
4 and 5 caught here (`struct fields are separated by \`,\``, a stray \`}\`, a cast against \`<\`)
do not recur - the two `E0308`s this slice had to fix surfaced in the arm flip (②), which is the
only reading that type-checks the items.

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt-lane/sfnt_arm.o --crate-type lib --emit obj
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 0**, `0 errors`, and the object is **36,240 bytes** (slice 5's reading was 31,288); the
four warnings are all in *included mirrors*, none in this unit's own text:
1. `lib/include/stdbit.in.rs` 644 — "unused doc comment";
2. `src/include/lisp.rs` 3934 — "unused doc comment";
3. `lib/include/unistd.in.rs` 54 — "suspicious definition of the runtime `read` symbol";
4. `lib/include/unistd.in.rs` 55 — the same for `write`.

(The last two are a rustc lint that fires on the mirror's own declarations; they are not this
unit's text and this slice cannot remove them.)  **Slice 5's own control readings are the two
3-error runs the fixes below retired**: `3 × E0308` at `_root.rs` 5877, 5952 and 6032 - the two
`APPEND_STEP!` sites (`x` is the `float` loop variable of C 5157/5231 where `sfnt_insert_raster_step`
takes an `int`, and the macro's `y` is an `int` where that function's `scanline` is `size_t`) and
`sfnt_compute_fill`'s `if value < 0` (an `f32` against an integer literal) - and then `3 × E0308`
again (`TRAPEZOID_AREA!`'s `bot_start`/`top_start` were passed uncast, so an `int` met an `f32` at
`_root.rs` 5715, plus the same two `APPEND_STEP!` sites once the first fix was in).  Every one is
at a `_root.rs` line inside this unit's own text.  **A fourth `probe` run re-verified the revision**
after `struct sfnt_fedge`'s two shared C declarations were split one field per line for the
inventory's field check (Deviation 36) - i.e. this slice used **four of the three-run budget**,
reported in the lane's receipt and recorded in Owed 11.  **Slice 6's own control reading is the
5-error run the fix retired**, every one of them at a `_root.rs` line inside this unit's own text:
`2 × E0369` (`<` on the `sfnt_glyph` *record*, `_root.rs` 6360, 6367), `1 × E0369`
(`cannot subtract \`u32\` from \`sfnt_glyph\``, 6372) and `2 × E0605` (`non-primitive cast:
\`sfnt_glyph\` as \`isize\``, 6363, 6364) - all five because C 5542's scalar `sfnt_glyph`
parameter was spelled with the record's name in the port's signature (Deviation 42).  **Slice 4's
own control reading is the 3-error
run the fixes below retired**: `2 × E0308` (`y >= edges[edge].top` compared a `size_t` with an
`sfnt_fixed`, `_root.rs` 4739, and `edges[edge].bottom = y`, 4756, wrote the `size_t` into an
`sfnt_fixed` - both are C's implicit conversions, written `as size_t`/`as sfnt_fixed`) and
`1 × E0381` (`n`, `_root.rs` 4899: the `n = a->next` half of the resort loop's condition sits
inside the `&&`, which the port's initialisation analysis cannot see through; `n` is zeroed now,
Deviation 30), every one of them at a `_root.rs` line *inside this unit's own text*.  **Slice 3's
own control reading is the 41-error
run the fixes below retired**: `29 × E0614` (the port's first spellings of the context arrays —
`*(*(*context).x_coordinates…`, and `(*context)` on `sfnt_decompose_glyph`'s *local* record — plus
`*((*(*glyph).simple).end_pts_of_contours…)`'s missing `as size_t`), `6 × E0384` (C mutates its
`here`/`last` parameters; the port's signature needed `mut`), `2 × E0277` (`m3 * 1`/`m6 * 1`, C's
`int` constant in a `double` expression, carried as `1.0` — Deviation 16.), `1 × E0425`
(`abort`, now a use-site declaration — Deviation 15.) and `1 × E0308` (`u16` into a `size_t`
local), every one of them at a `_root.rs` line *inside this unit's own text* — which is the
reading that says the harness does check it.  The earlier failed arm reading — slice 2's control —
was **8 errors**, all named at `_root.rs` lines in this file: **6 × `E0308`** (`words[0]`/`words4[3]`, a `uint16_t` written into the `int16_t`
scale fields; the port now casts, C's own narrowing), **1 × `E0608`** (raw-pointer indexing
`glyph_index_array[j]`; the port writes `*(… .offset (j as isize))`) and **1 × `E0423`**
(`MIN` is the port's `macro_rules!`, so the call is `MIN! (…)`).  Slice 1's own control reading
was 8 errors of other classes (`E0428`/`E0530`/`E0600`/`E0381`/`E0308`), retired by Owed 1 and
the fixes below.

**One intermediate arm reading is not about this unit.**  While this slice was being written, one
run ended on `error[E0204]`/`error[E0277]` in `lib/include/signal.rs` (`sigaction`'s
`#[derive(Clone, Copy)]` against a `sigset_t` whose derives the materializer had hoisted onto the
other `#[cfg]` arm): another lane's **in-flight** mirror edit, reverted by the time of the reading
above — `git status` showed ` M lib/include/signal.rs` during that run and does not now.
P-86 ⑦ is exactly this case: while the include graph is broken the arm flip does not check this
unit's names, so that run is recorded as a blocked reading and **not** as a reading of this port.
**Slice 7's own control reading is the 2-error run the two casts retired**: `2 × E0308`
(`versor = (*interpreter).state.freedom_vector.x`, `_root.rs` 7468, and its `.y` twin at 7494 -
C 10475/10501 widen an `sfnt_f2dot14` into an `sfnt_f26dot6`, and the port's `versor` is the 26.6
type, so both sites now carry `as sfnt_f26dot6`; Deviation 46).  Four further runs are the clean
reading above (probes 2 and 3 on the pre-header revision, probe 4 after the header rewrite, probe
5 on the delivered md5), so this slice used **five `probe` runs of three**: what the fourth and
fifth buy is an arm reading of the exact bytes this ledger's header names, and the overrun is
reported rather than hidden (Owed 15).  The four
warnings are all in *included mirrors* (`lib/include/stdbit.in.rs` 644 and `src/include/lisp.rs`
3934, unused doc comments; `lib/include/unistd.in.rs` 54/55, the `suspicious definition of the
runtime \`read\`/\`write\` symbol` lint) - the same four slice 6 read, none in this unit's own
text - and the object is **39,664 bytes** (slice 6: 36,240), a compile artefact of the twenty-four
new definitions, not runtime evidence (see "Not claimed").

**③ Inventory (the live checklist).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` → **FAIL**, with
**268 `- missing …` lines — 118 functions, 147 macros, 3 constants (the three
`SFNT_RUN_CONTEXT_*`), 0 file-scope objects, 0 fields** — down from 291 (138/147/3/0/0) at the end
of slice 5, i.e. **all 20 of slice 6's own functions match** (`sfnt_read_hmtx_table`,
`sfnt_lookup_glyph_metrics`, `sfnt_scale_metrics`, `sfnt_get_scale`, `sfnt_read_name_table`,
`sfnt_find_name`, `sfnt_read_meta_table`, `sfnt_find_metadata`, `sfnt_read_ttc_header`,
`sfnt_read_cvt_table`, `sfnt_read_fpgm_table`, `sfnt_read_prep_table`, `sfnt_div_f26dot6`,
`sfnt_mul_f26dot6`, `sfnt_mul_f26dot6_round`, `sfnt_mul_f2dot14`, `sfnt_mul_f26dot6_fixed`,
`sfnt_floor_f26dot6`, `sfnt_ceil_f26dot6` and `sfnt_round_f26dot6`; the function count falls by
exactly 20), the macro count is unchanged (this slice defines no macro) and the constant count is
unchanged too - the three `SFNT_RUN_CONTEXT_*` live in C 6533-6541's enum, which is past this
slice's C 6306 end (it is the first item of the remainder row).  **This slice produces no named
row at all**: the reading's only non-`missing` line is
`- line count drifted 66.1%: C 21215 vs Rust 7191` (the whole file against six slices; slice 1's
reading was 93.9%/1,298, slice 2's 86.7%/2,811, slice 3's 80.6%/4,113, slice 4's 75.6%/5,181,
slice 5's 70.5%/6,251).  There is therefore **nothing new owed to
`docs/ports/macro-body-notes.tsv`** (Owed 10's seven rows were written by the integrator in
3.470 and the reading confirms the class is empty), and no use-site declaration was needed either
(Deviation 44).

**Slice 7's reading has named rows again - the first since slice 5.**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` → **FAIL**, with
**244 `- missing …` lines — 94 functions, 147 macros, 3 constants (the three
`SFNT_RUN_CONTEXT_*`), 0 file-scope objects, 0 fields** — down from 268 (118/147/3/0/0) at the end
of slice 6, i.e. **all 24 of this slice's own functions match** (`sfnt_round_none`,
`sfnt_round_to_grid`, `sfnt_round_to_double_grid`, `sfnt_round_down_to_grid`,
`sfnt_round_up_to_grid`, `sfnt_round_to_half_grid`, `sfnt_round_super`, `sfnt_round_super45`,
`sfnt_project_onto_x_axis_vector`, `sfnt_project_onto_y_axis_vector`, `sfnt_dot_fix_14`,
`sfnt_project_onto_any_vector`, `sfnt_dual_project_onto_any_vector`, `sfnt_move_x`, `sfnt_move_y`,
`sfnt_move`, `sfnt_short_frac_dot`, `sfnt_validate_gs`, `sfnt_set_freedom_vector`,
`sfnt_set_projection_vector`, `sfnt_read_fvar_table`, `sfnt_read_gvar_table`,
`sfnt_read_avar_table` and `sfnt_read_packed_points`), the function count falling by exactly 24
while the macro and constant counts hold.  **Five of the reading's rows are new and belong to
`docs/ports/macro-body-notes.tsv`** (Owed 14): `sfnt_validate_gs` → `break,case,switch`
(C 10543's `switch` is the port's `match`, with the `_ => {}` that C's missing `default:`
forces - Deviation 52), `sfnt_read_fvar_table` → `alloca,goto` (C 14096's `alloca` is P-05's
`sa_stack_alloc` and C 13977's `goto bail` is a `break 'bail` - Deviations 47/48),
`sfnt_read_gvar_table` → `goto`, `sfnt_read_avar_table` → `goto` and `sfnt_read_packed_points` →
`goto` (the `bail`/`bail1` labels; the same citing).  The reading's other non-`missing` line is
`- line count drifted 60.2%: C 21215 vs Rust 8447` (slice 1's reading was 93.9%/1,298, slice 2's
86.7%/2,811, slice 3's 80.6%/4,113, slice 4's 75.6%/5,181, slice 5's 70.5%/6,251 and slice 6's
66.1%/7,191).

**④ Other gates (repo root, final revision).**
`python3 tools/check-dead-file-coverage.py` → **46 ledger(s) bound (a C range table), 0
failure(s)** (that count moves as the parallel lanes land their ledgers), and this ledger is one of
them: `info docs/ports/sfnt.md: 50 range row(s) cover src/sfnt.c (21215 lines; 0 blank line(s)
between blocks need no row)` plus the same 50 rows over `src/sfnt.h` (2151 lines, the bare-stem
fallback above) — the row added by this slice is `5452-6306`, and the old `5452-21215` row is now
`6307-21215`. —
`python3 tools/check-declarations.py` → **10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures** (this slice adds none: Deviation 44's list is every name it calls). —
`python3 tools/check-ledger-ticks.py` → **234 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 356 row(s) with no function tick**, 0 row(s) without a
ledger (this ledger carries a range table, not a tick list). —
`python3 tools/check-mirror-dupes.py` → **19 duplicate declaration(s), 19 ratchet row(s), 0
failure(s)** (a few minutes earlier another lane's in-flight, untracked
`src/include/gtk.rs` made the same run read 43/19/24; that file is not in this lane's
write set and is gone from the tree at the reading above). —
`python3 tools/check-cond-eval.py src/sfnt.c` → **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (the C file is frozen, so the evaluator's table cannot move with a port
change; this slice's range carries no `#if` at all - C 6307's very next lines are the first ones
that do, and they are in the remainder).
**Slice 7's readings of the other gates** (repo root, the delivered revision):
`python3 tools/check-dead-file-coverage.py` → **47 ledger(s) bound (a C range table), 0
failure(s)**, and this ledger's own line is now `info docs/ports/sfnt.md: 54 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 54 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback) - the one `6307-21215` row is now five
(`6307-10224`, `10225-10663`, `10664-13917`, `13918-14554`, `14555-21215`), which is P-91 ③'s
alternating shape and the first time this table carries a landed row that is not part of the
C 1-6306 run. —
`python3 tools/check-declarations.py` → **10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures**, unchanged: this slice adds no use-site declaration (Deviation 44's list
is every name it calls). —
`python3 tools/check-ledger-ticks.py` → **234 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 356 row(s) with no function tick**, 0 row(s) without a
ledger (unchanged; this ledger carries a range table, not a tick list) - the reading that would
fail if one of this slice's twenty-four `// C a-b` comments sat outside its row. —
`python3 tools/check-mirror-dupes.py` → **19 duplicate declaration(s), 19 ratchet row(s), 0
failure(s)**, the same numbers slice 6 read. —
`python3 tools/check-cond-eval.py src/sfnt.c` → **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (the C file is frozen; neither new range carries a `#if` - `awk` over
C 10225-10663 and C 13918-14554 prints no line starting with `#` - so this slice adds nothing to
the evaluator's set).

**Slice 8's readings.**  The slice's `build-tu` used all **three of three** and its arm flip
**three of three** (the third is the pre-slice control below): each reading had to identify its
errors before the delivered revision could be the one measured, and the last `build-tu` run
re-measures the file this ledger's header names.

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnt-lane/target cargo run -q -p xtask -- build-tu src/sfnt.c`
→ `built src/sfnt.rs -> ../build/femacs/sfnt.o`, **exit 0**, and `build/femacs/sfnt.o` is
**336 bytes** — the same empty object slices 1-7 measured, for the same reason (every item sits
behind the file gate, so the frozen feature set compiles only `include/config.rs` and the
`#![allow]` attribute).  The first run is a **type** reading of this unit's own new text, and of a
class only the frozen build can produce: because the file's own `include!`s are behind the file
gate, `uint16_t` is not in scope in that configuration, so an item written without the gate fails
there — `2 × E0425: cannot find type uint16_t` at `_root.rs` 9117 and 9121, the two fields of
`struct sfnt_gvar_glyph_header`, which had been spliced in without
`#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]`; the second run is the
clean reading above, and the third re-measures the **delivered** revision (md5
`387c378e9e25d7ce54ec1d297aeae8cb`) — exit 0 and the same 336 bytes — because the unit's header
comment was rewritten after the second run.  That header is comment text outside every gate (the
slice-4 note above), and `diff` shows it is the **only** difference between the revision the arm
flip measured (md5 `18fb814ec04a3fcf125a8633fa93795b`) and the delivered one.

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt-lane/sfnt_arm.o --crate-type lib --emit obj
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 1, and five of the seven `error` lines are not this unit's text**: `2 × E0425: cannot find
type time_t` (`lib/include/sys_time.rs` 44 and 90) and `3 × E0425: cannot find type timespec`
(`lib/include/sys_select.rs` 56, `src/include/thread.rs` 300 and 318).  **This slice's own text is
clean in that reading.**  The first run of it was **17 errors, all of them this unit's**:
`4 × E0308` — `*coords = *local.offset (j as isize)` and its `*points` twin needed
`as sfnt_f2dot14` (C 14878/14896 assign `uint16_t` into `sfnt_f2dot14 *coords`), and the two
`scale = sfnt_multiply_divide (…)` sites of `sfnt_compute_tuple_scale` needed `as sfnt_fixed`
(that helper returns `c_uint`, C 15163/15167's implicit conversion) — and `8 × E0381`: `tuple`,
`data_size`, `index` and `n_local_points` in `sfnt_vary_simple_glyph` and
`sfnt_vary_compound_glyph` are C locals that precede any write but are read by `&raw mut` in the
`sfnt_read_packed_points` call (C's declaration block, C 15497-15504/15870-15877); PORTING.md 1.5
zeroes them (Deviation 53).  Every one of the 17 is retired in the delivered revision, and what
remains is the mirror trio.  **That trio reproduces without this slice** — the third run of this
budget: `cp /tmp/sfnt-lane/sfnt.rs.before src/sfnt.rs` (md5
`80aed2e1701986817a5de1b3172e313c`, the revision slice 7's Evidence ⑤ records) and the same
command → the *same five* `E0425`s at the same five sites, and `error: aborting due to 5 previous
errors` again.  The cause is structural and belongs to the mirror graph: the probe's crate carries
exactly one `pub type time_t` (`lib/include/time.in.rs` 36) and one `timespec`, and **both
mirrors are included under C 52-53's `#ifdef TEST` arm** (`_root.rs` 175-178), which the arm flip's
feature set does not enable, while `src/include/thread.rs` 33 includes `lib/include/sys_select.rs`
unconditionally (C's `thread.h` 40 includes `sysselect.h` unconditionally) and `sys_select.rs` 27
includes `sys_time.rs`.  It is Owed 16, not a defect of C 14555-16447.  The two warnings are the
`lib/include/stdbit.in.rs` 644 "unused doc comment" and `src/include/lisp.rs` 3934 reading the
earlier slices recorded.  This reading's revision is md5 `18fb814ec04a3fcf125a8633fa93795b`; the
delivered revision (`387c378e9e25d7ce54ec1d297aeae8cb`) differs from it by the header comment's
prose alone (①), so the reading is of this slice's code with a staler header comment.

**③ The inventory check (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` → **230 missing rows**
(`/tmp/sfnt-lane/abi-after.txt`), **down from 244** (`/tmp/sfnt-lane/abi-before.txt` taken before
the first edit): **exactly the fourteen functions this range defines** — `sfnt_read_packed_deltas`,
`sfnt_read_cvar_table`, `sfnt_init_blend`, `sfnt_free_blend`, `sfnt_normalize_blend`,
`sfnt_compute_tuple_scale`, `sfnt_infer_deltas_2`, `sfnt_infer_deltas_1`, `sfnt_infer_deltas`,
`sfnt_vary_simple_glyph`, `sfnt_vary_compound_glyph`, `sfnt_vary_interpreter`,
`sfnt_read_OS_2_table`, `sfnt_read_post_table` — with the macro count (147) and the constant count
(3) unchanged (the range defines no macro and no constant, and `awk` over C 14555-16447 prints no
line starting with `#`).  The reading's other line is `- line count drifted 49.3%: C 21215 vs Rust
10763` (slice 7's was 60.2%/8,447; the ladder is now 93.9%/1,298, 86.7%/2,811, 80.6%/4,113,
75.6%/5,181, 70.5%/6,251, 66.1%/7,191, 60.2%/8,447, 49.3%/10,763).  **The eight `does not mention`
rows the range's first reading produced are written by this lane** and the reading is re-taken
(the ledger's Owed 17): `sfnt_read_packed_deltas` → `goto`, `sfnt_read_cvar_table` → `goto`,
`sfnt_compute_tuple_scale` → `uninit`, `sfnt_infer_deltas_2` → `goto`, `sfnt_infer_deltas_1` →
`goto`, `sfnt_vary_simple_glyph` → `alloca,case,default,goto,restrict,switch` and
`sfnt_vary_compound_glyph` → the same six, each with its C lines and the port's spelling
(`docs/ports/macro-body-notes.tsv`).  The delivered reading therefore has **no `does not mention`
row at all**: 230 `- missing …` rows and the line-count row, nothing else.

**④ Other gates (repo root, the delivered revision).**
`python3 tools/check-dead-file-coverage.py` → **143 ledger(s) bound (a C range table), 0
failure(s)**, and this ledger's own line is `info docs/ports/sfnt.md: 55 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 55 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback) — the table's last row, `14555-21215`, is now
two (`14555-16447`, `16448-21215`), so the rows still tile C 1-21215 contiguously and the last row
still reaches C 21215. —
`python3 tools/check-port-lines.py` → **0 file(s) under ten percent, 0 reference line(s); 0
ratchet row(s), 0 new, 0 stale** (this file's `docs/ports/line-drift-tolerance.tsv` row is still
Owed 6's stale one, untouched; the ratio this slice moves is C 21,215 -> 10,763, 49.3% drift). —
`python3 tools/check-ledger-ticks.py` → **238 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 352 row(s) with no function tick**, 0 row(s) without a
ledger (unchanged: this ledger carries a range table, not a tick list; the reading would fail if
one of this slice's fourteen `// C a-b` comments sat outside its row). —
`python3 tools/check-declarations.py` → **17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures**; this slice adds none (every name it calls is already declared by an
earlier slice or by the mirror). —
`python3 tools/check-mirror-dupes.py` → **17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)** (the count moves with the parallel lanes; this slice adds no mirror declaration — the
one new record, `sfnt_gvar_glyph_header`, is defined in the unit precisely because `sfnt.h` does
not have it). —
`python3 tools/check-citations.py` → **676 file(s) scanned, 0 drift, 0 range, 1814 external, 36
allowed, 0 stale**. —
`python3 tools/check-cond-eval.py src/sfnt.c` → **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (the C file is frozen and neither new range carries a `#if`).

**Slice 9's readings.**  This slice used **four `build-tu` runs of three and five `probe` runs of
three** — both overruns are reported rather than hidden (Owed 28), and every extra run bought a
distinct reading of text this lane had just changed; the last run of each pair measured the
revision **md5 `8a52f1bab66553e76fa1e5a0671647b3`** (13,174 lines), which is also the delivered
revision except for **thirteen comment lines** — the `// C a-b` end lines this lane then corrected
against the C file's own braces (`sfnt_interpret_ip` 9537->9527, `sfnt_deltac` 9668->9656,
`sfnt_interpret_mdap` 9704->9694, `sfnt_deltap` 9823->9815, `sfnt_interpret_call` 9868->9860,
`sfnt_set_srounding_state` 9936->9932, `sfnt_skip_code` 9994->9989,
`sfnt_interpret_unimplemented` 10023->10019, `sfnt_interpret_fdef` 10078->10074,
`sfnt_interpret_idef` 10136->10130, `sfnt_interpret_if` 10188->10185, and the forward
declaration's note `C 9825-9826`->`C 9817-9819`) plus their two ledger echoes.  **The delivered
revision is md5 `d2c6117793143ab47fdfe9732dfb0408`**, and this paragraph is the disclosure
slice 8's Evidence ① made for a header-comment change: the delta is comment text outside every
gate (`//`-comments and one block comment), so no reading in ②/③/④ can see it — the integrator can
re-pin it with one `build-tu`/`probe` pair if that is preferred to the overrun Owed 28 already
reports.  The two ranges are C 2,310 lines (C 6312-6557 + C 8161-10224) against the port's +2,411
(10,763 -> 13,174, +4.4% over the C range), and the whole-file drift falls from 49.3% to
**37.9%** (the ladder is now 93.9%/1,298, 86.7%/2,811, 80.6%/4,113, 75.6%/5,181, 70.5%/6,251,
66.1%/7,191, 60.2%/8,447, 49.3%/10,763, 37.9%/13,174).

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnt-lane/target cargo run -q -p xtask -- build-tu src/sfnt.c`
-> `built src/sfnt.rs -> ../build/femacs/sfnt.o`, **exit 0**, and `build/femacs/sfnt.o` is
**336 bytes** — the same empty object slices 1-8 measured, for the same reason (every item sits
behind the file gate, so the frozen feature set compiles only `include/config.rs` and the
`#![allow]` attribute).  **All four runs read exactly that**, including the first one: unlike
slices 4 and 5, **none of this slice's runs was a syntax reading of its own text** — the two
ranges were spliced in and parsed on the first try, and the class the frozen build *can* see
(gate placement, since the unit's own `include!`s are behind the file gate) did not bite: every
new item carries `#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]`.  The
runs are (i) the first splice, (ii) the re-measure after the `TRAP! (c"...".as_ptr ())` spelling
and the 56 zero-initialised out-parameters landed (②'s first two readings), (iii) the re-measure
after the last six of them (`sfnt_interpret_alignrp_1`'s and `sfnt_interpret_ip`'s `x`/`y`/
`original_x`/`original_y`, ③'s third reading), and (iv) the **delivered** revision, after the two
`PROJECT`/`DUAL_PROJECT` macro bodies were corrected to C's own text (③'s `does not mention`
rows).  Run (iv) is the reading this ledger's md5 names.

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt-lane/x.rlib --crate-type=lib -C panic=abort
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
-> **exit 1, `6` errors, and five of the six are not this unit's text**: `2 × E0425: cannot find
type time_t` (`lib/include/sys_time.rs` 44 and 90) and `3 × E0425: cannot find type timespec`
(`lib/include/sys_select.rs` 56, `src/include/thread.rs` 300 and 318) — the same five sites, and
the same cause, as Owed 16 records (both mirrors are reached only through C 52-53's `#ifdef TEST`
arm).  The **sixth is this slice's own, and it is P-91 ④'s class**: `E0425: cannot find function
sfnt_interpret_run` at `_root.rs` 9238, i.e. C 9832's `sfnt_interpret_run (interpreter, context)`
inside `sfnt_interpret_call` — the definition is C 11396 and is the next slice's, so this is an
*allowed, named* reading, **not a pass**; it closes when C 10664-13917 lands.  No object is
produced (rustc stops at the errors), so there is no `x.rlib` to measure.  **The four earlier runs
are the readings the fixes retired, and every error in them was this unit's own text**:
(i) **89 errors** — `42 × E0423` (`TRAP ("…")` written without the `!` a `macro_rules!` call
needs; the four `POP` copies' bodies included), `40 × E0381` (the out-parameters C declares and
writes by address: every `&raw mut x` in `sfnt_interpret_scfs` … `sfnt_interpret_isect`),
`1 × E0308` (C 6495's `sfnt_div_fixed (pixel_size * 64, head->units_per_em)` needed
`as sfnt_fixed`) and the six ④-class ones; (ii) **48 errors** — the same 42 `E0423` sites
retired into `42 × E0308`, because `TRAP!`'s argument is a `&str` where the parameter is
`*const c_char`, which is why the sites now read `c"…".as_ptr ()` (Deviation 72); (iii) **12
errors** — `6 × E0381` left (`sfnt_interpret_alignrp_1`'s and `sfnt_interpret_ip`'s `x`, `y`,
and the two `original_*`) plus the six ④-class ones.  The two warnings are in *mirrors*, not in
this unit's text: `lib/include/stdbit.in.rs` 645 and `src/include/lisp.rs` 3943, both "unused doc
comment" (the same class slice 8 recorded at 644/3934).

**③ The inventory check (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c src/sfnt.rs` -> **174 `- missing …`
rows** (`/tmp/sfnt-lane/abi.after2`, taken after the `macro-body-notes.tsv` rows below landed),
**down from 230** (`/tmp/sfnt-lane/abi.before`, taken before the first edit): **-49 functions**
(80 -> 31: the 46 definitions of the C 8161-10224 half plus `sfnt_init_graphics_state`,
`sfnt_make_interpreter` and `sfnt_interpret_trap` — `sfnt_line_to_standard_form` is *not* one of
them, because C 8460's `#if 0` is dead text), **-4 macros** (147 -> 143: `TRAP`, `POP`, `PROJECT`
and `DUAL_PROJECT`, which now exist as the P-57 in-function `macro_rules!` copies) and **-3
constants** (3 -> 0: `SFNT_RUN_CONTEXT_FONT_PROGRAM`, `_CONTROL_VALUE_PROGRAM` and
`_GLYPH_PROGRAM`, the enum C defines in the `.c`; the fourth member, `SFNT_RUN_CONTEXT_TEST`, sits
behind C 6538's `#ifdef TEST` and is carried under `feature = "test"`).  The reading's other line
is `- line count drifted 37.9%: C 21215 vs Rust 13174`.  **The ten `does not mention` rows the
first reading produced, plus the two the macro bodies produced, are retired**: the two macros by
correcting the bodies to C's own `sfnt_project_vector (interpreter, x, y)` /
`sfnt_dual_project_vector (interpreter, x, y)` (they had been written against the
`state.project` / `state.dual_project` fields — a *reading of C's text* the checker's token
comparison caught, and the reason `build-tu` ran a fourth time), and the ten functions by the
`docs/ports/macro-body-notes.tsv` rows this lane appended (Owed 26): `sfnt_address_zp2`/`_zp1`/
`_zp0` and `sfnt_interpret_isect`/`sfnt_normalize_vector` -> `goto`;
`sfnt_deltac`/`sfnt_deltap`/`sfnt_set_srounding_state` -> `break,case,default,switch`;
`sfnt_interpret_if`/`sfnt_interpret_else` -> `case,switch`.  The delivered reading therefore has
**no `does not mention` row at all**: 174 `- missing …` rows and the line-count row, nothing else.

**④ Other gates (repo root, the delivered revision).**
`python3 tools/check-dead-file-coverage.py` -> **145 ledger(s) bound (a C range table), 0
failure(s)**, and this ledger's own line is `info docs/ports/sfnt.md: 57 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 57 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback) — slice 9 replaces the single `6307-10224` row
with three (`6307-6557`, `6558-8159`, `8160-10224`), so the rows still tile C 1-21215 contiguously
and the last row still reaches C 21215. —
`python3 tools/check-port-lines.py` -> **0 file(s) under ten percent, 0 reference line(s); 0
ratchet row(s), 0 new, 0 stale** (this file's `docs/ports/line-drift-tolerance.tsv` row is still
Owed 6's stale one, untouched; the ratio this slice moves is C 21,215 -> 13,174). —
`python3 tools/check-ledger-ticks.py` -> **238 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 352 row(s) with no function tick**, 0 row(s) without a
ledger (this ledger carries a range table, not a tick list; the reading would fail if one of this
slice's 50 `// C a-b` comments sat outside its row). —
`python3 tools/check-declarations.py` -> **17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures**; this slice adds no *string* site, and its one new declaration
(`longjmp`, C 37's site) is a `/// C:`-annotated `unsafe extern "C"` block, the shape the mman
block above it already has. —
`python3 tools/check-mirror-dupes.py` -> **17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)** (a count that moves with the parallel lanes; this slice declares nothing in a
mirror — `sfnt_interpreter_run_context` is defined in the unit, like `sfnt_gvar_glyph_header`,
Owed 27). —
`python3 tools/check-citations.py` -> **676 file(s) scanned, 0 drift, 0 range, 1905 external, 36
allowed, 0 stale**. —
`python3 tools/check-cond-eval.py src/sfnt.c` -> **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (C 6538's `#ifdef TEST`, C 6427-6431's, the `#if 0` regions and the
macros' `#ifndef TEST`/`#else` pairs are all inside the frozen file the tool already verified).

## Deviations

1. **`TEST_STATIC` is a construct PORTING.md does not cover yet.**  C's `#define TEST_STATIC
   static` (C 128, the TEST arm) / `#define TEST_STATIC` (C 137) decides the linkage of the
   readers this file exports — nine of them now (slice 1's two plus slice 2's seven).  The port's
   expression is `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, which is the same
   bit and needs no second copy of any body.  It is the tree's heaviest user of the attribute
   (measured on this revision: `rg -c cfg_attr src/*.rs` → `src/sfnt.rs:12` — the
   nine attributes plus three mentions in prose — and `src/w32proc.rs:2`, another lane's
   in-flight file) and the tool's textual test for the attribute (`tools/extract-abi.py`
   1984-1989) does not see through it, which is what the nine
   `- function … has no #[unsafe(no_mangle)]` rows in Evidence ③ are.  Owed 1 asks PORTING.md to
   rule on the spelling.
2. **C's designated initialiser for `sfnt_table_names`.**  C 197-219 is `= { [SFNT_TABLE_CMAP] = …,
   … }` with all 18 slots designated; Rust has no designated initialiser, and the port's first
   spelling (a `let mut` local inside the `static`'s initialiser, keyboard.rs 5607's shape) is a
   measured **`E0530: let bindings cannot shadow statics`**.  The port therefore writes the same
   18 values positionally in C's order, each with `/* [SFNT_TABLE_*] = … */` — which is the same
   array (no slot is implicit) and keeps C's order and values on the page.
3. **Locals C leaves uninitialised and the port zeroes** (PORTING.md 1.5, `android.md`'s
   Deviations): `length` in `sfnt_read_cmap_format_8`/`_12` (C 671/762), `header` in
   `sfnt_read_cmap_table_1` (C 944), `buffer1`/`buffer` in `sfnt_read_cmap_format_14` (C 866/868),
   and in slice 2 `glyph` in `sfnt_lookup_glyph_2` (C 1137, written by the `memcpy` at C 1173 and
   read on the next line) and `flags`, `instruction_length`, `words[2]`, `words4[4]` in
   `sfnt_read_compound_glyph` (C 2180: `flags` is read by the `while (flags & 040)` test of C's
   first `do`-loop, and the arrays are `memcpy` destinations).  `E0381` is what leaving them
   uninitialised measures.
4. **`(T *) -1` is `usize::MAX as *mut T`** (21 sites in slice 1: the TTC returns of
   `sfnt_read_table_directory`, the six format readers' IO-error returns,
   `sfnt_read_cmap_table_1`'s five, and C 1096's `(void *) -1` test).  Slice 2 adds none (its
   `NULL` returns are all `std::ptr::null_mut ()`; C 1865's `MAP_FAILED` is the constant the
   use-site block above C 49 spells the same way, C 185's `MAP_FAILED`).
5. **C's `(uint16_t *) (what)` cast in `sfnt_swap16`/`sfnt_swap32` is two casts in Rust.**  The
   argument is already a `uint16_t *` at most sites but is `&format4->id_delta[i]` (an `int16_t
   *`) at C 577/578, `&delta` (an `int16_t`) at C 2109/2153, and `&head->xmin`-class fields
   (`sfnt_fword`, i.e. `int16_t`) at C 1513-1537, `&hhea->caret_slope_rise`-class at C 1597-1604
   and the `int16_t` union arms at C 2315-2323; Rust cannot cast a reference straight to a
   differently-typed raw pointer, so the macro keeps C's single cast as `as *mut _ as *mut X`
   (measured: both call shapes compile).
6. **`SFNT_ENDOF (struct X, field, uint16_t[256])`** (C 392/438) is
   `SFNT_ENDOF! (X, field, [uint16_t; 256])` — Rust's array type syntax; C 1763's
   `SFNT_ENDOF (struct sfnt_maxp_table, num_glyphs, uint16_t)` needs no such change.
7. **`ckd_mul`/`ckd_add` are written out at their sites** (C 711/717, C 802/808, C 887-888, C 946
   and slice 2's C 1970-1974) as `overflowing_*` plus a write-back — P-15's spelling (`buffer.rs`
   3602, the `androidselect.rs` 619-620 shape).  C 887-888's and C 1970-1974's `||` chains are
   three `if`s in the port because each `ckd_*` writes as well as returning the flag; the early
   action is the same control flow and the unobserved difference (C leaves the target alone on
   overflow, the port returns before reading it) is recorded here.  C 1970-1974's final action is
   `memory_full_up ()`, which the TEST arm defines at C 58-64 and the frozen arm reaches through
   lisp.rs 4034.
8. **C's `switch` is the port's `match`** with `_` for `default` (PORTING.md 2.2): slice 1's
   seven-way `sfnt_read_cmap_table_1` (C 973-1010) and slice 2's six-way `sfnt_lookup_glyph`
   (C 1437-1462).  C's second `switch` has **no `default:`** — control falls out to C 1464's
   `return 0`, which the port spells as an arm `_ => {}` followed by the same `return 0;`.
9. **C's implicit conversions are written as casts at their sites**, which is what Rust
   requires.  Slice 1 measured `header->length`/`uint16_t`/`uint32_t` against `size_t`/`off_t`,
   the `uint16_t i` loop counters, `format14`'s truncated `num_records` and
   `sfnt_table_names[table]`; slice 2 adds **`(*glyph).number_of_contours as c_int as size_t`**
   (C 1938 computes `number_of_contours * sizeof (uint16_t) + sizeof (uint16_t)` at `size_t`
   width, so a negative contour count wraps exactly as C's does), **`(min_size as size_t)`** and
   **`(i as size_t) < num_offsets`** (C's unsigned comparisons against a signed `ssize_t`/`int`),
   **`rc < (*directory).length as ssize_t`** (the opposite case: `ssize_t` against `uint32_t`
   converts the *unsigned* side up, so C 1647/1692's test is signed),
   **`code = character as uint16_t`** and `character as uint16_t` at the C 1294 call,
   **`(j as c_int)`/`first_code as c_int`** in `sfnt_lookup_glyph_2`'s and
   `sfnt_lookup_glyph_6`'s range tests, **`(first_code as c_int + entry_count as c_int) as
   sfnt_char`** (C 1308-1309 compares `unsigned int` against the promoted sum, so the port must
   compare unsigned too), **`words[0] as int16_t`**-class narrowings into the scale fields
   (C 2290-2350) and **`segment_address as isize - end_code as isize`** for C 1288's pointer
   difference (computed before the null test, as C does).
10. **`p[i]` on a raw pointer is `*(p.offset (i as isize))`** (21+ sites; slice 2's new ones are
    `subheaders[k]`… `glyph_index_array[j]`, `sub_header_keys[i]`, `end_pts_of_contours[…]`,
    `offsets[i]`, `flags[i]`, `x_coordinates[i]`, `y_coordinates[i]` and
    `components[i]`/`words[0]`).  Measured `E0608`: Rust has no `Index` for `*mut T`.  The two
    places where the port *does* keep `[…]` are the `[T; N]` fields (`sub_header_keys` C 1144,
    `glyph_index_array` C 1122 in `sfnt_lookup_glyph_0`), which are arrays and not pointers.
    This belongs in `docs/ports/token-aliases.tsv` (Owed 2).
11. **C's macro `MIN` is the port's `macro_rules! MIN`, so C 1768's call is `MIN! (…)`** — the
    first use of either `MIN`/`MAX` in this file (measured `E0423` without the `!`; the tree's
    own spelling is `android.rs` 750's `MIN! (16usize, length)`).
12. **A cast followed by `<` needs parentheses in Rust.**  C 1763's
    `directory->length < SFNT_ENDOF (…)` and C 1944's `glyf->size - min_size < offset` first
    became `… as size_t\n < …` / `… as size_t < offset`, and rustc parsed the cast's type as
    generic arguments (measured: `\`<\` is interpreted as a start of generic arguments for
    \`size_t\``, `_root.rs` 2033 and 2268).  The port writes `((*directory).length as size_t) < …`
    and `(min_size as size_t)` — the same comparison, spelled so the cast ends where C's does.
13. **`<sys/mman.h>` is a use-site block.**  The ref tree has no such header, so slice 2's four
    prototypes and five constants stand in the unit at C 41's site, each with a `/// C:` line
    (the P-88 ① shape, `pdumper.rs` 102-166's precedent for the same header).  The values are the
    ones `pdumper.rs` measured with `clang -E -dM -include sys/mman.h` on the frozen target.
14. **Line count**: slice 2 is C 1,295 lines (C 1113-2407) against the port's +1,513
    (1,298 → 2,811, +16.8%) — the per-item gates, the C-line comments,
    `std::mem::size_of::<T> ()` for C's `sizeof *p` and `sizeof (T)`, the `.offset (i as isize)`
    deref for C's `p[i]`, the cast lines C's implicit conversions become, and Rust's brace-only
    lines.
15. **`abort` is a use-site declaration** (P-88 ①, `androidvfs.rs` 228-234's shape): C 34's
    `#include <stdlib.h>` is mirrored by `lib/include/stdlib.rs`, which carries `malloc` and
    friends but **not** `abort`; C 2451's `abort ()` is this file's first use outside the TEST arm
    (`mem_full_up`'s) and the port declares C's prototype in a gated block at C 34's site.
    Measured: `error[E0425]: cannot find function \`abort\` in this scope` without it.
16. **C's `int` constant inside a `double` expression has no Rust spelling.**  C 2610/2613's
    `m3 * 1` / `m6 * 1`: PORTING.md 1.1 spells C's `double` as `f64`, so the `1` must be a float
    literal and the port writes `1.0` — the same value, one token C does not have.  (Measured
    `error[E0277]: cannot multiply \`f64\` by \`{integer}\``: Rust does not convert the literal for
    you.  The `double`-token row this function owes is Owed 3.)
17. **`sizeof` over a *place* keeps the place's name.**  C 2694/2709's
    `sizeof *context->x_coordinates` / `*context->contour_end_points` and C 3516's
    `memset (&context, 0, sizeof context)` are the port's `std::mem::size_of_val (&…)` rather
    than `size_of::<T> ()`, because the operand's `context` is a **registered token family**
    (`docs/ports/token-aliases.tsv` 30, `context`/`set_context`/`set_context_`) and the family's
    occurrence check counts it.  Both spellings are the same member of the `sizeof` family
    (token-aliases 15), but `size_of::<sfnt_fixed> ()` drops the operand token: measured,
    `- family context: the C body uses it 37 time(s), the port 35` and `… 23 time(s), the port 22`
    before this change, and neither row after it.
18. **C's `goto`s and labels are labelled blocks** (PORTING.md 2.2, `keyboard.rs` 2841's shape):
    `sfnt_decompose_compound_glyph`'s `skip_computation` (C 2872), `error_in_defer_offsets`
    (C 3103) and `skip_computation_from_defer_offsets` (C 3128), `sfnt_decompose_glyph_1`/`_2`'s
    `continue_loop` (C 3273/3413) and `exit` (C 3291/3431), and `sfnt_decompose_glyph`'s
    `early`/`fail` pair (C 3581/3588).  C's label tails are one copy after each block, and the
    two `break`s out of the `defer_offsets` nest are C's own forward jumps; the `goto` token
    cannot survive (the four rows Owed 3 owes).
19. **C's uninitialised locals are zeroed** (PORTING.md 1.5, `android.md`'s Deviations): C 2768's
    `size_t point UNINIT, point2 UNINIT, index;`, C 2765's `need_free` (written through
    `get_glyph`'s `bool *`), C 2774's `sub_metrics` (written through `get_metrics`), C 2771-2772's
    `x_base`/`y_base`/`flags_base`/`contour_base` (written through
    `sfnt_expand_compound_glyph_context`'s out-parameters) and `sfnt_read_glyph`'s `glyph`
    (C 2420, `memcpy`'d and then written field by field; the `memcpy` covers only up to `ymax`).
    Every one passed by `&raw mut` measured `error[E0381]` before the initialiser.  The `uninit`
    row of `sfnt_decompose_compound_glyph` is Owed 3.
20. **Two `size_t` wraps the port keeps.**  C 3200-3201's and C 3208's `last -= 1; here -= 1;`
    decrement `size_t` values that may be zero, and the C **relies on the wrap**: the following
    `i = here; while (++i <= last)` then starts its walk at 0 (a contour whose first point is off
    the curve and whose last is on it).  The port writes `wrapping_sub (1)`/`wrapping_add (1)`,
    where Rust's debug-build overflow check would turn that path into a panic; the loops are C's
    `while (++i <= last)` with the increment in the condition, the `chartab.rs` 533 shape.
21. **Line count**: slice 3 is C 1,187 lines (C 2408-3594) against the port's +1,302
    (2,811 → 4,113, +9.7%): the per-item gates, the C-line comments, the `as` casts C's implicit
    conversions become (the two `offsets[…] * 2` promotions, the sign extensions of
    `argument1.b`/`d`, `last_point + 1` at `int` width), the `.offset (i as isize)` derefs, the
    `#[cfg]` of the new use-site block.  The whole-file drift falls from 86.7% to 80.6%.
22. **C's tentative definition of `build_outline_context`.**  C 3610's `static struct
    sfnt_build_glyph_outline_context build_outline_context;` has no initialiser - i.e. zero - and
    Rust needs one on the item, so the port writes `unsafe { std::mem::zeroed () }` with a
    `(tentative)` note (the `android.rs` 458 reading of the same C shape).  The struct itself
    (C 3596-3606) is `#[repr(C)] #[derive (Clone, Copy)]`, the shape every other record in
    `sfnt.h` has.
23. **C 4070's `alloca` is the port's `sa_stack_alloc`.**  The port has no `alloca` at all
    (PORTING.md P-05; `lib/include/alloca.in.rs` carries no name), so the small-outline arm goes to
    the stack pool `AVAIL_ALLOCA!`/`SAFE_ALLOCA!` allocate from - the shape `alloc.rs` 7936,
    `pdumper.rs` 2241, `eval.rs` 1598 and `font.rs` 2530 already use.  C's `edges_alloc` stays
    `NULL` in that arm, so C 4166's `xfree (edges_alloc)` is the same no-op, and the pool's release
    rule stays `alloc.rs`'s (C's frame-local `alloca` has no explicit free either).  The C token
    `alloca` therefore cannot reappear, which is the `- function sfnt_build_outline_edges does not
    mention alloca` row of Evidence ③ (Owed 3), and the TEST arm's side of the same gap is Owed 9.
24. **C 4322's `#ifndef NDEBUG` is `feature = "no_ndebug"`.**  The block - `row_end` (C 4323) and
    the four `eassert`s at C 4367/4385/4414/4428 - is *dead text* in the frozen configuration,
    which **defines** `NDEBUG` (`docs/baseline/config/darwin-arm64.macros.tsv` 2892, the same
    measurement `docs/ports/include-conf_post.md` 39 records), so P-51's rule applies and the
    feature is named after the non-frozen side (the `no_signbit`/`no_rint` shape).  C's `#else` is
    empty, so there is no second arm to carry.  **The name is not registered** -
    `docs/baseline/feature-coverage.tsv` and `xtask/Cargo.toml` are outside this lane's
    write set - which is Owed 8.
25. **`xnmalloc`'s two arguments are cast to `isize`.**  C 4072 passes `outline->outline_used` (a
    `size_t`) and `sizeof *edges` to `xnmalloc`, whose live prototype is `lisp.h` 5544's
    `void *xnmalloc (ptrdiff_t, ptrdiff_t)` - reached through C 138's include in the non-TEST arm -
    while C's own TEST-arm definition (C 98-105) takes `size_t`.  The port's single text follows
    the live arm and casts, the tree's spelling at every other `xnmalloc` call site (`emacs.rs`
    2308, `profiler.rs` 112); the TEST-arm half of that difference is Owed 9.
26. **C's chained assignments and its `*p++` are split, in C's order.**  C 3662-3663's
    `outline->xmin = outline->xmax = x;` and `ymin`/`ymax` twin become two statements each in C's
    right-to-left order (`composite.md` 139's rule), C 4452's `x0 = x1 = 0;` and C 4476's
    `x0 = x1 = edge->x;` likewise, and C 4400/4418/4433's `*start++ = sfnt_saturate_short (a)` is
    the load-then-step pair (`alloc.rs` 257's `*p = …; p = p.offset (1);`).
27. **C's `for`-loop increments are folded into each `continue`.**  C 4081's
    `for (i = 0; i < outline->outline_used; ++i)` has three `continue`s (C 4092, 4096, 4144), and
    Rust's `continue` would skip a trailing `i += 1`, so the port writes `{ i += 1; continue; }` at
    each of them plus the increment once at the loop's end - the `dispnew.rs` 4644 shape.
    `sfnt_edge_sort`'s and `sfnt_poly_edges`'s loops have no `continue` and keep the plain form.
28. **C's `long long int` arithmetic is spelled with the fixed-width aliases.**  C 3678-3714,
    3719-3733 and 3772-3776's `unsigned long long int`/`long long int` products and quotients are
    the port's `uint64_t`/`int64_t` with the conversions C performs implicitly (each operand
    widened by `as`, the result narrowed at the `return`); C 3783/3791's `0177777`/`037777600000`
    are `0o177777` and `0o37777600000`, the second applied at `uint32_t` width because C's octal
    constant has type `unsigned int` (`sfnt_fixed` is signed).  C's no-op `(sfnt_fixed)` casts are
    kept as `as sfnt_fixed` (C 3667-3670's `MIN`/`MAX` arguments, C 3968's `origin` promotion).
29. **`abs` is a use-site declaration.**  C 34's `#include <stdlib.h>` is mirrored by
    `lib/include/stdlib.rs`, which carries `malloc` and friends but **neither** `abort` nor `abs`;
    slice 4 is the first text here to call `abs` (C 3712-3713, C 3812, C 4134), so C's prototype
    joins `abort`'s block at C 34's site (P-88 ①, Deviation 15's shape).
30. **Two C locals the port zeroes, and three `mut` parameters.**  C 4226's
    `struct sfnt_edge *active, **prev, *a, *n;` has `n` read in the resort loop's condition
    (`(a = *prev) && (n = a->next)` at C 4283), which the port's `while { a = *prev; !a.is_null ()
    && { n = (*a).next; !n.is_null () } }` cannot hand rustc's initialisation analysis - measured
    `E0381` - so `n` is zeroed (PORTING.md 1.5).  `sfnt_curve_to_and_build_1`'s three parameters
    (C 3823-3825), which C passes by address to `sfnt_lerp_half` (`&control0 …`, C 3844-3846),
    need `mut` bindings for `&raw mut`; the same applies to `sfnt_fill_span`'s `x0`/`x1`
    (C 4314-4315, written at C 4329/4335).
31. **Line count**: slice 4 is C 949 lines (C 3595-4543) against the port's +1,068
    (4,113 → 5,181, +12.5%): the per-item gates, the C-line comments, the `as` casts C's implicit
    conversions become, the `.offset (i as isize)` and `&raw mut` spellings, the `while { … }`
    conditions, the four `#[cfg (feature = "no_ndebug")]` blocks and Rust's brace-only lines.  The
    whole-file drift falls from 80.6% to 75.6%.
32. **Three of C's own arithmetic edges are carried, not smoothed over** (the reference's bugs are
    kept, per P-86 ①): the reference's `int` overflow at C 4334 (`x1 > raster->width * 65536`
    promotes the `unsigned short` to `int`, so a width above 32767 is undefined in C and wraps in
    practice) is the port's plain `(*raster).width as c_int * 65536`, which a *debug* build would
    panic on where C wraps -- no reading here can see it (nothing runs) and `wrapping_mul` would
    be a behaviour C does not have; C 4143/4149's `y >= edges[edge].top` compares a `size_t`
    against a signed fixed value (so a negative `sfnt_poly_grid_ceil` result wraps to a huge `y`
    and the edge survives), carried as the two conversions C performs, `as size_t` and
    `as sfnt_fixed` at C 4159; and C 3849/3852's recursion in `sfnt_curve_to_and_build_1` has no
    depth limit, which C's own comment ("Maybe apply a recursion limit here?") acknowledges - the
    port keeps the recursion as it stands.

33. **C 4547-4554's three macros are `macro_rules!`.**  `sfnt_add`/`sfnt_sub`/`sfnt_mul` are
    `((int) ((unsigned int) (a) … (unsigned int) (b)))` - `unsigned int` arithmetic whose wrap C
    relies on - so the port writes `wrapping_add`/`wrapping_sub`/`wrapping_mul` around C's two
    casts (PORTING.md 1.1), and each argument is evaluated once, as in C.  They have no call site
    until the interpreter lands (which is why no use moved), and `unused_macros` is allowed at the
    crate root.
34. **C's statement macro `APPEND_STEP` keeps its semicolon.**  C 4959-4960's body is
    `sfnt_insert_raster_step (&raster, x, coverage, y);` *including* the `;`, and every call site
    adds another; the port's `macro_rules!` rule ends with `;` too, so `APPEND_STEP! (…)` expands
    to a statement whose extra `;` is Rust's empty statement.  C's implicit conversions at that
    call - the `float` `x` of C 5157/5231's loop into the `int` parameter, and the `int` `y` into
    `size_t scanline` - are the `as c_int`/`as size_t` the macro body carries (measured:
    `3 × E0308` without them, Evidence ②).
35. **The `#undef`s have no Rust text.**  `TRIANGLE_AREA` (C 5083, `#undef` at C 5262),
    `APPEND_STEP` (C 4959, `#undef` at C 5265) and `TRAPEZOID_AREA` (C 5011, `#undef` at C 5266)
    are `macro_rules!` inside the enclosing block, and Rust's macros are textually scoped: the
    block's end *is* the `#undef`.  The three sites carry a comment naming the C directive.
36. **`struct sfnt_fedge`'s two shared C declarations are one field per line.**  C 4713's
    `float x, top, bottom;` and C 4716's `float step_x, step_y;` declare four fields the port
    needs, but `tools/extract-abi.py`'s field check reads `pub name: type,` lines: written the way
    C groups them it reported `- struct sfnt_fedge: missing fields ['top', 'bottom', 'step_y']`.
    The layout is identical either way; the split is recorded here because the inventory reading is
    what found it (Evidence ③), and because it forced the fourth `probe` run (Owed 11).
37. **C's two `goto`s are labelled blocks** (PORTING.md 2.2): `sfnt_insert_raster_step`'s
    `goto found`/`found:` (C 4857/4881) and `sfnt_poly_edges_exact`'s `goto next`/`next:`
    (C 4984/5275).  The second wraps a region C indents at columns 10-14, so that region is carried
    at C's columns **+2** (the `'fail`/`'exit` shape of slice 3) - its interior comment art
    therefore sits two columns right of C's, the only place in five slices where a carried
    comment's leading whitespace is not C's own.  The `next:` tail stays at C's columns.
38. **C's `switch (n)` ladder in `sfnt_poly_set_steps`** (C 5327-5354) is a `match` whose arms
    carry the statements of the arms below them, with C's `FALLTHROUGH;` kept as `FALLTHROUGH! ()`
    at each site - one copy per entry point, the `regcomp.rs` 2044/2080 shape and the rule
    `docs/ports/etags.md` 414 states - so the ladder is spelled seven arms deep; `case 0:`'s
    `break` is the arm's end and `default:` is `_`.  The `switch`/`case`/`default`/`break` tokens
    have no Rust site (Owed 10).
39. **This slice adds no use-site declaration.**  The two new external names it calls are already
    in this unit's include graph: `sfnt_fixed_float` (the mirror's `f32` function for C 126's macro,
    `src/include/sfnt.rs` 192) and `xcalloc` (`src/include/lisp.rs` 3984's `(usize, usize)`
    declaration, reached through C 138's `lisp.h`; C 5439's `xcalloc (1, size)` needs no cast).
    `memset` (C 5353) is `lib/include/string.rs` 24's and `FALLTHROUGH` (C 5331 etc.) is
    `lib/include/attribute.rs` 154's no-op macro, whose token therefore survives at its sites.
40. **Line count**: slice 5 is C 908 lines (C 4544-5451) against the port's +1,070
    (5,181 → 6,251, +17.8%): the per-item gates, the C-line comments, the ladder's repeated arms,
    the `'next` block's extra level, the `.offset (…)`/`&raw mut` spellings and the `as` casts
    C's implicit conversions become.  The whole-file drift falls from 75.6% to 70.5%.
41. **Three of C's own arithmetic edges are carried, not smoothed over** (the reference's bugs are
    kept, per P-86 ①, the slice-4 Deviation 32's class): C 4912's `sfnt_floor_fixed (MAX (0,
    edges[0].bottom))` reads `edges[0]` **before** any `nedges` test - the function only checks
    `height`, so a `nedges == 0` call reads past the array, and this port keeps it as written
    (`(*(edges.offset (0))).bottom`; the only protection is `sfnt_build_outline_fedges`'s
    `if (edge)` gate at C 4823); C 4922's `for (; y != height; y += 1)` compares an `int` with a
    `size_t`, so a negative `y` (which `sfnt_floor_fixed` can produce) becomes a huge value and the
    loop would run `2^64 - height` times - carried as `(y as size_t) != height`; and C 5008-5009's
    `(int) (x_min)`/`(int) (x_max)` truncate *toward zero*, so `x_pixel_min`/`x_pixel_max` are not
    floors (the trapezoid arithmetic of C 5019-5260 compensates), carried as `as c_int`.

42. **C's scalar `sfnt_glyph` is the port's `sfnt_glyph_index` (P-67).**  C 5542's
    `sfnt_glyph glyph` parameter is C 128's scalar typedef (`typedef unsigned int sfnt_glyph`),
    which the tree renames because C gives the same spelling to the *record* at C 661
    (`include/sfnt.rs` 852); spelling it `sfnt_glyph` in the port's signature therefore named the
    record and produced `2 × E0369` (`<` on a struct), `1 × E0369` (`glyph - …`) and
    `2 × E0605` (`as isize`) - Evidence ②'s control reading.  `docs/ports/include-sfnt.md` owns the
    rename; this is its second body-level cost (slice 2's `sfnt_lookup_glyph_4` row was the first).
43. **C's `MAYBE_UNUSED TEST_STATIC` is the port's `cfg_attr` alone.**  Three of this slice's
    readers - `sfnt_scale_metrics` (C 5581), `sfnt_get_scale` (C 5597) and `sfnt_find_metadata`
    (C 5922) - carry both attributes; the port keeps the linkage half
    (`#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`) and drops `MAYBE_UNUSED` (the
    fileio.rs 273 reading of the attribute).  Unlike the `alloca`/`goto` class, this produced **no**
    `does not mention` row (Evidence ③), because the tool does not track the attribute's name.
44. **Two spellings of C's "read returned short" test, and `offsetof`.**  C 5507/5656/5690/5748/
    5830/5966/5996/6019 write `rc == -1 || rc < size` (an `ssize_t` against a `size_t`, so C
    converts the *signed* side up; the port writes `rc == -1 || (rc as size_t) < size`), while
    C 5880/6119/6171/6218 write `rc < directory->length` (a `uint32_t`, which `ssize_t` can hold,
    so that comparison stays *signed*; the port writes `rc < (*directory).length as ssize_t`).
    Both are C's own arithmetic, not a port choice.  C 6015-6017's `offsetof (struct sfnt_ttc_header,
    ul_dsig_tag)` is `std::mem::offset_of! (sfnt_ttc_header, ul_dsig_tag)`, the same `sizeof`
    family's member the mirror's `SFNT_ENDOF!` uses.  This slice adds no use-site declaration: every
    name it calls (`sfnt_find_table`, `xaddmalloc`, `xaddrealloc`, `xmalloc`, `xfree`, `read`,
    `lseek`, `SEEK_SET`, `memcpy`, `sfnt_swap16!`/`sfnt_swap32!`, `SFNT_ENDOF!`, `memory_full_up`,
    `MAX!`) is already in this unit's include graph.
45. **Line count**: slice 6 is C 852 lines (C 5455-6306) against the port's +940
    (6,251 → 7,191, +10.3% over the C range: the per-item gates, the C-line comments, the four
    `ckd` write-backs, the `.offset (…)` derefs and the `as` casts C's implicit conversions
    become).  The whole-file drift falls from 70.5% to 66.1%.  This slice also **removes the one
    stray form feed slice 5 carried** (C's carriage-return-class artifact: the port's convention is
    to drop `\f` lines, and one slipped through the slice-5 splice; the file now has none, and no
    text changed).

46. **C's implicit conversions, made explicit.**  Every widening C relies on is an `as` in this
    slice, and the two that the compiler refused are the two the arm flip read as `2 × E0308`
    (Evidence ②): `versor = interpreter->state.freedom_vector.x`/`.y` (C 10475/10501) is
    `… as sfnt_f26dot6`.  The rest are the classes the earlier slices established: the two
    `sfnt_dot_fix_14` call sites pass `sfnt_f2dot14` into C's `int` parameters (C 10389-10390,
    C 10408-10409, `as c_int`); `gs->vector_dot_product` is written from an `int` expression
    twice (C 10608-10611's `sfnt_short_frac_dot (…) + sfnt_short_frac_dot (…)`, whose two halves
    are `as c_int` first so the sum is C's `int` sum and not an `i16` one, and C 10618-10619's
    `? -0x4000 : 0x4000`), both `as sfnt_f2dot14`; the two `sfnt_move` calls narrow their
    `sfnt_f2dot14` product `as c_int` (C 10486-10488, C 10508-10510, the callee's third
    parameter); `sfnt_short_frac_dot` casts both operands `as c_long` and its result
    `as sfnt_f2dot14` (C 10529); and the readers' `uint16_t`/`uint32_t` fields are `as size_t`
    wherever C compares or sizes with them (C 13963, C 14009-14015, C 14041-14057, C 14072-14078,
    C 14095-14105, C 14133, C 14143, C 14220-14232, C 14373, C 14404, C 14491, C 14504), with
    `UINT16_MAX as uint16_t` (C 14481), `*buffer … as sfnt_f2dot14` (C 14428-14429) and
    `first as uint16_t` (C 14526/14540) for the three narrowings.

47. **`alloca` → `sa_stack_alloc`, and `alignof` → `std::mem::align_of`.**  C 14096's
    `buffer = alloca (fvar->instance_size)` is P-05's stack pool, the same spelling slice 4 used
    for C 4070 (no `alloca` token can exist in the port; Owed 14's row records it).  C 14045's
    `pad = alignof (struct sfnt_instance)` is `std::mem::align_of::<sfnt_instance> ()`, the
    spelling the include mirrors' own layout assertions use; C 14046's `pad -= min_bytes &
    (pad - 1)` is unchanged arithmetic.

48. **C's `goto bail` / `goto bail1` / `goto fail` are labelled blocks** (PORTING.md 2.2, the
    `sfnt_decompose_glyph` shape of slice 3): each `'label: { … }` encloses the code that can jump,
    every `goto label` is `break 'label`, and the C label's own tail is written *after* the block,
    where the `break` lands.  `sfnt_read_fvar_table`'s single `bail` (C 13977-14154) and
    `sfnt_read_packed_points`'s (C 14509-14553) are one block each; `sfnt_read_gvar_table`'s
    (C 14198-14317) likewise; `sfnt_read_avar_table` has two, and C 14438-14440's `bail1:` tail
    **falls through** into `bail:`'s, which the port writes as the `break 'bail` that ends the
    `'bail1` tail (`xfree (buffer)` then `xfree (avar)`).  The `goto` token cannot survive, which
    is why four of this slice's five `does not mention` rows name it.  **P-91 ④ is empty for this
    slice**: no ported call site here names a `static` whose definition is still missing.

49. **C's `restrict` is dropped** at C 10418/10438/10457 (`sfnt_f26dot6 *restrict x`) and
    C 14461-14464 (`unsigned char *restrict data`/`end`, `unsigned char *restrict *location`):
    Rust has no spelling for the qualifier, and the function-pointer type these functions are
    stored in (`sfnt_graphics_state.move`, mirror C 1814-1817) is the qualifier-free one.

50. **`(uint16_t *) -1`** (C 14483, `sfnt_read_packed_points`'s no-points return) is
    `usize::MAX as *mut uint16_t`, the third use site of the spelling Owed 2's
    `docs/ports/token-aliases.tsv` row names (after C 262/284's two `(void *) -1`s in slice 1).

51. **A defect in slice 3's landed text, found here and *not fixed here*** (Owed 13):
    `sfnt_decompose_glyph`'s `goto fail` (C 3566/3575) is `break 'fail`, but the `'fail` block is
    the *last* thing inside `'early`, so the break leaves `'early` and runs the `early:` tail -
    `return 0` - where C 3588-3593 returns **1**.  Both C paths do the same four `xfree`s, so the
    difference is invisible to the compiler and to every gate; a 12-line standalone `rustc` program
    of exactly that shape (`'early: { … 'fail: { break 'fail; } }` with both tails after it) prints
    `0` for the `break 'fail` path, which is the measurement.  `sfnt_decompose_glyph` reports a bad
    contour index (C 3565) and a failed `sfnt_decompose_glyph_2` (C 3574) through that jump, so the
    port reports success on both.  The fix belongs to whoever owns C 3470-3594's text (give
    `'fail` a tail that is not `'early`'s, e.g. a `return 1;` in the `'fail` block); this lane's
    write set is its own two ranges plus this ledger.

52. **`sfnt_validate_gs`'s `switch` is a `match`, with `_ => {}` for the missing `default:`**
    (PORTING.md 2.2, the slice-1 `sfnt_read_cmap_table_1` precedent): C 10543's eight `case`s are
    the port's eight arms in C's order (5, 0, 1, 2, 4, 3, 6, 7) and C's empty fall-through tail is
    the `_ => {}` arm Rust requires for exhaustiveness.  The `break,case,switch` row this owes
    `docs/ports/macro-body-notes.tsv` is Owed 14.

53. **C's four pre-write locals in the two appliers are zeroed** (PORTING.md 1.5, Deviation 3's
    class; measured: the arm flip's first run reports **8 × E0381**, Evidence ②).  `tuple` (read by
    `data_offset += tuple - data` at C 15572/15941, where C passes `&tuple` as an out-parameter at
    C 15562/15931), `data_size` and `index` (the tuple header's two `memcpy` destinations, whose
    values `sfnt_swap16` then reads in place at C 15608-15609/15969-15970) and `n_local_points`
    (the out-parameter at C 15683/16044) in `sfnt_vary_simple_glyph`/`sfnt_vary_compound_glyph`.
    `sfnt_read_cvar_table`'s `data_size`/`index` (C 14637) are zeroed for the same reason even
    though rustc's analysis accepts them (they are written by the `memcpy` before any value is
    read); in every case the read follows a full write, so the zero is unobservable.

54. **C's `int`-wide sums that narrow back are widened and wrapped.**  `distortion->origin +=
    sfnt_mul_fixed_round (…)` (C 15732-15733, 15789, 15795, 16116-16117, 16135, 16141) is the
    port's `(*distortion).origin = ((*distortion).origin as c_int).wrapping_add (…) as sfnt_fword`:
    C adds at `int` width and converts back into the `sfnt_fword` (int16) member, and
    `wrapping_add` is that machine arithmetic with no Rust overflow check; `interpreter->cvt[i] +=
    delta` (C 16271) is `(*(*interpreter).cvt.offset (i as isize)).wrapping_add (delta)` for the
    same reason (both operands are already `sfnt_f26dot6`).  The `x_coordinates[i] += fword`,
    `d1 = (x_coordinates[..] - x[..])` and `position - max_pos` sites keep the plain form with
    `as c_int` on each `int16_t` operand, because two `int16_t`s cannot overflow an `int32_t`
    (Deviation 9's cast class); `delta = sfnt_sub! (max_pos, min_pos)` is `c_int` already.

55. **`sfnt_multiply_divide` returns `c_uint`.**  C 15163/15167 assign it into the `sfnt_fixed
    scale`; the port writes `as sfnt_fixed` at both sites (Deviation 9's cast class).
    `sfnt_multiply_divide_signed` takes and returns `c_int`, so its three call sites (C 15068,
    15150, 15226-15229's pair is `sfnt_div_fixed`) need no cast.

56. **`sfnt_vary_interpreter`'s `cvt[i] += delta` keeps C's `i`** (Owed 19): C 16271 indexes the
    control value table with the *tuple* index `i`, not the CVT index `index` it has just computed
    at C 16259 — and C 16252's `ndeltas`/C 16259's `index` exist only for the loop bound and the
    bounds test.  It is C's text, copied as it stands and recorded so that a later reviewer does
    not "fix" it into a behaviour change.

57. **C's mid-function declarations sit at C's positions.**  `void *coords_alloc` (C 15580/15949)
    and `void *touched_alloc`/`original_x_alloc` (C 15594-15595) are `let mut …` at exactly those
    lines of the port, which Rust allows (the C 3644 `size_t size;` precedent in this unit's own
    slice-4 text, where a `let mut size: size_t;` also stands mid-body).  No initialiser moves and
    no declaration is hoisted.

58. **The `bail`/`fail` cascades are nested blocks.**  C's `goto bail2` lands on `bail2:` (frees
    `points`), which falls into `bail1:` (frees `buffer`), which falls into `bail:` (frees `cvar`);
    the port expresses that as `'bail: { … 'bail1: { … 'bail2: { … return cvar; } if points … }
    xfree (buffer) } xfree (cvar)`, so every `break` lands on the same cleanup text C's three
    labels do (PORTING.md 2.2, the `sfnt_read_avar_table` precedent).  The two appliers use the same
    shape one level deeper — `'fail1: { 'fail3: { <loop> <success tail> } <fail3's three frees> }`
    — with `'fail1` *outside* `'fail3`: `break 'fail1` (C's `goto fail1`, nine sites in
    `sfnt_vary_simple_glyph` and eight in the compound twin) skips C 15837-15840/16201-16204's
    three `xfree`s, while falling out of `'fail3` runs them and then C 15841/16205's `fail1` tail —
    C's fall-through, with no cleanup text duplicated and no `goto` left.

59. **`while (ntuples--)` keeps C's decrement-before-test order.**  C 14720, 15597 and 15958 are the
    port's `while { ntuples = ntuples.wrapping_sub (1); ntuples != -1 }` (size_t: `!= SIZE_MAX`), so
    the body sees the decremented value and the loop runs once per original unit, exactly as C's
    post-decrement does.  The `for`-loop equivalents keep the `while i < ….as c_int { …; i += 1; }`
    shape, with `i += 1; continue;` at each `continue` (Deviation 27's rule) — the sites are
    C 15226's and C 15818's `continue`s (`sfnt_infer_deltas`'s `i` still gets its plain `i += 1` at
    the loop's end), and in the appliers C 15673, 15790, 15796, 15801, 16034, 16137, 16143, 16147.

60. **C's ternaries and boolean arguments are `if`/`else` expressions and comparisons.**
    `ndeltas = (variation->points ? variation->num_points : interpreter->cvt_size)` (C 16252-16254)
    and `index = variation->points ? variation->points[j] : j` (C 16259) are the port's
    `if !(*variation).points.is_null () { … } else { … }` with `as size_t` on the `uint16_t` arm;
    C's `index & 0x4000` boolean argument (C 15668/16029) is `(index & 0x4000) != 0`; and C's
    `!scale` (C 15672/16033/16245) is `scale == 0`.

61. **`(void *) -1` / `(uint16_t *) -1` / `(size_t) -1`.**  C 14937, 14943, 15818, 15828, 15848,
    16185, 16192 and 16208's pointer tests are `!= (usize::MAX as *mut uint16_t)` (Deviation 4's
    spelling; `sfnt_read_packed_points`' no-points return, Owed 2's row), and C 15368/15397's
    `SIZE_MAX`/`(size_t) -1` pair is the port's `SIZE_MAX` (`include/stdint.in.rs` 244 spells it
    `usize::MAX`).

62. **The record `sfnt_gvar_glyph_header` is defined in the unit, not the mirror** (Owed 20).  C
    15078-15088 declares it in the `.c`; the port defines it in `src/sfnt.rs` under the file
    gate with `#[repr(C)]`, `Clone, Copy` and `pub` fields in C's order, so the mirror carries
    nothing (`check-mirror-dupes` accordingly sees no new duplicate).  Its two `uint16_t` fields
    have C's alignment; the frozen arm cannot measure it (nothing compiles the file) and the arm
    flip only type-checks it.  The first `build-tu` reading's two `E0425`s were this item's: the
    record needs the file gate like every other item in this file.

63. **`(signed char) *data++` is `*data as int8_t as sfnt_fword`.**  C 14605 sign-extends the byte
    into `sfnt_fword deltas[i]`, where `sfnt_read_packed_points`' own byte walk (C 14519, 14534)
    reads the same byte *unsigned*; the port spells the two differently so the sign extension is on
    the page (the `unsigned char`/`signed char` class the `sfntfont_android_*` rows record), and
    C 14596's two-byte value is `((*data as c_uint) << 8u32) as uint16_t` for the same reason.

64. **The C 6559-8159 macros are ported *in their callers*, not at their C position** (P-57): C's
    `TRAP` (C 6562), `POP` (C 6597, the live `#ifndef TEST` arm), `PROJECT` (C 7777) and
    `DUAL_PROJECT` (C 7780) are `macro_rules!` defined inside every function of this slice that
    calls them (26 copies: 20 `TRAP`, 3 `POP`, 2 `PROJECT` and 1 `DUAL_PROJECT`, one per using function), because each body names that function's
    `interpreter` parameter and a `macro_rules!` resolves local names at its **definition** site.
    Every call site keeps C's own name and argument list (`TRAP! (…)`, `POP! ()`,
    `PROJECT! (x, y)`, `DUAL_PROJECT! (x, y)`), which is what the `coding.rs`/`ccl.rs` precedents
    do and why the whole C 6559-8159 region is not carried where C writes it.  The ~130
    instruction macros (C 6671-8159) have **no Rust text yet**: their only caller is
    `sfnt_interpret_run` (C 11396), and a `macro_rules!` body cannot `goto` the caller's
    `skip_step`/`next_instruction`/`loopcall_begin`/`deltac*_start`/`deltap*_start` labels either,
    so that slice has to carry them together with the labels they jump to (Owed 22).
65. **Four dead `#if 0` regions are recorded, not carried** (the `android.md` 1125-1360 /
    `androidfns.md` 1015-1022 precedent; `tools/cinventory.py` skips `#if 0` unconditionally):
    C 8460-8487 (the whole `sfnt_line_to_standard_form` definition), C 8983-8990 (the
    `determinant`/`a0`/`b0`/`a1`/`b1`/`c0`/`c1`/`px`/`py` declaration run of
    `sfnt_interpret_isect`), C 8992-9034 (that function's Cramer's-rule block, which calls the
    dead `sfnt_line_to_standard_form`) and C 9065-9071 (the two `sfnt_div_fixed (dx, determinant)`
    / `(dy, determinant)` arguments the live `#else` arm replaces with `dx, dy`).  Dead in every
    configuration, so no gate can reach them; the live arms stand in C's own order.
66. **`sfnt_normalize_vector`'s `goto fail` enters a block** (C 9172's `fail:` label sits *inside*
    C's first `if`, and C 9208's `goto fail` jumps into it from below).  A Rust labelled block
    cannot be entered from outside, so the port puts the label's three statements after the
    `'fail: { … }` block, makes C's first `if` `if vx == 0 && vy == 0 { break 'fail; }` (same
    condition, same position, C's fall-through into the label spelled as the break), and gives the
    success path an explicit `return;` where C falls off the end of the function.  The C
    `goto`/`fail:` tokens have no Rust site, which is the `sfnt_normalize_vector -> goto` row of
    `docs/ports/macro-body-notes.tsv`.  Nothing else in the body moves, and the values are the
    same (the tail's `04000`/`0` and the two `sfnt_div_fixed` quotients).
67. **The 56 out-parameters C leaves uninitialised are zeroed** (PORTING.md 1.5, Deviation 3's and
    19's class): `x`/`y`/`x_org`/`y_org` in `sfnt_interpret_scfs` (C 8782), `sfnt_interpret_miap`
    (C 8831), `sfnt_address_*`'s six parameters at every call site, `rp0x`/`rp0y` in
    `sfnt_interpret_alignrp` (C 8920), the four points of `sfnt_interpret_alignpts` (C 8949-8950),
    the eight of `sfnt_interpret_isect` (C 8978-8980), the eight of `sfnt_line_to_vector`
    (C 9233-9235), the eight of `sfnt_measure_distance` (C 9282-9283), the six of
    `sfnt_interpret_msirp` (C 9315-9317), the eight of `sfnt_interpret_ip` (C 9371-9376) and
    `px`/`py` in `sfnt_interpret_mdap` (C 9671).  Every one is written by the callee before it is
    read, so the zero is unobservable; the *compiler* sees every one of them (42 `&raw mut` uses,
    measured as the `E0381`s of Evidence ②'s runs (i) and (iii)).
68. **C's `while (interpreter->state.loop--)` keeps C's decrement-before-test order** as
    `while { (*interpreter).state.r#loop = (*interpreter).state.r#loop.wrapping_sub (1);
    (*interpreter).state.r#loop != 0xffff }` (Deviation 59's shape, at the unsigned short's own
    width: the field is `c_ushort`, so C's wrap target is `0xffff`, not `-1`).  The four sites are
    `sfnt_interpret_flippt` (C 8743), `sfnt_interpret_alignrp` (C 8926), `sfnt_interpret_ip`
    (C 9424) and `sfnt_interpret_if`'s `nifs` is *not* one of them.  `sfnt_interpret_call`'s
    `interpreter->call_depth++ >= 128` (C 9847) reads the value before the increment through the
    `let call_depth_old: c_int = …; (*interpreter).call_depth += 1; call_depth_old` block the
    `bidi.rs` 1605 precedent uses.
69. **C's `longjmp` is a use-site declaration at C 37's site** (P-88 ①, the mman block's shape):
    this tree has no `<setjmp.h>` mirror, and C 6556's `longjmp (interpreter->trap, 1)` is its
    first *plain* `longjmp` (`image.c`'s two calls go through C's own `sys_longjmp` spelling).
    **Measured, not assumed**: a `setjmp`/`longjmp` program on this target links
    `_setjmp`/`_longjmp` in `nm -u`'s notation — the two plain functions, not the
    `_setjmp`/`_longjmp` pair `lisp.rs` 3618/3623's `sys_*` wrappers wrap — and
    `sizeof (jmp_buf)` prints **192**, the width `lisp.rs` 1068 declares.  The port's declaration
    is `pub fn longjmp (env: *mut jmp_buf, val: c_int) -> !;` (C's `__dead2` is the port's `-> !`),
    so C's name and C's argument survive; the alternative (`sys_longjmp`, which is `_longjmp`) is a
    *different* function — it does not restore the signal mask — and Owed 24 asks the integrator to
    confirm the choice.
70. **C's 8-bit and `long`-width arithmetic is spelled at its own width.**  `operand >> 4` and
    `operand & 0xf` on a `c_uchar` are `(operand >> 4) as c_int` / `match operand & 0xf` (C's
    promotion to `int`), `ppem` is `c_int`, and `delta *= 1l << (6 - delta_shift)` (C 9635/9797,
    `long` times `int` narrowed back to `int`) is
    `delta = (delta as c_long * ((1 as c_long) << (6 - delta_shift as c_int))) as c_int` —
    Deviation 28's fixed-width spelling.  `sfnt_sqrt_fixed`'s `rem_lo = n` (C 9134, `int32_t` into
    `unsigned int`) and its `return root` (`unsigned int` into `sfnt_fixed`) are `n as c_uint` and
    `root as sfnt_fixed`; `sfnt_normalize_vector`'s two `vector->x = (sfnt_div_fixed (…) / 4)`
    (C 9211-9212, `sfnt_fixed` into `sfnt_f2dot14`) carry `as sfnt_f2dot14`.  C's `vx = vx * 2`
    (C 9187-9188) stays the plain form: it is UB in C for the values that reach it and the port
    does not invent a wrap C does not have (Deviation 32's rule).
71. **C's implicit conversions, made explicit** (Deviation 9's class, 40 new sites): the two
    `sfnt_div_fixed (pixel_size * 64, head->units_per_em)` arguments (`units_per_em as sfnt_fixed`,
    measured as Evidence ②'s one `E0308`), `max_stack_elements`/`max_twilight_points`/`max_storage`/
    `max_function_defs`/`max_instruction_defs` and `num_elements` into `size_t` products,
    `(*maxp).max_instruction_defs as c_int + (*maxp).max_function_defs as c_int` before the
    `as size_t` widening (C's `(int)` sum), `cvt as usize`/`index as usize` against
    `interpreter->cvt_size`, `p as usize` against `num_points`, `number as uint32_t` against
    `twilight_zone_size`, `state.rp0/rp1/rp2 as uint32_t` into the addressers and `p as uint16_t`
    back into them, `byte as c_int` at `sfnt_skip_code`'s five `IP`/`nbytes` computations,
    `instructions[IP] as c_int`, the `opcode as c_int` narrowing of the two push counts, the
    `cvt->values[i] as sfnt_f26dot6` CVT load (C 6497, `int16_t` into `sfnt_f26dot6`) and the
    `x_coordinates[…]/y_coordinates[…] as sfnt_f26dot6` reads of `sfnt_interpret_ip`.
72. **`TRAP`'s argument is `c"…".as_ptr ()`.**  C's `TRAP ("…")` passes a string literal, which
    reaches `const char *` through C's array decay; the port spells that decay with a C-string
    literal and `as_ptr ()` (the `alloc.rs` 2342 spelling, measured as the 42 `E0308`s of
    Evidence ②'s run (ii)).  The macro's *body* keeps C's own text
    (`sfnt_interpret_trap (interpreter, $why)`), so the conversion sits at the call sites, where
    C's decay is.
73. **`sfnt_interpret_utp`'s mask arithmetic.**  C 8217/8220's `mask &= ~SFNT_POINT_TOUCHED_X` is
    `mask &= !(SFNT_POINT_TOUCHED_X as c_uchar)` — `mask` is C's `unsigned char` (`c_uchar`), the
    constant is a `c_int`, and C's `~` at `int` width then narrowed is the same `0x7f` the
    `c_uchar` complement gives.  `sfnt_interpret_fliprgoff`/`_fliprgon`/`_flippt`'s
    `flags[i] &= ~01` / `|= 01` and `sfnt_store_zp2`'s `flags[number] |= flags` are
    `&= !0o1` / `|= 0o1` on the `u8` the mirror declares and `|= flags as u8`, the same bytes.
74. **`sfnt_set_srounding_state`'s two `case 3: default:` pairs are merged arms.**  C 9893-9894
    (`period`'s reserved-period trap) and C 9914-9915 (`phase`'s `period * 3 / 2`) each fall
    through, so the port writes one arm each (`_ => { TRAP! (…); }` and `_ => { phase = period * 3
    / 2; }`) with C's `case 3`/`default` keywords recorded in the `macro-body-notes.tsv` row;
    C 9898's `(((int) (operand & 0x0f) - 4) * period / 8)` keeps its `(int)` as `as c_int`.
75. **The `sfnt_interpreter_run_context` enum is a type alias plus four constants**
    (C 6533-6541): the `.c` defines it, so `sfnt.h`'s mirror carries neither the type nor the
    members (the `sfnt_gvar_glyph_header` precedent, Owed 27).  `pub type
    sfnt_interpreter_run_context = c_int;` with `SFNT_RUN_CONTEXT_FONT_PROGRAM` = 0,
    `_CONTROL_VALUE_PROGRAM` = 1, `_GLYPH_PROGRAM` = 2 and — behind C 6538's `#ifdef TEST`, under
    `feature = "test"` — `SFNT_RUN_CONTEXT_TEST` = 3, which retires the three constants the
    checker listed and leaves the TEST member for the arm that uses it.
76. **Line count**: slice 9 is C 2,310 lines (C 6312-6557 + C 8161-10224) against the port's
    +2,411 (10,763 -> 13,174, +4.4% over the C range) — the per-item gates, the C-line comments,
    the 26 in-function macro copies (P-57), the `&raw mut`/`.offset (…)` spellings, the zeroed
    out-parameters (no new lines) and the `as` casts C's implicit conversions become.  The
    whole-file drift falls from 49.3% to 37.9%, the largest single move since slice 2.
77. **`sfnt_interpret_call`'s `sfnt_interpret_run` call is the one P-91 ④ site** this slice leaves
    (C 9832 -> C 11396).  It is an *allowed* arm-flip reading, named with its C line, **not** a
    pass: Evidence ②'s sixth `E0425` is it, and it disappears when C 10664-13917 lands.  Nothing
    else in the two ranges names a definition that is not already in the file.

78. **Slice 10's four shapes.**  (a) **The `<time.h>`-mirror edge has no C line of its own.**
    C's `sfnt.c` names `time_t`/`struct timespec` because C 138's `lisp.h` reaches `thread.h`
    29-31's `<time.h>` (dead: inside `#ifdef MSDOS`) and `thread.h` 40's `sysselect.h`, and the
    *system* `<sys/select.h>`/`<sys/time.h>` supply both; the mirrors (`lib/include/sys_time.rs`
    44/90, `lib/include/sys_select.rs` 56, `src/include/thread.rs` 302/320) take them from
    `lib/include/time.in.rs`, and in this unit that file was reached only through C 165's
    `#ifdef TEST` include - so a non-`test` configuration had no `time_t` at all (the arm flip's
    five `E0425`s, measured).  The port writes the edge at the include that needs it (before
    `lisp.rs`), ungated, with the note at the site (P-56's class, the `intervals.rs` 80 and
    `sys_select.rs` 28 ordering); it must come *before* the `test`-gated copy, or the build's
    dedupe keeps the gated one and expands neither.  (b) **The five macros are in-function
    `macro_rules!`s** (`TRAP`, `POP`, the four point accessors and `IUP_SINGLE_PAIR ()`), because
    their bodies name the caller's `interpreter`, `opcode` and locals (P-57, the
    `sfnt_interpret_flippt` reading), and `load_unscaled`'s `int16_t` read carries the `as
    sfnt_f26dot6` that is C's promotion.  (c) **The four `goto`s of `sfnt_interpret_iup_1` are
    three labelled blocks**: `'untouched` encloses the function's body, `'skip33` encloses the
    body plus C 11033's statement (so C 11016's `goto wraparound_1` = `break 'skip33` skips it and
    lands on C 11039), and `'body` ends at C 11033 (so C 11002's `goto wraparound` = `break 'body`
    runs it); the first `break 'wraparound_1` draft was an `E0426` (a `break` cannot name a block
    that does not enclose it), which is why the labels are nested that way.  (d) **C's four
    out-parameters of `sfnt_address_zp0`/`zp1` in `shz`/`shc`/`shp` are zero-initialised**
    (Deviation 10's class - their addresses are taken and C reads them after the call), and
    `shz`'s `twilight_zone_size` (a `uint16_t`) is `as size_t` for `sfnt_move_twilight_zone`'s
    parameter, C's own promotion written out.

79. **Slice 11's three shapes.**  (a) **C 11275's `goto project_normally` is one labelled block**:
    `sfnt_project_zp1_zp0_org`'s simple-glyph path is `'project_normally: { … }` and the `break` is
    the jump, with C 11289's label body (the two `sfnt_address_zp1`/`zp0` calls and the final
    `DUAL_PROJECT`) after the block; the function's six-part guard is C's, spelled with `== 0` for
    the two zone tests and `is_null ()` for the two pointers, and `p1`/`p2` are `as usize` for the
    `number_of_points` comparisons.  (b) **C's narrowing is written out**: `projection` is an
    `sfnt_fword` and `DUAL_PROJECT` returns an `sfnt_f26dot6`, so the assignment carries `as
    sfnt_fword` (the first run's `E0308`), while the `x1 - x2`/`y1 - y2` pairs are `as sfnt_f26dot6`
    because C's `int16_t - int16_t` promotes to `int` before the macro sees it.  (c) **The four
    in-function macro copies** (`TRAP`, `POP`, `PROJECT`, `DUAL_PROJECT` - `sfnt_interpret_mirp` and
    `sfnt_interpret_mdrp` each take the ones they use, and `sfnt_project_zp1_zp0_org` only
    `DUAL_PROJECT`), plus the twelve zero-initialised out-parameters of the four
    `sfnt_address_zp0`/`zp1` calls (Deviation 10's class), and C's `(original_projection ^ distance)
    < 0` kept as written.

80. **Slice 12's four shapes.**  (a) **The two UVS readers' buffers and six bindings are
    zero-initialised**: C 13380's `char data[512]` and C 13450's `char data[500]` are written by
    `read` before they are read, and `num_ranges`/`num_mappings`/`template` plus the three locals
    the `bsearch` calls pass by address are C's own uninitialised locals (PORTING.md 1.5, Deviation
    10's class; four `E0381`s measured).  (b) **`sfnt_create_uvs_context`'s five `goto bail`s are one
    labelled block** (`'bail` spans C 13560-13683 and the `bail:` body is after it - the
    `sfnt_open_tables` reading; the token row above carries `goto`).  (c) **The three comparators are
    `unsafe extern "C" fn`** because `qsort`/`bsearch` take their addresses (PORTING.md 1.4), and
    both are declared at the use site (`<stdlib.h>`, no mirror carries them; the `buffer.rs` 129-134
    shape) with `bsearch`'s result cast from the untyped pointer C assigns.  (d) **The `sizeof`-over
    -a-place sites keep C's place**: C 13628's `sizeof *context` and C 13631's
    `sizeof *context->records` are `std::mem::size_of_val (&(*context))` and
    `std::mem::size_of_val (&(*(*context).records))`, because `context` is a registered token family
    (Deviation 17's ruling and the C 2694/2709 sites above); the operands are C's own - a NULL
    pointer and a not-yet-assigned member - and `size_of_val` never reads them.  C's `TEST_STATIC`
    (C 128/137) is the file's attribute, and the three functions inside C's `#ifndef TEST` arms
    (C 13795/C 13871) carry `not (feature = "test")` on top of the file gate.

81. **Slice 13's three shapes.**  (a) **The two `goto fail`s of `sfnt_decompose_instructed_outline`
    are one labelled block**: C 12164's `fail:` is the single `return 1` after the loop, so the loop
    is `'fail: { … }` with the two `break 'fail`s as the jumps (the token row above carries `goto`),
    and the `while (here < num_outline->num_points && flags[here] & SFNT_POINT_PHANTOM)` phantom
    skip keeps C's promotion with `(... as c_int) & SFNT_POINT_PHANTOM`.  (b) **The builder is
    `TEST_STATIC`** (the file's `cfg_attr`, C 12175), and its `memset`/`xmalloc` pair are the
    ported sibling's spellings (`std::ptr::write_bytes` over `size_of_val (&build_outline_context)`,
    and `size_of::<sfnt_glyph_outline> () + 40 * size_of::<sfnt_glyph_outline_command> ()` for C
    12186's `sizeof *outline + 40 * sizeof (*outline->outline)`), with the three proc typedefs'
    values passed as `Some (…)`.  (c) **`sfnt_compute_phantom_points` writes C's two narrowings
    out**: C 12275-12276's `f1`/`f2` are `sfnt_fword`s whose expressions are `sfnt_fixed`s
    (`(xmin - lbearing) as sfnt_fword`, `(f1 + advance) as sfnt_fword`) and the two
    `sfnt_mul_f26dot6_fixed` calls take `f1 as sfnt_f26dot6` (Deviation 16's class);
    `sfnt_transform_f26dot6`'s body is `sfnt_transform_coordinates`' with `sfnt_fixed` replaced,
    and its `double`/`1.0` spellings are that function's row.

82. **Slice 14's four shapes.**  (a) **The instruction macros are `macro_rules!`s inside
    `sfnt_interpret_run`'s body** (P-57), because their C bodies name the caller's `interpreter`
    and `is_prep`; the eight that also jump into that function's own labels are defined inside the
    labelled blocks that declare them - `ELSE`/`JMPR`/`FDEF`/`IF`/`JROT`/`JROF`/`IDEF` at `break
    'skip_step` and `CALL` at `break 'next_instruction` - because a `macro_rules!` resolves a label
    at its **definition** site (measured: a body's `break 'label` does not see a label declared
    later in the caller).  `LOOPCALL`'s two `goto loopcall_begin` (C 6941/6952) are a `'search`
    labelled block plus the `'loopcall_begin: loop`'s `continue`, and the six `DELTAC*`/`DELTAP*`
    bodies' `…_start:` labels are those macros' `loop` heads exactly where C puts them - C's `n =
    POP ()` sits *before* the label and stays outside the loop here.  (b) **C 11420's `switch
    (opcode)` is the port's `match opcode`**: the same 146 arms in C's order, C 11949's `default:`
    as `_ =>` with C 11949-11967's MIRP/MDRP/PUSHW/PUSHB/NOT_IMPLEMENTED fork, and C's `break`
    (which leaves the switch) as the match's arm ends plus the dispatch's `loop`; C 11970's
    `next_instruction:` is the inner labelled block's end (C 11975's `IP++` follows it, outside) and
    C 11980's `skip_step:` is the outer one's (the `continue`).  (c) **C's implicit conversions are
    written out**: the three `DELTAC*` calls narrow `operand2` and the three `DELTAP*` calls narrow
    `argn` to C's `unsigned char operand` (`as c_uchar`, C 9538/9705), `sfnt_interpret_if`'s
    condition is C's `uint32_t` against the port's `bool` (`condition != 0`), C 11411's `while
    (interpreter->IP < interpreter->num_instructions)` compares C's `int` against a `uint16_t` (`as
    c_int`), and `MOVE!`'s `memmove` carries the two `c_void` casts C's implicit pointer conversion
    gives it.  (d) **Deviation 10's class again**: the four out-parameters Rust cannot prove
    initialised - `SPVTL`'s, `SFVTL`'s and `SDPVTL`'s `vector` (`std::mem::zeroed ()`) and
    `SHPIX`'s `dx`/`dy` (`0`) - are the eight `E0381`s the third `probe` run reads (each of those
    four macros is expanded twice).
83. **Slice 15's three shapes.**  (a) **C's plain `setjmp` is declared and called by C's own
    name**: `<setjmp.h>` has no mirror in this tree (the ruling C 6556's `longjmp` already got), so
    the declaration stands at C 37's site with the other P-88 ① prototypes; the three-line C
    measurement above says what the reference binds (`nm -u` prints `_setjmp`/`_longjmp`, Mach-O's
    underscore for the C identifiers `setjmp`/`longjmp`), and the port's
    `setjmp (&raw mut (*interpreter).trap)` calls the same symbol rather than the `sigsetjmp` pair.
    (b) **Both entries are `TEST_STATIC` in C**: the port's `#[cfg_attr (not (feature = "test"),
    unsafe (no_mangle))] pub unsafe extern "C" fn` shape (slice 1's eleven readers' shape), whose
    non-TEST arm the `nm -g` reading above measures as the two C symbols.  (c) **C's implicit
    conversions are written out**: the interpreters' `num_instructions` is `uint16_t` where
    `sfnt_fpgm_table`/`sfnt_prep_table`'s is `usize` (`as uint16_t`, two sites), C 12044's
    `instruct_control & 2` is `((*interpreter).state.instruct_control & 2) != 0` (the file's
    `& N` spelling), the two `NULL`s are `std::ptr::null_mut ()`/`std::ptr::null ()` by pointee
    constness, and C 12080's `sizeof *state` is `std::mem::size_of_val (&*state)` (Deviation 17's
    place spelling).
84. **Slice 16's four shapes.**  (a) **C's two `volatile` locals keep C's names and C's saving and
    restoring statements**: `preserved_zone` (C 12312) and `zone_was_allocated` (C 12316) exist so
    the `setjmp` path survives the jump, and Rust has no `volatile` local - the callproc.rs 1699
    shape, a recorded deviation - so C 12431's `preserved_zone = zone` and C 12446's
    `zone = preserved_zone` are carried as written.  (b) **C 12339's `alloca` is the stack pool**:
    `sa_stack_alloc (zone_size)` (PORTING.md P-05), with C's `1024 * 16` threshold and the
    `xmalloc` arm's `zone_was_allocated = true` unchanged; C 12436/12479's two `xfree`s take the
    `as *mut c_void` cast the other `xfree` sites use.  (c) **The four `sizeof *zone->…` sites are
    type spellings, not place spellings**: C 12325/12328/12331/12333 ask for `sizeof *zone->x_points
    * 4` (and the two sibling sizes) *before* `zone` is initialised, so `size_of_val (&*zone)` (the
    spelling Deviation 17 rules for a live place, and the one C 12453's sibling uses) would form a
    reference to an uninitialised place; the port writes `std::mem::size_of::<T> ()` for the
    pointee's type instead.  (d) **The two `ckd_mul`s use the associated-function form**: C's call
    is `ckd_mul (&temp, glyph->simple->number_of_points + 2, sizeof *zone->x_points * 4)`, whose
    multiplicand sits *inside* the call, and `tools/extract-abi.py`'s occurrence check reads exactly
    the literals inside a family call - so the port writes
    `size_t::overflowing_mul ((*(*glyph).simple).number_of_points + 2, std::mem::size_of::<…> () * 4)`
    rather than a receiver-style call, which is the same expression with the literal in the same
    place.  (e) **Deviation 10's class again**: the first draft's two `E0381`s are C 12313/12314's
    `phantom_point_1_y`/`phantom_point_2_y`, both written by `sfnt_compute_phantom_points` before
    they are read (C 12409-12412), so both are zero-initialised.
85. **Slice 17's two shapes.**  (a) **The zone-building block is slice 16's, statement for
    statement**: C 12622-12638's sizing chain (the two `ckd_mul`s in the associated-function form,
    so C's `num_points + 2` stays inside the call the occurrence check reads - Deviation 84 (d)),
    the `alloca`/`xmalloc` fork (`sa_stack_alloc`, PORTING.md P-05) and the two `volatile` locals
    (`preserved_zone`, `zone_was_allocated`, Deviation 84 (a)) are the same statements with this
    function's own names, and C 12645-12651's three `sizeof *zone->…` sites take the type spelling
    again (Deviation 84 (c)).  The function's two `eassert`s (C 12610, C 12681) are the port's
    `eassert!`, which the file already uses.  (b) **The phantom points leave through two pointer
    locals**: C 12721/12722's `x_base`/`y_base` are `&context->x_coordinates[i - 2]` and
    `&context->y_coordinates[i - 2]`, where `i` is the loop variable C leaves at `num_points`, so
    the port writes `&raw mut *(*context).x_coordinates.offset ((i - 2) as isize)` and reads
    C's `x_base[0]`/`x_base[1]` back as `*x_base.offset (0)`/`*x_base.offset (1)` (the same for
    `y_base`), with the six `context->phantom_point_*` writes reading those two slots.
86. **Slice 18's three shapes.**  (a) **The component loop is `sfnt_decompose_glyph`'s, statement
    for statement** (slice 3, this file): C 12836-12867's flag tests are the file's `& 0oN` form
    (C's `02`, `01`, `04000` and `04` are `0o2`, `0o1`, `0o4000`, `0o4`), the two anchor-path
    failures are the `free_glyph.unwrap () (subglyph, dcontext)` shape, the recursive call passes
    `get_glyph`/`free_glyph` straight through, and C 12949/12959's tests are the port's
    `(*subglyph).number_of_contours != 0` and `!error.is_null ()`.  (b) **C's two chained
    phantom-point assignments are split in C's right-to-left order** (Deviation 26): C 13047-13049's
    `context->phantom_point_1_x = context->phantom_point_1_s = value->x_points[last_point]` is the
    `..._1_s` write followed by `..._1_x = (*context).phantom_point_1_s`, and C 13052-13054's pair
    the same way - the two slots the `USE_MY_METRICS` component is asked to replace.  (c) **The
    wrapper's `memset` over a stack context stays a `write_bytes`** (slice 13's builder shape),
    with the local zero-initialised at its declaration for the reason Deviation 10 records (the
    measured `E0381`); C 13271-13276's chain keeps its literals inside the two `ckd_mul` calls -
    the `sizeof *outline->x_points * 2` is the second mul's *own argument*, so the plain argument
    form is enough there, unlike slice 16's `num_points + 2` receiver.
87. **Slice 19's five shapes.**  (a) **The TEST block's gate is C's own condition**: every item
    carries `all (feature = "have_android", feature = "not_android_stubify", feature = "test")`,
    the spelling C 49's harness already uses, and the whole block is one `#ifdef TEST` (C 16448)
    whose `#endif` is C 21215, the file's last line.  (b) **C's X11 names keep C's spelling**:
    `Display`, `Window`, `GC`, `XEvent`, `XDrawPoint`, `XNextEvent`, `XDestroyWindow`,
    `XCloseDisplay`, `XFlush` and the two event codes `KeyPress`/`Expose` keep C's spelling, which
    is `xterm.rs` 824-827's ruling for the same names.  **Corrected in slice 20 (3.707)**: the
    parenthetical this row first carried - "no mirror in this tree / this machine has no
    `<X11/Xlib.h>`" - was wrong on both halves.  `include/xlib.rs` *is* the tree's mirror of the
    X11 SDK's header (`have_x_windows`-gated, with measured constants in its `x11_constants` module
    and a measured 192-byte `XEvent` at its 1181), and the machine has the headers - slice 19's
    `clang` probe simply omitted `-I/opt/homebrew/include`, where `X11/Xlib.h` and
    `X11/extensions/Xrender.h` (a symlink to libxrender 0.9.12) both live.  What is true is narrower:
    the mirror carries only the surface other units name, so this arm's names have **0 hits** there
    today, and Owed 30 lists them with the header line each would cite - plus the absent Xrender
    mirror.
    (c) **C 16596's prototype is dropped** (measured: a bodyless `unsafe fn` is `error: free
    function without a body`); Rust has no function prototype, and this file's own header already
    records that freedom as why the port may walk a different order.  (d) **Dead text is recorded,
    not transcribed**: C 16562/16581's two `#elif 0` arms inside `sfnt_test_span` and C 16680-16690's
    `#ifdef TEST_VERTEX` arm inside `sfnt_test_edge` (which also names an `sfnt_edge.y` member the
    record C 774-783 does not have) are C's dead branches - the class `tools/cinventory.py` drops.
    (e) **Two spellings of C's own text**: C 16556's concatenated format `"… %"PRIi32" %"PRIi32"
    …"` is written with `%i` twice, because the inttypes mirror's `PRIi32` is `c"i"` (frozen, and
    measured there), and the four X11 statics take `unsafe { std::mem::zeroed () }` for C's static
    zero-initialisation (the `frame.rs` 4813-4829 ruling) - while C 16610's function-scope
    `static sfnt_fixed y` stays inside `sfnt_test_edges` as a Rust `static mut`.
88. **Slice 20's four shapes.**  (a) **C 16761's `goto depth_found` is one labelled block**: the
    label sits after C 16764's `exit (0)`, so the search loop and that exit are the block and
    `break 'depth_found` is the jump (the `sfnt_decompose_instructed_outline` shape).  (b) **C
    16769/16770's two `alloca`s go to the stack pool**: `sa_stack_alloc (std::mem::size_of::<Pixmap>
    () * nrasters as size_t)`, with C's `sizeof *pixmaps` spelled as the pointee type (the
    `sfnt_build_outline_edges` row's class; `Picture` likewise).  (c) **C's implicit conversions are
    written out**: `image.width = rasters[i]->width` widens a `c_ushort` to the `XImage` field's
    `c_int` (`as c_int`, and the same in `x`, `y`, `bytes_per_line` and the four
    `XRenderComposite`/`XCreatePixmap` arguments), `hhea->ascent * 65536` promotes an `sfnt_fword`
    (`as sfnt_fixed`), `image.data = (char *) rasters[i]->cells` is C's own `as *mut c_char`, and
    `gc = None;` is `None as GC` - Xlib's `#define None 0L` (measured) written out as the
    null-pointer constant - with `if (!gc)` as `gc.is_null ()` and `if (!font)` as `font == 0`.
    (d) **The X11 names are C's**: `Display`, `Window`, `Pixmap`, `Picture`, `Visual`, `XImage`,
    `GC`, `XGCValues`, `XEvent`, `XRenderColor`, `XRenderPictFormat`, `Font` and the `X*`/`XRender*`
    procedures keep C's spelling, with this file's edge to `include/xlib.rs` added at C 57's
    position (Deviation 87 (b)'s correction, Owed 30).
89. **Slice 21's six shapes.**  (a) **The fixtures are struct literals**: C 16890-16941's four
    positional initialisers (a `sfnt_maxp_table` with fifteen values, a ten-element `sfnt_fword`
    array, a `sfnt_cvt_table`, a `sfnt_head_table` with nineteen) become `static mut` objects
    naming every field in C's order - the port's records are `#[repr(C)]` with those fields, so the
    positional text has one Rust spelling (`Lisp_Subr_intspec`'s shape).  (b) **C's file-scope
    compound literals are promoted arrays behind pointers**: each `(uint32_t []) { … }` is written
    `&[…] as *const [uint32_t; N] as *mut uint32_t`, one expression per call site and no invented
    item name (the `xmenu.rs` 3854 / `keyboard.rs` 7767 rulings for C's compound literals, whose
    scalar cases are the casts `image.rs` 7486 records); a file-scope compound literal has *static*
    storage, which the promotion matches.  (c) **C 18002-18016's three initialiser macros are
    written out**: each expands to several initialiser *elements*, and Rust has no spelling for that
    in `[MACRO! (), …]`, so the two arrays write the expansion with C's own arithmetic and name the
    macro in the line's comment - with the element counts **measured** by compiling C's text rather
    than counted by eye (Evidence ⑯ (i), which caught 42-vs-54 and 7-vs-9).  (d) **`countof (x)` in
    a static initialiser is the value with C's spelling in the comment**: the mirror's
    `countof!` is `size_of_val`-based and a `static mut` cannot be read from another initialiser
    (measured: `E0133`), so the four sites carry C's own call as a comment beside the number it
    denotes - a *measured* number, not a re-derived one.  (e) **The checks use the file's own
    idioms**: `fprintf (stderr, c"…".as_ptr (), …)` (the `alloc.rs`/`bidi.rs` shape), C's
    `(char *) SP - (char *) stack` byte count as `((*interpreter).SP as *const c_char).offset_from
    ((*interpreter).stack as *const c_char) as size_t` (the `ralloc.rs` 431 idiom, which is the
    byte-wise difference C's cast asks for - the element-wise `offset_from` on `uint32_t *` would be
    four times too large), the `%td` argument as the same `.offset_from` in `c_int`, `memcmp`'s
    pointer difference inside the call, and C's adjacent string literals folded into the one
    `c"…"` they denote (Deviation 87 (e)).  (f) **`sfnt_run_interpreter_test` keeps C's `setjmp`
    dispatch**: `if setjmp (&raw mut (*interpreter).trap) != 0 { test->check (…, true) } else { … }`,
    with the record's function-pointer member spelled `Option<unsafe extern "C" fn (…)>` (P-27) and
    called through `.unwrap ()` like the `sfnt_decompose_glyph` callbacks.
90. **Slice 22's three shapes.**  (a) **The table's 94 entries are transcribed as struct literals**
    - `sfnt_interpreter_test { name: c"…".as_ptr (), instructions: …, num_instructions: …,
    arg: … , check: Some (…) }` - with C's per-entry comments carried verbatim above the
    `instructions` line they annotate (the port's usual place for a C comment).  (b) **The
    instruction member's three C shapes each get one**: the 91 plain compound literals are promoted
    arrays (Deviation 89 (b)), the one *sized* literal (C 18232's `(unsigned char [103]) { 0x40,
    101, }`) becomes a `const` block that fills C's two values and leaves the other 101 bytes zero
    (Rust's array literal has no partial fill; measured: it is the only sized compound literal in
    the 94), and the two rounding entries take the address of slice 21's arrays
    (`&raw mut sfnt_sround_instructions as *mut c_uchar`).  (c) **C's two casts are written out**:
    C 18321/18345's `- (signed char) 32` / `31` are `(-(32 as c_schar)) as c_uchar` /
    `(-(31 as c_schar)) as c_uchar` - C's cast, its negation, and the array element conversion, each
    in its own Rust operation; writing C's text verbatim is a parse error (measured on the frozen
    probe, which parses `#[cfg]`-disabled items: `expected one of … found char`).  (d) **The two
    `countof` fields keep C's spelling beside the number**: C's `countof (sfnt_sround_instructions)`
    is written `54` with the call in the line's comment, the number being slice 21's *measured* one
    (the mirror's `countof!` cannot be read from a static initialiser: Deviation 89 (d)).  (e)
    **`arg` is the address of the `_test_args` object** C names (`&raw mut npushb_test_args as *mut
    c_void`) or `NULL` as `std::ptr::null_mut ()`.
91. **Slice 23's six shapes.**  (a) **C's function-scope `static const char *const
    opcode_names[256]` cannot be a Rust `static`**: the element type is a bare pointer, which is not
    `Sync`, and a `static` of it is refused (measured), so the array is a `static mut` like the
    counter in `sfnt_verbose` - C's `const` has no spelling here and nothing writes the array.
    (b) **The two `char` buffers**: C 19978's local `char buffer[80]` is `[c_char; 80]` with
    `buffer.as_mut_ptr ()` for the `sprintf` and `buffer.as_ptr ()` for the `strlen` (the `alloc.rs`
    5520 shape), and C 20077's function-scope `static char buffer[256]` is `static mut buffer:
    [c_char; 256] = [0; 256]`, returned as `&raw mut buffer as *const c_char`.  (c) **The function
    pointer comparison**: C 20003/20006 and C 20161/20164 compare `interpreter->state.project` with
    a function's address; the port's member is an `Option<unsafe extern "C" fn (…)>`, so the
    comparison is written `== Some (sfnt_project_onto_x_axis_vector)` - the first of its kind in
    this unit, where the call sites spell the same member `.unwrap () (…)`.  (d) **C's pointer
    arithmetic stays C's**: `where >= exec_prep->instructions` and its `<` twin are raw-pointer
    comparisons as written, the three `%td` differences are `.offset_from (…) as isize`, and C's
    `while waitpid (…) != pid && errno == EINTR` is `waitpid (…) != pid && *errno () == EINTR` (the
    `android.rs` 1309 shape).  (e) **C's post-increment in an argument is the load-then-step pair**
    (Deviation 26): C 20157's `printf (…, instructions++)` prints the old count and then steps, and
    C 20033-20038's `#ifdef TEST_BREAK_AFTER` arm is dead text recorded, not transcribed
    (Deviation 87 (d)'s class).  (f) **Three conversions written out**: `int32_t alternate = value;`
    from a `uint32_t` is `value as int32_t` (twice), the `%`-`PRIi32` concatenations are the folded
    `%i` the inttypes mirror's measured `c"i"` denotes, and `sfnt_verbose`'s `struct
    sfnt_instructed_outline temp` - which C fills field by field - is zero-initialised at its
    declaration (Deviation 10's class).
92. **Slice 24's seven shapes.**  (a) **The two `goto free_lab` sites (C 20576, C 20579) are one
    labelled block** (Deviation 89 (a)'s class, `'exit` at sfnt.rs 3905 the precedent): the block
    opens above C 20522's `if (fvar)` - the outermost block the two arms share with C 21178's label
    - and closes after the `while (true)` loop, with C's `free_lab:` code standing after it, which
    is where `break 'free_lab` lands the two arms.  (b) **C's braces-less one-statement bodies get
    braces** (the six parse errors Evidence ⑲ names): C 20451-20453, C 20545-20547, C 20565-20567,
    C 20625-20626, C 20630 and C 20742-20743, plus the four `fprintf` bodies at C 20746, C 20758,
    C 20827 and C 21020 - the last being C's `for` body, whose step is written inside the braced
    statement (the file's standing `while { …; i += 1; }` shape), and the two arms of C 21094's
    `if (glyph->simple)` each a block.  (c) **C's implicit argument promotions are written out** as
    `as c_int`/`as c_uint`: every `%u`/`%x`/`%i` whose C argument is a `uint16_t`/`int16_t` struct
    field (`table->num_subtables`, `head->flags`, `head->units_per_em`, `post->underline_position`,
    `record.length`, `fvar->axis_count`, `name->format`, `maxp->num_glyphs`, `outline->origin`,
    `distortion.origin`, …).  Rust refuses such an argument to a C variadic (E0617, measured), so
    the cast is C's promotion *and* the compiler's requirement; `%PRIi32`/`%PRIu32`/`%PRIx32`/
    `%PRIi16`/`%PRIu16` are the folded `%i`/`%u`/`%x` the inttypes mirror's measured values denote,
    and the five multi-line C concatenations (C 20524-20529, C 20777-20789, C 20827-20835,
    C 20848-20854, C 21154-21156) each become one literal.  (d) **The two `#define` pairs inside
    `main` are in-block `macro_rules!`** (P-57): `GG`/`FG` (C 21101-21102) expand to the two
    callback names, so the call writes `Some (GG! ())`/`Some (FG! ())`, and `LB`/`RB`
    (C 21130-21131) expand to C's two differences, used inside `sfnt_coerce_fixed`.  C's four
    `#undef`s (C 21113-21114, C 21136-21137) have no Rust counterpart and are recorded in the
    comments beside the definitions.  (e) **C 20473-20474's two function-scope `#define FANCY_PPEM
    18`/`#define EASY_PPEM 18` are `const`s at C's position** - the value-shaped answer
    `macro_body_of` documents for a macro whose body is a value.  (f) **C's two `alloca`s
    (C 20682-20683) go to the stack pool** (P-05's `sa_stack_alloc`), each keeping C's own
    `sizeof *x * length` as `std::mem::size_of::<T> () * length`, and C's `memset (&x, 0, sizeof
    x)` sites are `memset (&raw mut x as *mut c_void, 0, std::mem::size_of::<T> ())` (the standing
    shape at sfnt.rs 18620).  (g) **`clock_gettime` and `CLOCK_THREAD_CPUTIME_ID` stand at their
    use site** (P-88 ①, the `waitpid` block's shape): C 52's `#include <time.h>` is inside the
    `#ifdef TEST` block and the tree's `<time.h>` mirrors carry neither name, so the declaration is
    an `unsafe extern "C"` block beside the arm that calls it and the constant is the frozen
    target's **measured** value (`clang -dM -E` reads `CLOCK_THREAD_CPUTIME_ID
    _CLOCK_THREAD_CPUTIME_ID`, whose enum member is 16) - the `timefns.rs` 107 shape.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`TEST_STATIC`'s Rust form** — either a `docs/ports/excluded-macros.txt` row (`kind=syntax`,
   "the port expresses the linkage with `#[cfg_attr (not (feature = "test"), unsafe
   (no_mangle))]`", the `AVOID` row's shape) or a PORTING.md §11 entry; today `--check` counts it
   as named rows and a missing macro.  ✅ **Done in 3.427** for the
   `E0428` half — **`src/include/sfnt.rs`: the declaration run now sits in
   `pub mod sfnt_decls { use super::*; … } use sfnt_decls::*;`** (the `include/android.rs` shape) —
   and ✅ **done in 3.441** for the export half: the tool now reads the `cfg_attr`, so the eleven
   `TEST_STATIC` readers produce no `- function … has no #[unsafe(no_mangle)]` row (Evidence ③'s
   reading has none).  ✅ **Closed in 3.701 (slice 14)** for the macro itself: the
   `docs/ports/excluded-macros.txt` row exists (`TEST_STATIC\tsyntax\t…`, the `AVOID` row's shape),
   so `--check` counts it as a recorded counterpart instead of a missing macro - this unit's
   reading now has 6 `missing function` rows and the line-count row and **no `missing macro` row
   at all** (Evidence ⑨).


2. **Two `docs/ports/token-aliases.tsv` rows**: `(T *) -1` → `usize::MAX as *mut T`, and `p[i]`
   on a raw pointer → `*(p.offset (i as isize))` (Deviation 4 / Deviations 9-10's note).
   Slice 7 adds a third `(T *) -1` use site to the first row (C 14483's `(uint16_t *) -1`,
   Deviation 50) - the row's shape does not change, only its count.
3. **`docs/ports/macro-body-notes.tsv` rows** — slice 1's `sfnt_read_cmap_table_1` →
   `case,default,switch` is in place; slice 2 owes **`sfnt_lookup_glyph` → `case,switch`**
   (C 1437-1462's `switch` is the port's `match`, now with a `_ => {}` arm) and
   **`sfnt_lookup_glyph_4` → `sfnt_glyph`** (C 1270's `sfnt_glyph glyph;` is the *scalar*
   typedef, which the P-67 rename spells `sfnt_glyph_index`; `docs/ports/include-sfnt.md` owns
   that rename, and this is its first body-level cost) — ✅ both written in 3.442.  **Slice 3 owes
   six**: `sfnt_transform_coordinates` → `double` (C's six `double` locals are PORTING.md 1.1's
   `f64`; the `WORDS_PER_DOUBLE`/`json_out_float` rows' shape), `sfnt_decompose_compound_glyph` →
   `goto,uninit` and `sfnt_decompose_glyph_1`/`sfnt_decompose_glyph_2`/`sfnt_decompose_glyph` →
   `goto` (PORTING.md 2.2's labelled blocks, Deviations 18-19; the `uninit` half is C 2768's
   `point`/`point2` `UNINIT` and C 2765's `need_free`, PORTING.md 1.5).  **Slice 4 owes one**:
   `sfnt_build_outline_edges` → `alloca` (C 4070's `alloca` is the port's `sa_stack_alloc`,
   Deviation 23; the token cannot reappear because the port has no `alloca` at all).  ✅ **Written by
   the integrator in 3.463** (`docs/ports/macro-body-notes.tsv`); Evidence ④'s `--check` reading
   accordingly has no `does not mention` row left.  Evidence ③'s
   reading shows the earlier rows are all in place and this is the only `does not mention` row
   left, so with it the reading's non-`missing` lines are the line-count row alone.
4. **`have_mmap` (or an explicit ruling)** — C 1827's `#if defined HAVE_MMAP && !defined TEST`
   is carried as `not (feature = "test")` alone, because `docs/baseline/feature-coverage.tsv` has
   no `have_mmap` row and slice 1 already ruled C 41's `#ifdef HAVE_MMAP` live.  Either register
   the feature (and gate C 1827-1906 with it here and C 41-43 in slice 1) or record the ruling as
   the tree's answer for this macro.  ✅ **Ruling (3.463, integrator): no feature belongs here.**
   `HAVE_MMAP` is `1` in the frozen table (`docs/baseline/config/darwin-arm64.macros.tsv` 1469),
   and C 1827's `#if` has **no `#else`** — the block's last line is C 1907's `#endif /* HAVE_MMAP */`
   (`awk` over C 1827-1912 prints the nested `#ifdef HAVE_POSIX_MADVISE`/`#endif` pair and that one
   `#endif`, nothing else) — so `not (feature = "test")` *is* the whole selection and there is no
   arm text for a `have_mmap`/`no_have_mmap` name to carry.
5. ~~`time_t`/`timespec` for `lib/include/sys_time.rs`/`sys_select.rs`/`src/include/thread.rs`
   through `lisp.rs`'s chain~~ — **closed in 3.429**: root was P-89's materializer, and the arm
   now reads exit 0.  `<sys/mman.h>` (this slice), `<setjmp.h>` and the TEST arm's
   `<sys/wait.h>`/`<X11/*>` still have no mirror; slice 2's own text takes nothing from
   `<setjmp.h>`/`<sys/wait.h>`/`<X11/*>`, and takes `mmap`/`munmap`/`posix_madvise`/`madvise`,
   `PROT_READ`, `MAP_PRIVATE`, `MAP_FAILED`, `POSIX_MADV_WILLNEED` and `MADV_WILLNEED` from the
   use-site block (Deviation 13).
6. **`tools/check-port-lines.py --write-pending`**: this file's ratchet row is stale (75.6%
   drift after slice 4) — deliberately left to a batch close, so that the rewrite does not encode
   the parallel lanes' in-flight units.
7. **C 3516's `sizeof context`**: the port's `size_of_val (&context)` (Deviation 17) is the
   *place* spelling the `sizeof` family sanctions, chosen because `size_of::<T> ()` would drop the
   `context` family's occurrence.  Nothing is owed by name here; it is recorded so the next lane
   does not "restore" the type spelling and re-open `family context: … the port 22`.
8. **`no_ndebug`** — C 4322's `#ifndef NDEBUG` (Deviation 24) is the
   first `NDEBUG` site in a ported unit.  ✅ **Registered by the integrator in 3.463**:
   `xtask/Cargo.toml` carries `no_ndebug = []  # !NDEBUG` and
   `python3 tools/feature-coverage.py --write` re-recorded the table, so the name is a real feature
   a `dead-arms` flip can turn on.  **Still owed**: the flip itself — `docs/ports/`'s
   `xtask dead-arms` FLIPS table has no `no_ndebug` row and `config.rs`'s `NDEBUG` constant is what
   a flip would have to move with it; the arm is unmeasured text until then.
9. **This unit's TEST arm** — C's TEST arm gets `alloca` from `<alloca.h>` (C 39, *outside* the
   `#ifdef TEST`) and `xnmalloc (size_t, size_t)` from C 98-105, while the port's TEST arm has
   neither: `sa_stack_alloc`'s prototype reaches the non-TEST arm through C 138's `lisp.h`
   (Deviation 23) and the file's own TEST-arm `xnmalloc` takes `size_t` where the port's call is
   cast to `isize` (Deviation 25).  No gate builds that arm today (`sfnt.c` is not in
   `docs/baseline/frozen-build-sources.txt` - `check-all-units.sh` therefore never sweeps it - and
   the P-86 ③⑴ arm flip passes only the file gate), so this is an owed, unmeasured reading rather
   than a failed one: the lane that first builds `feature = "test"` here owns it.

10. **`docs/ports/macro-body-notes.tsv` rows for slice 5** — the seven Evidence ③ names:
    `sfnt_build_outline_fedges` → `alloca` (the `fedge` twin of the row the integrator wrote in
    3.463), `sfnt_insert_raster_step` → `goto` and `sfnt_poly_edges_exact` → `float,goto`
    (Deviation 37's labelled blocks; C's `float` locals are PORTING.md 1.1's `f32`),
    `sfnt_poly_steps` → `float`, `sfnt_poly_set_steps` → `break,case,default,switch` (Deviation
    38), `TRAPEZOID_AREA` → `f,float` and `TRIANGLE_AREA` → `f` (C's `(float)` casts and its
    `2.0f` suffixes against the port's `as f32`/`2.0f32`).  **The `TRIANGLE_AREA` row can be
    avoided instead of written** by taking the `sfnt_fixed_float` mirror's `let f: f32 = 2.0;`
    spelling - the tree's own way of keeping an `f` token - but that changes a line of this lane's
    verified revision, so it is left as the integrator's choice.  ✅ **All seven written by the
    integrator in 3.470** (`docs/ports/macro-body-notes.tsv`); Evidence ③'s `--check` reading now has
    **no `does not mention` row at all**.  The `TRIANGLE_AREA` alternative (taking the
    `sfnt_fixed_float` mirror's `let f: f32 = 2.0;` spelling, which keeps an `f` token) was **not**
    taken: it would edit a line of the lane's verified revision for a manifest's convenience, and the
    row records the same fact without touching text.
11. **The lane's `probe` budget overrun (4 of 3)** — reported rather than hidden: the fourth run
    re-verified the delivered revision after `struct sfnt_fedge`'s fields were split one per line
    (Deviation 36), so that the file this ledger's Evidence ② describes is the file the reading
    measured.  `build-tu` stayed inside its budget (3 of 3).

12. **The C 6308-6522 block is blocked by a forward-declared callee and an untranscribed TEST
    arm** (this is why slice 6 stops at C 6306 instead of carrying the interpreter's setup): (a)
    `sfnt_init_graphics_state` (C 6316) calls `sfnt_validate_gs`, whose only other mention before
    its definition is C 6308-6310's forward declaration - the definition is at **C 10538**, deep in
    the remainder - and Rust has no prototype for a `static` function, so a slice that carries C
    6316 cannot compile until C 10538 lands; (b) `sfnt_make_interpreter` (C 6358-6522) calls C
    6316, so it is blocked by (a) as well, and its C 6427-6431 `#ifdef TEST` block assigns
    `interpreter->run_hook`/`push_hook`/`pop_hook`, three members `include/sfnt.rs` 31-32
    deliberately does **not** transcribe ("C 2026-2038 `#ifdef TEST` is dead"), so that arm has no
    Rust text to carry either.  The resolution is the integrator's call; the two shapes that look
    workable are (i) one slice covering **two C ranges** (6308-6541 and 10538-10600) with the port
    text kept in C's order, or (ii) a ruling that records the `TEST` hook block as a
    `docs/ports/excluded-macros.tsv`-class non-text item (its three members are TEST-only and the
    frozen arm never compiles them).  Nothing else in C 6307-6541 is blocked: the enum
    `sfnt_interpreter_run_context` (C 6533-6541) and its `#ifdef TEST` arm are self-contained.  ✅ **Half of it is closed by slice 7**: the
    `sfnt_validate_gs` definition (C 10538) is now in the file (the C 10225-10663 range, at its own
    C position per P-91 ②), so C 6316's call site type-checks and `sfnt_init_graphics_state` is the
    next live definition.  What is left is (b) alone - `sfnt_make_interpreter`'s C 6427-6431
    `#ifdef TEST` hook block, whose three members `src/include/sfnt.rs` deliberately does
    not transcribe - and it is what still gates the rest of C 6307-10224.

13. **The `sfnt_decompose_glyph` `fail`-tail defect slice 7 found** (Deviation 51): the port's
    `break 'fail` runs the `early:` tail and returns 0 where C returns 1, so the two failure paths
    in C 3565/3574 report success.  It is slice 3's verified text and this lane did not touch it;
    the fix is small (give `'fail` its own tail) but it changes the file's delivered bytes, so it
    needs a lane that owns C 3470-3594 - and, like every change to landed text, a fresh
    `build-tu`/arm reading afterwards.  Nothing in this ledger's tables or readings depends on it.

14. **`docs/ports/macro-body-notes.tsv`: the five rows slice 7's `--check` reading names**
    (Evidence ③): `sfnt_validate_gs` → `break,case,switch` (Deviation 52), `sfnt_read_fvar_table`
    → `alloca,goto` (C 14096's `alloca` is `sa_stack_alloc`, Deviation 47; C 13977's `goto bail`
    is `break 'bail`, Deviation 48), `sfnt_read_gvar_table` → `goto`, `sfnt_read_avar_table` →
    `goto` and `sfnt_read_packed_points` → `goto` (the `bail`/`bail1` labels).  All five are the
    `does not mention` class - the token cannot survive in the port - and they are the reading's
    only named rows besides the line count.  ✅ **All five written by the integrator in 3.478**
    (`docs/ports/macro-body-notes.tsv`, each with its C lines and the port's spelling): the
    `--check` reading now has **no `does not mention` row at all** — 244 `- missing` rows (the
    unported remainder) and the 60.2% line count, and nothing else.

15. **The lane's `probe` budget overrun (5 of 3)**, reported rather than hidden: the fourth run
    re-measured the revision after this slice rewrote `src/sfnt.rs`'s header comment and the
    fifth the final one after three of the slice's own `/* … */` notes were corrected, so that the
    file this ledger's header names is the file the arm reading measured.  All five readings agree
    where they overlap (exit 0, 0 errors, 39,664 bytes, the same four mirror warnings), i.e. the
    two extra runs bought a claim about md5 `80aed2e1701986817a5de1b3172e313c` rather than a
    different result.  `build-tu` used its whole budget (3 of 3) for the same reason.

16. **The arm flip's five mirror `E0425`s** (Evidence ②'s second and third run, md5
    `80aed2e1701986817a5de1b3172e313c` for the control): `2 × cannot find type time_t`
    (`lib/include/sys_time.rs` 44/90) and `3 × cannot find type timespec`
    (`lib/include/sys_select.rs` 56, `src/include/thread.rs` 300/318).  The probe's crate has
    exactly one definition of each — `lib/include/time.in.rs` 36 / `timespec.rs` — and **both
    mirrors are included under C 52-53's `#ifdef TEST` arm** (`build/probes/sfnt/_root.rs`
    175-178), which the arm flip's feature set (`have_android`, `not_android_stubify`) does not
    enable, while `src/include/thread.rs` 33 includes `sys_select.rs` unconditionally and
    `sys_select.rs` 27 includes `sys_time.rs`.  **This reproduces on the pre-slice revision**, so it
    is not this slice's text; but it *does* mean the arm flip's reading is no longer the "exit 0"
    slice 7's Evidence ⑤ recorded, and one of two things has to give: either the probe for this
    unit carries `--cfg 'feature="test"'` (which would also start type-checking the C 16448-21215
    TEST programme, Owed 21), or `lib/include/time.in.rs`/`timespec.rs` need an include edge that
    the non-TEST arm reaches (C's non-TEST `lisp.h` chain does reach `<sys/time.h>` through
    `thread.h` 40's `sysselect.h`, so the mirror edge is the missing half of a real C edge).

17. **`docs/ports/macro-body-notes.tsv` rows for slice 8** — the eight `does not mention` rows
    Evidence ③'s first reading produced.  ✅ **Written by this lane** (appended, with the C lines
    and the port's spelling): `sfnt_read_packed_deltas` → `goto`; `sfnt_read_cvar_table` → `goto`;
    `sfnt_compute_tuple_scale` → `uninit` (Deviation 53's zeroing of C 15103's two `UNINIT`);
    `sfnt_infer_deltas_2` → `goto`; `sfnt_infer_deltas_1` → `goto`; and
    `sfnt_vary_simple_glyph`/`sfnt_vary_compound_glyph` → `alloca,case,default,goto,restrict,switch`
    (Deviation 47's `sa_stack_alloc`, Deviation 8's `match`, Deviation 58's two blocks, and the
    `restrict` qualifier, the `sfntfont_android_put_glyphs` row's class).  The re-taken reading has
    no `does not mention` row left.  **It is the integrator's to review**: this lane wrote to an
    append-only table, and `docs/ports/excluded-macros.txt` needed nothing (the range defines no
    macro).

18. **This lane's budgets**: `build-tu` **3 of 3** and `probe` **3 of 3**, and the overrun the
    earlier slices reported does not recur — every run bought a distinct reading.  The three
    `build-tu` runs are (i) the `2 × E0425: cannot find type uint16_t` reading of the new record's
    missing gate (Deviation 62), (ii) the first clean reading, (iii) the re-measure of the
    delivered revision after the header comment changed (Evidence ①).  The three `probe` runs are
    (i) the 17-error reading of this slice's own text, (ii) the clean reading (five mirror
    `E0425`s and nothing else), (iii) the pre-slice control that shows those same five reproduce on
    md5 `80aed2e1701986817a5de1b3172e313c` (Evidence ②, Owed 16).  The first `build-tu` reading
    and every `probe` reading are reported above, not hidden.

19. **`sfnt_vary_interpreter`'s C-level indexing oddity** (Deviation 56) — recorded, not fixed:
    C 16271 writes `cvt[i]` with the tuple index.  If the integrator decides this is a genuine
    reference bug to be repaired, it is a *behaviour change* to C's text and needs its own decision
    (this lane's read is that PORTING.md 0.3's "one statement = one statement, same order" keeps it
    as it is).

20. **The new record's home.**  `sfnt_gvar_glyph_header` is defined in the unit (Deviation 62)
    because `sfnt.h` does not carry it; if the integrator prefers every record in a mirror, the
    move is mechanical but changes this file's delivered bytes and would need a fresh arm reading.
    Nothing else in the slice defines a type.

21. **C 16448-21215 (the TEST programme) is the next lane's range, and no gate type-checks it.**
    C 16448's `#ifdef TEST` matches the file's last `#endif` at C 21215, so everything from
    `struct sfnt_test_dcontext` to the end is test-only text; the arm flip's feature set carries no
    `test`, `build-tu` compiles nothing, and the tree has no `<X11/Xlib.h>`/`<X11/extensions/Xrender.h>`
    mirror for C 57-58's include or C 16596-16690's `Display`/`Window`/`GC` statics.  A lane that
    takes it needs a considered ruling (P-86 ③'s feature set, or an explicit
    `docs/ports/excluded-macros.txt`-class exclusion for the X11 half), not a silent splice.  Owed
    12's `sfnt_make_interpreter` TEST hook block (C 6427-6431) is the same class.
    **Slice 19 (3.706) takes the first block**: C 16448-16704 (the context record, the eight
    callbacks, the X11 debugger state and its three functions) is transcribed behind exactly the
    gate this item prescribed, `all (file gate, feature = "test")`, with C's X11 spellings kept
    (Deviation 87 (b)); C 16705-21215 is what remains.  The item's second half is still open, and
    now has numbers: flipping `test` today reads **3,101 errors**, the bulk of them in the mirrors'
    own gates (`include/sfnt.rs` 2,507, `unistd.in.rs` 572, `stdbit.in.rs` 465 - Evidence ⑭), so
    the flip is a tree-wide job rather than one cfg - which also prices Owed 16's first option.

22. **The C 6559-8159 macros need a home inside `sfnt_interpret_run` (C 11396) and a ruling on
    *how*.**  P-57 gives the rule (one in-function copy per using function) and this slice's four
    copies follow it, but the remaining ~130 instruction macros cannot simply be copied into that
    function: twenty of them (`ELSE`, `JMPR`, `FDEF`, `IF`, `JROT`, `JROF`, `IDEF`, `INS_MAX`,
    `CALL`'s two, `LOOPCALL`'s two and the six `DELTAC*`/`DELTAP*` bodies) end in a `goto` to a
    label of `sfnt_interpret_run` itself (`skip_step`, `next_instruction`, `loopcall_begin`,
    `deltac1_start` … `deltap3_start`).  A `macro_rules!` body cannot `break` a caller's label
    (labels are hygienic), so that slice has to choose between (i) defining such macros *inside*
    the `'skip_step`/`'loopcall_begin`/… blocks they jump to - which is what P-57 prescribes and
    what `src/coding.rs` does with its own `'no_more_source` - and (ii) a label-parameter
    interface (`CALL! ('next_instruction, …)`, a shape no unit in this tree uses yet).  This lane's
    read is (i); either way the choice belongs to the lane that lands C 10664-13917, and the range
    table's `6558-8159` row records that the region is deliberately unported here rather than
    silently missing.  The C 6559-6566/6568-6589 guard macros (`STACKSIZE`, `TRAP`, `MOVE`,
    `CHECK_*`) come with it: `TRAP` is already ported here (this slice's 20 copies), and the other
    three are used only by that function.
23. **This unit's TEST arm has three holes, all pre-existing and all now load-bearing.**  (a)
    `sfnt_make_interpreter`'s C 6427-6431 `#ifdef TEST` block assigns `interpreter->run_hook`,
    `push_hook` and `pop_hook` — the three members `include/sfnt.rs` 2153-2157 deliberately does
    not transcribe; the port records the block and carries no text for it (Deviation 64's class).
    (b) `POP`/`POP_UNCHECKED`/`PUSH`/`PUSH_UNCHECKED`'s `#else` arms (C 6604-6618, C 6643-6661)
    call `pop_hook`/`push_hook`, so the four in-function copies in this slice carry C's live
    `#ifndef TEST` arms only.  (c) `longjmp`'s declaration (Deviation 69) and the mirror's own
    `trap: jmp_buf` member both need `jmp_buf`, and `include/lisp.rs` is included under
    `not (feature = "test")` (C 138's `#else` arm) — so a `feature = "test"` build of this unit
    has no `jmp_buf` at all.  No gate builds that arm today (Owed 9's reading), and the fix (the
    three members plus a TEST-arm `jmp_buf` edge) is the mirror's, not this slice's.
24. **The `longjmp` use-site declaration wants a ruling** (Deviation 69): the tree has no
    `<setjmp.h>` mirror, so the port declares C's own `longjmp` at C 37's site.  The alternative
    would be to re-spell the call as `sys_longjmp` — but that is `_longjmp` (measured: the two are
    different symbols and `_longjmp` does not restore the signal mask), so it would be a
    *behaviour* change, not a spelling.  What the integrator should decide is whether a
    `setjmp`/`longjmp` pair belongs in `lisp.rs`'s mirror (where `jmp_buf` already is) for the
    four later `setjmp` call sites (C 11999, 12028, 12433, 12695, 16967 — every one of them in
    this file's remainder) or stays a per-unit declaration.
   ✅ **Ruling (3.600, integrator): it stays a per-unit declaration.**  The judgement is P-88 ①'s
   "how many consumers does this surface have": the four later `setjmp` sites (C 11999/12028/12433/12695/16967)
   are **all in this unit**, and `lisp.rs`'s mirror already carries `jmp_buf` - moving `setjmp`/`longjmp` there
   would be building a mirror for a second copy of one unit's TEST arm, which P-88 ⑦ argues against.  The
   `sys_longjmp` alternative stays out: it is a *behaviour* change (`_longjmp` does not restore the signal
   mask), not a spelling.  Nothing more is owed here.

25. **The one P-91 ④ reading this slice leaves**: `sfnt_interpret_call` (C 9832) calls
    `sfnt_interpret_run`, whose definition is C 11396 and therefore still unported.  The arm flip
    reports `E0425: cannot find function sfnt_interpret_run` (Evidence ②, the sixth error); it is
    an allowed, named reading and **not** a pass, and it closes when C 10664-13917 lands.
    **Slice 10 (status 3.697) reads the same one error and nothing else**: the five `time_t`/
    `timespec` `E0425`s the flip also carried were *not* this item - they were an undocumented
    include-graph hole, now fixed (Deviation 78 (a)), so this item is the flip's only remaining
    error, exactly as recorded.  ✅ **Closed in 3.701 (slice 14)**: the flip reads **0 errors** on
    that slice's text and the same 1 at HEAD - the `E0425` names the definition C 11396-11983,
    which the slice lands.  The 0-error arm produces an object (9,219,424 bytes) that nothing
    loads or runs (Evidence ⑨), so the closure is a compile reading, not a behavioural one.
26. **`docs/ports/macro-body-notes.tsv`: ten rows appended by this lane** (review them): the four
    `goto` rows (`sfnt_address_zp2`/`_zp1`/`_zp0`, `sfnt_interpret_isect`,
    `sfnt_normalize_vector`), the three `break,case,default,switch` rows (`sfnt_deltac`,
    `sfnt_deltap`, `sfnt_srounding_state`'s twin) and the two `case,switch` rows
    (`sfnt_interpret_if`, `sfnt_interpret_else`).  With them (and the corrected
    `PROJECT`/`DUAL_PROJECT` bodies) the `--check` reading has no `does not mention` row left.
    `docs/ports/excluded-macros.txt` needed nothing: the ranges define no macro of their own.
27. **The enum's home.**  `sfnt_interpreter_run_context` is defined in the unit (Deviation 75)
    because `sfnt.h` does not carry it — Owed 20's class for records.  If the integrator prefers
    a mirror, the move is mechanical but changes this file's delivered bytes and needs a fresh
    arm reading.   ✅ **Ruling (3.600, integrator): it stays in the unit**, the Owed 20 shape - a record `sfnt.h` does
   not carry has no mirror to live in, and P-88 ① gives a single consumer the unit.  The move to a mirror
   would change this file's delivered bytes and need a fresh arm reading for no gain; recorded, not owed.

28. **This lane's budgets, reported rather than hidden: `build-tu` 4 of 3 and `probe` 5 of 3.**
    The four `build-tu` runs are (i) the first splice, (ii) after the `TRAP!`/`as_ptr`/zero-init
    fixes, (iii) after the last six zero-inits and (iv) the delivered revision — every one exit 0
    with the same 336 bytes, so three of them bought only the "the delivered md5 is the measured
    md5" property (Evidence ①), the same reason Owed 11's overrun gives.  The five `probe` runs
    are the four error readings of Evidence ② (89, 48, 12, 6 errors — each a distinct, retired
    class except the last) plus (v) the delivered revision, which was re-measured *because* the
    `PROJECT`/`DUAL_PROJECT` bodies changed after run (iv): a change inside macro bodies no gate
    expands, so only a re-run could keep the reading on the delivered bytes.  The alternative -
    citing a reading taken on an older revision - is what this ledger's Evidence ① calls out, and
    this lane chose the overrun.
29. **Owed 6's ratchet row is still stale**, and this slice moves the ratio it would record:
    `tools/check-port-lines.py` reads 0 ratchet rows for this file, while the drift is now 37.9%
    (C 21,215 -> 13,174).  A batch close can take it with the parallel lanes' units, as Owed 6
    intended.

30. **The TEST arms' X11 surface, name by name (slices 19-20's measured debt).**  `include/xlib.rs`
    is the tree's mirror of `<X11/Xlib.h>` and the right home for the X11 half of the
    `feature = "test"` arms (its own header says it declares "the Xlib surface the *gated* arms of
    `src/*.c` and `src/*.h` name"), but it carries only what the units that came before needed.  The
    names these arms still owe it, with the header line each would cite (all present on this machine
    under `/opt/homebrew/include`): `XOpenDisplay` (Xlib.h 1479), `XCloseDisplay` (2170),
    `XDestroyWindow` (2253), `XDrawPoint` (2371), `XFlush` (2517), `XNextEvent` (2851),
    `XSelectInput`, `XMapWindow`, `XCreateSimpleWindow`, `XCreatePixmap`, `XFree`, `XFreeGC`,
    `XListDepths`, `XLoadFont`, `XInitImage`, `XPutImage`, `DefaultRootWindow`/`DefaultScreen`/
    `DefaultVisual`, `WhitePixel`/`BlackPixel`, and the constants `KeyPress`/`Expose` (X11/X.h 181's
    family), `ZPixmap`, `MSBFirst`, `LSBFirst` (the mirror already has `GCForeground`, `GCFont`,
    `ExposureMask` and `None` in its `x11_constants`); `XImage` and `XGCValues` are already there.
    Measured: each of the names above has **0 hits** in the mirror today.  **`<X11/extensions/
    Xrender.h>` has no mirror at all** - it is a symlink to libxrender 0.9.12 on this machine, and
    `xlib.rs`'s header note says a second mirror "would need a second first line and a second
    ledger" - so `sfnt_x_raster`'s XRender half is owed with it: `Picture`, `XRenderPictFormat`,
    `XRenderColor`, `XRenderQueryExtension`, `XRenderQueryVersion`, `XRenderFindVisualFormat`,
    `XRenderFindStandardFormat`, `XRenderCreatePicture`, `XRenderCreateSolidFill`,
    `XRenderFillRectangle`, `XRenderComposite` and the `PictStandardA8` (Xrender.h 506, = 2) /
    `PictOpSrc` / `PictOpOver` constants.  Nothing here blocked transcription - both slices
    delivered - and nothing compiles these arms either: their gate is `feature = "test"`, which no
    gate sets, and Evidence ⑭ measured what flipping it costs today (3,101 errors, the bulk of them
    in the mirrors' own gates).

### Not claimed

Every line of these slices is behind gates the frozen configuration leaves false: **① is an
empty object, the arm reading (②) produced a 36,240-byte object and slice 15's `--emit obj`
object is 90,168 bytes - all three read by the compiler and never by a running program**, so there is
**no runtime evidence of any kind** for this port — no font was opened, no cmap subtable was
parsed, no table directory was read, no glyph was outlined, no hmtx/name/meta/cvt/fpgm/prep table
was read, no `read`/`lseek` returned a byte this port can claim to have interpreted, and no 26.6
value was ever rounded, floored or scaled.  C 20203-21215 is **not ported here**: `sfnt_test_uvs` and `main` - the file's last two functions,
both `#ifdef TEST` text (Owed 21).  The delivered `--check` reads `OK` and that reading is exact and
narrow: every *declaration* the tool compares is present and equal and the line count is inside
tolerance; the tool cannot see that one block is still unported (it drops `#ifdef TEST` text) and it
says nothing about behaviour.  This slice's own text again has no type-checking reading
(`feature = "test"` is carried by no gate; flipping it reads 3,101 errors, the bulk in the mirrors -
Evidence ⑭) - only the syntax check the frozen probe performs, which this block passed on its first
run.  The `.offset (i as isize)`  The `.offset (i as isize)`  The `.offset (i as isize)`  The `.offset (i as isize)` / `&raw mut` / `overflowing_*` / `as` rewrites are
compile-level and were measured by the compiler, not by a running program; so is the whole
`feature = "no_ndebug"` arm (nothing flips it yet — Owed 8), and so are the four `MAYBE_UNUSED
TEST_STATIC` readers' TEST arm and C's `#ifdef TEST` hooks in the remainder (Owed 9, Owed 12).
Slice 6's twenty new definitions are **readers and helpers, called by nothing in the port yet**:
`sfnt_read_hmtx_table`/`sfnt_read_name_table`/`sfnt_read_meta_table`/`sfnt_read_ttc_header`/
`sfnt_read_cvt_table`/`sfnt_read_fpgm_table`/`sfnt_read_prep_table` are C's public table readers
whose callers (`sfntfont.c`, `sfnt.c`'s own interpreter setup and TEST harness) are all in the
remainder; `sfnt_lookup_glyph_metrics`, `sfnt_scale_metrics` and `sfnt_get_scale` are metrics
helpers nothing calls yet; `sfnt_find_name`/`sfnt_find_metadata` are lookups whose C callers are in
the remainder; and the eight 26.6 helpers feed `sfnt_make_interpreter` and the interpreter
instructions, none of which is ported.  The `E0369`/`E0605` control readings of Evidence ② are
*compiler* readings of the errors this slice retired, not behaviour tests, and the 4,952 bytes the
arm object grew (31,288 → 36,240) are a compile artefact, not evidence that anything runs.

Slice 7's twenty-four new definitions are the same kind of statement.  C 10225-10663's twenty are
the graphics-state machinery **nothing calls yet**: `sfnt_validate_gs` is reached only by
`sfnt_set_freedom_vector`/`sfnt_set_projection_vector` (also this slice), and those two and the
eight `round`/`sfnt_move*` callbacks are reached only from C 6316-6541's interpreter setup, which
is still the first unported block - so the `switch` on `round_state`, the two `0o40000` tests, the
`vector_dot_product` computation and the three `move` arms have never run, and the 39,664-byte arm
object (②) is a compile artefact of their presence, not evidence.  C 13918-14554's four readers
(`sfnt_read_fvar_table`, `sfnt_read_gvar_table`, `sfnt_read_avar_table`,
`sfnt_read_packed_points`) are C's public table readers whose callers (`sfnt_init_blend`,
`sfnt_read_cvar_table` and the remainder's glyph-variation code) are not ported either: no font
file was opened, no `read`/`lseek`/`xmalloc`/`sa_stack_alloc` call inside them was executed, no
`fvar`/`gvar`/`avar` byte was swapped, no packed point was unpacked and no delta was
sign-extended.  The `E0308` control reading of ② is a *compiler* reading of two casts this slice
added, the 3,424 bytes the arm object grew (36,240 → 39,664) are a compile artefact too, the
`(uint16_t *) -1` return of `sfnt_read_packed_points` (C 14483) has never been compared against
anything, and the `fail`-tail defect Deviation 51 reports is a *reading of the source*, not
something this lane ran.

Slice 8's fourteen definitions are the same kind of statement, one group further along.  The
**`cvar` reader** `sfnt_read_cvar_table` (C 14626-14950) and the two appliers
`sfnt_vary_simple_glyph`/`sfnt_vary_compound_glyph` (C 15488-15851, C 15861-16212) are C's public
table reader and its glyph-variation entry points, and **nothing in the port calls them**: their C
callers (`sfntfont.c`, `sfnt_init_blend`'s caller, the interpreter's variation setup and C
16448+'s TEST programme) are all in the remainder, so no font file was opened, no `read`/`lseek`/
`xmalloc`/`xrealloc`/`sa_stack_alloc` call inside them was executed, no `cvar` tuple was read, no
packed delta was unpacked, no `sfnt_swap16` touched a byte of a real table, and no tuple scale,
`gvar` outline or `OS/2`/`post` header was ever computed.  `sfnt_init_blend`/`sfnt_free_blend`/
`sfnt_normalize_blend` (C 14959-15074) are the same: the `xmalloc`-and-split of `coords`/
`norm_coords`, the three-way normalize and the `avar` walk have never run — nothing allocates a
`struct sfnt_blend` in the port yet.  `sfnt_compute_tuple_scale` (C 15096-15174) and the three
interpolators (C 15184-15457) are reached only from the two appliers and from
`sfnt_vary_interpreter`, so their `MIN`/`MAX` fit tests, their `sfnt_multiply_divide*` calls and
their contour wrap-around have never been evaluated; `sfnt_infer_deltas`'s three-way bounds
`return` (C 15449-15452) is the only early exit of that group and has never been taken either.
`sfnt_vary_interpreter` (C 16221-16277) needs `struct sfnt_interpreter`, which the port does not
build (C 6316's `sfnt_init_graphics_state` and C 6358-6522's `sfnt_make_interpreter` are still the
remainder's first block), so its `cvt[i] += delta` (Deviation 56) has never been executed and the
C-level index oddity there is a *reading of the source*, not something this lane ran.
`sfnt_read_OS_2_table`/`sfnt_read_post_table` (C 16292-16444) are readers whose callers
(`sfntfont.c`'s font-attribute code) are in the remainder, so neither has opened a table.
The **frozen reading (①) is an empty 336-byte object and the arm reading (②) produced an object
that nothing ran** — and on this revision ② does not even reach an object: its five `E0425`s are
the mirror graph's (Owed 16), with **this slice's own text clean of all 17 errors the first run
reported** (Evidence ②).  The `.offset (… as isize)` / `&raw mut` / `wrapping_*` / `as` rewrites
are compile-level and were measured by the compiler, not by a running program, and this file still
has **no runtime evidence of any kind**.  C 16448-21215 is **not ported here**; the 230
`- missing …` rows of Evidence ③, the 80 missing functions and the 49.3% drift are that boundary
(C 14555-16447 moved fourteen functions and 2,316 Rust lines out of it).

Slice 9's two ranges are the same kind of statement, one layer further into the interpreter.
`sfnt_init_graphics_state` (C 6316-6344) and `sfnt_make_interpreter` (C 6358-6522) are the
*setup* path nothing in the port calls: the interpreter's one allocation, its `ckd_add`-sized
zones, the CVT load and the `sfnt_validate_gs` call have never run, no `xmalloc`/`memset` inside
them was executed, and `sfnt_interpret_trap` (C 6550-6557) has never trapped — its `longjmp` would
need a `setjmp` frame and there is none in this unit.  The 46 helper definitions of C 8161-10224
are reached *only* from `sfnt_interpret_run` (C 11396) and the instruction macros, neither of
which is ported: so the whole zone-addressing family, `sfnt_interpret_utp` … `sfnt_interpret_else`,
`sfnt_deltac`/`sfnt_deltap`, `sfnt_sqrt_fixed`, `sfnt_normalize_vector` and the rest have never
executed a statement.  Nothing here has read a font, addressed a zone, projected a vector,
rounded a distance, unpacked a delta, or touched the graphics state — the numbers in the bodies
(`040000`, `0104`, `0x200000`, the sixteen delta magnitudes) are C's text transcribed, and every
`sfnt_f26dot6`/`sfnt_fixed`/`sfnt_f18dot14` conversion is a *reading* of C's implicit conversions,
not a measured one.  The 50 `// C a-b` comments, the 26 P-57 macro copies and the ten
`macro-body-notes.tsv` rows are hand-checked against C's lines; the compiler read the file twice
(① and ②) and neither reading is a behaviour test.  The `E0381`/`E0423`/`E0308` control readings
of Evidence ② are *compiler* readings of errors this slice retired, and the one error left
(`sfnt_interpret_run`) is a call into text that is not ported yet.  C 6559-8159 and C 10664-13917
are **not ported here** — the 174 `- missing …` rows of Evidence ③ and the 31 functions they name
are that boundary, and the macros' absence is a *place* rule (P-57), not a gap.
   ✅ **Ruling (3.600, integrator): still deferred, and no tolerance row.**  The ratchet row stays with
   Owed 6's batch close (`check-port-lines.py --write-pending` would encode the parallel lanes' in-flight
   units into the table), and this unit gets **no** `line-drift-tolerance.tsv` row: it is still a partial
   port (37.9% drift), and tolerance rows are only written for finished files (3.434 ⑤, 3.489).

   ✅ **Ruling (slice 24): the two claims above are superseded, and this is the correction.**  (i)
   **`sfnt_test_uvs` and `main` are transcribed now.**  The paragraph above says "C 20203-21215 is
   **not ported here**"; that was true of the revision it was written on, and the range table's
   last row plus Evidence ⑲ are this revision's reading.  (ii) **The unit has a
   `line-drift-tolerance.tsv` row now** (0.09, 8.2%).  The 3.600 ruling above says rows are written
   only for finished files - and the file *is* finished, so its **reason** ("it is still a partial
   port") no longer holds while its **rule** is exactly what licenses the row.  Nothing else in
   this section changes: the unit's forty-one TEST-arm items and the `main` that drives them are
   still compiled by no gate (`feature = "test"` has no `xtask` `FLIPS` entry - Evidence ⑭), so
   this slice's text has **no type-checking reading at all** - only the frozen probe's syntax pass,
   which is what caught its six errors (Evidence ⑲).  `main` in particular has never opened a font,
   read a cmap subtable, benchmarked an outline, rastered a glyph or printed a timing: the
   `clock_gettime` constant above is a *header* reading, not a measurement of this program, and the
   `--check`'s `OK` is a declaration reading the tool itself calls narrow.
