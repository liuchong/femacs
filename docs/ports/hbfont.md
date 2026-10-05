# emacs/src/hbfont.c (612 lines)

`docs/port-map.md` binds this ledger to `src/hbfont.c` alone - the reference has no
`src/hbfont.h`, so there is no mirror for this unit to `include!` (P-90 ② has no second surface to
check here).

## Decisions

- **The file gate is the build-level `feature = "harfbuzz"`, and no `#if` in the file names it.**
  `hbfont.o` joins `FONT_OBJ` only when `HAVE_HARFBUZZ` is defined (`emacs/src/Makefile.in`
  341-342), the frozen `FONT_OBJ=` is empty, `docs/baseline/config/darwin-arm64.macros.tsv` 1389
  records `HAVE_HARFBUZZ	undef`, and `docs/baseline/frozen-build-sources.txt` lists no
  `src/hbfont.c`.  The frozen configuration therefore compiles nothing from this file and this
  target's `build-tu` object is the 336-byte empty one (Evidence ①).  The name is registered
  (`femacs/xtask/Cargo.toml` 112 as `harfbuzz = []  # HAVE_HARFBUZZ`,
  `docs/baseline/feature-coverage.tsv` 22), so nothing has to be added for the port to be gated;
  the gate goes on every item *and* on C 20-29's include graph (P-86 ⑧).
- **Both arms of every `#if` are carried.**  C 31's `#ifdef HAVE_NTGUI` arm is
  `feature = "have_ntgui"` (the C macro's literal spelling, `femacs/xtask/Cargo.toml` 230), nested
  inside the file gate; `HAVE_NTGUI` is `undef` here and this machine has no Windows cross
  toolchain, so that arm's evidence is `text-only` (P-86 ③⑵).  C 458-460's `#if 0` block is dead
  text in every configuration and is carried as a comment.  C 87-114's 28 object-like `#define`s
  (`hb_x` -> `fn_hb_x`) have no Rust counterpart, so every call site of the body is spelled the way
  C's preprocessor leaves it - `fn_<func> (…)` under `have_ntgui`, the plain name otherwise (the
  `femacs/src/gnutls.rs` 1206-1210 shape, `femacs/src/lcms.rs` 42-45's record of the same class).
  There are 34 such call sites; each sits under `#[cfg (not (feature = "have_ntgui"))]` and
  `#[cfg (feature = "have_ntgui")]` in turn.
- **`<hb.h>`/`<hb-ot.h>` have no file in the reference tree, so the surface stands in the unit**
  (P-88 ①) at C 21-22's position, and every layout number and constant in it is a clang reading of
  this machine's header (HarfBuzz 14.4.0; Evidence ⑤), never a recollection.  HarfBuzz has more
  than one consumer in the reference (`ftfont.c`, `sfntfont.c` and `w32uniscribe.c` all
  `#include <hb.h>`, and `font.h`/`ftfont.h` carry `hb_font_t` members), so P-90 ① says the
  surface belongs in a mirror of its own, `femacs/src/include/hb.rs`; this lane may not create it,
  and `## Owed` 2 names the move (P-88 ⑤).
- **C 33's `#include "w32common.h"` is consumed through a module + glob import.**  The tree's
  mirror keeps its `unsafe extern "C"` items at its own top level while this unit *defines*
  `hbfont_init_w32_funcs` (C 118-150; w32common.h 99-101 declares it) - the P-39 `E0428` class,
  whose fix is a `pub mod …` plus `use …::*;` *inside the mirror* (the shape
  `femacs/src/include/sfntfont.rs` 96-141 already has).  The module here is that mechanism applied
  from the consumer, so the mirror's prototype and this unit's definition coexist exactly as C's
  prototype and definition do; `## Owed` 1 deletes the wrapper when the mirror is fixed.  Measured
  (rustc, both items at one scope): `error[E0428]: the name `foo` is defined multiple times`; with
  the mirror in a module and a glob import, the same pair compiles.
- **Four names the body reads are missing from the mirrors that own them** and are declared at
  their use sites with the reference header's line (P-72 rule 4, P-55): `lround` (`<math.h>` 474),
  `get_unicode_property` (lisp.h 4697), `bidi_mirror_char` (dispextern.h 3553) and
  `font_intern_prop` (font.h 885-886).  `## Owed` 3-6 names the mirror each belongs to.
- **The include graph carries the edges the six headers reach through other headers** (P-86 ⑧):
  `include/systime.rs` (`termhooks.h`'s `Time`), `include/character.rs` (lisp.h's `MAX_CHAR` and
  the category enum), `include/coding.rs`, `include/window.rs`, `include/intervals.rs` and
  `../lib/include/{string.rs,stdcountof.rs,limits.in.rs}` - the same set
  `femacs/src/abi_shim.rs` 24-42 spells for this neighbourhood.  Without them the flip stops in
  the mirrors before this unit is read (Evidence ⑥).
- **The three measured records are pinned with `core::assert!`, each carrying the file gate**
  (PORTING.md 1.3): the asserts name types the gate defines, and ungated they outlive the types -
  measured, `build-tu src/hbfont.c` reported five `E0425`s until the attributes were added (the
  frozen reading is clean now, Evidence ①).

## C line ranges

The rows are contiguous and cover C 1-612 (the blank lines between blocks need no row): 18, 23,
30, 32, 86, 115, 152, 215, 239, 244, 266, 343, 349, 375 and 607.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①/0.1-19; the port opens with the `// femacs: port of …` line) |
| 19 | `#include <config.h>` | ungated - C's own text has it outside every `#if` |
| 20 | `#include <math.h>` | file gate (`lround` is the one name read from it) |
| 21-22 | `#include <hb.h>`, `#include <hb-ot.h>` | file gate; the declarations stand in the unit (P-88 ①, `## Owed` 2) |
| 24-29 | `lisp.h`, `frame.h`, `composite.h`, `font.h`, `dispextern.h`, `buffer.h` | file gate; each mirror plus the chain edges (`## Decisions` 6) |
| 31-151 | `#ifdef HAVE_NTGUI`: C 33's `w32common.h`, C 38-85's 28 `DEF_DLL_FN`, C 87-114's 28 renames, C 116-150 `hbfont_init_w32_funcs` | `have_ntgui` (nested in the file gate); text-only |
| 153-214 | `hbfont_otf_features` | file gate; every `hb_*` call spelled per arm |
| 216-238 | `hbfont_otf_capability` (C 219's uninitialised `position_unit`) | file gate; per-arm calls |
| 240-243 | the two file-scope objects (`combining_class_loaded`, `canonical_combining_class_table`) | file gate |
| 245-265 | `uni_combining` | file gate |
| 267-342 | `uni_general` (C 271-339's `switch`) | file gate; `match` (Deviations 3) |
| 344-348 | `uni_mirroring` | file gate |
| 350-374 | `get_hb_unicode_funcs` (C 362-366 and 368-371 the two commented-out blocks) | file gate |
| 376-393 | `hbfont_shape`'s doc comment | file gate (carried as a comment) |
| 394-606 | `hbfont_shape` (C 419's bare `alloca`, C 458-460's `#if 0`, C 513's two `UNINIT`) | file gate; per-arm calls |
| 608-612 | `hbfont_combining_capability` | file gate |

## Findings

1. **The gate is not in the file.**  `src/hbfont.c` has exactly two conditionals - C 31's
   `#ifdef HAVE_NTGUI` and C 458's `#if 0` - and neither is the file gate; P-86 ⑧'s "the gate goes
   on the include graph" therefore had to be read off `emacs/src/Makefile.in` 341-342 and the
   frozen macro table, not off the file's own text.  The frozen extraction is the empty one for
   that reason: `extract-abi.py emacs/src/hbfont.c` lists 0 items until the port has an `fn`, and
   after that it lists the 8 live functions and the 2 file-scope objects (C 88-114's 28 macros and
   C 118-150's function are inside the dead `#ifdef HAVE_NTGUI` and are *not* required - P-86 ⑩).
2. **The arm flip is blocked in a mirror, not in this unit.**  `femacs/src/include/font.rs` 577
   asserts `size_of::<font_driver> () == 208` ungated, while the two fields that move it
   (`begin_hb_font`, `end_hb_font`, font.rs 562-565) are `#[cfg (feature = "harfbuzz")]`: with the
   arm on the record is 224 bytes and the assertion fires (E0080) before this unit is read.  The
   reading is in Evidence ②, the fix is `## Owed` 7, and the masked-copy reading in Evidence ③ is
   what shows this unit's own text is error-free.
3. **`include/w32common.rs` declares the function this unit defines** (w32common.h 99-101's
   `extern bool hbfont_init_w32_funcs (HMODULE);`), so consuming that mirror as C does is `E0428`
   while the mirror keeps its externs at its top level.  This unit is the first consumer to hit it
   (`decompress.rs`/`lcms.rs`/`gnutls.rs` all include the mirror and define other names).
4. **`unicode_category_t`'s members need a glob import at each consumer**: the enum lives in
   `include/character.rs` 537-571 and C names its members bare (`case UNICODE_CATEGORY_Cc:`), so
   the port writes `use unicode_category_t::*;` (the unit `femacs/src/character.rs` 72 does the
   same; `femacs/src/composite.rs` 1099 spells the path out instead).  Measured: without it the
   flip reported 31 `E0425`s, one per arm.
5. **`buffer.rs`'s field is `bidi_display_reordering_`** (buffer.rs 495) and the port's `BVAR!`
   names C's trailing-underscore idiom at the call site (`BVAR! (current_buffer,
   bidi_display_reordering_)`, the `alloc.rs` 6450 `undo_list_` shape); C 446's bare
   `bidi_display_reordering` is not a name the mirror carries.
6. **The catalogues the port's first edition got wrong were found by the flip, not by the text
   gates** (the P-86 ⑦/⑩ class): the `HB_DIRECTION_IS_BACKWARD` call needed its `!` (the flip
   reported `E0423: cannot find function … in this scope … a macro named
   HB_DIRECTION_IS_BACKWARD exists in another namespace`), the five record assertions needed the
   file gate (`build-tu` reported five `E0425`s), and the 31 category arms and the one `BVAR!`
   field name are Findings 4 and 5.  All four of those are the port's own defects and are
   corrected in the landed revision.
7. **HarfBuzz's enumerators are not what the C's names suggest** where the two languages' orders
   differ: `HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED` is **2** (not 0), the category enum's order is
   `CONTROL, FORMAT, UNASSIGNED, PRIVATE_USE, …`, and `HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES`
   is 0 while `…_MONOTONE_CHARACTERS` is 1.  Every value in the unit's segment is the clang
   reading in Evidence ⑤.

## Evidence

① frozen reading: `build-tu src/hbfont.c` -> **exit 0**, `build/femacs/hbfont.o` **336 bytes** -
the empty object the file gate leaves.  This is *not* a comparison against a reference object: the
frozen reference builds no `hbfont.o` either (`build/emacs-ref/src/Makefile` 342 `FONT_OBJ=`), and
`docs/baseline/frozen-build-sources.txt` lists no `src/hbfont.c`.

② **arm flip (a real compile - HarfBuzz is on this machine)**:
`probe src/hbfont.rs /tmp/hbfont-lane/x.rlib --crate-type=lib -C panic=abort --cfg
'feature="harfbuzz"'` -> **exit 1, 1 error, 35 warnings, no rlib** (rustc stops on the const-eval
error).  The one error is the mirror gap of Finding 2:
`error[E0080]: evaluation panicked: assertion failed: std::mem::size_of::<font_driver>() == 208`,
in `build/probes/hbfont/_…_include_font_rs.rs` (i.e. `femacs/src/include/font.rs` 577).  The 35
warnings are the house classes: 34 `unused doc comment` (the `/// C:` lines on the in-unit
`unsafe extern "C"` blocks - rustdoc documents no extern block) and 1 `unused attribute`
(`include/frame.rs` 1360's `#[inline(always)]`), none of them in this unit's own statements.

③ **the same flip with that one assertion masked in a `/tmp` copy of the materialized graph** (the
tree is untouched; the whole graph was copied and the copy's `include!` paths rewritten):
`rustup run nightly-2026-10-01 rustc --edition 2024 -O /tmp/hbfont-stub/_root.rs -o
/tmp/hbfont-stub/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="harfbuzz"'` -> **exit 0,
0 errors, 36 warnings, a 14,574,920-byte rlib**.  This is the "this unit's own text is
error-free" reading; it is *not* a reading of the tree (Evidence ② is), and the missing rlib is
why §"Not claimed" says the arm does not pass today.

④ `extract-abi --check emacs/src/hbfont.c femacs/src/hbfont.rs` -> **one `-` row**:
`line count drifted 161.3%: C 612 vs Rust 1599` - the ledger-owed tolerance row (`## Owed` 8).
Everything else is clean: the 8 live functions find their Rust definitions, the 2 file-scope
objects are declared, and every function body passes the token check with the two note rows this
unit adds to `docs/ports/macro-body-notes.tsv` (`uni_general` `switch,case`; `hbfont_shape`
`alloca,uninit`).

⑤ clang measurements of this machine's header (`/opt/homebrew/include/harfbuzz`,
`Cellar/harfbuzz/14.4.0`, `HB_VERSION_MAJOR/MINOR/MICRO` 14/4/0).  Probe:
`clang -I/opt/homebrew/include/harfbuzz -o /tmp/hb-probe /tmp/hb-probe.c -L/opt/homebrew/lib
-lharfbuzz && /tmp/hb-probe` -> exit 0, printing:
`hb_tag_t 4 hb_codepoint_t 4 hb_bool_t 4 hb_position_t 4 hb_mask_t 4`; `hb_var_int_t 4
hb_language_t 8 hb_unicode_funcs_t* 8 hb_face_t* 8 hb_font_t* 8 hb_buffer_t* 8
hb_destroy_func_t 8`; `hb_direction_t 4 hb_unicode_combining_class_t 4
hb_unicode_general_category_t 4 hb_buffer_content_type_t 4 hb_buffer_cluster_level_t 4`;
`hb_glyph_info_t 20 codepoint 0 mask 4 cluster 8 var1 12 var2 16`; `hb_glyph_position_t 20
x_advance 0 y_advance 4 x_offset 8 y_offset 12 var 16`; `hb_feature_t 16 tag 0 value 4 start 8
end 12`; `HB_OT_TAG_GSUB 1196643650 HB_OT_TAG_GPOS 1196445523
HB_OT_LAYOUT_DEFAULT_LANGUAGE_INDEX 65535`; `HB_DIRECTION_INVALID 0 LTR 4 RTL 5 TTB 6 BTT 7`;
`HB_DIRECTION_IS_BACKWARD(LTR) 0 HB_DIRECTION_IS_BACKWARD(RTL) 1`;
`HB_BUFFER_CONTENT_TYPE 0 1 2  CLUSTER_LEVEL 1 0 2  DEFAULT 0` (that last field prints
MONOTONE_CHARACTERS, MONOTONE_GRAPHEMES, CHARACTERS and DEFAULT in that order: 1, 0, 2, 0);
`HB_UNICODE_COMBINING_CLASS_NOT_REORDERED 0`; and the 30 general categories in declaration order
`UNASSIGNED 2 CONTROL 0 FORMAT 1 PRIVATE_USE 3 SURROGATE 4 LOWERCASE_LETTER 5 MODIFIER_LETTER 6
OTHER_LETTER 7 TITLECASE_LETTER 8 UPPERCASE_LETTER 9 SPACING_MARK 10 ENCLOSING_MARK 11
NON_SPACING_MARK 12 DECIMAL_NUMBER 13 LETTER_NUMBER 14 OTHER_NUMBER 15 CONNECT_PUNCTUATION 16
DASH_PUNCTUATION 17 CLOSE_PUNCTUATION 18 FINAL_PUNCTUATION 19 INITIAL_PUNCTUATION 20
OTHER_PUNCTUATION 21 OPEN_PUNCTUATION 22 CURRENCY_SYMBOL 23 MODIFIER_SYMBOL 24 MATH_SYMBOL 25
OTHER_SYMBOL 26 LINE_SEPARATOR 27 PARAGRAPH_SEPARATOR 28 SPACE_SEPARATOR 29`.

⑥ the flip's *first* two readings, kept because they are what the include graph cost: the first
run stopped in the mirrors before this unit was read - `E0425: cannot find type `Time``
(`termhooks.rs` 110, from `include/frame.rs`'s `include!("termhooks.rs")`) and `E0425: cannot find
value `MAX_CHAR`` (`dispextern.rs` 258/267) - the edges of `## Decisions` 6; the second reported
the port's own four error classes (Findings 4-6) and was 34 errors.

⑦ the other gates, this revision: `check-mirror-dupes.py` -> `17 duplicate declaration(s), 17
ratchet row(s), 0 failure(s)` (no row names `femacs/src/hbfont.rs`);
`check-declarations.py` -> `10 declaration sites (0 forbidden, 10 allowed literals), 0 failures`
(neither of this unit's two files is among the ten);
`check-citations.py` -> whole tree `676 file(s) scanned, 2 drift, 0 range, 912 external, 35
allowed, 0 stale`; **neither drift row is this unit's** (both are `femacs/src/ftfont.rs`, another
lane's file), and this unit's 57 cited paths are all *external* - the HarfBuzz headers are not in
the reference tree, which is counted and never failed; `check-dead-file-coverage.py` ->
`info docs/ports/hbfont.md: 16 range row(s) cover src/hbfont.c (612 lines; 11 blank line(s)
between blocks need no row)`, `62 ledger(s) bound (a C range table), 0 failure(s)`.

## Deviations

1. **The 28 renames are spelled at the call sites, per arm.**  C's `#define hb_x fn_hb_x`
   (C 87-114) is preprocessor text; the port writes `fn_x (…)` under `have_ntgui` and `x (…)`
   otherwise, at all 34 call sites (the `gnutls.rs`/`lcms.rs` precedent).  No other statement
   changes, and the two arms are the same order of statements.
2. **The `w32common.h` include is wrapped in a module + glob import** (`## Decisions` 4).  This is
   a scoping device for Rust's one-namespace-per-scope rule, not a re-spelling of anything C has;
   `## Owed` 1 removes it.
3. **C 271-339's `switch (XFIXNUM (category))` is a guard-form `match`** whose arms keep C's order
   and C's `return` in every arm (`cat if cat == UNICODE_CATEGORY_Cc as EMACS_INT => { return …; }`
   - an `as` cast is not a Rust pattern, so the guards carry it).  `switch` and `case` have no
   counterpart; `docs/ports/macro-body-notes.tsv` records the two tokens with the reason
   (PORTING.md 2.2, the `sigdescr_np`/`cmgoto` precedent).  The semantics are C's: a value that
   matches no arm falls through to the same trailing `return HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED`
   C has after the `switch`.
4. **`alloca` and `UNINIT`.**  C 419's bare `alloca (text_len * sizeof (int))` is the port's stack
   pool, `sa_stack_alloc` (PORTING.md P-05/3.2; the `Freplace_region_contents` note row), and C
   513's `to UNINIT, cluster_offset UNINIT` are lisp.h's "deliberately uninitialised" markers,
   which a Rust `let mut` has no counterpart for, so the port writes the zeroes the marker stands
   for (PORTING.md 1.5/0.1-13; the `cons_to_unsigned` note row).
5. **Uninitialised locals are zeroed or bound where C assigns them.**  Named here: C 219's
   `double position_unit` (bound to `0.0` before `begin_hb_font` fills it), C 399's `ptrdiff_t i`
   (bound to `0` at C 420's `for` init), C 401-402's `info`/`pos` (bound at C 511-512), C 519's
   `xoff/yoff/wadjust` (assigned at C 595-597 before their first read) and C 513's two `UNINIT`
   locals (Deviation 4).  C 224's `if (!hb_font)`, C 234's `if (font->driver->end_hb_font)` and
   the four `hb_*` tests are spelled with the mirror's `is_null ()`/`is_some ()`/`!= 0`, and C's
   `driver` field is read through the `*const font_driver` cast the mirror's `include/font.rs`
   124-129 documents (the `font.rs` 3431 shape).
6. **The three measured records carry gated layout assertions** (`## Decisions` 7).  The mirrors
   pin their own frozen records the same way; the gate is what keeps the frozen `build-tu` clean.
7. **Line count.**  C 612 vs the port's 1,599 (161.3%): 34 call sites are spelled twice (the
   `have_ntgui` arm), the HarfBuzz surface is ~430 lines of declarations with their `/// C:` lines,
   the `#ifdef HAVE_NTGUI` arm is ~230 lines (the 28 `DEF_DLL_FN` expansions and the 28
   `LOAD_DLL_FN` calls with C's names in long form), and P-86's per-item `#[cfg]` gates each cost a
   line.  `docs/ports/line-drift-tolerance.tsv` owes this file a row (`## Owed` 8).

## Owed by the integrator (nothing here is claimed as done)

1. **`femacs/src/include/w32common.rs`**: move its crate-root `unsafe extern "C"` items into a
   `pub mod w32common_decls { … }` plus `use w32common_decls::*;` (P-39's fix; the
   `include/sfntfont.rs` 96-141 shape), then delete this unit's `mod`/`use` wrapper.
   `docs/ports/include-w32common.md` needs the row.  **NOT DONE.**
2. **`femacs/src/include/hb.rs`**: create the HarfBuzz surface's mirror (P-90 ①) and move this
   unit's C 21-22 segment into it in the same commit, deleting the copy here (P-88 ⑤).  The
   segment's `/// C:` lines and the layout assertions travel with it.
   `docs/ports/include-hb.md` would be its ledger.  **NOT DONE.**
3. **`femacs/lib/include/math.rs`**: `lround` (`extern long int lround (double)`; the mirror
   carries `cbrt`/`hypot`/`M_PI` but not this one), then delete this unit's use-site copy.
   **NOT DONE.**
4. **`femacs/src/include/lisp.rs`**: `get_unicode_property` (lisp.h 4697
   `extern Lisp_Object get_unicode_property (Lisp_Object, int);`), then delete the copy.
   **NOT DONE.**
5. **`femacs/src/include/dispextern.rs`**: `bidi_mirror_char` (dispextern.h 3553
   `extern int bidi_mirror_char (int);`), then delete the copy.  **NOT DONE.**
6. **`femacs/src/include/font.rs`**: `font_intern_prop` (font.h 885-886), then delete the copy.
   **NOT DONE.**
7. **`femacs/src/include/font.rs` 577**: gate the frozen-layout size assertion (and any other
   frozen-layout assertion a `harfbuzz`-gated field moves) on `not (feature = "harfbuzz")`.  Until
   then the arm flip cannot pass and this unit has no rlib reading of its own (Finding 2, Evidence
   ②).  **NOT DONE.**
8. **`docs/ports/line-drift-tolerance.tsv`**: a `hbfont` row, basis `total`, tolerance **1.62**
   (measured drift 161.3%, Evidence ④, Deviation 7).  **DONE (status 3.494, integrator):** the row is
   appended and `extract-abi.py --check emacs/src/hbfont.c femacs/src/hbfont.rs` is now **OK with zero
   `-` rows**.
9. **`docs/baseline/port-lines-pending.tsv`**: regenerate with
   `tools/check-port-lines.py --write-pending` - the `src/hbfont.c` row is now above the ten-percent
   threshold and must leave the ratchet.  **DONE (status 3.492/3.494, integrator):** the run took it
   off (`53 ratchet row(s), 0 new, 0 stale`, 53 files under ten percent / 69,407 reference lines).
10. **`docs/baseline/feature-coverage.tsv`**: regenerate with `tools/feature-coverage.py --write` -
    its `harfbuzz` row reads 176 sites / 6 files while a fresh run reports **181 / 7** (this unit's
    five gated layout assertions landed after the last regeneration); the `have_ntgui` row (595/14)
    already agrees.  No new feature name is needed: both are registered in `femacs/xtask/Cargo.toml`
    (`harfbuzz`, `have_ntgui`).  **NOT DONE.**
11. **`docs/status.md`**: the running-log entry for this revision (the file-count and ratchet
    numbers this lane cannot write).  **NOT DONE.**

## Not claimed

- **No run evidence.**  Nothing was linked, started or exercised: no frame, no font, no shaped
  string and no HarfBuzz call.  The 14 MB rlib of Evidence ③ is a compile artifact, and it is the
  *masked /tmp copy's*, not the tree's.
- **The arm flip does not pass today.**  The tree's own reading is Evidence ② (exit 1, one mirror
  error).  The "0 errors" statement is about this unit's own text with that one mirror gap masked,
  and it says nothing about the mirror.
- **The whole `#ifdef HAVE_NTGUI` arm is text-only** (P-86 ③⑵: no Windows cross toolchain on this
  machine): the 28 `DEF_DLL_FN` expansions, the 28 `LOAD_DLL_FN` calls, the 28 per-arm call-site
  spellings and `hbfont_init_w32_funcs` have no compile reading at all, and neither has this unit's
  `mod`/`use` wrapper for the `w32common.h` mirror.
- **The frozen `build-tu` object says nothing about the body** - it is the 336-byte empty object
  the file gate leaves (Evidence ①), and the reference builds no object from this file for it to be
  compared with.
- **Behavior is not claimed.**  `hbfont_shape`'s cluster/`TO`/`adjustment` arithmetic, the
  `canonical_combining_class_table` cache with its `staticpro` root, and the Unicode-category
  mapping are ported text that has never been executed; a HarfBuzz shaper differential is owed.
- **The mirror-dupes / line-count / pending-ratchet questions are not settled by this lane**: the
  first has no row for this file (Evidence ⑦) but the other two are Owed 8-9.

The integrator's remaining steps for this unit: `tools/feature-coverage.py --write` has been re-run
(status 3.494; the `harfbuzz` row now reads the fresh count) and the round is written up in
`docs/status.md` 3.494.  Items 1-7 above are mirror/tool rounds and stay named.
