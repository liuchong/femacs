# emacs/src/sfntfont.c (4,222 lines) — first slice: C 1-1240

> **P-86's unit `src/sfntfont.c`, ported in slices.**  The frozen configuration compiles nothing
> here: `configure.ac` 2645 adds `sfntfont.o` to `ANDROID_OBJ` only inside the `XCONFIGURE =
> android` branch (the `REALLY_ANDROID` one, 2620-2654 — `sfnt.o` is on the same line, `android.o`
> one line earlier at 2598; 2605-2606 is the other branch, where `ANDROID_STUBIFY` is defined), and
> `docs/baseline/frozen-build-sources.txt` lists no `sfntfont.o`.  What follows is P-86 ②'s **C
> line-range table** for this slice, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/sfntfont.rs` (1,554 lines, `grep -c ''`), both arms of every `#if` carried.
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
| 1241-4222 | **the remainder** — C 1241's blank, C 1242's form feed, C 1244's banner and from C 1246 (`sfntfont_charset_for_name`, **the next live definition**) to C 4222: the five cmap selectors and the cmap reader (C 1269-1500), the character lookup (C 1502-1612), `sfntfont_list_1` and the entity/registry helpers (C 1613-2057), the outline/raster caches (C 2059-2563), `struct sfnt_font_info` and the open/close machinery (C 2565-3470), the eight driver callbacks (C 3483-3960), the mmap sigbus pair (C 3961-3983) and the HarfBuzz entry points (C 3989-4110), and the three initialization subrs (C 4115-4222) | not this slice |

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

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`femacs/src/include/sfntfont.rs`: wrap the declaration run in
   `pub mod sfntfont_decls { use super::*; … } use sfntfont_decls::*;`** (the `include/sfnt.rs`
   shape landed in 3.427, P-86 ⑩).  Until then the arm flip's **only** error is the `E0428` for
   `sfnt_enum_font` at that mirror's line 55, and this unit's own text is never *fully* compiled
   (Evidence ④).  The mirror is another lane's in-flight file
   (`docs/ports/include-sfntfont.md` is its ledger) and is outside this lane's write scope.
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
   close rather than edited here.
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
`lseek`/`read`/`memmem` returned.  C 1241-4222 is **not ported here**; the 48 `- missing …` lines
and the 63.2% drift of Evidence ④ are that boundary, not a defect of this slice.  The clang probe's
numbers are layout, not behavior.  The one feature this unit uses besides the file gate's pair
(`harfbuzz`) is the tree's registered name, not this lane's invention, and `have_mmap` is
deliberately unused (the condition has no second arm).
