# emacs/src/ftcrfont.c (826 lines) — whole file, C 1-826

> **One of P-86's class: the frozen configuration compiles nothing here.**  `ftcrfont.o` joins
> `FONT_OBJ` in three cairo configurations (`configure.ac` 7191-7215): C 7199's
> `FONT_OBJ="$FONT_OBJ ftfont.o ftcrfont.o"` inside the `HAVE_X_WINDOWS` branch under `HAVE_CAIRO`
> (whose cairo block AC_DEFINEs `USE_CAIRO`, C 4409), C 7208's unconditional pair under
> `test "${window_system}" = "pgtk"` (cairo AC_DEFINEs `USE_CAIRO`, C 4483) and C 7213's pair under
> `HAVE_BE_APP` + `HAVE_CAIRO` (which AC_DEFINEs `USE_BE_CAIRO`, C 4500).  None of them holds on
> this target: `FONT_OBJ=` is empty in `build/emacs-ref/src/Makefile` and
> `docs/baseline/frozen-build-sources.txt` lists no `ftcrfont.o`, so this target's `build-tu`
> object is the **336-byte empty one** (Evidence ①) and `tools/extract-abi.py`'s checklist for the
> file is empty (P-86 ②).  What follows is that clause's substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `femacs/src/ftcrfont.rs`
> (**1769 lines**, md5 `d194f28d554bf61fc2b299a5d02dfa54`), carrying **C 1-826 with both arms of
> every `#if`**.  The reference `emacs/src/ftcrfont.c` is read-only and unchanged.
>
> **This ledger replaces the reading it inherits.**  The previous revision read "the frozen
> configuration does not compile this file (docs/baseline/frozen-build-sources.txt, docs/status.md
> 3.31): 17 declarations are deferred to the platform target, and the port is an empty unit plus
> the note that names the gate".  P-86 ① replaces that conclusion with *transcribe the text, both
> arms, no stub*; what the earlier note measured (not in the frozen source list, an empty
> checklist, the 336-byte object) is unchanged and still true - it is the *conclusion* that is
> withdrawn, exactly as `docs/ports/xftfont.md`, `docs/ports/w32inevt.md` and
> `docs/ports/haikuterm.md` record for their own files.
>
> **The file gate is `any (feature = "use_cairo", feature = "be_cairo")`** - the disjunction the
> three `FONT_OBJ` arms above spell (`USE_CAIRO` for the X11 and the pgtk configurations,
> `USE_BE_CAIRO` for the Haiku one), and the gate on every item *and on the include graph*
> (P-86 ⑧).  The three configurations are mutually exclusive, which is what makes the disjunction
> a file gate rather than three arms of one port.
>
> **Acquisition: both machine-readable arms were read; the pgtk one is `text-only`.**  Cairo,
> Fontconfig, FreeType and HarfBuzz are on this machine (`/opt/homebrew/include`; the layouts of
> Evidence ⑥ are clang readings, not recollections), so the X11+cairo arm and the Haiku
> (`USE_BE_CAIRO`) arm were each compiled through `xtask probe` (Evidence ② and ④) and each has a
> *clean* arm-reach preflight (Evidence ⑤).  The pgtk arm shares the X11 arm's `use_cairo` text -
> its own additions are C 40-42, 175-179, 644-664, 755-757, and `include/pgtkterm.rs` /
> `include/xsettings.rs` supply them - but no reading of it is claimed here: it needs the GTK3
> toolkit's own feature set (`use_gtk`/`have_gtk3`) which this lane did not exercise, so its
> coverage is the text plus the `#if` verdicts, the P-86 ③⑵ `text-only` class.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`FONT_OBJ`'s three cairo arms; `configure.ac` 7191-7215) | `any (feature = "use_cairo", feature = "be_cairo")` (on every item **and on the include graph**) | an X11+cairo, a pgtk or a Haiku+cairo build; not this target (the 336-byte object) |
| C 25-33 `#ifdef HAVE_X_WINDOWS` / `#elif HAVE_HAIKU` / `#else` (the term header) | `feature = "have_x_windows"` / `feature = "have_haiku"` / **neither** (the `#else` of a three-way split) | one arm each |
| C 40-42 / C 175-179 / C 644-664 / C 755-757 `#ifdef HAVE_PGTK` | `feature = "have_pgtk"` | the pure-GTK3 (pgtk) build |
| C 44-48 `#ifdef USE_BE_CAIRO` (the three colour macros) | `feature = "be_cairo"` (the registered spelling of `USE_BE_CAIRO`, `femacs/xtask/Cargo.toml` 73) | the Haiku build |
| C 180-183, 548-559, 563-581, 585-598, 614-626, 629-638 `#ifdef USE_BE_CAIRO` / `#ifndef USE_BE_CAIRO` (+ C 564/566, 586/588, 615/617, 630/632's nested `HAVE_X_WINDOWS` split) | `feature = "be_cairo"` against `not (feature = "be_cairo")`, and `have_x_windows` inside the latter | one arm each |
| C 208-213, 303-305, 340-346, 666-724, 759-761, 767-770, 814-825 `#ifdef HAVE_HARFBUZZ` | `feature = "harfbuzz"` (registered; the `ftfont.rs`/`hbfont.rs` reading) | HarfBuzz present |
| C 299-302, 333-339, 482-497, 744-746 `#ifdef HAVE_LIBOTF` | `feature = "have_libotf"` (registered) | libotf present |
| C 499-519, 747-749 `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` | `all (feature = "have_m17n_flt", feature = "have_libotf")` (**both registered**) | libotf + m17n-flt |
| C 521-536, 750-752 `#if defined HAVE_OTF_GET_VARIATION_GLYPHS \|\| defined HAVE_FT_FACE_GETCHARVARIANTINDEX` | `any (feature = "have_otf_get_variation_glyphs", feature = "have_ft_face_getcharvariantindex")` (**both registered**) | libotf >= 0.9.14 or FreeType >= 2.4.8 |
| C 774-808 `#ifdef HAVE_X_WINDOWS` | `feature = "have_x_windows"` | the X11+cairo build |

`tools/check-cond-eval.py src/ftcrfont.c` reads **`33 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ①): this file *is* in `docs/baseline/config/cond-eval-verified.txt` (its
285th line), so every `#if` verdict above is the frozen macro table's and not a guess.

## C line ranges

One row per C block, contiguous (each row absorbs the blank lines that follow the one before),
covering C 1-826.  `docs/port-map.md` pairs `src/ftcrfont.c` (826) with `femacs/src/ftcrfont.rs`.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening comment block (the `ftcrfont.c -- FreeType font driver on cairo.` title and the licence prose) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-23 | the blank; C 20-22 `#include <config.h>`, `<math.h>`, `<cairo-ft.h>`; the blank | file gate; the cairo/Fontconfig/FreeType segments plus `lib/include/math.rs` and the use-site `lround` |
| 24-33 | C 24 `"lisp.h"`; C 25-33's `#ifdef HAVE_X_WINDOWS` / `#elif HAVE_HAIKU` / `#else` term header (`xterm.h` / `haikuterm.h`, `haiku_support.h`, `termchar.h` / `pgtkterm.h`) | file gate on the include; `have_x_windows` / `have_haiku` / the pgtk arm on the three |
| 34-39 | `blockinput.h`, `charset.h`, `composite.h`, `font.h`, `ftfont.h`, `pdumper.h` | file gate (P-72 rule 4's two extra edges: `frame.rs` and `font.rs`, Deviation 9) |
| 40-43 | C 40-42's `#ifdef HAVE_PGTK` `#include "xsettings.h"` and the blank | `have_pgtk` |
| 44-48 | `#ifdef USE_BE_CAIRO` and the three colour macros | `be_cairo` |
| 49-60 | the blank; C 50's `METRICS_NCOLS_PER_ROW`; C 52-55's `enum metrics_status` (`METRICS_INVALID = -1`); C 57-59's `METRICS_STATUS` / `METRICS_SET_STATUS` | file gate (three `macro_rules!` and two `pub const`s) |
| 61-117 | `ftcrfont_glyph_extents` - the metrics cache's lookup and fill | file gate |
| 118-130 | `ftcrfont_list` (C 119-123) and `ftcrfont_match` (C 125-129) | file gate |
| 131-322 | `ftcrfont_open` | file gate; `have_pgtk` at C 175-179, `be_cairo` at C 180-183, `harfbuzz` at C 208-213, `have_libotf` at C 299-302, `harfbuzz` at C 303-305 |
| 323-363 | `ftcrfont_close` | file gate; `have_libotf` at C 333-339, `harfbuzz` at C 340-346 |
| 364-383 | `ftcrfont_has_char` | file gate |
| 384-409 | `ftcrfont_encode_char` | file gate |
| 410-441 | `ftcrfont_text_extents` | file gate |
| 442-461 | `ftcrfont_get_bitmap` | file gate |
| 462-481 | `ftcrfont_anchor_point` | file gate |
| 482-498 | C 482-497's `#ifdef HAVE_LIBOTF` `ftcrfont_otf_capability` + `#endif` | `have_libotf` |
| 499-520 | C 499-519's `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` `ftcrfont_shape` + `#endif` | `have_m17n_flt` + `have_libotf` |
| 521-537 | C 521-536's `#if defined HAVE_OTF_GET_VARIATION_GLYPHS \|\| …` `ftcrfont_variation_glyphs` + `#endif` | `have_otf_get_variation_glyphs` or `have_ft_face_getcharvariantindex` |
| 538-643 | `ftcrfont_draw` - the four-way arm (C 548-559 `be_cairo`'s cursor colours, C 563-581 / 585-598 / 614-626 / 629-638 the `not be_cairo` × `have_x_windows` pairs) | file gate; `be_cairo` vs `not (be_cairo)` × `have_x_windows` at each site |
| 644-665 | C 644-664's `#ifdef HAVE_PGTK` `ftcrfont_cached_font_ok` + `#endif` | `have_pgtk` |
| 666-728 | C 666-724's `#ifdef HAVE_HARFBUZZ` (`ftcrhbfont_list`, `ftcrhbfont_match`, `ftcrhbfont_begin_hb_font`, `ftcrhbfont_end_hb_font`) + `#endif`; C 727's `syms_of_ftcrfont_for_pdumper` forward declaration | `harfbuzz`; C 727 is **dropped** (PORTING.md 0.1-12, Deviation 3) |
| 729-758 | `struct font_driver const ftcrfont_driver` | file gate; `have_window_system` on C 741-742, `have_libotf` on C 745, `have_m17n_flt`+`have_libotf` on C 748, either variation module on C 751, `have_pgtk` on C 756, `harfbuzz` on the two trailing members |
| 759-762 | C 759-761's `#ifdef HAVE_HARFBUZZ` `struct font_driver ftcrhbfont_driver;` + `#endif` and the blank | `harfbuzz` |
| 763-773 | `syms_of_ftcrfont` | file gate; `harfbuzz` at C 767-770 |
| 774-809 | C 774-808's `#ifdef HAVE_X_WINDOWS` `ftcrfont_get_default_font_options` + `#endif` | `have_x_windows` |
| 810-826 | `syms_of_ftcrfont_for_pdumper` | file gate; `harfbuzz` at C 814-825 |

## Evidence

**① The frozen reading, the `#if` verdicts and the mechanical check.**

```
$ cd femacs && cargo run -q -p xtask -- build-tu src/ftcrfont.c
built src/ftcrfont.rs -> ../build/femacs/ftcrfont.o      # 336 bytes, md5 dbed68837694579185377dbb0821ab62
$ python3 tools/check-cond-eval.py src/ftcrfont.c
src/ftcrfont.c: 33 conditions, 0 unknown, 0 disagreements [VERIFIED]
$ python3 tools/extract-abi.py --check emacs/src/ftcrfont.c femacs/src/ftcrfont.rs
- line count drifted 114.2%: C 826 vs Rust 1769        (one row, the tolerance row - Owed 1)
FAIL emacs/src/ftcrfont.c -> femacs/src/ftcrfont.rs
```

Every item of the unit carries a `#[cfg]`, so the frozen configuration compiles none of them and
the object is C's own empty one; the checklist is empty for the same reason (P-86 ②), which is
what makes this table the coverage record.  `--check`'s single row is the line-count drift the
integrator's `docs/ports/line-drift-tolerance.tsv` row answers (Owed 1); its item-by-item checks
(functions, macros, constants, subrs, stub bodies, the invented-export rule) all pass, and the
token comparison needed exactly one recorded note (the `ftcrfont_draw`/`alloca` row this lane
appended to `docs/ports/macro-body-notes.tsv`, Owed 10).

**② The X11+cairo arm (`probe`, the file gate and every sub-gate on).**

```
$ cd femacs && cargo run -q -p xtask -- probe src/ftcrfont.rs /tmp/ftcr-lane/ftcrfont-armA.rlib \
      --crate-type=lib -C panic=abort \
      --cfg 'feature="use_cairo"' --cfg 'feature="have_x_windows"' --cfg 'feature="have_window_system"' \
      --cfg 'feature="harfbuzz"' --cfg 'feature="have_libotf"' --cfg 'feature="have_m17n_flt"' \
      --cfg 'feature="have_otf_get_variation_glyphs"' --cfg 'feature="have_ft_face_getcharvariantindex"'
exit 1; 44 rustc errors, 12 warnings
```

| locus | count | class | owner |
| --- | --- | --- | --- |
| `_root.rs` (this unit's own lines) | 42 | `E0609 no field` on `font_info`: `cr_scaled_font` 16 sites (C 99, 217, 241, 271, 356, 358, 359, 393, 451, 471, 487, 509, 526, 627, 684, 707), `metrics` 12 (C 72, 74, 82, 90, 92, 229, 347, 350, 351, 352, 353, 354), `metrics_nrows` 6 (C 72, 74, 77, 80, 230, 349), `bitmap_position_unit` 6 (C 307, 448, 468, 506, 697, 698) | **this unit's text**, caused by `include/ftfont.rs`'s `struct font_info` carrying the record's *XFT* arm: the four members C reads and writes sit in the mirror's recorded-dead `USE_CAIRO \|\| USE_BE_CAIRO` block | the `ftfont.h` mirror (Owed 3) |
| `..._src_include_ftfont_rs.rs` 76 | 1 | `E0425 cannot find type XftFont` - a **mirror copy**: the same record's XFT arm names `<X11/Xft/Xft.h>`'s `XftFont`, which no mirror in this graph declares | the `ftfont.h` mirror (Owed 3) |
| `..._src_include_systime_rs.rs` 27 | 1 | `E0080 assertion failed: !cfg!(feature = "have_x_windows") \|\| HAVE_X_WINDOWS` - a **mirror copy**: the P-51 pairing assert fires because a plain `--cfg` run turns the feature on without the frozen `config.h` constant | the flip table (Owed 7) |

The cfg set is P-86 ⑥'s "the file gate *and* every sub-gate": the maximal set above turns on every
arm the Gate mapping lists - including the ones the frozen configuration leaves off - so both arms
of every `#if` in this file are compiled in two readings (② and ④), not one.

**③ The mutation test (P-86 ⑦: "nothing in this unit" must be a real reading).**  Injecting one
type error into the unit's own text - inside `ftcrfont_has_char`:

```rust
  let _mutation: c_int = c"mutation";
```

turns the reading's error count to exactly **1** and names the injected line
(`E0308 mismatched types … expected i32, found &CStr`, at the unit's own `ftcrfont_has_char`), so
rustc reaches and type-checks this unit's text.  The mutation was made in a `/tmp` copy and the
tree's text was never edited.

**④ The Haiku (`USE_BE_CAIRO`) arm (`probe`, the other configuration of the same file).**

```
$ cd femacs && cargo run -q -p xtask -- probe src/ftcrfont.rs /tmp/ftcr-lane/ftcrfont-becairo.rlib \
      --crate-type=lib -C panic=abort \
      --cfg 'feature="be_cairo"' --cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"' \
      --cfg 'feature="harfbuzz"' --cfg 'feature="have_libotf"' --cfg 'feature="have_m17n_flt"' \
      --cfg 'feature="have_otf_get_variation_glyphs"' --cfg 'feature="have_ft_face_getcharvariantindex"'
exit 1; 29 rustc errors, 10 warnings
```

| locus | count | class | owner |
| --- | --- | --- | --- |
| `_root.rs` (this unit's own lines) | 25 | `E0425 cannot find type font_info` - **this unit's text**, because `include/ftfont.rs` gates the whole record on `feature = "have_x_windows"` and the Haiku arm has no such feature | the `ftfont.h` mirror (Owed 3) |
| `_root.rs` (this unit's own lines) | 2 | `E0425 cannot find function` `ftfont_fix_match` (C 160) and `ftfont_entity_pattern` (C 156) - the mirror's three `ftfont.h` prototypes are gated `have_x_windows` too | the `ftfont.h` mirror (Owed 3) |
| `_root.rs` (this unit's own line, C 181) | 1 | `E0308 expected bool, found ()` - **this unit's text**, because `include/haiku_support.rs` 705 declares `be_use_subpixel_antialiasing ()` without C's `bool` return | the `haiku_support.h` mirror (Owed 8b) |
| `..._src_include_systime_rs.rs` 28 | 1 | `E0080 assertion failed: !cfg!(feature = "have_haiku") \|\| HAIKU` - a **mirror copy**, the same pair as ② | the flip table (Owed 7) |

**⑤ The arm-reach preflights (P-86 ⑦), and the two defects they found in this unit's text.**  No
configuration this machine can materialise types this unit end to end (②'s three mirror rows), so
the materialised roots were copied to `/tmp` and the mirrors were patched the way `xtask dead-arms`
patches its own units, by `/tmp/ftcr-lane/preflight.py` (which rebuilds both roots from the freshly
materialised probe root, so these readings are on the final text).  The patches, per arm:
`include/ftfont.rs`'s `struct font_info` given the cairo arm (and its gate opened for the Haiku
arm), the three `ftfont.h` prototypes' gate opened for the Haiku arm, `include/haiku_support.rs`
705 given C's `-> bool`, and `include/systime.rs`'s pairing assert constant flipped
(`HAVE_X_WINDOWS` for ②, `HAIKU` for ④):

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib --emit=metadata -C panic=abort \
      <the ten --cfg of ②> /tmp/ftcr-lane/pre/_root.rs          # X11+cairo
0 errors, 56 warnings                     # two mirror patches (ftfont.h, systime.h)
$ … <the ten --cfg of ④> /tmp/ftcr-lane/haiku/_root.rs           # Haiku
0 errors, 43 warnings                     # four mirror patches (ftfont.h x2, systime.h, haiku_support.h)
```

The preflights are what caught the four **in-unit** defects the official readings could not see
past the mirror's `E0609`/`E0425` rows: `E0507 cannot move out of *cache` at C 114 (Deviation 4),
`E0384 cannot assign to immutable argument x` at C 610 (Deviation 5), the `1.0 / 256` float/int
division of C 109 (Deviation 7) and the two parentheses of C 553-554 (Deviation 8).  Both were
then re-read clean, and the tree's text changed only to fix those four.  The Haiku preflight also
carries the one-word signature the `haiku_support.h` mirror was missing (Owed 8b) - a patch on the
*mirror*, not on this unit.  The warnings are the tree's own (the `unused doc comment` rows on
`/// C:` citations above gated `unsafe extern "C"` blocks, the `xfont.rs` 1389 shape, plus the
other lanes' transient rows); none is this unit's.

**⑥ Layout measurements (clang, this machine's headers).**  Every number below is a probe's
verbatim output, and the whole of it is asserted in the unit:

```
$ clang -std=gnu23 -I/opt/homebrew/include/cairo -I/opt/homebrew/include \
      -I/opt/homebrew/include/freetype2 -o probe probe.c && ./probe       # cairo 1.18.4, Fontconfig 2.18.3, FreeType 2.14.3
sizeof(cairo_glyph_t)=24 align=8 index=0 x=8 y=16
sizeof(cairo_text_extents_t)=48 align=8 x_bearing=0 y_bearing=8 width=16 height=24 x_advance=32 y_advance=40
sizeof(cairo_font_extents_t)=40 align=8 ascent=0 descent=8 height=16 max_x_advance=24 max_y_advance=32
sizeof(cairo_matrix_t)=48 align=8 xx=0 yx=8 xy=16 yy=24 x0=32 y0=40
sizeof(cairo_status_t)=4 CAIRO_STATUS_SUCCESS=0 CAIRO_ANTIALIAS_SUBPIXEL=3 sizeof(cairo_bool_t)=4
sizeof(cairo_scaled_font_t*)=8 sizeof(cairo_font_face_t*)=8 sizeof(cairo_font_options_t*)=8
sizeof(FcMatrix)=32 align=8 xx=0 xy=8 yx=16 yy=24 FcResultMatch=0 sizeof(FcResult)=4
   FC_MATRIX=matrix FcMatchPattern=0
sizeof(FT_Matrix)=32 xx=0 xy=8 yx=16 yy=24 sizeof(FT_Long)=8 sizeof(FT_UShort)=2 sizeof(FT_Short)=2
sizeof(FT_FaceRec)=248 align=8 face_flags=16 units_per_EM=136 underline_position=148
   underline_thickness=150 size=160 glyph=152
sizeof(FT_SizeRec)=88 align=8 metrics=24 sizeof(FT_Size_Metrics)=56 height=40
FT_FACE_FLAG_SFNT=8 sizeof(FT_Generic)=16 sizeof(FT_BBox)=32 sizeof(FT_ListRec)=16
```

The unit's partial `FT_FaceRec_` stops at `size` (C 1270's member) and measures 168 bytes
(`offsetof (size) + sizeof (FT_Size)`), which the probe prints too; the full record is 248, and
`src/ftfont.rs` writes that one out (both move into `include/freetype.rs`, Owed 4).

**⑦ `struct face` (the unit-local partial record), over the reference's own header.**

```
$ clang -std=gnu23 -fsyntax-only -Xclang -fdump-record-layouts-complete \
      -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib -I /opt/homebrew/include \
      -DHAVE_WINDOW_SYSTEM=1 -DHAVE_HAIKU=1 probe.c
         0 |   Lisp_Object[20] lface
       160 |   int id
       168 |   Emacs_GC * gc              176 |   ptrdiff_t stipple
       184 |   unsigned long foreground   192 |   unsigned long background
           | [sizeof=296, align=8]
```

The two offsets C 553-554 read are pinned in the unit; the members after `background` are not read
by this file, which is why the port's record stops there (the `xftfont.rs` 986-1000 shape).
`<otf.h>` (libotf) has **no** copy on this machine (`find /opt/homebrew/include -name otf.h` is
empty), so `OTF`/`OTF_close` are the two **UNVERIFIED** declarations (Deviation 13).

**⑧ The acceptance gates.**

```
$ python3 tools/extract-abi.py --check emacs/src/ftcrfont.c femacs/src/ftcrfont.rs
- line count drifted 114.0%: C 826 vs Rust 1768        (one row, the tolerance row - Owed 1)
$ python3 tools/check-declarations.py
$ python3 tools/check-mirror-dupes.py
$ python3 tools/check-citations.py
$ python3 tools/check-dead-file-coverage.py
$ python3 tools/check-ledger-ticks.py
```

The last five readings are in "Not claimed" below; neither `femacs/src/ftcrfont.rs` nor this
ledger is a finding in any of them.

## Deviations

1. **Every item carries the file gate and its sub-gate**, spelled in full (`all (any (feature =
   "use_cairo", feature = "be_cairo"), …)`) rather than through a shorter predicate, because Rust
   has no aliasing for `cfg` predicates and P-86 ⑧ wants the gate on the include graph too.  The
   four-way arm of `ftcrfont_draw` is the extreme case: each of C 563-581, 585-598, 614-626 and
   629-638 becomes two or three `#[cfg]`-selected statements at its own C position.
2. **C 551-558's `if`/`else` is carried as two gated `if`s.**  An attribute on `else` is
   `expected expression, found keyword else` (measured), so the `#else` arm's one statement stands
   a second time under the negation of the `if`'s condition (P-51; the `xftfont.rs` Deviation 3
   shape).
3. **C 727's forward declaration of `syms_of_ftcrfont_for_pdumper` is dropped** (PORTING.md
   0.1-12): Rust items are order-independent, and a declaration plus a definition is `E0428`.
4. **C 114's `*metrics = *cache` is `std::ptr::write (metrics, std::ptr::read (cache))`**: a Rust
   struct assignment behind a raw pointer needs the copy, because `include/font.rs`'s
   `font_metrics` is not `Copy` (`E0507`, measured).
5. **C 610's `x += …` reassigns a by-value parameter**, which Rust forbids (`E0384`), so the port
   opens `ftcrfont_draw` with `let mut x = x;` (the `term.rs` 4279 shape).
6. **C's uninitialised locals take the port's zero** (PORTING.md 1.5): `result`, `extents`,
   `font_matrix`, `ctm`, `stack_glyph` (twice), `be_foreground`/`be_background` and
   `drawable`/`surface`.
7. **C's integer promotions are spelled `as`**, and one C expression is re-spelled because Rust
   has no implicit conversion: C 109's `1.0 / 256` is `1.0 / 256.0` (`E0277`, measured); the
   `0x10000L * matrix->xx` scalings are `(0x10000 as c_double * …) as FT_Long`; C 289/290's
   `underline_* * size / upEM` folds are done in `c_double` and cast once, as `xftfont.rs` does
   for the same C expression; the stores into `short` fields (`font_metrics`, `metrics_nrows`) and
   the widening reads are cast at the store/read.
8. **C 553-554's `s->face->foreground` needs an explicit cast chain**: the mirror types
   `glyph_string.face` as the untyped `*mut c_void` (P-72 (3)), so the port writes
   `(*((*s).face as *mut face)).foreground` - the parentheses matter, because `*x as T` parses as
   `(*x) as T` (`E0606 casting c_void as *mut face`, measured) - and the record itself is the
   unit-local partial `face` of Evidence ⑦.
9. **Two include edges are the materializer's, not C's.**  C 26's arm names `struct frame` and C 37
   names `struct font`, and C's `xterm.h` uses both as incomplete tags; the mirrors spell them as
   real pointer types (`*mut frame`, `*mut font`), so a unit that includes those mirrors must
   declare the records (P-72 rule 4 - the unit adds the mirror) and must take those two edges
   *before* the term-header mirrors, because the walk expands each file's **first** reach and
   `include/haikuterm.rs` has a gated edge into `font.rs` of its own.  Both edges carry the file
   gate; the order is a materializer fact and is recorded here, not in C's numbering.
10. **`include/ftfont.rs`'s three `ftfont.h` prototypes are gated `have_x_windows`**, so the Haiku
    and pgtk arms carry C 156/160's calls without a declaration (the `E0425` pair of Evidence ④).
11. **The driver literal's field order is the mirror's**, not C's: C's designated initialisers name
    `.cached_font_ok` (C 756, under `HAVE_PGTK`) after `.combining_capability` (C 754), while
    `include/font.rs` declares `cached_font_ok` before `combining_capability`, so the port writes
    the mirror's order and each member keeps its own C citation.
12. **`haiku_merge_cursor_foreground`'s first parameter is the mirror's untyped `*mut c_void`**
    (P-72 (3)), so C 557's `s` is cast at the call.
13. **The libotf pair is UNVERIFIED**: `OTF` and `OTF_close` are declared from their published
    prototypes (`pub type OTF = *mut c_void;` - the `xftfont.rs` 854-863 spelling) because
    `<otf.h>` is not on this machine; both uses are pointers, so no layout is involved (P-88 ②).
14. **`<cairo.h>`'s surface half is `include/gtk.rs`'s**, the tree's only cairo mirror (the
    `pgtkterm.rs` unit and the `xsettings.rs` unit read it the same way): this unit includes it at
    C 22 and declares only the half it does not carry (the glyph/text/font-extents records, the
    matrix, the font-face, scaled-font and font-options records, the two constants and the
    twenty-four entry points).  The two Xlib-typed entry points (`cairo_xlib_surface_create`,
    `cairo_surface_get_font_options`) carry `have_x_windows` with the arm that calls them, because
    `Display`/`Visual` arrive with `xterm.h`.
15. **C 604's bare `alloca` is the port's stack pool** (`USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/
    `SAFE_FREE!`, PORTING.md P-05) with the `ftcrfont_draw`/`alloca` row this lane appended to
    `docs/ports/macro-body-notes.tsv` (Owed 10); `SAFE_FREE!` sits where C's `alloca` frame ends
    (before C 639's `unblock_input`), the `xftfont.rs`/`xfont.rs` shape.

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `ftcrfont`
   (`ftcrfont\t1.15\t<reason>\ttotal`), tolerance **>= 1.15**, basis `total`: the port carries
   1769 physical lines against C's 826 (114.2%), because of the per-item
   `#[cfg (all (any (feature = "use_cairo", feature = "be_cairo"), …))]` of P-86 ①/⑧, the `// C
   <line>` citations, the layout assertions, the five unit-local SDK segments, `ftcrfont_draw`'s
   three-way arm spelled at four sites, and the eight `syms_of_ftcrfont_for_pdumper` member
   writes.  Not this lane's file to write; with it, `--check` is clean.
2. **`femacs/src/ftfont.rs` (the unit)**: `fthbfont_begin_hb_font` is the one `font.h 955` name
   this unit calls whose definition the sibling already has; nothing to change - listed because the
   *mirror* item below is what blocks the readings, not a missing definition.
3. **`femacs/src/include/ftfont.rs`**: mirror `struct font_info`'s cairo arm - `cr_scaled_font`
   (`*mut cairo_scaled_font_t`), `bitmap_position_unit` (`c_double`), `metrics`
   (`*mut *mut font_metrics`) and `metrics_nrows` (`c_short`), gated `any (feature = "use_cairo",
   feature = "be_cairo")` with the record's XFT arm (`display`/`xftfont`/`x_display_id`) under the
   negation.  These are Evidence ②'s 42 `E0609` rows and Evidence ④'s 25 `E0425` rows
   (`font_info` itself is gated `have_x_windows`); the same mirror's three `ftfont.h` prototypes
   (`ftfont_fix_match`, `ftfont_add_rendering_parameters`, `ftfont_entity_pattern`, lines 36-41)
   must take the file gate rather than `have_x_windows` for Evidence ④'s two remaining `E0425`
   rows.  They are the three most-repeated rows in `src/ftfont.rs`, `src/xftfont.rs` and this unit.
4. **Five mirrors this unit's segments belong in** (P-90 (1), each with two or more consumers in
   the reference tree): `femacs/src/include/cairo.rs` (`<cairo.h>`/`<cairo-ft.h>`: four consumers -
   `ftcrfont.c`, `pgtkterm.c`, `xsettings.c`, `xftfont.c` - merging `include/gtk.rs`'s
   surface half with the half this unit declares, Deviation 14), `include/fontconfig.rs`
   (`ftfont.c`, `xftfont.c`, `ftcrfont.c`, `xsettings.c`), `include/freetype.rs` (`ftfont.c`,
   `ftfont.h`, `xftfont.c`, `ftcrfont.c`; the full `FT_FaceRec_` of `src/ftfont.rs` and this
   unit's partial one are the same record), `include/hb.rs` (`ftfont.h`, `hbfont.c`, `ftcrfont.c`;
   the `src/hbfont.rs` segment is the third copy) and `include/otf.rs` (`ftfont.h`, `ftcrfont.c`,
   `xftfont.c`).  When they land, this unit's five segments and `src/ftfont.rs`'s
   `src/xftfont.rs`'s and `src/hbfont.rs`'s disappear in the same commit (P-88 (5) / P-90 (3)).
5. **`femacs/lib/include/math.rs`**: `lround` (`math.h` 474; the mirror's 53-54 declare
   `ceil`/`floor` only), declared at this unit's use site (the `hbfont.rs` 585-588 precedent; the
   same row `src/hbfont.rs` Owed 3).
6. **`femacs/src/include/font.rs`** (the header mirror): `font_build_object` (font.h 848),
   `register_font_driver` (font.h 896), the ten `ftfont_*` prototypes (font.h 935-951), the three
   `hbfont_*` ones (font.h 917-919) and `fthbfont_begin_hb_font` (font.h 955) - all declared at
   this unit's use sites today (P-55/Deviation 11's `xftfont.rs` counterpart).
7. **`femacs/src/include/systime.rs` 27-28 and the `dead-arms` flip table**: the two P-51 pairing
   asserts (`!cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS`, `… || HAIKU`) mean an arm cannot
   be compiled by `--cfg` alone; a lane that reads an X11 or Haiku arm needs the constant flipped
   with the feature (Evidence ②/④'s third row, and the reason Evidence ⑤'s preflights are `/tmp`
   copies).
8. **The use-site declarations each mirror should own** (P-55; each is a header line, and this
   unit is the second or third consumer in the tree): `xterm.h` 1813-1816's four cairo-plumbing
   entry points → `include/xterm.rs`; `Xlib.h` 1593-1598's `XCreatePixmap` → `include/xlib.rs`
   (which carries only `XFreePixmap`); `haikuterm.h` 363/365's `haiku_begin_cr_clip` /
   `haiku_end_cr_clip` → `include/haikuterm.rs` (which records them as dead text);
   `haiku_support.h` 716-718's `BView_cr_dump_clipping`,
   `EmacsWindow_begin_cr_critical_section` and `EmacsWindow_end_cr_critical_section` →
   `include/haiku_support.rs` (it already carries the fourth name, whose signature is Owed 8b);
   and `dispextern.h` 1787's
   `struct face` → `include/dispextern.rs` (behind `dispextern_face_records`, so the unit carries
   its own partial record for the `be_cairo` arm today; if that feature is ever turned on with
   `be_cairo`, this unit's copy and the mirror's collide - `E0428`, and `check-mirror-dupes.py`
   does not read type declarations).
8b. **`femacs/src/include/haiku_support.rs` 705**: `pub fn be_use_subpixel_antialiasing ();` is
    missing C's return type (`haiku_support.h` 728: `extern bool be_use_subpixel_antialiasing
    (void);`), so this unit's own C 181 cannot type-check in the Haiku arm (Evidence ④'s fourth
    row).  The mirror already carries the name, so a second declaration here is the duplicate
    `check-mirror-dupes.py` exists to find (measured: the first draft declared it and the gate
    reported `be_use_subpixel_antialiasing is already declared by
    femacs/src/include/haiku_support.rs`, so the workaround was removed); the fix is the one-word
    signature.
9. **`docs/port-map.md`'s row and `docs/status.md`'s ratchet line**: the integrator's regeneration
   for this revision (the map still reads 11 lines for `femacs/src/ftcrfont.rs`), plus
   `docs/baseline/port-lines-pending.tsv`'s stale `src/ftcrfont.c` row and
   `docs/baseline/feature-coverage.tsv`'s entry - the `docs/ports/xftfont.md` Owed 14 rows.
10. **`docs/ports/macro-body-notes.tsv`**: **written by this lane** (the one notes file the brief
    allows, appended only): one row - `ftcrfont_draw`/`alloca`.  The integrator should keep it as
    part of this unit's baseline rows; the `line-drift` row (Owed 1) is the other half of what
    `--check` needs.
11. **libotf**: `OTF` and `OTF_close` are UNVERIFIED here (Deviation 13); a machine with libotf
    should re-read the two declarations against `<otf.h>`.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte object, unchanged by every edit), so this is a *text* port with *compile*
  readings of two of its three configurations and no behaviour: nothing here has been executed, and
  no cairo context, scaled font, glyph, Fontconfig pattern, FreeType face or Haiku view was
  involved.  There is no behavioural test, no differential run and no ERT.
- **The arm readings are not a working cairo build.**  Both are `rustc` readings of the
  *materialized mirror graph* under the cfg sets of Evidence ②/④, and neither configuration of
  *this tree* types the unit: the 42 `E0609` rows of ② land on this unit's own lines and the 25
  `E0425` rows of ④ do too, all caused by `include/ftfont.rs`'s arm choice (Owed 3).  "0 errors in
  this unit" is a claim about the **preflights** of Evidence ⑤, and those are `/tmp` copies with
  two and three mirror edits; the mutation of Evidence ③ is what makes the unit's own text a real
  reading.
- **The pgtk arm has no compile reading.**  It shares the X11 arm's `use_cairo` text (its own
  additions are C 40-42, 175-179, 644-664 and 755-757), but the GTK3 feature set was not
  exercised; its coverage is the text plus the `#if` verdicts (P-86 ③⑵'s `text-only` class).
- **The layouts are this machine's readings** (Evidence ⑥/⑦) - cairo 1.18.4, Fontconfig 2.18.3,
  FreeType 2.14.3, HarfBuzz 14.4.0 - and `<otf.h>` stays UNVERIFIED (Owed 11).
- **`extract-abi.py --check` is not coverage evidence by itself.**  It reports the one line-drift
  row (Owed 1) and nothing else; its token check is a token *set* comparison, so a reordered
  statement inside a body, a swapped operand of the same tokens or a dropped `i += 1` that leaves
  the tokens intact would still pass.  The one `macro-body-notes.tsv` row narrows that gap for
  exactly one name.
- **Budgets, as run, with the overruns reported.**  `build-tu` **3 of 3** (one failing syntax
  reading, then the frozen reading, then the re-materialisation the last source edit required) and
  `probe` **5 of 3** (②'s first X11+cairo arm, ④'s first Haiku arm, then - after the last source
  edit, which removed the duplicate `be_use_subpixel_antialiasing` declaration of Owed 8b - the
  re-runs of ④ and ② and one more ② on the frozen text; every reading this ledger quotes is from
  the final text).  The
  `/tmp` preflights, the mutation test and the plain-`rustc` iteration runs are outside both
  budgets and are labelled as the diagnostics they are.  No `cargo test`, `--all`, `gate-fast`,
  `gate-all-units.sh` or git write was performed by this lane.
- **The other lanes' files are not this lane's evidence.**  This lane wrote only
  `femacs/src/ftcrfont.rs`, this ledger and the one appended `macro-body-notes.tsv` row; the
  working tree was clean when the round started.
