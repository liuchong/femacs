# emacs/src/sfntfont.c (4,222 lines) — slice 1: C 1-1240, slice 2: C 1241-1426

> **P-86's unit `src/sfntfont.c`, ported in slices.**  The frozen configuration compiles nothing
> here: `configure.ac` 2645 adds `sfntfont.o` to `ANDROID_OBJ` only inside the `XCONFIGURE =
> android` branch (the `REALLY_ANDROID` one, 2620-2654 — `sfnt.o` is on the same line, `android.o`
> one line earlier at 2598; 2605-2606 is the other branch, where `ANDROID_STUBIFY` is defined), and
> `docs/baseline/frozen-build-sources.txt` lists no `sfntfont.o`.  What follows is P-86 ②'s **C
> line-range table** for this slice, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/sfntfont.rs` (**4,210 lines** after slice 11, `grep -c ''`), both arms of every `#if`
> carried.  **The line-count drift row is gone** as of this slice: 4,210 against C's 4,222 is inside
> the 5% the rule allows (Evidence ㉖).
>
> **This ledger replaces the previous revision's conclusion** ("the file is not in the frozen
> build, so the port is the empty unit plus a note"): P-86 ① rules that a three-line header
> comment is *not started*, not *done* — the `docs/ports/sfnt.md` and `docs/ports/android.md`
> sequence.
>
> **One measurement differs from `src/android.c`'s, and it decides what the tool can say.**
> `src/sfntfont.c` **is** in `docs/baseline/config/cond-eval-verified.txt` (line 355, between
> `src/sfnt.c`/`src/sfnt.h` at 353-354 and `src/sfntfont.h` at 356), so `tools/extract-abi.py` evaluates its `#if`s and
> builds a real inventory — the frozen evaluation of its 27 `#if`s names **57 functions, 7 types
> and 13 file-scope objects** for this file.  The tool prints its "not compiled by
> the frozen configuration … nothing in this file is required here" note and *clears* that
> inventory only when the port has no `fn` at all (`tools/extract-abi.py` 2225-2227); the 11-line
> placeholder had none, which is why `--check` read `OK` on it.  This slice does, so `--check` is
> **not vacuous here**: its reading (Evidence ④) is a live coverage statement about the 4,222 C
> lines and counts out what the remaining slices owe.
>
> **The slice is C 1-1240** (through the `}` of `sfnt_enum_font`).  P-86 ④ says to stop at a
> whole-item boundary: the next definition is `sfntfont_charset_for_name` (C 1246-1267) under C
> 1244's `/* Font discovery and matching.  */` banner, so this slice ends at C 1240's `}`.  **The
> next live definition is C 1246's `sfntfont_charset_for_name`**, and the table's last row is "the
> remainder".  Six blocks: the include graph and the file's own records (C 1-169), the coding-system
> helpers (C 170-320), the name-table decoders and the style parser (C 321-574), the design-language
> and registry helpers (C 575-766), the cmap identification, registry grep and duplicate-font
> replacement (C 767-943), and the two font enumerators (C 944-1240).
>
> **Slice 2 is C 1241-1426** - C 1244's `/* Font discovery and matching.  */` banner and the three
> cmap selectors: `sfntfont_charset_for_name` (C 1246-1264), `sfntfont_charset_for_cmap`
> (C 1269-1334) and `sfntfont_select_cmap` (C 1344-1425).  P-86 ④'s boundary is the `}` of
> `sfntfont_select_cmap` (C 1425).
>
> **Slice 3 is C 1427-1604** - the cmap reader and the character lookup: `sfntfont_read_cmap`
> (C 1431-1496), `sfntfont_glyph_valid` (C 1502-1515), `sfntfont_lookup_char` (C 1524-1585) and
> `sfntfont_registries_compatible_p` (C 1596-1603), with C 1427-1430's, C 1498-1501's, C 1517-1522's
> and C 1587-1594's comments.
>
> **Slice 4 is C 1605-1871** - the specification matcher and the registry helper: `sfntfont_list_1`
> (C 1613-1824, under C 1605-1611's comment, with its five `goto fail` legs, the instance loop and
> the extras block), the two file-scope statics `sfnt_vendor_name` (C 1827) and `sfnt_font_driver`
> (C 1830), and `sfntfont_registry_for_desc` (C 1841-1871, under C 1832-1839's comment).  The
> boundary is C 1871's `}`.
>
> **Slice 5 is C 1873-2050** - the entity builder and the two driver entry points:
> `sfntfont_desc_to_entity` (C 1879-1933), `sfntfont_compare_font_entities` (C 1939-1959), the subr
> record `Scompare_font_entities` (C 1964-1971), `sfntfont_list` (C 1977-2027), `sfntfont_match`
> (C 2032-2042) and C 2046-2050's anonymous enum (the two cache sizes), each with its own comment.
>
> **Slice 6 is C 2051-2172** - the caching subsystem's records and helpers: C 2052-2058's comment,
> `struct sfnt_outline_cache` (C 2059-2072), `struct sfnt_raster_cache` (C 2074-2084),
> `struct sfntfont_get_glyph_outline_dcontext` (C 2086-2102), and the four helpers
> `sfntfont_get_glyph` (C 2108-2134), `sfntfont_free_glyph` (C 2139-2142), `sfntfont_get_metrics`
> (C 2150-2158) and `sfntfont_dereference_outline` (C 2164-2172).  The three records' layouts are
> **measured** (Evidence ⑮).  The boundary is C 2172's `}`.
>
> **Slice 7 is C 2173-2559** - the caching subsystem's rest: `sfntfont_get_glyph_outline`
> (C 2198-2412, whose C 2174-2196 comment is its own), `sfntfont_free_outline_cache` (C 2418-2437),
> `sfntfont_dereference_raster` (C 2443-2451), `sfntfont_get_glyph_raster` (C 2458-2534) and
> `sfntfont_free_raster_cache` (C 2540-2559).  The boundary is C 2559's `}`: C 2561's form feed and
> C 2563's `/* Opening fonts.  */` banner open the next block.  **The next live definition is
> C 2633's `sfnt_open_tables`** (C 2565-2631 is `struct sfnt_font_info`), and the table's last row
> carries it.
>
> **Slice 8 is C 2560-2742** - the "Opening fonts" block's first four items: C 2561's form feed and
> C 2563's banner, `struct sfnt_font_info` (C 2565-2645, whose 576-byte layout is Evidence ⑲), the
> `open_fonts` list (C 2651), `sfntfont_lookup_glyph` (C 2658-2679) and `sfntfont_probe_widths`
> (C 2692-2742), with C 2681-2682's forward declaration of `sfntfont_measure_pcm` carried as a
> use-site declaration (Deviation 23 (b)).  The boundary is C 2742's `}`: **the next live definition
> is C 2753's `sfntfont_setup_interpreter`**, and the table's last row carries it.
>
> **Slice 9 is C 2743-2915** - `sfntfont_setup_interpreter` (C 2753-2860, under C 2744-2751's
> comment: the uninstructable-family regexp test, the cvt/fpgm/prep load, `sfnt_make_interpreter`,
> the two interpret passes with their `message_with_string` logging and the `bail1`/`bail` pair) and
> `sfnt_close_tables` (C 2865-2915, under C 2862-2863's comment).  The boundary is C 2915's `}`:
> **the next live definition is C 2920's `sfnt_open_tables`** (its comment is C 2917-2918), and the
> table's last row carries it.
>
> **Slice 10 is C 2916-3113** - `sfnt_open_tables` (C 2920-3113, under C 2917-2918's comment): the
> `xzalloc`, the open/seek, `sfnt_read_table_directory`'s `(T *) -1` test, the cmap/UVS selection and
> the three frees, the `HAVE_MMAP` glyf-map fallback, the loca/hmtx/instruction/distortion table
> reads, and C 3077-3112's six-label `bailN` ladder (Deviation 25 (a)).  The boundary is C 3113's
> `}`: **the next live definition is C 3119's `sfnt_reference_font_tables`** (its comment is
> C 3115-3116), and the table's last row carries it.
>
> **Slice 11 is C 3114-3472** - the reference-counting pair and the file's biggest function:
> `sfnt_reference_font_tables` (C 3119-3134), `sfnt_dereference_font_tables` (C 3140-3151) and
> `sfntfont_open` (C 3156-3472, under C 3153-3154's comment: the pixel-size/instance resolution, the
> `font_info` initialisation, `sfnt_reference_font_tables`, the table copies, the charset and
> ascent/descent/width arithmetic, the style `ASET`s, the interpreter setup, the blend/instance
> block with its `cancel_blend:` forward label, `sfntfont_probe_widths`, the XLFD and fullname
> properties, the `open_fonts` link, and C 3466-3471's `bail6:`/`bail:` pair).  The boundary is
> C 3472's `}`: **the next live definition is C 3483's `sfntfont_encode_char`** (the
> `/* Metrics computation … */` banner is C 3476), and the table's last row carries it.
>

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 22-42's include graph) | a *real* Android configuration |
| C 36 `#ifdef HAVE_HARFBUZZ` … C 39 `#endif` (`#include <hb.h>`, `#include <hb-ot.h>`) | `feature = "harfbuzz"` (`femacs/xtask/Cargo.toml` 93; `docs/baseline/feature-coverage.tsv` 122) | a HarfBuzz build |
| C 42 `#include TERM_HEADER` (`configure.ac` 7180 chooses it) | no gate of its own: `TERM_HEADER` is `androidterm.h` in exactly the configuration the file gate selects (`window_system=android`, 2594-2596), so the arm is one `include!` behind the file gate | every real Android configuration |
| C 85 `#ifdef HAVE_MMAP` … C 88 `#endif` (the member `glyf_table_mapped`) | **no gate**: `HAVE_MMAP` is 1 in `docs/baseline/config/darwin-arm64.macros.tsv` (1168) and the condition has no `#else` arm, so the live member is ordinary text (the `docs/ports/sfnt.md` C 41-43 ruling) | always, on this target |
| C 90 `#ifdef HAVE_HARFBUZZ` … C 96 `#endif` (the members `fd` and `directory`) | `feature = "harfbuzz"`, nested in the file gate | a HarfBuzz build |

**No file-scope C `#if` in this slice needs a feature this tree does not already have**: the three
names above are all long registered, and `have_mmap` is deliberately *not* used (the condition is
live and has no second arm).  C 1-1240 carries **3** of the file's 27 `#if`s (C 36, C 85 and C 90 -
all `HAVE_HARFBUZZ`/`HAVE_MMAP`); the other 24 are in later slices, and the one name among them
this tree has not registered yet is `HAVE_HB_FONT_SET_VAR_NAMED_INSTANCE` (C 4090).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block and the blank after it | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-23 | the blank and `#include <fcntl.h>` (C 22), `#include <c-ctype.h>` (C 23) | file gate |
| 24-34 | the rest of the file's own include list: C 25 `"lisp.h"`, C 27 `"blockinput.h"`, C 28 `"charset.h"`, C 29 `"coding.h"`, C 30 `"font.h"`, C 31 `"frame.h"`, C 32 `"math.h"` (the system header), C 33 `"sfnt.h"`, C 34 `"sfntfont.h"`; plus lisp.h 30's `<string.h>` (`memcpy`/`memset`/`strlen`/`strcmp`/`memmem`) and lisp.h 31's `<stdcountof.h>` (`countof`), and `<unistd.h>`, which C 22's gnulib `<fcntl.h>` pulls in (`lseek`/`read`/`SEEK_SET`) | file gate |
| 35-42 | C 36-39's `#ifdef HAVE_HARFBUZZ`/`<hb.h>`/`<hb-ot.h>`/`#endif` (dead, carried as a comment — no name in C 1-1240 is theirs) and C 42's `#include TERM_HEADER` | `harfbuzz` / file gate (`androidterm.h`) |
| 43-56 | C 44-49's banner comment, the form feed, and C 53-55's "Tables associated with each font" comment | file gate |
| 57-97 | `struct sfnt_font_tables`: C 60-83's eighteen members, C 85-88's `#ifdef HAVE_MMAP` member `glyf_table_mapped`, C 90-96's `#ifdef HAVE_HARFBUZZ` pair `fd`/`directory` (and the layout assertions) | file gate / no gate (live) / `harfbuzz` |
| 98-169 | C 99's comment; `struct sfnt_font_desc` (C 101-165: C 104-158's members, C 161's `bool_bf underline_position_set : 1`, C 164's `bool cmap_invalid : 1`, both folded into the one byte `bits` at 132); C 167-169's comment and `static struct sfnt_font_desc *system_fonts` | file gate |
| 170-224 | C 171-178's two comments and `sfnt_setup_coding_system` (C 180-224; the four-way `switch (id)` with `SFNT_PLATFORM_*` and the two nested `platform_specific_id` tests) | file gate |
| 225-291 | C 226-244's comments and the eight file-scope coding globals; `sfnt_safe_decode_coding_object_1` (C 246-257) and `_2` (C 259-265) — the two `unsafe extern "C" fn` items `internal_condition_case` takes; C 267-268's comment and `sfnt_safe_decode_coding_object` (C 270-291) | file gate |
| 292-320 | C 293-298's comment and `sfnt_decode_font_string` (C 300-320; C 309-312 the three packed-`bits` flag writers) | file gate |
| 321-373 | C 322-323's comment and `sfnt_decode_family_style` (C 325-373; C 342-354 the four `sfnt_find_name` attempts, C 372 the `? :` return) | file gate |
| 374-396 | C 375-376's comment and `sfnt_decode_instance_name` (C 378-396) | file gate |
| 397-447 | `struct sfnt_style_desc` (C 398-405, with its layout assertions) and the three tables `sfnt_weight_descriptions` (C 408-427, 17 rows), `sfnt_slant_descriptions` (C 430-434, 2) and `sfnt_width_descriptions` (C 437-447, 8) | file gate |
| 448-574 | C 449-452's comment and `sfnt_parse_style` (C 454-574; C 472's `SAFE_ALLOCA_STRING`, C 475's `strtok_r` loop, the four `goto next` sites at C 483/497/514/531, C 542's `c_toupper`, C 547's `CALLN`, C 563-568's `SREF`/`SSET` fixup, C 573's `SAFE_FREE`) | file gate |
| 575-616 | C 576-578's comment and `sfnt_parse_languages` (C 580-616; C 596/602's `sfnt_find_metadata`, C 609-615's four `memmem` tests and three `Fcons` appends) | file gate |
| 617-684 | C 618-627's comment and `sfnt_registry_for_subtable` (C 629-684; the outer `switch (platform_id)` with the nested Macintosh and Microsoft `switch`es) | file gate |
| 685-766 | C 686-697's comment and `sfntfont_identify_cmap` (C 699-766; the same three-way shape, C 715's `platform_specific_id < SFNT_UNICODE_2_0`) | file gate |
| 767-836 | C 768-772's comment and `sfnt_grok_registry` (C 774-836; C 782's `sfnt_read_cmap_table` call with a NULL `data`, the three `cmap->num_subtables` scans, the three `goto done` sites at C 803/815/828) | file gate |
| 837-943 | C 838-846's comment and `sfnt_replace_fonts_p` (C 848-943; C 871-900 the two nested `ASIZE` loops with the `goto next` at C 890, C 925-927's three `XFIXNUM` reads, C 942's `count_instance < 1`) | file gate |
| 944-1153 | C 945-948's comment and `sfnt_enum_font_1` (C 950-1153; C 969-971 the `xzalloc`/`path`/`memcpy` header, C 975-996 the four `goto bailN`s, C 1016-1018 the `ach_vendor_id` copy, C 1049-1052 the post-table block including the `underline_position_set` write, C 1087-1110 the fvar instance loop, C 1123-1134 the `system_fonts` splice, C 1143-1152's `bail4:`…`bail1:` ladder) | file gate |
| 1154-1240 | C 1155-1161's comment and `sfnt_enum_font` (C 1163-1240; C 1182's `(T *) -1` TTC probe and C 1206-1211's re-test, C 1197-1217 the TTC `ttc->offset_table[i]` loop, C 1236-1239's `bail0:`/`bail:` pair) | file gate |
| 1241-1426 | C 1241's blank, C 1242's form feed, C 1244's banner and the three cmap selectors: `sfntfont_charset_for_name` (C 1246-1264), `sfntfont_charset_for_cmap` (C 1269-1334) and `sfntfont_select_cmap` (C 1344-1425; the three `num_subtables` scans and the two format 14 searches) | file gate - **transcribed** (slice 2; the only structural rewrite is the `switch`/`match` pair of `sfntfont_charset_for_cmap`, Deviation 17) |
| 1427-1604 | C 1427-1430's comment and `sfntfont_read_cmap` (C 1431-1496), C 1498-1501's comment and `sfntfont_glyph_valid` (C 1502-1515), C 1517-1522's comment and `sfntfont_lookup_char` (C 1524-1585, the cache and the cmap fallback), C 1587-1594's comment and `sfntfont_registries_compatible_p` (C 1596-1603), and C 1604's blank | file gate - **transcribed** (slice 3; the three shapes are Deviation 18, and the `limits.in.rs` edge at C 25 is this slice's `INT_MAX`) |
| 1605-1871 | C 1605-1611's comment and `sfntfont_list_1` (C 1613-1824: the family/registry/spacing/style checks, the instance loop, and the extras block with its five `goto fail` legs), the two statics `sfnt_vendor_name` (C 1827) and `sfnt_font_driver` (C 1830), and C 1832-1839's comment with `sfntfont_registry_for_desc` (C 1841-1871) | file gate - **transcribed** (slice 4; Deviation 19's four shapes, and the token rows for `goto` and `cmap_invalid,true`) |
| 1872-2050 | C 1873-1877's comment and `sfntfont_desc_to_entity` (C 1879-1933), C 1935-1937's comment and `sfntfont_compare_font_entities` (C 1939-1959), C 1961-1963's comment and the subr record `Scompare_font_entities` (C 1964-1971), C 1973-1976's comment and `sfntfont_list` (C 1977-2027), C 2029-2031's comment and `sfntfont_match` (C 2032-2042), C 2043's blank and C 2044's form feed, and C 2046-2050's anonymous enum (the two cache sizes) | file gate - **transcribed** (slice 5; Deviation 20's five shapes) |
| 2051-2172 | C 2051's blank, C 2052-2058's comment, the three records (`sfnt_outline_cache` C 2059-2072, `sfnt_raster_cache` C 2074-2084, `sfntfont_get_glyph_outline_dcontext` C 2086-2102, each with its `clang`-measured `core::assert!` block) and the four helpers (`sfntfont_get_glyph` C 2108-2134, `sfntfont_free_glyph` C 2139-2142, `sfntfont_get_metrics` C 2150-2158, `sfntfont_dereference_outline` C 2164-2172) | file gate - **transcribed** (slice 6; the layouts are Evidence ⑮'s probe, the shapes are Deviation 21) |
| 2173-2559 | C 2173's blank, C 2174-2196's comment with `sfntfont_get_glyph_outline` (C 2198-2412), C 2414-2416's comment with `sfntfont_free_outline_cache` (C 2418-2437), C 2439-2441's comment with `sfntfont_dereference_raster` (C 2443-2451), C 2453-2456's comment with `sfntfont_get_glyph_raster` (C 2458-2534) and C 2536-2538's comment with `sfntfont_free_raster_cache` (C 2540-2559) | file gate - **transcribed** (slice 7; the five shapes are Deviation 22) |
| 2560-2742 | C 2561's form feed, C 2563's `/* Opening fonts.  */` banner, `struct sfnt_font_info` (C 2565-2645, with its measured `core::assert!` block), C 2647-2653's `open_fonts` list, C 2655-2656's comment with `sfntfont_lookup_glyph` (C 2658-2679), C 2681-2682's forward declaration (carried as a use-site declaration) and C 2684-2690's comment with `sfntfont_probe_widths` (C 2692-2742) | `have_window_system` on the two items that read `font.encoding_charset` (Deviation 23 (a)); the file gate on the rest - **transcribed** (slice 8) |
| 2743-2915 | C 2743's blank, C 2744-2751's comment with `sfntfont_setup_interpreter` (C 2753-2860, whose `bail1`/`bail` pair and zero-initialised locals are Deviation 24's (a)/(b)) and C 2862-2863's comment with `sfnt_close_tables` (C 2865-2915, whose `HAVE_MMAP` arms are ungated and whose `HAVE_HARFBUZZ` block is gated) | file gate - **transcribed** (slice 9) |
| 2916-3113 | C 2916's blank, C 2917-2918's comment and `sfnt_open_tables` (C 2920-3113: the open/seek/cmap/UVS block, the ungated `HAVE_MMAP` arms, the loca/hmtx/instruction/distortion reads and the six-label `bailN` ladder of Deviation 25) | file gate - **transcribed** (slice 10) |
| 3114-3472 | C 3114's blank, C 3115-3116's comment with `sfnt_reference_font_tables` (C 3119-3134), C 3136-3137's comment with `sfnt_dereference_font_tables` (C 3140-3151), and C 3153-3154's comment with `sfntfont_open` (C 3156-3472; Deviation 26's seven shapes) | file gate; `have_window_system` on `sfntfont_open` (Deviation 26 (a)) - **transcribed** (slice 11) |
| 3473-4222 | **the remainder** — C 3474's form feed, C 3476's banner and from C 3483 (`sfntfont_encode_char`, **the next live definition**) to C 4222: the metrics/callback family (C 3483-3960), the mmap sigbus pair (C 3961-3983) and the HarfBuzz entry points (C 3989-4110), and the three initialization subrs (C 4115-4222) - among them C 3504's `sfntfont_measure_pcm` definition, which replaces Deviation 23 (b)'s declaration | not this slice |

The 23 rows above tile C 1-1240 **contiguously** (blank lines included) rather than skipping
the blanks between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/sfntfont.rs` (this file's header's mirror) to this same `sfntfont.md` through
its bare-stem fallback, so a table with gaps at blanks would leave `src/sfntfont.h`'s non-blank
lines uncovered and fail (the `sfnt.md`/`androidterm.md` shape).  The last row reaches C 4222,
which is why the coverage reading below is `0 blank line(s) between blocks need no row` for
`src/sfntfont.c` and a covered table for `src/sfntfont.h`.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfntf-lane/cargo cargo run -q -p xtask -- build-tu
src/sfntfont.c` → `built src/sfntfont.rs -> ../build/femacs/sfntfont.o`, **exit 0**, and
`build/femacs/sfntfont.o` is **336 bytes** - an empty object, read on the final revision (1,554
Rust lines).  That is the expected reading, not a proof of the new text: every item of the port
sits behind a `have_android`/`not_android_stubify` gate, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute - P-86 ⑧'s criterion.  (**Three
`build-tu` runs**, the whole budget: run 1 failed to compile the unit and produced no object, on two
C-shaped expressions written without Rust's syntax - the C 663/664 shared `case` labels
(`SFNT_MICROSOFT_SYMBOL`/`SFNT_MICROSOFT_UNICODE_BMP`) written as two `=>` arms instead of an
or-pattern, and `platform_specific_id as c_int < SFNT_UNICODE_2_0` parsed as a generic argument
list; runs 2 and 3 are the same `built … 336 bytes` reading, run 3 on the final revision, and the
edits between 2 and 3 are comment text and the two `include!` trailing comments only.)

**② The layout probe (clang, frozen configuration).**
`clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
-I emacs/src -I emacs/lib /tmp/sfntf-lane/probe.c` (the `spikes/run.sh` invocation), whose C text
is this file's two records verbatim plus `struct sfnt_style_desc`, reads:

```
HAVE_MMAP=1 HAVE_HARFBUZZ=0 bool_bf=1
sizeof struct sfnt_font_tables 152   alignof 8
  cmap 0   cvar 112   cmap_data 120   cmap_subtable 128   uvs 136   glyf_table_mapped 144
sizeof struct sfnt_font_desc 136     alignof 8
  next 0   family 8   instances 56   width 64   path 80   char_cache 88   subtable 96
  offset 104   tables 112   num_glyphs 120   refcount 124
  underline_position 128   underline_thickness 130
sizeof struct sfnt_style_desc 16     alignof 8   c_string 0   value 8
sizeof (struct sfnt_cmap_encoding_subtable) 8   sizeof (sfnt_fword) 2   sizeof (off_t) 8
```

The two `bool_bf` members are a bitfield pair `offsetof` cannot name, so a second runtime probe in
the same file sets each and prints the byte at 132: `underline_position_set` → `0x01`,
`cmap_invalid` → `0x02`, both → `0x03`, and byte 133 stays `0x00` - the port's `bits: u8` at 132
with `1 << 0`/`1 << 1` is that measurement, and `sizeof 136` is the 3-byte pad it carries
(PORTING.md 1.3).  `HAVE_HARFBUZZ=0` is what makes the two `harfbuzz` members absent from the
152-byte record, which is why this unit's assertions for `sfnt_font_tables` are gated on
`not (feature = "harfbuzz")`.

**③ The HarfBuzz headers are not in the reference tree.**
`ls emacs/src/hb.h emacs/src/hb-ot.h` is empty and no `/// C: #include <hb.h>` segment exists
anywhere in `femacs/`, so C 37-38 are P-88's class; no name in C 1-1240 comes from them (the first
`hb_*` type in the file is `hb_blob_t`, C 3989), so the arm is a comment and the declaration
segment is owed with the slice that names one.

**④ The arm flip (P-86 ③⑴; the file gate plus the sub-gate).**
`cargo run -q -p xtask -- probe src/sfntfont.rs /tmp/sfntf-lane/arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` →
**exit 1, 1 `error` line and 5 warnings.**  The classification P-86 ⑦ asks for, by file:

- **1 in the mirror, 0 in this unit's own text.**  `include/sfntfont.rs` 55 declares
  `pub fn sfnt_enum_font (file: *const c_char) -> c_int;` at module top level, and this unit
  *defines* it (C 1163-1240), so the pair is `E0428` at that mirror line.  This is the mirror's
  **known shape gap** - the declaration run has not been wrapped in
  `pub mod … { use super::*; … } use …::*;` the way `include/sfnt.rs` was in 3.427 (P-86 ⑩) - and
  it is Owed 1 below, not port text: C includes `sfntfont.h` (C 34), so dropping the include is not
  the port's move, and the *only* collision is this one name.
- **5 warnings, all in mirrors** (`include/stdbit.in.rs` 644, `include/lisp.rs` 3934,
  `include/charset.rs` 697, `include/sfntfont.rs` 53: unused doc comments on `extern` blocks;
  `include/frame.rs` 1355-1360: a doubled `#[inline(always)]`).  None is in this unit.
- **No name-resolution abort**, and rustc got past name resolution and type-checked the whole
  unit: the unit's own text contributes **zero** errors under this gate set.  (It is still a
  type-check/codegen reading: no font was opened, no table directory was read, no cmap subtable
  was parsed.)  **Three `probe` invocations in total**, the whole budget: run 1 read **14 errors**
  (the two syntax fixes above plus the `E0695`/`E0381`/`E0614`/`E0425`/`E0308` classes the four
  follow-up edits retired - the labeled-block `continue`, the four possibly-uninitialized pointers
  the `bailN` ladder frees, the six `(*array[i])` places, the missing `strtok_r` declaration and the
  `c_int` truthiness test), run 2 read the **1 error** above, and run 3 is the same 1-error reading
  on the final revision.  The remaining iteration was done by re-running `rustc` directly on the
  materialized `build/probes/sfntfont/_root.rs` (from inside `femacs/`, so the pinned
  `nightly-2026-10-01` applies - a plain `rustc` from the repository root picks stable 1.98.1 and
  adds the mirror's `E0658 c_variadic` at `include/lisp.rs` 3756, which is a toolchain artifact and
  not this unit's).

**⑤ The `strtok_r` declaration and its symbol.**
`lib/include/string.rs` carries no `strtok_r`; `emacs/lib/string.in.h` 1206-1216's
`REPLACE_STRTOK_R` arm is **dead** in the frozen configuration
(`build/emacs-ref/lib/string.h` 1753 is `#if 0`), so the declaration is the system header's.  A
clang probe (`char s[] = "a b"; char *sp = NULL; strtok_r (s, " ", &sp);` with
`-Werror=implicit-function-declaration`) compiles clean and `nm` prints `_strtok_r`, which is why
this unit's use-site declaration carries no `#[link_name]` (the `memmem`/`rpl_memmem` contrast).

**⑥ The other gates (repository root, final revision), one line each.**
`python3 tools/check-cond-eval.py src/sfntfont.c` → `27 conditions, 0 unknown, 0 disagreements
[VERIFIED]` (it was already in `docs/baseline/config/cond-eval-verified.txt`; this revision does not
change that). — `python3 tools/check-dead-file-coverage.py` → the reading in the table's footnote
above (`src/sfntfont.c` 4,222 lines covered, `src/sfntfont.h` 78 lines covered, `0 failure(s)`). —
`python3 tools/check-declarations.py` → `10 declaration sites (0 forbidden, 10 allowed literals),
0 failures`. — `python3 tools/check-ledger-ticks.py` → nothing to bind (this ledger has no
`## Functions defined` section; the `androidterm.md` class). — `python3 tools/extract-abi.py
--check emacs/src/sfntfont.c femacs/src/sfntfont.rs` → **FAIL**, `59` lines: **48 `- missing …`
lines** (42 of the inventory's 57 functions, 2 constants - `SFNT_OUTLINE_CACHE_SIZE` and
`SFNT_RASTER_CACHE_SIZE` - and 4 file-scope objects: `sfnt_vendor_name`, `sfnt_font_driver`,
`open_fonts` and `sfnt_put_glyphs`; all of it C 1241-4222), **9 `- … does not mention …` rows** and
the `line count drifted 63.2%: C 4222 vs Rust 1554` row.  The nine rows are the port's structural
rewrites, each recorded in `## Deviations`: `sfnt_setup_coding_system` (`break, case, default,
switch`), `sfnt_decode_font_string` (`common_flags, mode`), `sfnt_parse_style` (`continue, goto`),
`sfnt_registry_for_subtable` and `sfntfont_identify_cmap` (`case, default, switch`),
`sfnt_grok_registry` and `sfnt_replace_fonts_p` (`goto`), `sfnt_enum_font_1` (`bail1, bail2,
bail3, bail4, goto, true, underline_position_set`) and `sfnt_enum_font` (`bail0, goto`).  The
63.2% drift is the slice boundary, not a gap in it: the tool compares the *whole* file. —
`python3 tools/check-port-lines.py` -> `75 file(s) under ten percent, 101,649 reference line(s);
79 ratchet row(s), 0 new, 4 stale`, this file's row one of the four (`stale src/sfntfont.c: no
longer under the threshold` - 1,554 Rust lines against the 422-line threshold); the other three
(`src/gtkutil.c`, `src/sfnt.c`, `src/w32proc.c`) are not this lane's. —
`python3 tools/check-citations.py` -> `675 file(s) scanned, 2 drift, 0 range, 711 external, 33
allowed, 0 stale` and **exit 1**: both drift rows are in **other lanes' in-flight ports**
(`femacs/src/gtkutil.rs:92`, `femacs/src/w32proc.rs:456`; at HEAD those two files are the 11-line
placeholder shape, so the rows are not this file's and not a HEAD reading), while this lane's own
two citation-looking lines were rewritten to drop that shape - Owed 6.
**⑦ Slice 2's frozen reading and its arm flip (HEAD as the baseline, P-86 ⑦).**
`cd femacs && cargo run -q -p xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - the same
empty object slices 1's reading recorded, because every item sits behind the file gate.  The arm flip
is Evidence ④'s command (`--crate-type lib --emit obj -C panic=abort --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'`) run twice, once on HEAD's file and
once on slice 2: **both `0 errors / 6 warnings`, and the two outputs are byte-identical** (an empty
`diff`) - the three new functions and the two new calls add no diagnostic of any kind, and the six
warnings stay the mirror-side `unused doc comment` class Owed 1 names.  The emitted object is
**14,800 bytes in both runs**: the three new functions are not yet *referenced* by the TU's live code
(the driver record that would call them is in an unported slice), so the size carries no signal this
round - the diagnostics are the reading, not the artifact size.

**⑧ Slice 2's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **46** `- ` rows: **39** `- missing
function …`, 2 `- missing constant …` (`SFNT_OUTLINE_CACHE_SIZE`, `SFNT_RASTER_CACHE_SIZE`), 4
`- missing file-scope object …` (`sfnt_vendor_name`, `sfnt_font_driver`, `open_fonts`,
`sfnt_put_glyphs`) and the line-count row, now `57.9%: C 4222 vs Rust 1776` (HEAD read 49 rows -
42 functions, 2 constants, 4 objects and the drift - and `63.2%: C 4222 vs Rust 1554`).  **The three
statics `sfntfont_charset_for_name`, `sfntfont_charset_for_cmap` and `sfntfont_select_cmap` are what
left the list**, and the `- does not mention …` class is **0** rows in both readings (the
`macro-body-notes.tsv` row `sfntfont_charset_for_cmap case,default,switch` is what keeps this
slice's only structural rewrite out of it, Deviation 17).  `python3
tools/check-dead-file-coverage.py` -> `145 ledger(s) bound (a C range table), 0 failure(s)` (the
table below tiles C 1-4222 with the new boundary at C 1426); `python3
tools/check-declarations.py` -> `17 declaration sites (0 forbidden, 17 allowed literals), 0
failures`; `python3 tools/check-port-lines.py` -> `0 file(s) under ten percent, 0 ratchet row(s), 0
new, 0 stale` (this file left that class at slice 1); `python3 tools/check-ledger-ticks.py` ->
nothing to bind (this ledger has no `## Functions defined` section, Owed 5's class);
`python3 tools/check-cond-eval.py src/sfntfont.c` -> `27 conditions, 0 unknown, 0 disagreements
[VERIFIED]` (slice 2 adds no `#if`).
**⑨ Slice 3's frozen reading and its arm flip (HEAD as the baseline, P-86 ⑦).**
`cd femacs && cargo run -q -p xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - every
item still sits behind the file gate.  Evidence ④'s arm flip, run as a baseline pair: HEAD **0
errors / 6 warnings** and slice 3 **0 errors / 6 warnings**, and the two outputs are
**byte-identical** (`diff` empty) - the four new functions add no diagnostic, and the six warnings
are still the mirror-side `unused doc comment` class Owed 1 names.  The emitted object is 14,800
bytes in both runs, which again carries no signal: the caller that would reach these functions
(`sfntfont_list_1`, C 1613) is in an unported slice, so nothing references them yet.  **What the
reading does prove is the type check**: the file gate plus `not_android_stubify` selects this text,
and rustc accepts it (the one new dependency, `INT_MAX` from `limits.in.rs`, is what the include edge
added at C 25 exists for).

**⑩ Slice 3's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **42** `- ` rows: **35** `- missing
function …`, 2 `- missing constant …`, 4 `- missing file-scope object …` and the line-count row,
now `53.5%: C 4222 vs Rust 1962` (slice 2 read 46 rows and `57.9%: C 4222 vs Rust 1776`).  The four
that left the function list are this slice's: `sfntfont_read_cmap`, `sfntfont_glyph_valid`,
`sfntfont_lookup_char` and `sfntfont_registries_compatible_p`.  The `- does not mention …` class
needed **two new rows** before it would read 0 again - `sfntfont_glyph_valid sfnt_glyph` (the scalar
typedef's port name is `sfnt_glyph_index`) and `sfntfont_lookup_char cmap_invalid` (C 164's 1-bit
member is the `SFNT_FONT_DESC_CMAP_INVALID` mask) - both in `docs/ports/macro-body-notes.tsv` with
the sibling rows' shape; with them the class reads **0** rows, as it does on HEAD.  The other gates:
`check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0 failure(s)` (the table below
tiles C 1-4222 with the new boundary at C 1604); `check-declarations.py` `17 declaration sites (0
forbidden, 17 allowed literals), 0 failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0
range, 2011 external, 36 allowed, 0 stale`; `check-port-lines.py` `0 file(s) under ten percent, 0
ratchet row(s), 0 new, 0 stale`; `check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0
disagreements [VERIFIED]` (slice 3 adds no `#if`).
**⑪ Slice 4's frozen reading and its arm flip (HEAD as the baseline, P-86 ⑦).**
`cd femacs && cargo run -q -p xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, md5 `dbed68837694579185377dbb0821ab62`.  Evidence
④'s arm flip as a baseline pair: HEAD (this round's HEAD, i.e. slice 3's revision) **0 errors / 6
warnings** and slice 4 **0 errors / 6 warnings**, the two outputs **byte-identical**.  The first run
of this slice read **3 `E0308` errors** - `FONT_WIDTH/WEIGHT/SLANT_NUMERIC!` return the macro's
`long` while `sfnt_font_desc`'s members are `int` - and the fix is Deviation 19 (widen the member,
which is what C's usual arithmetic conversions do); the second run is the 0-error reading above, i.e.
**the arm flip caught a real defect this slice that `--check` and `build-tu` cannot see** (the file
is gated off in the frozen build).  The emitted object is 14,800 bytes in both runs - no signal, as
before, because nothing calls these functions yet.

**⑫ Slice 4's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **38** `- ` rows: **33** `- missing
function …`, 2 `- missing constant …`, 2 `- missing file-scope object …` and the line-count row,
now `46.6%: C 4222 vs Rust 2256` (slice 3 read 42 rows and `53.5%: C 4222 vs Rust 1962`).  The
four that left the function list are `sfntfont_list_1` and `sfntfont_registry_for_desc`, plus the
two file-scope objects `sfnt_vendor_name` and `sfnt_font_driver` - the last two are **objects**,
so the `missing file-scope object` class is now 2 (it was 4; `sfnt_put_glyphs` and `open_fonts`
remain).  Two more token rows were needed to keep the `does not mention` class at 0:
`sfntfont_list_1 goto` (the five `goto fail` legs are the `'fail:` labelled block) and
`sfntfont_registry_for_desc cmap_invalid,true` (the 1-bit member's mask, and C 1858's `true` as its
value).  The other gates: `check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0
failure(s)`; `check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0
failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0
stale`; `check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**⑬ Slice 5's frozen reading and its arm flip (HEAD as the baseline, P-86 ⑦).**
`cd femacs && cargo run -q -p xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, md5 `dbed68837694579185377dbb0821ab62`.  Evidence
④'s arm flip as a baseline pair: HEAD **0 errors / 6 warnings** and slice 5 **0 errors / 6
warnings**, byte-identical output - but **the emitted object now grows: 14,800 -> 22,256 bytes**,
and that *is* a signal this time: `sfntfont_list` and `sfntfont_match` are `#[unsafe(no_mangle)]`
driver entry points, so LLVM has to keep them and everything they reach, where the previous three
slices' functions were all `static` and unreferenced.  The first run of this slice read **8 errors**
before the two fixes were in: six `cannot find function font_style_to_value` (the `FONT_SET_STYLE`
macro's callee, a `font.h` prototype no mirror carries - the use-site declaration, Deviation 20 (c))
and two `E0308` on `make_mint_ptr (desc)` (the mirror takes `*mut c_void`, Deviation 20 (a)).  The
second run is the 0-error reading above, i.e. **the arm flip again caught what `build-tu` and
`--check` cannot see**.

**⑭ Slice 5's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **32** `- ` rows: **29** `- missing
function …`, **2** `- missing file-scope object …` (`open_fonts`, `sfnt_put_glyphs`) and the
line-count row, now `40.7%: C 4222 vs Rust 2503` (slice 4 read 38 rows and `46.6%: C 4222 vs Rust
2256`).  **The `missing constant` class is now 0 rows** - C 2048's `SFNT_OUTLINE_CACHE_SIZE` and
C 2049's `SFNT_RASTER_CACHE_SIZE` are this slice's - and four functions left the list
(`sfntfont_desc_to_entity`, `sfntfont_compare_font_entities`, `sfntfont_list`, `sfntfont_match`).
No new token row was needed this time: the `does not mention` class reads **0** rows, as it does on
HEAD.  The other gates: `check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0
failure(s)`; `check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0
failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0
stale`; `check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**⑮ The three records' layouts are measured, not remembered (P-88 ②).**  A clang probe in the frozen
configuration - `clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I
build/emacs-ref/src -I emacs/src -I emacs/lib /tmp/layout.c` - whose C text is C 2059-2102's three
records *verbatim* plus `offsetof`/`sizeof` prints, reads:

```
sizeof struct sfnt_outline_cache 40 alignof 8
  next 0  last 8  outline 16  metrics 24  glyph 32
sizeof struct sfnt_raster_cache 32 alignof 8
  next 0  last 8  raster 16  glyph 24
sizeof struct sfntfont_get_glyph_outline_dcontext 56 alignof 8
  loca_long 0  loca_short 8  glyf 16  hmtx 24  hhea 32  maxp 40  blend 48
```

Those six numbers and the eighteen offsets are the `core::assert!`s at each record's site (the
`sfnt_style_desc` shape), which is why this port does not spell a layout from memory.  The probe is
the Evidence ② class and uses the same include set.

**⑯ Slice 6's frozen reading, its arm flip and `--check`.**  `cd femacs && cargo run -q -p xtask --
build-tu src/sfntfont.c` -> `built src/sfntfont.rs -> ../build/femacs/sfntfont.o`, exit 0, **336
bytes**, md5 `dbed68837694579185377dbb0821ab62`.  Evidence ④'s arm flip as a baseline pair: HEAD
**0 errors / 6 warnings** and slice 6 **0 errors / 6 warnings**, the two outputs byte-identical -
and the object is **22,256 bytes in both**, because these four helpers are not on a path the two
`no_mangle` entry points reach (the size signal slice 5 saw was those entry points, not the
statics).  The first run of this slice read **one `E0308`**: C 2124's `glyph->simple` is a *pointer*
(`sfnt.h` 677), so C's truth test is Rust's null test (Deviation 21 (a)) - another defect only the
arm flip can see (`build-tu` gates the whole file off and `--check` compares names and tokens).
`python3 tools/extract-abi.py --check emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**,
**28** `- ` rows: **25** `- missing function …`, the 2 remaining file-scope objects (`open_fonts`,
`sfnt_put_glyphs`) and the line-count row, now `36.4%: C 4222 vs Rust 2686` (slice 5 read 32 rows
and `40.7%: C 4222 vs Rust 2503`).  The `- does not mention …` class reads **0** rows in both
readings, and no new token row was needed.  The other gates: `check-dead-file-coverage.py` `145
ledger(s) bound (a C range table), 0 failure(s)`; `check-declarations.py` `17 declaration sites (0
forbidden, 17 allowed literals), 0 failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0
range, 2011 external, 36 allowed, 0 stale`; `check-port-lines.py` `0 file(s) under ten percent, 0
ratchet row(s), 0 new, 0 stale`; `check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0
disagreements [VERIFIED]`.
**⑰ Slice 7's frozen reading and its arm flip (HEAD as the baseline, P-86 ⑦).**
`cd femacs && cargo run -q -p xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, md5 `dbed68837694579185377dbb0821ab62`.  Evidence
④'s arm flip as a baseline pair: HEAD **0 errors / 6 warnings** and slice 7 **0 errors / 6
warnings**, byte-identical output, the object **22,256 bytes in both** (these five functions are not
on a path the two `no_mangle` entry points reach - the slice-5 distinction again).  The *first* run
of this slice read **six errors in four classes**, all now fixed and all invisible to `build-tu`
(whole file gated off) and to `--check` (names and tokens): the two `SFNT_ROUND_FIXED`/
`SFNT_FLOOR_FIXED` calls were written as macros but the mirror has them as *functions*; the two
`void *` arguments (`&dcontext` at C 2316 and C 2358) needed the untyped-pointer cast (P-72 (3));
and two bindings C leaves uninitialised and passes by address (`dcontext`, `advance`) had to be
zero-initialised (`E0381`, Deviation 10's class).  The second run is the 0-error reading above.

**⑱ Slice 7's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **23** `- ` rows: **20** `- missing
function …`, the 2 remaining file-scope objects (`open_fonts`, `sfnt_put_glyphs`) and the line-count
row, now `26.7%: C 4222 vs Rust 3096` (slice 6 read 28 rows and `36.4%: C 4222 vs Rust 2686`).  The
five that left the function list are this slice's (`sfntfont_get_glyph_outline`,
`sfntfont_free_outline_cache`, `sfntfont_dereference_raster`, `sfntfont_get_glyph_raster`,
`sfntfont_free_raster_cache`), and **one token row was needed** to keep the `does not mention`
class at 0: `sfntfont_get_glyph_outline goto` (C 2294's `goto fail` / C 2365's `fail:` are the
`'fail:` labelled block).  The other gates: `check-dead-file-coverage.py` `145 ledger(s) bound (a C
range table), 0 failure(s)`; `check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external,
36 allowed, 0 stale`; `check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0
stale`; `check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**⑲ `struct sfnt_font_info`'s layout is measured, not remembered (P-88 ②).**  The Evidence ⑮/② class
of probe, over C 2059-2102's three caching records plus C 2565-2645's record verbatim, in the frozen
configuration (`HAVE_MMAP` 1, `HAVE_HARFBUZZ` 0):

```
sizeof struct sfnt_font_info 576 alignof 8
  font 0  next 184  desc 192  cmap 200
  hhea 208  maxp 216  head 224  hmtx 232  glyf 240
  loca_short 248  loca_long 256  prep 264  fpgm 272  cvt 280
  cmap_data 288  cmap_subtable 296  uvs 304
  outline_cache 312  outline_cache_size 352  raster_cache 360  raster_cache_size 392
  interpreter 400  state 408  scale 512  blend 520  instance 568  glyf_table_mapped 572
```

Those numbers are the `core::assert!`s at the record's site, and the block is gated `not
(harfbuzz)` **and** `not (have_window_system)` because two other conditions move it: C 2635-2644's
three `harfbuzz` members (24 bytes) and the eight `HAVE_WINDOW_SYSTEM` members of the embedded
`struct font` (`font.h` 325-403, 8 bytes each).  The second gate is not stylistic - the arm flip
read `E0080 assertion failed: size_of::<sfnt_font_info> () == 576` on the first `have_window_system`
run, which is how the reading was found.

**⑳ Slice 8's readings.**  `build-tu src/sfntfont.c` -> `built src/sfntfont.rs ->
../build/femacs/sfntfont.o`, exit 0, **336 bytes**, the standing md5.  The arm flip is run in **two**
cfg sets, each as a HEAD-baseline pair (P-86 ⑦):
  * file gate only: HEAD **0 errors / 6 warnings**, slice 8 **0 / 6**, outputs byte-identical
    (`diff` empty), object 22,256 bytes in both.  **But this reading does not cover the two items
    that read `font.encoding_charset`** - they are `#[cfg (have_window_system)]` and are compiled
    out here, which is exactly P-86 ⑦'s "a cfg'd-out item is not a reading".
  * file gate **plus** `have_window_system`: HEAD **0 errors**, slice 8 **0 errors** - this is the
    run that type-checks the two gated items (and it is why the record's assertion block had to be
    gated out of that configuration first).
`python3 tools/extract-abi.py --check emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**,
**20** `- ` rows: **18** `- missing function …`, **1** `- missing file-scope object …`
(`sfnt_put_glyphs` - `open_fonts` is this slice's) and the line-count row, now `20.4%: C 4222 vs
Rust 3359` (slice 7 read 23 rows and `26.7%: C 4222 vs Rust 3096`).  The two functions that left the
list are `sfntfont_lookup_glyph` and `sfntfont_probe_widths`, and **two token rows** were needed to
keep the `does not mention` class at 0 (both are the `sfnt_glyph` scalar typedef, Deviation 18 (a)).
The other gates: `check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0
failure(s)`; `check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0
failures`; `check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0
stale`; `check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**㉑ Slice 9's frozen reading and its arm flips.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/sfntfont.c` -> `built src/sfntfont.rs -> ../build/femacs/sfntfont.o`, exit 0, **336 bytes**, the
standing md5.  Both cfg sets are run as HEAD-baseline pairs (P-86 ⑦): file gate alone **0 errors** on
both sides, and file gate plus `have_window_system` also **0 errors** on both sides, with both
outputs **byte-identical** and the object 22,256 bytes in both runs.  The *first* run of this slice
read one `E0384`: C 2762's `error` is assigned by *both* interpret calls (C 2817 and C 2835), so the
port's binding is `mut` - the arm flip's sixth consecutive slice finding something `build-tu` (whole
file gated off) and `--check` (names and tokens) cannot see.

**㉒ Slice 9's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **18** `- ` rows: **16** `- missing
function …`, **1** `- missing file-scope object …` (`sfnt_put_glyphs`) and the line-count row, now
`15.7%: C 4222 vs Rust 3560` (slice 8 read 20 rows and `20.4%: C 4222 vs Rust 3359`).  The two
functions that left the list are `sfntfont_setup_interpreter` and `sfnt_close_tables`, and **one
token row** was needed to keep the `does not mention` class at 0 (`sfntfont_setup_interpreter
bail1,goto` - the `'body: loop` + level-variable shape).  The other gates:
`check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0 failure(s)`;
`check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`;
`check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0 stale`;
`check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**㉓ Slice 10's frozen reading and its arm flips.**  `cd femacs && cargo run -q -p xtask -- build-tu
src/sfntfont.c` -> `built src/sfntfont.rs -> ../build/femacs/sfntfont.o`, exit 0, **336 bytes**, the
standing md5.  Both cfg sets are run as HEAD-baseline pairs (P-86 ⑦): file gate alone **0 errors** on
both sides, and file gate plus `have_window_system` also **0 errors** on both sides, with both
outputs **byte-identical** and the object 22,256 bytes in both runs.  **This slice's first run was
clean** - the first of the ten that needed no arm-flip fix, and worth saying so next to the six
slices that did: the flip is run every time, and a clean run is a reading, not a skipped step.

One thing the first draft got *wrong* and this round removed before measuring: an extra
`if (*tables).head != NULL &&` guard around C 3025's `index_to_loc_format` test.  C dereferences
`tables->head` there because C 3018-3020 has already validated it, and adding a check C does not
have would be inventing behaviour (PORTING.md's no-invention rule); the port keeps C's own text.

**㉔ Slice 10's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **17** `- ` rows: **15** `- missing
function …`, **1** `- missing file-scope object …` (`sfnt_put_glyphs`) and the line-count row, now
`10.0%: C 4222 vs Rust 3800` (slice 9 read 18 rows and `15.7%: C 4222 vs Rust 3560`) - the drift is
now the only remaining class the 5% rule would flag.  The one function that left the list is
`sfnt_open_tables`, and **one token row** was needed (`sfnt_open_tables bail1,bail2,bail3,bail4,
bail5,goto`; the row's first draft was keyed `sfntfont_open_tables` by mistake and the check caught
it - the function is `sfnt_open_tables`, the file is `sfntfont.c`).  The other gates:
`check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0 failure(s)`;
`check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`;
`check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0 stale`;
`check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
**㉕ Slice 11's frozen reading, its arm flips, and the object size.**  `cd femacs && cargo run -q -p
xtask -- build-tu src/sfntfont.c` -> `built src/sfntfont.rs -> ../build/femacs/sfntfont.o`, exit 0,
**336 bytes**, the standing md5.  Both cfg sets are run as HEAD-baseline pairs (P-86 ⑦): file gate
alone **0 errors** on both sides (this slice's two `have_window_system`-gated items are compiled out
there) and file gate plus `have_window_system` **0 errors** on both sides - and in that second run
**the emitted object grows from 22,256 to 29,656 bytes**, because `sfntfont_open` is a
`#[unsafe(no_mangle)]` driver entry point and it reaches the whole glyph/cache machinery.  That is
the slice-5 class again: code placement, not behaviour - and it is why the object size is reported
here rather than treated as a success signal.

The first run of this slice read **three errors** (`E0277 cannot multiply i32 by f64`, `E0277 cannot
divide f64 by {integer}`, `E0608 cannot index into a value of type *mut sfnt_instance`), and the
second read **two more** of the same two classes; all are Deviation 26's (a)-(c).  That is the arm
flip doing its job on the file's biggest function, where `build-tu` compiles nothing and `--check`
sees only names and tokens.

**㉖ Slice 11's `--check` and the gates.**  `python3 tools/extract-abi.py --check
emacs/src/sfntfont.c femacs/src/sfntfont.rs` -> **FAIL**, **13** `- ` rows: **12** `- missing
function …` and **1** `- missing file-scope object …` (`sfnt_put_glyphs`) - **and no line-count row
at all**: with 4,210 Rust lines against C's 4,222 the drift is inside the 5% the rule allows, so the
`line count drifted …` row this ledger has carried since slice 1 is **gone** (slice 10 read 17 rows
with `10.0%: C 4222 vs Rust 3800`).  The three functions that left the list are
`sfnt_reference_font_tables`, `sfnt_dereference_font_tables` and `sfntfont_open`, and **one token
row** was needed (`sfntfont_open bail6,goto,underline_position_set`).  The other gates:
`check-dead-file-coverage.py` `145 ledger(s) bound (a C range table), 0 failure(s)`;
`check-declarations.py` `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`;
`check-citations.py` `676 file(s) scanned, 0 drift, 0 range, 2011 external, 36 allowed, 0 stale`;
`check-port-lines.py` `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale`;
`check-cond-eval.py src/sfntfont.c` `27 conditions, 0 unknown, 0 disagreements [VERIFIED]`.

## Deviations

1. **`switch` → `match`**, with C's `break` implicit and the C line of each `case` written on its
   arm (the `sfnt.rs` 1004-1010 shape).  Three functions: `sfnt_setup_coding_system` (C 184-218),
   `sfnt_registry_for_subtable` (C 632-683), `sfntfont_identify_cmap` (C 702-765).  Two spots need
   a word more than the mechanical rewrite: (a) C's `case SFNT_MICROSOFT_SYMBOL:` /
   `case SFNT_MICROSOFT_UNICODE_BMP:` share one body, which is Rust's `A | B =>` or-pattern;
   (b) `sfntfont_identify_cmap`'s Microsoft `switch` has no `default:` and *falls through* into the
   outer `default: return 3;`, so the port returns 3 at that arm's tail with a comment, which is
   the same control flow and not an extra return.
2. **Forward `goto` to a tail label → a labeled block** (PORTING.md 2.2): `sfnt_grok_registry`'s
   three `goto done` (C 803/815/828) are `break 'done` out of a `'done: { … }` block whose end is
   C 833's `done:` label, and `sfnt_parse_style`'s four `goto next` (C 483/497/514/531) are
   `break 'next` out of a `'next: { … }` block whose end is C 552's `next:`.  Neither becomes an
   early `return`, so no cleanup is skipped.
3. **The `bailN` ladders are a jump depth, not four labels.**  `sfnt_enum_font_1`'s four
   `goto bail1..bail4` (C 977/982/986/996) jump *into the middle* of C 1143-1152's fall-through
   cascade, so the port carries the depth in `bail` and runs the four segments in C's order under
   `if bail >= N` at the tail - the same frees, in the same order, on the same paths (PORTING.md
   2.2's "must not become an early `return`").  `sfnt_enum_font`'s `bail0:`/`bail:` pair (C
   1236-1239, reached by `goto bail` at C 1177 and `goto bail0` at C 1188/1193/1228) is the same
   shape with `bail` 1 = `bail:` and 2 = `bail0:`.
4. **`(struct sfnt_offset_subtable *) -1` → `usize::MAX as *mut sfnt_offset_subtable`** (C
   1182/1211).  The token `1` of C's `-1` is what `extract-abi.py` counts as missing, which is the
   `docs/ports/token-aliases.tsv` row Owed 2 asks for (the `src/sfnt.c` ledger's token-aliases
   row, still owed there and now used here as well).
5. **`p[i]` on a raw pointer → `*(p.offset (i as isize))`**, and `&p[i]` → `&raw mut
   *(p.offset (i as isize))` (C 799/802/811/814/824/827, C 1091, C 1199/1201/1215).  This is the
   second half of the same owed `token-aliases` row.
6. **C's bitfields → one integer byte plus masks** (PORTING.md 1.3).  `sfnt_font_desc`'s
   `bool_bf underline_position_set : 1` and `bool cmap_invalid : 1` are the byte `bits` at offset
   132 (measured; Evidence ②), and C 1052's write is
   `(*desc).bits = (*desc).bits | SFNT_FONT_DESC_UNDERLINE_POSITION_SET`.  The two C names and the
   literal `true` are then absent from the body, which is the `macro-body-notes`/`token-aliases`
   pair Owed 2/3 ask for.
7. **`coding_system`'s `mode` and `common_flags` are the mirror's `bits` word** (coding.rs
   126-244, PORTING.md 1.3): C 309-312's three writers are
   `coding.bits |= (CODING_MODE_… as u32) << CODING_SYSTEM_MODE_SHIFT` and
   `coding.bits &= !((CODING_ANNOTATION_MASK as u32) << CODING_SYSTEM_COMMON_FLAGS_SHIFT)`.
8. **C's implicit conversions are written as casts at their sites**: `uint16_t` fields into the
   `sfnt_platform_id`/`int`/`size_t` parameters (C 360-369: `family_rec.platform_id as
   sfnt_platform_id`, `… as c_int`, `… as size_t`), the `u16`/`u32` fields against the `c_int`
   enums (C 632/702/715 and the `scaler_type` pair at C 975-976, `as c_int`/`as uint32_t`), the
   `size_t` `length` into `ptrdiff_t` parameters (C 315), `make_fixnum (temp.width as EMACS_INT)`
   (C 1105-1107), `(*maxp).num_glyphs as c_int` (C 1033), and `(*ttc).offset_table[i] as off_t`
   (C 1198/1229).
9. **`?:` is Rust's `if … { … } else { … }`** (C 372's `? 0 : 1`, C 1049's `? 100 : 0`), which
   the token check reads as the same tokens (neither `?` nor `:` is an identifier).
10. **Uninitialized C locals that Rust needs bound before their address is taken are
    zero-initialized** (PORTING.md 1.5): `coding` (C 305), `family_rec`/`style_rec` (C 329),
    `name_rec` (C 382), `map` (C 585), `temp` (C 963), `family`/`style` (C 964, the two
    out-parameters of `sfnt_decode_family_style`), `subtables` (C 779, the out-parameter of
    `sfnt_read_cmap_table`), and `head`/`name`/`maxp`/`meta` (C 956-959), which the `bailN` ladder
    frees - on every path C can reach each of those has been assigned before its label is jumped
    to, so the zeroing is only what makes the guards well-defined.
11. **`single[0] = c_toupper (single[0]);` is `*single = c_toupper (*single as c_int) as c_char;`**
    (C 542), and the `char c = SREF (…)` / `SSET (…, ' ')` pair (C 565/567) is
    `c = SREF (…) as c_char` / `SSET (…, b' ')` - C's `char`↔`int` promotions made explicit.
12. **`while ((single = strtok_r (…)))` is Rust's `while { single = …; !single.is_null () }`**
    (C 475), and `if (memmem (…))` is `if !memmem (…).is_null ()` (C 609-615) - the pointer
    truthiness tests C spells with a bare pointer.
13. **`prev`/`next` walk is the same loop without C's empty increment clause**:
    `for (; *next; prev = *next)` (C 1125) becomes `while !(*next).is_null () { …; prev = *next; }`,
    so the increment stays the body's last statement.
14. **The `harfbuzz` members are gated, the `HAVE_MMAP` member is not** - see the Gate mapping
    table.  The record's layout assertions are written for the frozen reading
    (`not (feature = "harfbuzz")`), because that is the configuration clang measured.
15. **Two use-site function declarations with the reference header's line** (P-72 rule 3/P-55):
    `setup_coding_system` (C 222, coding.h 686), `internal_condition_case` (C 286, lisp.h 4805) and
    `strtok_r` (C 475, `string.in.h` 1224-1227, no `#[link_name]` - Evidence ⑤).  The mirrors
    `include/coding.rs` and `include/lisp.rs` do not carry the first two.
16. **`min`/`max`-free**: nothing in this slice needs either, so `sfnt.c`'s `MIN`/`MAX` rows do not
    recur here.
17. **`sfntfont_charset_for_cmap`'s two `switch`es are `match`es, and the inner Microsoft one has no
    `default:` of its own.**  C 1305's inner `switch` names six `SFNT_MICROSOFT_*` cases and then
    falls out of the block into C 1331's outer `default: return NULL;`; the port writes that
    fall-out as an empty `_ => { }` arm plus the explicit `return std::ptr::null_mut ()`, the shape
    Deviation 2 (b) records for `sfntfont_identify_cmap` (whose Microsoft `switch` has the same
    hole).  The `case`/`default`/`switch` tokens C's body carries are in
    `docs/ports/macro-body-notes.tsv` (`sfntfont_charset_for_cmap	case,default,switch`), which is
    why the `--check`'s `does not mention` class stays at 0 rows.
18. **Slice 3's three new shapes.**  (a) **`sfnt_glyph` (the scalar typedef) is
    `sfnt_glyph_index`** in the port, because `sfnt_glyph` is `struct sfnt_glyph`'s name here -
    `sfntfont_glyph_valid` C 1507's local is that type, and the `macro-body-notes.tsv` row records
    the rename (the `sfnt_lookup_glyph_4` row's class).  (b) **C 1514's `glyph <= desc->num_glyphs`
    is an *unsigned* comparison** (the usual arithmetic conversions promote the `int` member to
    `unsigned int`), so the port writes `glyph <= (*desc).num_glyphs as c_uint` rather than casting
    the glyph down.  (c) **C 1546's `desc->cmap_invalid` and C 1554's write of it are the
    `SFNT_FONT_DESC_CMAP_INVALID` mask** on C 164's 1-bit member (Deviation 14's packing), with the
    `sfntfont_lookup_char` row in `macro-body-notes.tsv` - and C 1543's `?:` is the `if`/`else`
    expression of Deviation 9, and the two out-parameter locals of C 1437-1438 are Deviation 10's
    zero-initialisation.
19. **Slice 4's four shapes.**  (a) **C's implicit promotions are written out**: in `sfntfont_list_1`
    C compares `FONT_WIDTH_NUMERIC (spec)` (the macro yields `long`) with `desc->width`
    (`int`), and `xfnum` with `XFIXNUM (AREF (item, 2))` - the port widens the *member*
    (`(*desc).width as isize`) rather than narrowing the macro, which is C's usual arithmetic
    conversion; the arm flip read three `E0308` errors before the casts were there, so this is a
    measured shape rather than a style choice.  (b) **`goto fail` is the `'fail:` labelled block**
    (PORTING.md 2.2, the `androidselect.rs` 414 shape): C's five legs are `break 'fail;`, and the
    block ends where C's `fail:` label sits, so the function's tail (C 1822-1823) is the label's
    body.  (c) **C 1618's `cmap` and C 1620's `subtable` are zero-initialised** (Deviation 10's
    class): `cmap` is set to `NULL` at C 1754 on every path that reaches `xfree`, and `subtable` is
    copied into `desc->subtable` at C 1815 only when a cmap was read.  (d) **`FOR_EACH_TAIL_SAFE`
    is `include/lisp.rs` 4389's macro**, whose `break`/`continue` are the loop's - C 1650's `break`
    and C 1711/1717/1724/1729/1734's `continue`s are written inside the macro body exactly as C has
    them.
20. **Slice 5's five shapes.**  (a) **`make_mint_ptr (desc)` needs `desc as *mut c_void`**: the
    mirror's `make_mint_ptr` takes the untyped pointer (P-72 (3)'s class), and the arm flip read
    two `E0308`s before the cast.  (b) **`FONT_SET_STYLE`'s three uses are the mirror's macro**
    (`include/font.rs` 246), whose body calls `font_style_to_value` and `make_fixnum`, so the
    three `int` members are widened at the call site (`(*desc).width as EMACS_INT`), the same
    direction as Deviation 19 (a).  (c) **Two use-site declarations** (P-91 4): `font_make_entity`
    (C 1883) and `font_style_to_value` (C 1904's macro) are `font.h` 842 and 854 prototypes no
    mirror carries, and the arm flip read them as six `E0425`s before the block was there - the
    `src/ftfont.rs` 1306 / `src/haikufont.rs` 127 shapes.  (d) **The subr record
    `Scompare_font_entities` (C 1964-1971) is spelled out as `Aligned_Lisp_Subr`/`Lisp_Subr`**
    (the `alloc.rs` 8175 initialiser: `vectorlike_header`'s size, the `Lisp_Subr_function`'s `a2`,
    `min_args`/`max_args` 2, the symbol name as a `c"…"` pointer, the null `intspec`, `lisp_h_Qnil`
    command modes and `doc: 0`, with the four `have_native_comp` members gated as the mirror gates
    them); C's `sfntfont_compare_font_entities` therefore carries the C ABI (P-52), and `XSETSUBR`
    is the macro.  (e) **C 1981's `int instances[100]` is zero-initialised** (Deviation 10's
    class), and `countof! (instances) as c_int` is C's `countof (instances)` argument.
21. **Slice 6's four shapes.**  (a) **C's pointer truth tests are null tests**: C 2124's
    `glyph->simple` (and the same field at C 2262 in the next slice) is `struct sfnt_simple_glyph *`
    (`sfnt.h` 677), so the port writes `!(*glyph).simple.is_null ()`; the arm flip read one `E0308`
    before the fix.  (b) **C 2113's `distortion` is zero-initialised** (Deviation 10's class): C
    passes its address to `sfnt_vary_simple_glyph`/`sfnt_vary_compound_glyph` and never reads it
    first.  (c) **Three of the four helpers carry the C ABI** `unsafe extern "C" fn` because their
    addresses are taken - C 2313-2314 and C 2355-2357 hand `sfntfont_get_glyph`,
    `sfntfont_free_glyph` and `sfntfont_get_metrics` to the interpret functions - while
    `sfntfont_dereference_outline` is called by name only (C 2400/2431/3537/3726) and stays a plain
    `unsafe fn` (P-52).  (d) **The three records' `#[repr(C)]` layouts carry the clang probe's
    numbers as `core::assert!`s** (Evidence ⑮), and `sfnt_glyph`'s scalar typedef member is
    `sfnt_glyph_index` (Deviation 18 (a)).
22. **Slice 7's five shapes.**  (a) **`SFNT_ROUND_FIXED` and `SFNT_FLOOR_FIXED` are functions** in
    the mirror (`include/sfnt.rs` 1742/1749), not macros, so C 2335/C 2343's calls carry no `!`.
    (b) **C 2316's and C 2358's `&dcontext` are the untyped `void *`** in the mirror's signatures
    (`sfnt_interpret_compound_glyph`'s `arg9`, `sfnt_build_glyph_outline`'s `arg6`), so both sites
    write `(&raw mut dcontext) as *mut c_void` (P-72 (3)); the arm flip read two `E0308`s before
    the casts.  (c) **C 2217's `dcontext` and C 2222's `advance` are zero-initialised** (Deviation
    10's class): Rust wants the whole binding before the first field write, and C 2326 takes
    `advance`'s address - the arm flip read two `E0381`s before the initialisers.  (d) **C 2289's
    `index != -1 ? blend : NULL` is the `if`/`else` expression** (Deviation 9), and C 2294's
    `goto fail` with C 2365's `fail:` label is the `'fail:` block (Deviation 19 (b)'s shape, with
    its own `macro-body-notes.tsv` row).  (e) **C 2518's `SFNT_OUTLINE_CACHE_SIZE` test inside the
    *raster* cache is carried verbatim** - C compares the raster cache's size against the outline
    cache's constant there, and the port keeps C's text rather than "fixing" it (the ledger says so
    at the site).
23. **Slice 8's five shapes.**  (a) **Two items carry `have_window_system`**: `sfntfont_lookup_glyph`
    reads `font.encoding_charset`, one of the eight members C guards with `HAVE_WINDOW_SYSTEM`
    (`font.h` 325-403, which the mirror gates), and `sfntfont_probe_widths` calls it - so both carry
    `all (have_android, not_android_stubify, have_window_system)`.  Every branch that builds
    `sfntfont.o` is a window-system build (`configure.ac` 2645's Android branch), which is why C
    itself reads the member unconditionally; the ledger's Gate mapping states the implication.
    (b) **`sfntfont_measure_pcm` is declared at its use site** (C 2681-2682's prototype, the
    `ftfont.rs` 2066 `hbotf_open` shape, P-91 4) because C's own order puts the caller (C 2692)
    before the definition (**C 3504**); the slice that lands C 3504 replaces the declaration with
    the definition, and `## Owed` 9 carries that.  (c) **C 2726/2730/2731/2736's arithmetic widens
    `pcm.width`** (`font_metrics.width` is `c_short`) to `c_int`, which is what C's integer
    promotions do at those sites.  (d) **C 2697's `pcm` is zero-initialised** (Deviation 10's class:
    its address is passed to `sfntfont_measure_pcm`).  (e) **The two `HAVE_MMAP` members are
    ungated** (C 2572's `next`, C 2632's `glyf_table_mapped`): `HAVE_MMAP` is 1 in the frozen
    configuration and its arms have no `#else` - slice 1's `sfnt_font_tables` reading.
24. **Slice 9's six shapes.**  (a) **C's `bail1:`/`bail:` pair is the `'body: loop` + level
    variable** slice 1's `sfnt_enum_font` uses (PORTING.md 2.2): `goto bail` (C 2787/2796/2808)
    breaks with the level still 0, `goto bail1` (C 2824/2843) sets it to 1, and the tail runs
    `xfree (interpreter)` only for level 1 - C's label order (C 2856 `bail1:` then C 2858 `bail:`).
    (b) **C 2761's `interpreter` and C 2763's `state` are zero-initialised** (Deviation 10's class):
    the `bail1` path's `xfree` reads the first, and C 2836 passes the second by address - and C
    2763's is a *real* latent C bug (with `prep` set and `fpgm` NULL, C reads an uninitialised
    `state` at C 2851); the port gives it a defined value and the ledger says so rather than
    "fixing" C's text.  (c) **C 2762's `error` is `mut`** because both interpret calls assign it
    (the arm flip's `E0384`).  (d) **`message_with_string` is declared at its use site**
    (`lisp.h` 4397, no mirror carries it; the `src/fileio.rs` 4466 shape, P-91 4).  (e) **The
    `HAVE_MMAP` arms are ungated** (C 2868's `rc` local and C 2877-2887's `if`/`else`, whose `else`
    body sits *after* the `#endif`): `HAVE_MMAP` is 1 in the frozen configuration and the condition
    has no `#else`.  (f) **C 2902-2914's `HAVE_HARFBUZZ` block is `#[cfg (feature = "harfbuzz")]`**,
    because its two members (`fd`, `directory`) carry that gate in the record.
25. **Slice 10's four shapes.**  (a) **C's six-label ladder is the depth-carrying chain** slice 1's
    `sfnt_enum_font_1` uses (PORTING.md 2.2): the nine `goto bailN`s (C 2939/2945 → `bail`,
    C 2951 → `bail1`, C 2960 → `bail2`, C 2992 → `bail3`, C 3020/3031/3039 → `bail4`, C 3047 →
    `bail5`) become `break 'body` with `bail` set, and the six segments (C 3077 `bail5:` … C 3110
    `bail:`) run in C's order - including C 3100's `xfree (cmap_data)`, which C puts *inside*
    `bail4` "due to a peculiarity of how the four tables above are validated".  (b) **C 2924's
    `subtable` and C 2925's `fd` are zero-initialised** (Deviation 10's class): C's flow guarantees
    they are assigned before the `bail2`/`bail1` segments read them, but Rust's analysis cannot see
    through `bail`; the comment says exactly that.  (c) **C 2998-3013's and C 3084-3094's `HAVE_MMAP`
    `if`/`else` pairs straddle an `#endif`** (the `else` body is *after* the guard's end): since
    `HAVE_MMAP` is 1 and the condition has no `#else`, the port writes both arms out with their
    guards, which is what C's text means in this configuration.  (d) **C 3064-3073's
    `HAVE_HARFBUZZ` `#ifdef`/`#else`** is two `#[cfg]`-selected blocks (its two members carry that
    gate), and the port adds no defensive check C does not have (see Evidence ㉓).
26. **Slice 11's seven shapes.**  (a) **`sfntfont_open` carries `have_window_system`** on top of the
    file gate: it reads `font.encoding_charset`/`repertory_charset`, and the four
    `HAVE_WINDOW_SYSTEM` members C 3323-3326 clears (`baseline_offset`, `relative_compose`,
    `default_ascent`, `vertical_centering`), plus `FRAME_FONT`/`FRAME_DISPLAY_INFO` - the same
    reading as Deviation 23 (a).  (b) **C's two labels `bail6:`/`bail:` are the depth-carrying
    `'body: loop`** (0 = `goto bail` at C 3247, 6 = `goto bail6` at C 3279), and C 3410's
    **`cancel_blend:` is a *forward* label**: C 3386 jumps past the instance block, so the port
    wraps that block in `'cancel_blend: { … }` whose end is the label (the `sfnt.rs` shape).  (c)
    **C's implicit conversions are written as casts**: `font->ascent = ceil (…)` and
    `font->max_width = (…)` are `double`-to-`int` (the `as c_int` at C 3286-3298), `XFIXNUM`'s
    `EMACS_INT` values narrow to `c_int` at C 3171/3196, `sfnt_coerce_fixed (…) + 0.5` truncates at
    C 3335-3340, and `PT_PER_INCH` (a `double` in C too) makes C 3345's first argument a `double`
    (`pixel_size as c_double`, the `font.rs` 5379 call's shape).  (d) **The fvar instance is a
    pointer, not an array**: C 3369's `fvar->instance[i].coords[j]` is
    `(*(*fvar).instance.add (i)).coords.add (j)` in the port (both `instance` and `coords` are
    pointers in the mirror), and the `<=`/`<` guards around it are C's own.  (e) **The two `bool`
    members are the `bits` masks**: C 3328's `desc->underline_position_set` is
    `SFNT_FONT_DESC_UNDERLINE_POSITION_SET` (Deviation 14's packing).  (f) **`font_make_object` is
    declared at its use site** (`font.h` 846, no mirror carries it; the `src/haikufont.rs` 123
    shape).  (g) **The drivers' `HAVE_HARFBUZZ` blocks are `#[cfg]`-selected** (C 3234-3238's
    initialisers and C 3418-3424's fd/directory copies), while C 3231-3233's and C 3430-3434's
    `HAVE_MMAP` arms are ungated.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

9. **The use-site declaration of `sfntfont_measure_pcm`** (slice 8, Deviation 23 (b)): C 3504 defines
   it, and the slice that lands C 3504 must **replace** the `unsafe extern "C"` declaration with the
   definition (a second same-named item in one crate root is `E0428`).  Until then the arm flip in
   both cfg sets reads 0 errors because the declaration is there - the `ftfont.rs` 2066
   `hbotf_open` disposition.

1. **`femacs/src/include/sfntfont.rs`: wrap the declaration run in
   `pub mod sfntfont_decls { use super::*; … } use sfntfont_decls::*;`** (the `include/sfnt.rs`
   shape landed in 3.427, P-86 ⑩).  Until then the arm flip's **only** error is the `E0428` for
   `sfnt_enum_font` at that mirror's line 55, and this unit's own text is never *fully* compiled
   (Evidence ④).  The mirror is another lane's in-flight file
   (`docs/ports/include-sfntfont.md` is its ledger) and is outside this lane's write scope.
   ✅ **Closed by a mirror round and re-measured by the integrator in 3.479**: the module is in
   place (`include/sfntfont.rs` 73-134, `use sfntfont_decls::*;` at 134) and the same arm flip now
   reports **exit 0, 0 errors, 6 warnings, a 17,627,624-byte rlib** (`--cfg 'feature="have_android"'
   --cfg 'feature="not_android_stubify"'`) - the unit's text compiles end to end for the first
   time.  The six warnings are the mirror-side `unused doc comment` class this ledger already
   names (`stdbit.in.rs` 644, `lisp.rs` 3934, `charset.rs` 697, `sfntfont.rs` 53, `frame.rs`
   1355/1360).
2. **`docs/ports/token-aliases.tsv` rows**: (a) a **bitfield family** for `underline_position_set`
   - `underline_position_set<TAB>underline_position_set,sfnt_font_desc_underline_position_set`, the
     `dump_flags_*`/`charset_*` shape;
   (b) the `(T *) -1` → `usize::MAX as *mut T` row, already owed by the `src/sfnt.c` ledger's
   `## Owed` list (its item 2 in this revision) and now used by a second unit (C 1182/1211);
   (c) the `p[i]` on a raw pointer → `*(p.offset (i as isize))` row, the same pair.
3. **`docs/ports/macro-body-notes.tsv` rows** for the nine functions Evidence ④ names, each with
   the tokens the body legitimately cannot spell and the reason (the `PORTING.md 2.2` /
   `PORTING.md 1.3` rulings of Deviations 1-3, 6 and 7).  The `true` token of `sfnt_enum_font_1`
   is the `syms_of_indent`/`Fforce_mode_line_update` class (a C `bool` literal is Rust's `true`,
   and the port writes the *bit* instead).
4. **A P-88 declaration segment for `<hb.h>`/`<hb-ot.h>`** (C 37-38) with the slice that first names
   an `hb_*` type (C 3989's `hb_blob_t`); no name in C 1-1240 is theirs (Evidence ③).
5. **`tools/check-port-lines.py --write-pending`**: this file's ratchet row is stale (1,554 Rust
   lines against a 422-line threshold, so the file has *left* P-86's under-ten-percent class with
   this batch); the baseline file is outside this lane's write scope, so the row is left for a batch
   close rather than edited here.  **Re-read after slice 2 (status 3.681)**: the file is 1,776 Rust
   lines and the gate reads `0 file(s) under ten percent, 0 ratchet row(s), 0 new, 0 stale` - the
   stale row is gone, so nothing is owed under this item any more.
6. **`docs/baseline/citations-allowlist.tsv`** (outside this lane's write scope): the working
   tree's `tools/check-citations.py` reads **2 drift** rows, both belonging to *other* lanes'
   in-flight ports - `femacs/src/gtkutil.rs:92` (`lisp.h 30`, `strlen` is at lisp.h 4558) and
   `femacs/src/w32proc.rs:456` (`lisp.h 30`, `memset` is at lisp.h 2008) - so the gate exits 1 on
   work this lane does not own.  This lane's own two lines had that
   `lisp.h 30`-then-a-backticked-identifier shape until they were rewritten out of it; the fix for
   the other two is an allowlist row or the same rewrite, and belongs to those lanes or to the
   batch close.

### Not claimed

Every line of this slice is behind a gate the frozen configuration leaves false: **the frozen
object (①) is 336 bytes of nothing, and the arm reading (④) never produced an object** - the arm
flip stops at the mirror's `E0428`.  So there is **no runtime evidence of any kind** for this port:
no font file was opened, no name table was decoded, no style was parsed, no cmap was identified, no
`lseek`/`read`/`memmem` returned.  The reading above is slice 1's, and it still holds item for item;
**slices 2-11 add no runtime evidence either** (Evidence ⑦/⑨/⑪/⑬/⑯/⑰/⑳/㉑/㉓/㉕: every arm flip's
diagnostics are unchanged; slices 5 and 11 grew their objects because of `no_mangle` entry points
that reach the glyph machinery - all of it code placement rather than behaviour).
C 3473-4222 is **not ported here**; the 12 `- missing function` rows and the 1 remaining object
(`sfnt_put_glyphs`) of Evidence ㉖ are that boundary, not a defect of these eleven slices (the
line-count row the earlier readings carried is gone).  The clang probe's
numbers are layout, not behavior.  The one feature this unit uses besides the file gate's pair
(`harfbuzz`) is the tree's registered name, not this lane's invention, and `have_mmap` is
deliberately unused (the condition has no second arm).
