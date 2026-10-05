# emacs/src/sfnt.c (21,215 lines) — slices 1-2: C 1-2407

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2645 adds `sfnt.o` to `ANDROID_OBJ` only inside the `XCONFIGURE = android`
> branch (the `REALLY_ANDROID` one, 2620-2654) — `android.o` is added one branch earlier, at
> 2598, which is why `src/android.c`'s file gate is `have_android` alone and this one is the
> pair — and `docs/baseline/frozen-build-sources.txt` lists no `sfnt.o`.  What follows is P-86
> ②'s **C line-range table** for these slices, the `## Evidence` readings and the
> `## Deviations`.  Port: `femacs/src/sfnt.rs` (2,811 lines, `grep -c ''`; `extract-abi.py`'s
> own count, 2,811, is the same newline count), both arms of every `#if` carried.
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
> **The slices are C 1-2407.**  Slice 1 is C 1-1112 (through the `}` of `sfnt_read_cmap_table`
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
> (C 1908-2407).  **The next live definition is C 2413's `sfnt_read_glyph`**, and the table's
> last row is "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 22-39's include graph) | a *real* Android configuration |
| C 41 `#ifdef HAVE_MMAP` … C 43 (`#include <sys/mman.h>`) | **no gate**: `HAVE_MMAP` is 1 on this target (`docs/baseline/config/darwin-arm64.macros.tsv` 1469), so the include is live in every configuration.  Slice 2 is the first text here to *name* what the include provides, so the unit now carries C's four prototypes and the five constants in a `#[cfg (file gate)]` block at C 41's site (the P-88 ① shape; Owed 4 records that `docs/baseline/feature-coverage.tsv` has no `have_mmap` row) | always, on this target |
| C 45 `#if defined __GNUC__ && !defined __clang__` … C 47 (`#pragma GCC diagnostic`) | **dropped**: a diagnostic pragma has no Rust counterpart (the `-Wstringop-overflow` class) | never |
| C 49 `#ifdef TEST` … C 134 (the harness: `memory_full_up`, the six allocators, `TEST_STATIC`, `eassert`, `AVOID`) | `all (file gate, feature = "test")` — `femacs/xtask/Cargo.toml` 564 registers C's `TEST` as `test` | a test build |
| C 136 `#else` … C 139 (`#define TEST_STATIC` + `#include "lisp.h"`) | `all (file gate, not (feature = "test"))` | the frozen-shaped configuration |
| C 128 / C 137 (`#define TEST_STATIC static` / `#define TEST_STATIC`) | `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]` on the nine readers C exports through it (slice 1's `sfnt_read_table_directory`/`sfnt_read_cmap_table`; slice 2's `sfnt_lookup_glyph`, `sfnt_read_head_table`, `sfnt_read_hhea_table`, `sfnt_read_loca_table_short`, `sfnt_read_loca_table_long`, `sfnt_read_maxp_table`, `sfnt_read_glyf_table`) — a construct PORTING.md does not yet cover, see Deviations 1 and Owed 1 | linkage only; the item is on the file gate either way |
| C 224 / C 232 `#ifndef WORDS_BIGENDIAN` (the two swap helpers' bodies) | `not (feature = "words_bigendian")` (`femacs/xtask/Cargo.toml` 614); the other arm is C's empty body — the big-endian byte order needs no swap, so it is not a branch the port invents | little-endian |
| C 1827 `#if defined HAVE_MMAP && !defined TEST` … C 1906 `#endif` | `all (file gate, not (feature = "test"))`: `HAVE_MMAP` is 1 on this target and no `have_mmap` feature is registered, so the port carries that half live (C 41's ruling, above) and gates C's `!defined TEST` half.  **C's `#if` has no `#else`**, so the other arm is empty text | a real Android non-TEST configuration |
| C 1871 `#ifdef HAVE_POSIX_MADVISE` … C 1874 `#elif defined HAVE_MADVISE` … C 1876 `#endif` | `not (feature = "no_have_posix_madvise")` for the live `posix_madvise` arm; `feature = "no_have_posix_madvise"` + `cfg! (feature = "have_madvise")` for the `madvise` arm (the `pdumper.rs` 5281-5284 shape; `docs/baseline/feature-coverage.tsv` 241/436 register both names).  The third arm — neither macro — has no text in C | `posix_madvise`: always on this target (`HAVE_POSIX_MADVISE` is 1, 1492; `HAVE_MADVISE` is 1 too, 1445) |
| C 1837 / C 1891 (`sfnt_map_glyf_table`/`sfnt_unmap_glyf_table` are not `TEST_STATIC`: sfnt.h declares them `extern` at C 1490-1491) | plain `#[unsafe(no_mangle)] pub unsafe extern "C"` under the file gate + `not (feature = "test")` — no `cfg_attr` | a real Android non-TEST configuration |

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
| 2408-21215 | **the remainder** — the blank at C 2408 and from C 2409's comment through C 21215: `sfnt_read_glyph` (**the next live definition**, C 2413), `sfnt_free_glyph` (C 2529-2538), the compound-glyph decomposition (C 2551-3595), the outline builders and the rasterizer (C 3596-6426), and the whole TrueType interpreter, `sfnt_interpret_*` (C 6427-21215, including the `#ifdef TEST` programs at C 6437-11000) | not this slice |

The thirty-six rows above tile C 1-2407 **contiguously** (blank lines included) rather than
skipping the blanks between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/sfnt.rs` (this file's header's mirror) to this same `sfnt.md` through its
bare-stem fallback, so a table with gaps at blanks would leave `src/sfnt.h`'s non-blank lines
uncovered and fail (the `android.md`/`w32term.md` shape).  The last row reaches C 21215, which is
what the gate requires of the final row.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnt-lane/cargo cargo run -q -p xtask -- build-tu src/sfnt.c`
→ `built src/sfnt.rs -> ../build/femacs/sfnt.o`, **exit 0**, and `build/femacs/sfnt.o` is
**336 bytes** — an empty object, measured on the final 2,811-line revision.  That is the expected
reading and **not** a proof of the new text: every item sits behind the file gate, so under the
frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute.  Two
further `build-tu` runs are part of the same evidence: slice 1's first run reported its syntax
errors (`unclosed delimiter` ×2, `expected {, found header` ×1), and slice 2's first run
reported exactly two (`\`<\` is interpreted as a start of generic arguments for \`size_t\``, at
`_root.rs` 2033 and 2268 — a cast run against a following `<`, now parenthesised; Deviation 12).

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt-lane/sfnt_arm.o --crate-type lib --emit obj
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 0**, `0 errors`, and the object is **10,128 bytes**; the four warnings are all in
*included mirrors*, none in this unit's own text:
1. `lib/include/stdbit.in.rs` 644 — "unused doc comment";
2. `src/include/lisp.rs` 3934 — "unused doc comment";
3. `lib/include/unistd.in.rs` 54 — "suspicious definition of the runtime `read` symbol";
4. `lib/include/unistd.in.rs` 55 — the same for `write`.

(The last two are a rustc lint that fires on the mirror's own declarations; they are not this
unit's text and this slice cannot remove them.)  The earlier failed arm reading — the control
that shows the harness does check this unit — was **8 errors**, all named at `_root.rs` lines in
this file: **6 × `E0308`** (`words[0]`/`words4[3]`, a `uint16_t` written into the `int16_t`
scale fields; the port now casts, C's own narrowing), **1 × `E0608`** (raw-pointer indexing
`glyph_index_array[j]`; the port writes `*(… .offset (j as isize))`) and **1 × `E0423`**
(`MIN` is the port's `macro_rules!`, so the call is `MIN! (…)`).  Slice 1's own control reading
was 8 errors of other classes (`E0428`/`E0530`/`E0600`/`E0381`/`E0308`), retired by Owed 1 and
the fixes below.

**③ Inventory (the live checklist).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c femacs/src/sfnt.rs` → **FAIL**, with
**343 `- missing …` lines — 184 functions, 153 macros, 5 constants (`SFNT_BLOCK_STEPS`,
`SFNT_POLY_ALIGNMENT`, the three `SFNT_RUN_CONTEXT_*`), 1 file-scope object (`build_outline_context`)**
— down from 364 (205/153/5/1) at the end of slice 1, i.e. **all 21 of slice 2's own functions
match**, and 4 named rows:
`- function sfnt_lookup_glyph_4 does not mention sfnt_glyph` and
`- function sfnt_lookup_glyph does not mention case, switch` (both owed to
`docs/ports/macro-body-notes.tsv`, Owed 3), the nine
`- function … is not static in C but has no #[unsafe(no_mangle)]` rows for the `cfg_attr`-gated
readers (Owed 1; `sfnt_map_glyf_table`/`sfnt_unmap_glyf_table` carry a plain `#[unsafe(no_mangle)]`
and are *not* listed), and `- line count drifted 86.7%: C 21215 vs Rust 2811` (the whole file
against two slices; slice 1's reading was 93.9%/1,298).

**④ Other gates (repo root, final revision).**
`python3 tools/check-dead-file-coverage.py` → **29 ledger(s) bound (a C range table), 0
failure(s)**, and this ledger is one of them: `info docs/ports/sfnt.md: 36 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 36 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback above). —
`python3 tools/check-declarations.py` → **10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures** (slice 1's first reading named 14/1, and the one failure was
`femacs/src/android.rs`'s `PACKAGE_STRING` literal — another lane's in-flight file, since
repaired). —
`python3 tools/check-ledger-ticks.py` → **234 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 356 row(s) with no function tick**, 0 row(s) without a
ledger. —
`python3 tools/check-mirror-dupes.py` → **19 duplicate declaration(s), 19 ratchet row(s), 0
failure(s)**. —
`python3 tools/check-cond-eval.py src/sfnt.c` → **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (slice 2 adds no new `#if` to the file: C 1827 and C 1871 are inside the
31).

## Deviations

1. **`TEST_STATIC` is a construct PORTING.md does not cover yet.**  C's `#define TEST_STATIC
   static` (C 128, the TEST arm) / `#define TEST_STATIC` (C 137) decides the linkage of the
   readers this file exports — nine of them now (slice 1's two plus slice 2's seven).  The port's
   expression is `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, which is the same
   bit and needs no second copy of any body.  It is the tree's heaviest user of the attribute
   (measured on this revision: `rg -c cfg_attr femacs/src/*.rs` → `femacs/src/sfnt.rs:12` — the
   nine attributes plus three mentions in prose — and `femacs/src/w32proc.rs:2`, another lane's
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

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`TEST_STATIC`'s Rust form** — either a `docs/ports/excluded-macros.txt` row (`kind=syntax`,
   "the port expresses the linkage with `#[cfg_attr (not (feature = "test"), unsafe
   (no_mangle))]`", the `AVOID` row's shape) or a PORTING.md §11 entry; today `--check` counts it
   as nine named rows and a missing macro (Evidence ③).  ✅ **Done in 3.427** for the
   `E0428` half — **`femacs/src/include/sfnt.rs`: the declaration run now sits in
   `pub mod sfnt_decls { use super::*; … } use sfnt_decls::*;`** (the `include/android.rs` shape).
2. **Two `docs/ports/token-aliases.tsv` rows**: `(T *) -1` → `usize::MAX as *mut T`, and `p[i]`
   on a raw pointer → `*(p.offset (i as isize))` (Deviation 4 / Deviations 9-10's note).
3. **`docs/ports/macro-body-notes.tsv` rows** — slice 1's `sfnt_read_cmap_table_1` →
   `case,default,switch` is in place; slice 2 owes **`sfnt_lookup_glyph` → `case,switch`**
   (C 1437-1462's `switch` is the port's `match`, now with a `_ => {}` arm) and
   **`sfnt_lookup_glyph_4` → `sfnt_glyph`** (C 1270's `sfnt_glyph glyph;` is the *scalar*
   typedef, which the P-67 rename spells `sfnt_glyph_index`; `docs/ports/include-sfnt.md` owns
   that rename, and this is its first body-level cost).
4. **`have_mmap` (or an explicit ruling)** — C 1827's `#if defined HAVE_MMAP && !defined TEST`
   is carried as `not (feature = "test")` alone, because `docs/baseline/feature-coverage.tsv` has
   no `have_mmap` row and slice 1 already ruled C 41's `#ifdef HAVE_MMAP` live.  Either register
   the feature (and gate C 1827-1906 with it here and C 41-43 in slice 1) or record the ruling as
   the tree's answer for this macro.
5. ~~`time_t`/`timespec` for `lib/include/sys_time.rs`/`sys_select.rs`/`src/include/thread.rs`
   through `lisp.rs`'s chain~~ — **closed in 3.429**: root was P-89's materializer, and the arm
   now reads exit 0.  `<sys/mman.h>` (this slice), `<setjmp.h>` and the TEST arm's
   `<sys/wait.h>`/`<X11/*>` still have no mirror; slice 2's own text takes nothing from
   `<setjmp.h>`/`<sys/wait.h>`/`<X11/*>`, and takes `mmap`/`munmap`/`posix_madvise`/`madvise`,
   `PROT_READ`, `MAP_PRIVATE`, `MAP_FAILED`, `POSIX_MADV_WILLNEED` and `MADV_WILLNEED` from the
   use-site block (Deviation 13).
6. **`tools/check-port-lines.py --write-pending`**: this file's ratchet row is stale (86.7%
   drift) — deliberately left to a batch close, so that the rewrite does not encode the parallel
   lanes' in-flight units.

### Not claimed

Every line of these slices is behind gates the frozen configuration leaves false: **① is an
empty object and the arm reading (②) produced a 10,128-byte object that nothing ran**, so there
is **no runtime evidence of any kind** for this port — no font was opened, no cmap subtable was
parsed, no table directory was read, no glyph was outlined, no mapping was `mmap`'d, and no
`read`/`lseek` returned.  C 2408-21215 is **not ported here**; the 343 `- missing …` lines of
Evidence ③, the 184 missing functions and the 86.7% drift are that boundary.  The
`.offset (i as isize)` / `usize::MAX` / `overflowing_*` / cast rewrites are compile-level and were
measured by the compiler, not by a running program.  `sfnt_read_simple_glyph` and
`sfnt_read_compound_glyph` now exist but nothing calls them in this slice (`sfnt_read_glyph` is
C 2413, the next definition), and `sfnt_map_glyf_table`/`sfnt_unmap_glyf_table` are behind
`not (feature = "test")`, which the frozen configuration leaves false as well.
