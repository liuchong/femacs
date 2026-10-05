# emacs/src/xftfont.c (832 lines)

> **P-86's Xft font driver, ported whole.**  The frozen configuration compiles nothing here:
> `configure.ac` 7194-7203 adds `xftfont.o` to `FONT_OBJ` only inside the `HAVE_X_WINDOWS` branch,
> under `HAVE_CAIRO`, `HAVE_XFT` (the `#elif` arm that names this file) or `HAVE_FREETYPE`; `HAVE_XFT`
> is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1696, `FONT_OBJ=` is empty in
> `build/emacs-ref/src/Makefile` 342, and `docs/baseline/frozen-build-sources.txt` lists no
> `xftfont.o`.  The file's own body sits under no `#if` of its own (the gate is the *build's* - the
> `lwlib-Xlw.c`/`xfont.c` shape), so `extract-abi.py`'s checklist is **not** empty and this ledger
> carries P-86 (2)'s substitute (the C line-range table) and the readings.  Port:
> `femacs/src/xftfont.rs` (**1980 lines**, `wc -l`), every arm of every `#if` carried.
>
> This ledger replaces the previous revision's conclusion ("not compiled by the frozen
> configuration, so the port is the empty unit plus a note"): P-86 (1) rules that a header comment is
> *not started*, not *done*.  The old measurements are still true and are still where the file gate
> comes from; what changed is the conclusion drawn from them.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`FONT_OBJ`'s XFT arm; `configure.ac` 7194-7203) | `feature = "have_xft"` (on every item **and on the include graph**) | an X11 configuration with Xft |
| `have_x_windows` (`HAVE_X_WINDOWS`), implied by the branch that reaches this file | not a `#[cfg]` of its own: the mirrors' declarations are keyed on it (`struct font_info`, `struct x_output`, `glyph_string.gc`) and the probe sets it with the file gate | the same configuration |
| `have_window_system` (`HAVE_WINDOW_SYSTEM`), implied in turn | not a `#[cfg]` of its own: `struct font`'s eight window-system members and `font_driver`'s `draw`/`get_bitmap`/`free_bitmap` are behind it, and C 208-320 writes them | the same configuration |
| C 36-38 `#ifdef HAVE_XRENDER` (`<X11/extensions/Xrender.h>`) | `feature = "have_xrender"` (the include line carries it; the names it brings are declared under the pair below) | X11 with Xrender |
| C 546 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` … C 576 | `all (feature = "have_xrender", feature = "render_version_ge_0_2")` (**both registered**) | X11 with Xrender >= 0.2 |
| C 40-43 `#ifndef FC_LCD_FILTER` | **no second copy**: the name is declared once in the Fontconfig segment (`FC_LCD_FILTER`, fontconfig.h 132) | — |
| C 209-215, 285-287, 312-314, 354-360, 713-746, 769-770, 785-787, 793-796, 820-830 `#ifdef HAVE_HARFBUZZ` | `feature = "harfbuzz"` (registered; the `ftfont.rs` reading of `HAVE_HARFBUZZ`) | HarfBuzz present |
| C 308-311, 347-353 `#ifdef HAVE_LIBOTF`; C 612 / 772 `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` | `feature = "have_libotf"` (registered) / `all (feature = "have_m17n_flt", feature = "have_libotf")` (**`have_m17n_flt` not registered**; Owed 2) | libotf present |
| C 247-249 `#ifdef FC_DUAL`, C 250-252 `#ifdef FC_CHARCELL` | `feature = "fc_dual"`, `feature = "fc_charcell"` (**neither registered**; Owed 2); both are **live** on this machine's Fontconfig 2.17.1 (measured) | a Fontconfig that defines them |
| C 321-336 `#ifdef FT_BDF_H` | `feature = "ft_bdf_h"` (**not registered**; Owed 2); live on any FreeType of the Xft era (measured) | FreeType with `FT_BDF_H` |
| C 655-666, 781-783 `#ifdef HAVE_XDBE` | `feature = "have_xdbe"` (registered) | X11 with the DBE extension |
| C 692-696 `#ifdef FC_EMBOLDEN` | `feature = "fc_embolden"` (registered) | a Fontconfig that defines it (this one does) |
| C 775-777 `#if defined HAVE_OTF_GET_VARIATION_GLYPHS \|\| defined HAVE_FT_FACE_GETCHARVARIANTINDEX` | `any (feature = "have_otf_get_variation_glyphs", feature = "have_ft_face_getcharvariantindex")` (**neither registered**; Owed 2) | libotf >= 0.9.14 or FreeType >= 2.4.8 |
| C 380-387, 403-408 `#if false` | **no gate**: dead in every configuration, recorded and not transcribed | never |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's opening comment block | dropped (P-86 (1); the port opens with the `// femacs: port of …` line) |
| 22-24 | `<config.h>`, `<X11/Xlib.h>`, `<X11/Xft/Xft.h>` | file gate; the Xlib/Xft/Fontconfig segment |
| 26-35 | `"lisp.h"`, `"xterm.h"`, `"frame.h"`, `"blockinput.h"`, `"charset.h"`, `"composite.h"`, `"font.h"`, `"ftfont.h"`, `"pdumper.h"` | file gate |
| 36-38 | `#ifdef HAVE_XRENDER` / `#include <X11/extensions/Xrender.h>` / `#endif` | `have_xrender` |
| 40-43 | `#ifndef FC_LCD_FILTER` / `# define FC_LCD_FILTER "lcdfilter"` / `#endif` | no second copy (see Gate mapping) |
| 46-56 | "Xft font driver.", "Structure pointed by (struct face *)->extra" and `struct xftface_info` | file gate |
| 58-141 | the comment at 58-61 and `xftfont_get_colors` (63-141) | file gate |
| 143-147 | `xftfont_list` | file gate |
| 149-153 | `xftfont_match` | file gate |
| 155 | `static FcChar8 ascii_printable[95];` | file gate |
| 157-339 | `xftfont_open` (C 209-215 `harfbuzz`, C 247-252 `fc_dual`/`fc_charcell`, C 308-314 `have_libotf`+`harfbuzz`, C 321-336 `ft_bdf_h`) | file gate + those four |
| 341-373 | `xftfont_close` (C 347-353 `have_libotf`, C 354-360 `harfbuzz`) | file gate + those two |
| 375-396 | `xftfont_prepare_face` (C 380-387 `#if false`) | file gate |
| 398-428 | `xftfont_done_face` (C 403-408 `#if false`) | file gate |
| 430-450 | `xftfont_has_char` | file gate |
| 452-460 | `xftfont_encode_char` | file gate |
| 462-477 | `xftfont_text_extents` | file gate |
| 479-496 | `xftfont_get_xft_draw` | file gate |
| 498-610 | `xftfont_draw` (C 546-576 the Xrender pair; C 578 the `#else` arm) | file gate; `have_xrender` + `render_version_ge_0_2` for C 546-576 |
| 612-624 | `#if defined HAVE_M17N_FLT && defined HAVE_LIBOTF` / `xftfont_shape` / `#endif` | `have_m17n_flt` + `have_libotf` |
| 626-648 | `xftfont_end_for_frame` | file gate |
| 650-666 | the comment at 650-653 and, under `#ifdef HAVE_XDBE`, `xftfont_drop_xrender_surfaces` | file gate; `have_xdbe` for C 655-666 |
| 668-711 | `xftfont_cached_font_ok` (C 692-696 `fc_embolden`) | file gate; `fc_embolden` for C 693-695 |
| 713-746 | `#ifdef HAVE_HARFBUZZ` / the four Xft-HarfBuzz entry points / `#endif` | `harfbuzz` |
| 748 | `static void syms_of_xftfont_for_pdumper (void);` | dropped (PORTING.md 0.1-12) |
| 750-787 | `struct font_driver const xftfont_driver` (750-784; C 768-783 the four gated members) and, under `#ifdef HAVE_HARFBUZZ`, `struct font_driver xfthbfont_driver;` (785-787) | file gate; `have_libotf`, `have_m17n_flt`, `have_otf_get_variation_glyphs`/`have_ft_face_getcharvariantindex`, `have_xdbe`, `harfbuzz` for their members |
| 789-814 | `syms_of_xftfont` (C 793-796 `harfbuzz`) | file gate; `harfbuzz` for C 794-795 |
| 816-832 | `syms_of_xftfont_for_pdumper` (C 820-830 `harfbuzz`) | file gate; `harfbuzz` for C 821-830 |

## Evidence

**1. The frozen configuration (`build-tu`), and the syntax reading it gives the whole file.**
The first run failed - `error: expected expression, found keyword \`else\`` at the materialised
`_root.rs`'s `xftfont_draw` - because C 546-578's `#if`'s `else` clause cannot take a `#[cfg]` of its
own; the arm is now the two cfg-selected statements Deviation 3 records.  With that fixed:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xft-lane/cargo cargo run -q -p xtask -- build-tu src/xftfont.c
built src/xftfont.rs -> ../build/femacs/xftfont.o          # exit 0
$ ls -l build/femacs/xftfont.o                             # 336 bytes
$ md5 build/femacs/xftfont.o
MD5 (build/femacs/xftfont.o) = dbed68837694579185377dbb0821ab62
```

A 336-byte object is the **empty** one: the file gate compiles not one item of this unit, so this is
no evidence of the text - it is evidence of the gate, and (like `ftfont.c`'s reading) it is also a
cheap syntax gate over the whole unit: rustc parses the gated-off text and reported the one error
above before anything else ran.  The object was rebuilt from the final text
(`shasum femacs/src/xftfont.rs` = `032f2c1c7b57a80c97eef9d0cdf032a81b74052d`).

**2. The arm reading (`xtask probe`), with the file gate and every sub-gate on.**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xft-lane/cargo cargo run -q -p xtask -- probe src/xftfont.rs \
      /tmp/xft-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_xft"' --cfg 'feature="have_x_windows"' \
      --cfg 'feature="have_window_system"' --cfg 'feature="have_xrender"' \
      --cfg 'feature="render_version_ge_0_2"' --cfg 'feature="have_xdbe"' \
      --cfg 'feature="harfbuzz"' --cfg 'feature="have_libotf"' --cfg 'feature="have_m17n_flt"' \
      --cfg 'feature="ft_bdf_h"' --cfg 'feature="fc_dual"' --cfg 'feature="fc_charcell"' \
      --cfg 'feature="fc_embolden"' --cfg 'feature="have_otf_get_variation_glyphs"' \
      --cfg 'feature="have_ft_face_getcharvariantindex"'
exit 1; 10 rustc errors, 16 warnings
```

| locus | count | class | owner |
| --- | --- | --- | --- |
| `build/probes/xftfont/_root.rs` 1307, 1308, 1312, 1350, 1352, 1353, 1356, 1358, 1359 | 9 | `E0609 no field \`maybe_otf\`/\`otf\`/\`hb_font\` on type \`font_info\`` - **this unit's own text** (C 309/310/313, 348/350/351, 355/357/358), caused by `include/ftfont.rs`'s `struct font_info`, which records those three members as dead text instead of mirroring them | the `ftfont.h` mirror (Owed 3) |
| `build/probes/xftfont/_…_src_include_systime_rs.rs` 27 | 1 | `E0080 assertion failed: !cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS` - **a mirror copy**: the P-51 pairing assert fires because a plain `--cfg` run turns the feature on without the frozen `config.h` constant | the flip table (Owed 4) |

The cfg names are P-86 (6)'s "the file gate *and* every sub-gate": the maximal set above turns on
every arm the Gate mapping lists, including the three dead ones of the frozen configuration, so that
both arms of every `#if` in this file are compiled in one reading.  `have_window_system` and
`have_x_windows` are what the file's own gate implies (Deviation 2); `render_version_ge_0_2` is C
546's version half and is true on this machine (`RENDER_MAJOR` 0 / `RENDER_MINOR` 11,
`render.h` 35-36); `ft_bdf_h`, `fc_dual` and `fc_charcell` are live with this machine's FreeType and
Fontconfig (measured).

**3. The mutation test (P-86 (7): "0 errors in this unit" must be a real reading).**
Injecting one type error into the unit's own text - inside `xftfont_get_colors`:

```rust
  let _mutation: c_int = c"mutation";
```

and re-running the **same** command turns the 10 errors into 11, the new one naming the injected
line: `E0308 mismatched types … expected i32, found &CStr` at `_root.rs:1043` (this file's line
1043).  So rustc reaches and type-checks this unit's own body in this arm; the nine rows above are
the unit's text being checked, not a graph that stopped before it.  Restoring the line and
`shasum -c` (OK) returns the reading to 10 with the same nine loci.

**4. The reading on the final text (probe re-run after the last source change).**
The last source change was one initialiser in `xftfont_open` (Deviation 7's `matrix`), so the arm
reading was taken again on the final text (`shasum` `032f2c1c…`): **the same 10 errors at the same
10 loci**.

**5. The arm-reach preflight (P-86 (7)'s substitute for a clean configuration), and the one in-unit
defect it found.**  No configuration this machine can materialise types this unit end to end
(Evidence 2's two mirror rows), so the same materialised root was copied to `/tmp`, the three
`font_info` members were stubbed in the mirror copy and `HAVE_X_WINDOWS` was flipped to `true` in
the `config.rs` copy (exactly what `xtask dead-arms` does for its own units), and the same cfg set
was compiled:

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -C panic=abort <the same 15 --cfg> /tmp/xft-lane/pre3/_root.rs
0 errors, 16 warnings           # exit 0
```

That preflight is what caught the one **in-unit** defect the official reading could not see:
`E0381 used binding \`matrix\` isn't initialized` at C 223's `FcPatternGetMatrix (…, &raw mut matrix)`
- a later compiler phase than the type check that Evidence 2 stops in.  Fixed in the unit the way
PORTING.md 1.5 prescribes (`let mut matrix: *mut FcMatrix = std::ptr::null_mut ();`), after which the
preflight is clean.  The 16 warnings are `unused doc comment` rows on `/// C:` citations that stand
above gated `unsafe extern "C"` blocks - the `xfont.rs` 1389 shape, not new to this unit.

**6. Layout measurements (clang, this machine's headers).**  Three probes; every number below is
their verbatim output, and the whole of it is asserted in the unit.

```
$ clang -std=gnu23 -I/tmp/xft-lane/hdr -I/opt/homebrew/include -I/opt/homebrew/include/freetype2 \
        -o probe probe.c && ./probe      # /tmp/xft-lane/hdr carries libXft 2.3.6's own Xft.h
sizeof(XftFont)=32 align=8        offsetof(XftFont,ascent)=0 offsetof(XftFont,descent)=4
offsetof(XftFont,height)=8        offsetof(XftFont,max_advance_width)=12
offsetof(XftFont,charset)=16      offsetof(XftFont,pattern)=24
sizeof(XftColor)=16 align=8       offsetof(XftColor,pixel)=0 offsetof(XftColor,color)=8
sizeof(XGlyphInfo)=12 align=2     offsetof(XGlyphInfo,width)=0 height=2 x=4 y=6 xOff=8 yOff=10
sizeof(XRenderColor)=8 align=2    offsetof(XRenderColor,red)=0 green=2 blue=4 alpha=6
sizeof(XGCValues)=128 align=8     offsetof(XGCValues,foreground)=16 offsetof(XGCValues,background)=24
GCForeground=4 GCBackground=8
sizeof(FcBool)=4 sizeof(FcResult)=4 sizeof(FcType)=4 sizeof(FcChar8)=1 sizeof(FcChar32)=4
sizeof(FcMatrix)=32 align=8       offsetof(FcMatrix,xx)=0 xy=8 yx=16 yy=24
sizeof(FT_Matrix)=32 align=8      offsetof(FT_Matrix,xx)=0 xy=8 yx=16 yy=24
sizeof(FT_Size)=8 sizeof(FT_UInt)=4 sizeof(FT_Face)=8
FT_FACE_FLAG_SFNT=8 BDF_PROPERTY_TYPE_INTEGER=2
sizeof(BDF_PropertyRec)=16 align=8  offsetof(BDF_PropertyRec,type)=0 u=8
sizeof(FT_FaceRec)=248 align=8    offsetof(FT_FaceRec,face_flags)=16 units_per_EM=136
offsetof(FT_FaceRec,max_advance_width)=144 underline_position=148 underline_thickness=150
offsetof(FT_FaceRec,glyph)=152 size=160
sizeof(FT_SizeRec)=88 align=8     offsetof(FT_SizeRec,face)=0 metrics=24
PictOpSrc=1
$ clang -std=gnu23 -I/opt/homebrew/include -o probe2 probe2.c && ./probe2
FC_DUAL yes   FC_CHARCELL yes   FC_LCD_FILTER yes lcdfilter   FC_EMBOLDEN yes
FC_MATRIX yes matrix   FC_PROPORTIONAL=0
```

The three Xft records are measured against the libXft **2.3.6 release's own headers**, because
`/opt/homebrew/include/X11/Xft` is absent on this machine (`libXft` is not installed; the ledger's
Owed 13 names it).  Everything else is this machine's own: `xorgproto`/`libx11`/`libXrender`
(`/opt/homebrew/include`, `RENDER_MAJOR` 0 `RENDER_MINOR` 11), Fontconfig 2.17.1, FreeType 2.14.3
and HarfBuzz 14.4.0 (keg-only).  `<otf.h>` has **no** copy here: `OTF`/`OTF_close` are the two
UNVERIFIED declarations (Deviations 13).

**7. `struct face` (the unit-local partial record), measured over the reference's own header.**

```
$ clang -std=gnu23 -fsyntax-only -Xclang -fdump-record-layouts-complete \
      -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib -I /opt/homebrew/include \
      -DHAVE_WINDOW_SYSTEM=1 -DHAVE_X_WINDOWS=1 -DHAVE_XFT=1 probe.c
       168 |   GC gc                   176 |   ptrdiff_t stipple
       184 |   unsigned long foreground 192 |   unsigned long background
       232 |   struct font * font      240 |   int fontset        296 |   void * extra
           | [sizeof=304, align=8]
```

`xftface_info` is measured with the Xft segment's headers: `sizeof` **40**, align **8**,
`bg_allocated_p`@0, `fg_allocated_p`@1, `xft_fg`@8, `xft_bg`@24 - asserted in the unit.

**8. The acceptance gates.**

```
$ python3 tools/extract-abi.py --check emacs/src/xftfont.c femacs/src/xftfont.rs
- line count drifted 138.0%: C 832 vs Rust 1980        (one row, the tolerance row - Owed 1)
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
$ python3 tools/check-citations.py
676 file(s) scanned, 0 drift, 0 range, 1511 external, 36 allowed, 0 stale   # the external count
                                          # moves with the other lanes' files
$ python3 tools/check-dead-file-coverage.py
97 ledger(s) bound (a C range table), 0 failure(s)       # this ledger: 28 rows, 832 lines, 0 gaps
$ python3 tools/check-ledger-ticks.py
233 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
357 row(s) with no function tick, 0 row(s) without a ledger
```

All six readings are the final ones; neither `femacs/src/xftfont.rs` nor this ledger has a finding in
any of them.  (An earlier run of `check-declarations.py` in this session reported one `title` finding
in `femacs/src/xsettings.rs`, a bug-address URL - another lane's file, `git status` shows it modified
by that lane, and the final read is clean.)  The token half of
`--check` reports nothing else: the three rows this lane appended to `docs/ports/macro-body-notes.tsv`
(Owed 12) cover the only three C tokens the unit's bodies legitimately cannot spell.

## Deviations

1. **The opening comment block is not carried** (C 1-20, P-86 (1) / PORTING.md 0.1-19): the port
   opens with `// femacs: port of src/xftfont.c @ fd7afced403.` and carries no reference product name.
2. **The file gate is the build's, not C's `#if`** (`configure.ac` 7194-7203): `feature = "have_xft"`
   is on every item *and* on the include graph (P-86 (8)).  `have_x_windows` is implied by the branch
   that reaches this file and `have_window_system` in turn (the Gate mapping's rows 2/3); both are in
   the probe's cfg set.
3. **C 546-578's `if`/`else` pair is two cfg-selected statements.**  C keeps the `else` clause
   outside its `#if`, so Rust cannot put the attribute on the `if` alone (measured: an attribute on
   `else` is `expected expression, found keyword \`else\``); the port writes the gated `if`/`else`
   and a second copy of the `else`'s one statement under `not (…)` (P-51), which is why the
   `XftDrawRect` line stands twice in this text.
4. **C 246-252's two `#ifdef` conjuncts are folded into a local** (`spacing_not_proportional`,
   C 246/248/251): a middle conjunct of one condition cannot carry a `#[cfg]`, so each arm is a
   cfg-selected statement that folds itself in (P-51).
5. **C 380-387 and 403-408's `#if false` is dead in every configuration** and is recorded at its
   place (the comment and both statements quoted in the port), not transcribed (P-08).
6. **C 748's forward declaration of `syms_of_xftfont_for_pdumper` is dropped** (PORTING.md 0.1-12);
   the definition follows at C 816 and `syms_of_xftfont` reaches it from there.
7. **C's uninitialised locals take the port's zero** (PORTING.md 1.5): `result`, `extents` (twice),
   `xgcv`, `colors`, `rec`, `b1`/`b2`/`i1`/`i2`/`r1`/`r2`, and `matrix` - the last one is the
   in-unit defect Evidence 5's preflight found (`&raw mut matrix` needs an initialised binding,
   `E0381`).
8. **C's integer promotions are spelled `as`** where Rust needs them: `(*font).pixel_size = size
   as c_int`, the four `0x10000L * matrix->xx` scalings (`as FT_Fixed`), `upEM`'s two
   `underline_* * size / upEM` folds and the C 255-268 width chain.  Where C's *arithmetic* is done
   in `int` and only the store narrows (`font_metrics`'s five `short`s at C 472-476, the
   `lower_yoff - upper_yoff` of C 542-544), the operands are promoted in the port too, so a Rust
   `i16` operation cannot differ from C's result at the edges.
9. **C's chained assignments are split right-to-left** (C 215, 255-256, 262-263, 542-544): a Rust
   assignment is `()`, so `a = b = c` cannot stand.  The order is C's (the rightmost store first).
10. **`<X11/X.h>`'s `GCForeground`/`GCBackground` collide with the mirror's pair.**
    `include/dispextern.rs` 92-94 declares both names unconditionally with the Emacs_GC arm's values
    (`0x01`/`0x02`, the w32/ns GC representation `src/w32term.rs` uses); in an X11 build C has
    `(1L<<2)`/`(1L<<3)` = 4/8 (`X.h` 566-567, measured).  A second declaration in this unit is
    `E0428` *and* a `check-mirror-dupes.py` failure (both measured), so this unit uses the mirror's
    names as they stand and **this X11 arm passes mask 3 where C passes 12** - a real defect owned by
    that mirror (Owed 5).
11. **Every entry point whose C parameter is a `struct frame *`/`struct face *` takes the
    `font_driver` table's untyped `*mut c_void`** and converts at the use point (P-72 (3), the
    `xfont.rs` shape); the same for the three `ftfont.h` names whose mirror parameter/return is
    `*mut c_void` (`ftfont_entity_pattern`, `ftfont_fix_match`, `ftfont_add_rendering_parameters`).
12. **The twelve C-`static` driver members are `unsafe extern "C" fn`** (the mirror's fn-pointer
    types demand C's ABI) and, being C-`static`, carry no `#[unsafe(no_mangle)]`;
    `xftfont_get_xft_draw` is a plain `unsafe fn`.
13. **The external SDK surfaces stand in the unit, at the C `#include` lines** (P-88 (1)): Xlib's
    five names, Xft's three records and sixteen entry points, Fontconfig's six records/enums and
    five entry points plus ten object-name macros and six spacing constants, XRender's
    `XGlyphInfo`/`XRenderColor`/`Picture`/`PictOpSrc` and two entry points, and FreeType's records,
    scalars and the BDF pair.  Each declaration carries its header's own line; every layout is a
    clang reading (Evidence 6/7) except libotf's `OTF`/`OTF_close`, which are **UNVERIFIED** (no
    `otf.h` on this machine - both are opaque pointers, so there is no layout to measure).
    `XftPatternDestroy` is C's macro (`XftCompat.h` 126) and stays a `macro_rules!` in this unit so
    C 195's own name is the one at the call site.
14. **Use-site declarations the mirrors do not carry**, each with its header's line:
    `XScreenNumberOfScreen` (Xlib.h 1839, which `include/xterm.rs` 1166's own
    `FRAME_X_SCREEN_NUMBER!` body calls and no mirror declares), `XGetGCValues`, `Region`,
    `x_query_colors` (xterm.h 1797), the three `x_xr_*` helpers (xterm.h 1826-1830, gated where C
    gates them), `font_build_object` (font.h 848), `font_put_frame_data`/`font_get_frame_data`
    (font.h 923-924), `register_font_driver` (font.h 896), `xfont_get_cache` (font.h 961) and the
    eleven `ftfont_*`/`hbfont_*` declarations (font.h 917-955).  `x_mark_frame_dirty` and
    `font_add_log` are **not** redeclared: `include/xterm.rs` 228 and `include/font.rs` 299 declare
    them now (status 3.521/3.524) and a copy would be the duplicate that `check-mirror-dupes.py`
    exists to find.
15. **`s->clip` is the mirror's `[c_int; 2]`.**  `include/dispextern.rs` 1497 types
    `glyph_string.clip` as the `NativeRectangle` *fallback* (`int[2]`), while C's X11 arm has
    `XRectangle[2]`; the two are 8 bytes alike, and the port casts the array pointer at C 522 and C
    561 (Owed 11).
16. **`xfthbfont_driver = xftfont_driver` is `std::ptr::read (addr_of! (…))`** (C 821): a Rust
    `static` cannot be moved and `include/font.rs`'s `font_driver` is not `Copy`; the record's zero
    value at C 786 is `std::ptr::null_mut ()` for the `Lisp_Object` first member.
17. **`ascii_printable` is a `static mut`** (C's `static FcChar8 ascii_printable[95]`), and it
    carries the file gate like every other item; `s->padding_p` and `first_glyph->type` are the
    mirror's packed-bit accessors (`glyph_string_bits!`, `glyph_bits! (type, …)`), and
    `u.glyphless.method` is this unit's `glyphless_bits! (method, …)` over
    `GLYPH_U_GLYPHLESS_METHOD_SHIFT` (the `w32term.rs` 1733 shape; `GLYPHLESS_DISPLAY_*` is
    qualified through `dispextern_unit_names`, the module the mirror declares them in).

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `xftfont`
   (`xftfont\t1.39\t<reason>\ttotal`), tolerance **>= 1.39**, basis `total`: the port carries 1980
   physical lines against C's 832 (138.0%), because of the `// C <line>` citations, the per-item
   `#[cfg (feature = "have_xft")]` of P-86 (1) and its sub-gates, the layout assertions, the five
   unit-local SDK segments and the two cfg-selected statements of Deviation 3.  Not this lane's file
   to write.
2. **`femacs/xtask/Cargo.toml`**: register six feature names this file's arms use and the table does
   not carry - `have_m17n_flt` (`HAVE_M17N_FLT`), `ft_bdf_h` (`FT_BDF_H`), `fc_dual`/`fc_charcell`
   (`FC_DUAL`/`FC_CHARCELL`), `have_otf_get_variation_glyphs`/`have_ft_face_getcharvariantindex`
   (`HAVE_OTF_GET_VARIATION_GLYPHS`/`HAVE_FT_FACE_GETCHARVARIANTINDEX`).
3. **`femacs/src/include/ftfont.rs`**: mirror `struct font_info`'s three recorded members -
   `maybe_otf`/`otf` (`HAVE_LIBOTF`) and `hb_font` (`HAVE_HARFBUZZ`).  They are Evidence 2's nine
   `E0609` rows: C 309/310/313 and 348/350/351/355/357/358 cannot type-check until the mirror
   carries them.
4. **`femacs/src/include/systime.rs` 27 and/or the `dead-arms` flip table**: `const _: () =
   assert! (!cfg! (feature = "have_x_windows") || HAVE_X_WINDOWS);` means an arm cannot be compiled
   by `--cfg` alone; a lane that reads an X11 arm needs the constant flipped with the feature
   (Evidence 2's tenth error, and the reason Evidence 5's preflight is a `/tmp` copy).
5. **`femacs/src/include/dispextern.rs` 92-94**: split the `GCForeground`/`GCBackground` pair by arm
   (the Emacs_GC `0x01`/`0x02` under the arms that build an `Emacs_GC`, `<X11/X.h>` 566-567's
   `(1L<<2)`/`(1L<<3)` under `have_x_windows`).  Deviation 10 is the defect it causes here.
6. **`femacs/src/include/xlib.rs`**: the `<X11/Xlib.h>` surface this unit declares at its use site
   (P-90 (1)) - `XGCValues` (177-203), `XGetGCValues` (2623), `XScreenNumberOfScreen` (1839) and
   `Region` (`<X11/Xutil.h>` 273).  `include/xlib.rs` already carries `Display`, `GC`, `XColor`,
   `XRectangle` and the rest.  When they land, this unit's Xlib segment disappears in the same commit
   (P-90 (3) / P-88 (5)).
7. **`femacs/src/include/xft.rs`** (P-90 (1): four consumers in the tree - `src/ftfont.c`,
   `src/xftfont.c`, `src/ftcrfont.c` and `src/ftfont.h`): the whole `<X11/Xft/Xft.h>` surface -
   `XftFont`, `XftColor`, `XftDraw`, the sixteen entry points and the `XftPatternDestroy` macro,
   with the layouts of Evidence 6 and the provenance Deviation 13 records.
8. **`femacs/src/include/fontconfig.rs`** (two or more consumers): the Fontconfig records/enums,
   the five entry points, the ten object-name macros and the six spacing constants.
9. **`femacs/src/include/freetype.rs`**: the FreeType surface (`FT_FaceRec_` and the records it
   names, the scalars, `BDF_PropertyRec`, `FT_Get_BDF_Property`, `FT_FACE_FLAG_SFNT`) - the same
   segment `src/ftfont.rs` carries, which is what makes one mirror the right home (P-88 (5)).
10. **`femacs/src/include/xrender.rs`**: `XRenderColor`, `XGlyphInfo`, `Picture`, `PictOpSrc` and
    the two entry points.
11. **`femacs/src/include/dispextern.rs`**: `glyph_string.clip` should be `[XRectangle; 2]` under
    `have_x_windows` instead of the `int[2]` fallback (Deviation 15).
12. **`docs/ports/macro-body-notes.tsv`**: **written by this lane** (the one notes file the brief
    allows, appended only): three rows - `xftfont_draw`/`alloca`,
    `xftfont_cached_font_ok`/`goto`, `syms_of_xftfont`/`0,doc`.  The integrator should keep them as
    part of this unit's baseline rows; the `line-drift` row (Owed 1) is the other half of what
    `--check` needs.
13. **libotf**: `OTF` and `OTF_close` are UNVERIFIED here (no `<otf.h>` on this machine); a machine
    with libotf should re-read the two declarations.  The `<X11/Xft/Xft.h>` layouts were measured
    against the 2.3.6 release's headers, not an installed SDK.
14. **`docs/port-map.md`'s row and `docs/status.md`'s ratchet line**: the integrator's to refresh
    (the map still says 11 lines for `femacs/src/xftfont.rs`).

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence 1: a 336-byte empty object), so this is a *text* port with *compile* readings of its arms
  and no behaviour: nothing here has been executed, and no X11 display, Xft font, Fontconfig pattern
  or Xrender picture was involved.
- **Neither arm reading is clean, and no configuration on this machine compiles this unit end to
  end.**  Evidence 2's nine `E0609` rows are in this unit's own text (an `include/ftfont.rs` gap, Owed
  3) and the tenth is a mirror copy's assert (Owed 4); Evidence 5's preflight is the reading that
  reaches 0 errors, and it is a `/tmp` copy with two mirror edits, not a configuration of this tree.
- **The preflight and the mutation re-runs are diagnostics, not `xtask probe`.**  The probe budget was
  spent (Evidence 2's run, the mutation, the restore and the final-text re-run); the preflight, the
  `/tmp` re-materialisations and the extra `build-tu` runs are labelled as the diagnostics they are,
  and the budget overruns are reported below.
- **`extract-abi.py --check` is not coverage evidence by itself.**  It reports the line-drift row
  (Owed 1) and nothing else; its token check is a token *set* comparison, so a reordered statement
  inside a body, a swapped operand of the same tokens or a dropped `i += 1` that leaves the tokens
  intact would still pass.  The three `macro-body-notes.tsv` rows narrow that gap for exactly three
  names.
- **The layout asserts are this machine's readings** (Evidence 6/7), and the Xft ones come from the
  2.3.6 release's headers: another SDK may add members to `XftFont` (it is a public record) or move
  the FreeType privates.
- **Budgets, as run, with the overruns reported.**  `build-tu` **7 of 3** (one failing syntax
  reading, then a rebuild per source revision: five full runs plus the final-text run; every one of
  them is also the only materialiser available to the diagnostics), `probe` **4 of 3** (the arm
  reading, the mutation, the restore, and the final-text re-run after the last source change).  The
  `/tmp` preflight and the plain-`rustc` diagnostics are outside both budgets.  No `cargo test`,
  `--all`, `gate-fast` or `check-all-units.sh` was run, and no git write was performed by this lane.
- **`femacs/src/xftfont.rs` may move under a later round** (the mirror moves of Owed 6-10), and the
  `/tmp` scratch this lane worked in is deleted.
- **The other lanes' files are not this lane's evidence.**  `git status` shows four other units
  modified (one lane's `xsettings.rs` among them, whose transient `check-declarations.py` finding is
  Evidence 8's parenthetical); this lane wrote only `femacs/src/xftfont.rs`, this ledger and the three
  appended `macro-body-notes.tsv` rows, and ran no git write at all.
