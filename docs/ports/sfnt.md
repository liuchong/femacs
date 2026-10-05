# emacs/src/sfnt.c (21,215 lines) — slices 1-5: C 1-5451

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2645 adds `sfnt.o` to `ANDROID_OBJ` only inside the `XCONFIGURE = android`
> branch (the `REALLY_ANDROID` one, 2620-2654) — `android.o` is added one branch earlier, at
> 2598, which is why `src/android.c`'s file gate is `have_android` alone and this one is the
> pair — and `docs/baseline/frozen-build-sources.txt` lists no `sfnt.o`.  What follows is P-86
> ②'s **C line-range table** for these slices, the `## Evidence` readings and the
> `## Deviations`.  Port: `femacs/src/sfnt.rs` (6,251 lines, `grep -c ''`; `extract-abi.py`'s
> own count, 6,251, is the same newline count), both arms of every `#if` carried.
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
> **The slices are C 1-5451.**  Slice 1 is C 1-1112 (through the `}` of `sfnt_read_cmap_table`
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
> (C 5425-5451).  **The next live definition is C 5468's `sfnt_read_hmtx_table`** - C 5453's form
> feed and C 5455's "Glyph metrics computation." banner open the metrics readers - and the table's
> last row is "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 22-39's include graph) | a *real* Android configuration |
| C 41 `#ifdef HAVE_MMAP` … C 43 (`#include <sys/mman.h>`) | **no gate**: `HAVE_MMAP` is 1 on this target (`docs/baseline/config/darwin-arm64.macros.tsv` 1469), so the include is live in every configuration.  Slice 2 is the first text here to *name* what the include provides, so the unit now carries C's four prototypes and the five constants in a `#[cfg (file gate)]` block at C 41's site (the P-88 ① shape; Owed 4 records that `docs/baseline/feature-coverage.tsv` has no `have_mmap` row) | always, on this target |
| C 45 `#if defined __GNUC__ && !defined __clang__` … C 47 (`#pragma GCC diagnostic`) | **dropped**: a diagnostic pragma has no Rust counterpart (the `-Wstringop-overflow` class) | never |
| C 49 `#ifdef TEST` … C 134 (the harness: `memory_full_up`, the six allocators, `TEST_STATIC`, `eassert`, `AVOID`) | `all (file gate, feature = "test")` — `femacs/xtask/Cargo.toml` 564 registers C's `TEST` as `test` | a test build |
| C 136 `#else` … C 139 (`#define TEST_STATIC` + `#include "lisp.h"`) | `all (file gate, not (feature = "test"))` | the frozen-shaped configuration |
| C 128 / C 137 (`#define TEST_STATIC static` / `#define TEST_STATIC`) | `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]` on the eleven readers C exports through it (slice 1's `sfnt_read_table_directory`/`sfnt_read_cmap_table`; slice 2's `sfnt_lookup_glyph`, `sfnt_read_head_table`, `sfnt_read_hhea_table`, `sfnt_read_loca_table_short`, `sfnt_read_loca_table_long`, `sfnt_read_maxp_table`, `sfnt_read_glyf_table`) — a construct PORTING.md does not yet cover, see Deviations 1 and Owed 1 | linkage only; the item is on the file gate either way |
| C 224 / C 232 `#ifndef WORDS_BIGENDIAN` (the two swap helpers' bodies) | `not (feature = "words_bigendian")` (`femacs/xtask/Cargo.toml` 614); the other arm is C's empty body — the big-endian byte order needs no swap, so it is not a branch the port invents | little-endian |
| C 1827 `#if defined HAVE_MMAP && !defined TEST` … C 1906 `#endif` | `all (file gate, not (feature = "test"))`: `HAVE_MMAP` is 1 on this target and no `have_mmap` feature is registered, so the port carries that half live (C 41's ruling, above) and gates C's `!defined TEST` half.  **C's `#if` has no `#else`**, so the other arm is empty text | a real Android non-TEST configuration |
| C 1871 `#ifdef HAVE_POSIX_MADVISE` … C 1874 `#elif defined HAVE_MADVISE` … C 1876 `#endif` | `not (feature = "no_have_posix_madvise")` for the live `posix_madvise` arm; `feature = "no_have_posix_madvise"` + `cfg! (feature = "have_madvise")` for the `madvise` arm (the `pdumper.rs` 5281-5284 shape; `docs/baseline/feature-coverage.tsv` 241/436 register both names).  The third arm — neither macro — has no text in C | `posix_madvise`: always on this target (`HAVE_POSIX_MADVISE` is 1, 1492; `HAVE_MADVISE` is 1 too, 1445) |
| C 4547-4554's three file-scope `#define`s (`sfnt_add`/`sfnt_sub`/`sfnt_mul`), C 4959's `APPEND_STEP`, C 5011's `TRAPEZOID_AREA` and C 5083's `TRIANGLE_AREA` (the last three inside `sfnt_poly_edges_exact`), and their `#undef`s (C 5262, C 5265-5266) | `macro_rules!` at the same scope: file scope for the first three, the enclosing block for the three function-local ones — Rust's macros are textually scoped, so the `#undef`s have no text (Deviation 35) | every configuration (C's macros are unconditional) |
| C 4322 `#ifndef NDEBUG` … C 4429 (`sfnt_fill_span`'s `row_end` and its four `eassert`s) | `feature = "no_ndebug"` — the frozen configuration **defines** `NDEBUG` (`docs/baseline/config/darwin-arm64.macros.tsv` 2892), so this is dead text and P-51's rule names the feature after the non-frozen side (the `no_signbit`/`no_rint` shape); C's `#else` is empty, so there is no second arm.  The name is **not registered** yet — Owed 8 | a configuration with `NDEBUG` undefined |
| C 1837 / C 1891 (`sfnt_map_glyf_table`/`sfnt_unmap_glyf_table` are not `TEST_STATIC`: sfnt.h declares them `extern` at C 1490-1491) | plain `#[unsafe(no_mangle)] pub unsafe extern "C"` under the file gate + `not (feature = "test")` — no `cfg_attr` | a real Android non-TEST configuration |
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
| 5452-21215 | **the remainder** — C 5453's form feed, C 5455's "Glyph metrics computation." banner and the metrics/name-table readers from C 5468 (`sfnt_read_hmtx_table` - **the next live definition** - `sfnt_lookup_glyph_metrics`, `sfnt_scale_metrics`, `sfnt_get_scale`, `sfnt_read_name_table`, `sfnt_find_name`) and the whole TrueType interpreter, `sfnt_interpret_*` (through C 21215, including the `#ifdef TEST` programs; the three `SFNT_RUN_CONTEXT_*` constants are its) | not this slice |

The forty-nine rows above tile C 1-5451 **contiguously** (blank lines included) rather than
skipping the blanks between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/sfnt.rs` (this file's header's mirror) to this same `sfnt.md` through its
bare-stem fallback, so a table with gaps at blanks would leave `src/sfnt.h`'s non-blank lines
uncovered and fail (the `android.md`/`w32term.md` shape).  The last row reaches C 21215, which is
what the gate requires of the final row.

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
above.

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt-lane/sfnt_arm.o --crate-type lib --emit obj
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 0**, `0 errors`, and the object is **31,288 bytes** (slice 4's reading was 25,856); the
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
reported in the lane's receipt and recorded in Owed 11.  **Slice 4's own control reading is the 3-error
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
run ended on `error[E0204]`/`error[E0277]` in `femacs/lib/include/signal.rs` (`sigaction`'s
`#[derive(Clone, Copy)]` against a `sigset_t` whose derives the materializer had hoisted onto the
other `#[cfg]` arm): another lane's **in-flight** mirror edit, reverted by the time of the reading
above — `git status` showed ` M femacs/lib/include/signal.rs` during that run and does not now.
P-86 ⑦ is exactly this case: while the include graph is broken the arm flip does not check this
unit's names, so that run is recorded as a blocked reading and **not** as a reading of this port.

**③ Inventory (the live checklist).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c femacs/src/sfnt.rs` → **FAIL**, with
**291 `- missing …` lines — 138 functions, 147 macros, 3 constants (the three
`SFNT_RUN_CONTEXT_*`), 0 file-scope objects, 0 fields** — down from 304 (147/153/4/0/0) at the end
of slice 4, i.e. **all 9 of slice 5's own functions match** (`sfnt_build_outline_fedges`,
`sfnt_insert_raster_step`, `sfnt_poly_edges_exact`, `sfnt_compute_fill`, `sfnt_poly_set_steps`,
`sfnt_poly_steps`, `sfnt_raster_steps`, `sfnt_raster_edges_exact`,
`sfnt_raster_glyph_outline_exact`; the function count falls by exactly 9), the **macro count falls
by six** (the three `sfnt_add`/`sfnt_sub`/`sfnt_mul` plus the function-local `APPEND_STEP`,
`TRAPEZOID_AREA` and `TRIANGLE_AREA`) and the constant count by one (`SFNT_BLOCK_STEPS`, C
4679-4682), leaving the three `SFNT_RUN_CONTEXT_*` as the file's whole constant debt.  **Seven
named rows are this slice's**: `- function sfnt_build_outline_fedges does not mention alloca`
(C 4737's `alloca` is the port's `sa_stack_alloc`, Deviation 23 - the `fedge` twin of the row the
integrator wrote in 3.463 for `sfnt_build_outline_edges`), `- function sfnt_insert_raster_step
does not mention goto` (C 4857's `goto found`), `- function sfnt_poly_edges_exact does not mention
float, goto` (C's `float` locals are PORTING.md 1.1's `f32`, and C 4984's `goto next` is a labelled
block), `- function sfnt_poly_steps does not mention float`, `- function sfnt_poly_set_steps does
not mention break, case, default, switch` (the `FALLTHROUGH` ladder of Deviation 38), `- macro
TRAPEZOID_AREA does not mention f, float` and `- macro TRIANGLE_AREA does not mention f` (C's
`(float)` casts and its `2.0f` suffixes; `TRIANGLE_AREA`'s `f` could be kept instead with the
`sfnt_fixed_float` mirror's `let f: f32 = 2.0;` spelling, if the tree prefers the token to the
row).  The seven are owed to `docs/ports/macro-body-notes.tsv` (Owed 10).  The other non-`missing`
line is `- line count drifted 70.5%: C 21215 vs Rust 6251` (the whole file against five slices;
slice 1's reading was 93.9%/1,298, slice 2's 86.7%/2,811, slice 3's 80.6%/4,113, slice 4's
75.6%/5,181).  **One row this slice introduced and closed before delivery**: `- struct sfnt_fedge:
missing fields ['top', 'bottom', 'step_y']`, read while those declarations shared C's own lines
(C 4713's `float x, top, bottom;` and C 4716's `float step_x, step_y;`) - Deviation 36 spells the
split, and the field class is empty again.  The nine `- function … is not static in C but has no
#[unsafe(no_mangle)]` rows and slice 4's `does not mention alloca` row are **gone from this
reading** (the integrator wrote the latter in 3.463; `3.441` taught the tool to read the `cfg_attr`
export, so this slice's new `TEST_STATIC` reader - `sfnt_raster_glyph_outline_exact` - produces no
row either).

**④ Other gates (repo root, final revision).**
`python3 tools/check-dead-file-coverage.py` → **46 ledger(s) bound (a C range table), 0
failure(s)** (that count moves as the parallel lanes land their ledgers: it read 45 earlier in
this run), and this ledger is one of them: `info docs/ports/sfnt.md: 49 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 49 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback above) — the row added by this slice is
`4544-5451`, and the old `4544-21215` row is now `5452-21215`. —
`python3 tools/check-declarations.py` → **10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures** (this slice adds no use-site block: every name it calls is already in this
unit's include graph, Deviation 39). —
`python3 tools/check-ledger-ticks.py` → **234 ledger(s) bound (a function tick to compare), 0
failure(s), 0 carried by another file, 356 row(s) with no function tick**, 0 row(s) without a
ledger (this ledger carries a range table, not a tick list). —
`python3 tools/check-mirror-dupes.py` → **43 duplicate declaration(s), 19 ratchet row(s), 24
failure(s)** — **not this unit**: every failure line names another lane's **in-flight, untracked**
`femacs/src/include/gtk.rs`, which re-declares cairo/GTK names `femacs/src/image.rs` and
`femacs/src/xwidget.rs` declare too; the same run read **19/19/0** a few minutes earlier, before
that file appeared, and neither of this lane's two files appears in any failure line. —
`python3 tools/check-cond-eval.py src/sfnt.c` → **31 conditions, 0 unknown, 0 disagreements
[VERIFIED]**, unchanged (the C file is frozen, so the evaluator's table cannot move with a port
change; C 4322's `#ifndef NDEBUG` is still the third `#if` inside the ported ranges, and slice 5's
range carries no `#if` at all - only `#define`/`#undef`, Deviation 35).

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
    `docs/baseline/feature-coverage.tsv` and `femacs/xtask/Cargo.toml` are outside this lane's
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
    `femacs/src/include/sfnt.rs` 192) and `xcalloc` (`src/include/lisp.rs` 3984's `(usize, usize)`
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

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`TEST_STATIC`'s Rust form** — either a `docs/ports/excluded-macros.txt` row (`kind=syntax`,
   "the port expresses the linkage with `#[cfg_attr (not (feature = "test"), unsafe
   (no_mangle))]`", the `AVOID` row's shape) or a PORTING.md §11 entry; today `--check` counts it
   as named rows and a missing macro.  ✅ **Done in 3.427** for the
   `E0428` half — **`femacs/src/include/sfnt.rs`: the declaration run now sits in
   `pub mod sfnt_decls { use super::*; … } use sfnt_decls::*;`** (the `include/android.rs` shape) —
   and ✅ **done in 3.441** for the export half: the tool now reads the `cfg_attr`, so the eleven
   `TEST_STATIC` readers produce no `- function … has no #[unsafe(no_mangle)]` row (Evidence ③'s
   reading has none; the `TEST_STATIC` macro itself is still one of the 153 missing macros).


2. **Two `docs/ports/token-aliases.tsv` rows**: `(T *) -1` → `usize::MAX as *mut T`, and `p[i]`
   on a raw pointer → `*(p.offset (i as isize))` (Deviation 4 / Deviations 9-10's note).
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
   `femacs/xtask/Cargo.toml` carries `no_ndebug = []  # !NDEBUG` and
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

### Not claimed

Every line of these slices is behind gates the frozen configuration leaves false: **① is an
empty object and the arm reading (②) produced a 31,288-byte object that nothing ran**, so there
is **no runtime evidence of any kind** for this port — no font was opened, no cmap subtable was
parsed, no table directory was read, no glyph was outlined, no curve was flattened, no span or
step was filled, no coverage value was computed, no mapping was `mmap`'d, and no `read`/`lseek`
returned.  C 5452-21215 is **not ported here**; the 291 `- missing …` lines of Evidence ③, the
138 missing functions and the 70.5% drift are that boundary.  The `.offset (i as isize)` /
`&raw mut` / `wrapping_*` / `overflowing_*` / `as` rewrites are compile-level and were measured by
the compiler, not by a running program; so is the whole `feature = "no_ndebug"` arm (nothing flips
it yet — Owed 8), and so is every floating-point op in the scaler (the arm flip only *compiles*
them).  Slice 5's nine new definitions are **wired only inside the slice**: `sfnt_poly_edges_exact`
is called by `sfnt_raster_edges_exact`, `sfnt_raster_edges_exact` and `sfnt_poly_steps` reach
`sfnt_raster_glyph_outline_exact`/`sfnt_raster_steps` as `Some (…)` values, `sfnt_compute_fill`
and `sfnt_poly_set_steps` are `sfnt_poly_steps`' helpers, `sfnt_insert_raster_step` is reached only
through `APPEND_STEP!`'s expansions, and `sfnt_build_outline_fedges` only from
`sfnt_raster_glyph_outline_exact` — whose C caller (`sfntfont.c` 2494) is outside the port so far;
`sfnt_raster_glyph_outline_exact` is a `TEST_STATIC` export no other unit names yet.  The three
`sfnt_add`/`sfnt_sub`/`sfnt_mul` macros have **no call site at all**: the interpreter that expands
them (C 7257 onwards) is in the remainder.  Slice 3's `sfnt_read_glyph`/`sfnt_free_glyph` and
slice 4's `sfnt_build_glyph_outline`/`sfnt_raster_glyph_outline` are still called from nowhere in
the port (`sfntfont.rs` has not reached `sfntfont.c` 2353/2492/2494).  The `E0308` control
readings of Evidence ② are *compiler* readings of the errors this slice retired, not behaviour
tests, and the 5,432 bytes the arm object grew (25,856 → 31,288) are a compile artefact, not
evidence that anything runs.
