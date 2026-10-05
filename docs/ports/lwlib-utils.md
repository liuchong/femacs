# emacs/lwlib/lwlib-utils.c (291 lines)

## Decisions

`emacs/lwlib/lwlib-utils.c` @ `fd7afced403`; port `femacs/lwlib/lwlib-utils.rs` (**740 lines**);
mirror `femacs/lwlib/include/lwlib-utils.rs` (195 lines).  The gate is `feature = "use_lucid"`
(P-86 clause 8), the build's condition rather than one the C text spells: the reference lists
`lwlib-utils.o` inside the widget library's object list (`lwlib/Makefile.in` 58:
`OBJS = lwlib.o $(TOOLKIT_OBJS) lwlib-utils.o`, whose toolkit lists are line 52's `LUCID_OBJS` and
line 54's `MOTIF_OBJS`), and `configure.ac` 7231's
`AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])` is the condition this
port's gate is written as.  `USE_LUCID`, `USE_X_TOOLKIT`, `HAVE_X11` and `USE_CAIRO` are `undef` in
docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of this
file and the port's frozen object is the empty one.

The header shares this ledger (`docs/port-map.md` binds both `lwlib/lwlib-utils.c` and
`lwlib/lwlib-utils.h` to it): its 63 lines are inside the table below, and its mirror is
`femacs/lwlib/include/lwlib-utils.rs`, whose declarations moved into `lwlib_utils_decls` (P-39) so
that this unit's definitions shadow the foreign declarations instead of colliding with them
(`E0428`).

### Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`OBJS`, i.e. the X toolkit's object list) | `feature = "use_lucid"`, on every item and on C 22-30's include graph | an X toolkit configuration (the frozen one is not) |
| C 141 `#ifdef USE_CAIRO` … C 291 `#endif /* USE_CAIRO */` | `all (feature = "use_lucid", feature = "use_cairo")` | a build that also draws with cairo |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's opening block (title, copyright lines, warranty and copying text) | dropped (P-86 clause 1) |
| 20-21 | blank line, `#include <config.h>` | ungated (every unit includes it) |
| 22-24 | `#include <setjmp.h>` and `#include <lisp.h>` | file gate (`setjmp.h` has no edge of its own: nothing here names its types, and `lisp.rs` carries them) |
| 25-30 | the three `<X11/*>` headers, `lwlib-utils.h`, `lwlib.h` | file gate |
| 31 | blank line | - |
| 32-51 | `XtNoClearRefreshWidget` (C 32 the one-line comment above it) | file gate |
| 52-81 | `XtApplyToWidgets` (C 63-65 its children-list comment, C 72-73 the commented-out `XtIsWidget` guard) | file gate |
| 82-108 | `XtApplyUntilToWidgets` (C 84-87 its comment) | file gate |
| 109-133 | `XtCompositeChildren` (C 111-113 its comment) | file gate |
| 134-139 | `XtWidgetBeingDestroyedP` | file gate |
| 140-146 | `#ifdef USE_CAIRO` and the `<math.h>` / `<cairo-ft.h>` / `<cairo-xlib.h>` includes | cairo gate |
| 147-208 | `crxft_font_open_name` (the `XftFontOpenName` renaming) | cairo gate |
| 209-215 | `crxft_font_close` (the `XftFontClose` renaming) | cairo gate |
| 216-238 | `crxft_draw_create` (the `XftDrawCreate` renaming) | cairo gate |
| 239-247 | `crxft_set_source_color` (static) | cairo gate |
| 248-256 | `crxft_draw_rect` (the `XftDrawRect` renaming) | cairo gate |
| 257-270 | `crxft_draw_string` (the `XftDrawStringUtf8` renaming) | cairo gate |
| 271-291 | `crxft_text_extents` (the `XftTextExtentsUtf8` renaming) and `#endif /* USE_CAIRO */` | cairo gate |

## Findings

1. **C 141-291 is the cairo half of a file whose other half is Xt-only** (C 32-139).  The C text
   spells that split only as `#ifdef USE_CAIRO`; the build spells the outer half as the X toolkit's
   object list.  Both gates are on every item of their half here (`## Deviations` 1).
2. **`XtNoClearRefreshWidget` builds an `XEvent` by hand** (C 36-50) with every member of the
   `xexpose` arm and calls the widget class's `expose` through the class record - the only place in
   this file that reaches into `CorePart`/`CoreClassPart`.
3. **C 147-208's `crxft_font_open_name` is a fontconfig pipeline**: parse the name, substitute the
   config, fall back to `XGetDefault`/`sscanf` for the DPI, match a font, build a cairo scaled font
   from it and read back its metrics.  It is the only user of `lround` and `sscanf` in this file.

## Evidence

**① The frozen configuration.**
`cd femacs && CARGO_TARGET_DIR=/tmp/lwutils-lane cargo run -q -p xtask -- build-tu
lwlib/lwlib-utils.c` → `built lwlib/lwlib-utils.rs -> ../build/femacs/lwlib-utils.o`, **exit 0**,
and `build/femacs/lwlib-utils.o` is **336 bytes** - the empty object the file gate leaves (run
five times: after the first write, after the `E0308` corrections, before the arm flips, and once
more after each of the two comment-only rounds - the lint fix and the citation-line fix; the
reading is identical each time, and it is the expected reading, not a proof of the text).

**② The arm flip (P-86 clause 3(1); the file gate plus the sub-gate).**
The final revision, compiled from the root `build-tu` materialized
(`build/femacs/units/lwlib-utils/_root.rs`, the same flags `xtask probe` passes):

- `rustc --edition 2024 -O --crate-type=lib -C panic=abort --cfg 'feature="use_lucid"'
  --cfg 'feature="use_cairo"'` → **exit 0, 0 errors, 2 warnings**, rlib **8,185,432 bytes**.
- `rustc … --cfg 'feature="use_lucid"'` (the file gate alone) → **exit 0, 0 errors, 2 warnings**,
  rlib **7,872,008 bytes**.
- Both warnings are in *included mirrors* (`lib/include/stdbit.in.rs` 644 and
  `src/include/lisp.rs` 3934, the `///`-on-an-`extern`-block class); **0 are in this unit's own
  text**.

`xtask probe lwlib/lwlib-utils.rs …` itself ran **3 times** on the two earlier revisions
(1: exit 1, 6 × `E0308` **all in the unit's own text** - a `c_long`/`c_int` narrowing and two
`c_short`/`c_ushort` casts; 2: exit 0 / rlib 8,184,016; 3: exit 0 / rlib 7,871,600) - see
`## Deviations` 5 for the five signature defects the clang probe then found.

**③ The cairo/fontconfig half, measured with clang** (this machine has `<cairo.h>` 1.18.4,
`<cairo-ft.h>` and `<fontconfig/fontconfig.h>` 2.18.3 under /opt/homebrew/include; the probe is
`clang -I/opt/homebrew/include/cairo -I/opt/homebrew/include -I/opt/homebrew/include/freetype2 p.c`
with a function-pointer assignment per declaration and `sizeof`/`offsetof` prints).  Its first run
**failed on five of the port's declarations** (they were C's spellings, not readings):

| the port had | the header says |
| --- | --- |
| `cairo_scaled_font_create (…, options: *mut cairo_font_options_t)` | `const cairo_font_options_t *` |
| `cairo_ft_font_options_substitute (options: *mut …, pattern: *const FcPattern)` | `(const cairo_font_options_t *, FcPattern *)` |
| `cairo_set_scaled_font (…, scaled_font: *mut cairo_scaled_font_t)` | `const cairo_scaled_font_t *` |
| `FcConfigSubstitute (…)` returning `()` | returns `FcBool` |
| `FcPatternGetDouble (p: *mut FcPattern, …)` | `const FcPattern *` |

After those five were fixed, the probe is clean and prints: `cairo_matrix_t` 48 (align 8; xx 0,
yx 8, xy 16, yy 24, x0 32, y0 40); `cairo_font_extents_t` 40 (ascent 0, descent 8, height 16,
max_x_advance 24); `cairo_text_extents_t` 48 (x_bearing 0, y_bearing 8, width 16, height 24,
x_advance 32, y_advance 40); `FcBool` 4; `FcResult` 4, `FcResultMatch` 0, `FcResultNoMatch` 1,
`FcMatchPattern` 0; `FC_DPI` = `"dpi"`, `FC_PIXEL_SIZE` = `"pixelsize"`; `XftFont` 24 (scaled_font
0, ascent 8, descent 12, height 16, max_advance_width 20); `XftColor` 16 (pixel 0, color 8, red 8,
alpha 14); `XGlyphInfo` 12 (width 0, height 2, x 4, y 6, xOff 8, yOff 10); and the four
`<math.h>`/`<stdio.h>` prototypes this file adds by hand (`lround`, `sscanf`) match.
`cairo_xlib_surface_create` (C 217's call) is cairo-xlib.h 49's prototype, read as text: its
`Display *` / `Drawable` / `Visual *` come from the `<X11/Xlib.h>` this machine does not have.
The five use-site citations that carry a line number were then read off the installed headers
rather than from memory: `math.h` 514 (`extern long int lround(double);` - the SDK's line; the
`src/hbfont.rs` 585 citation of the same prototype is the glibc spelling at 474), `_stdio.h` 282
(`int sscanf(const char * __restrict, const char * __restrict, ...);`, which is where this SDK's
`stdio.h` declares it), `fontconfig.h` 279-285 / 334-341 / 116 / 95 (the two enums, `FC_DPI` and
`FC_PIXEL_SIZE`) and `cairo-xlib.h` 49-54.

**④ The mutation reading (P-86 clause 7).**  Injecting `xfree (1);` into
`XtWidgetBeingDestroyedP`'s body of the materialized `_root.rs` (the unit's own text) turns the
both-gates reading from **0 errors** into **1 error** (`E0308`, `_root.rs:559`, expected `*mut
c_void`, found `usize`); the file was restored afterwards and the 0-error reading re-taken.  The
unit's own text is therefore really type-checked, and "0 errors" is a reading rather than an
early abort in the include graph.

**⑤ Inventory.**  `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-utils.c
femacs/lwlib/lwlib-utils.rs` → one `- ` row only:
`- line count drifted 154.3%: C 291 vs Rust 740`, then `FAIL`.  All five live functions, their
bodies' tokens and the `#[unsafe(no_mangle)]` set are otherwise clean (the cairo half is dead text
in the frozen configuration, so this check does not see it - the C range table above is its
coverage).  The tolerance row is owed by the integrator (`## Owed` 3).

**⑥ The other gates** (repo root, the final revision): `tools/check-declarations.py` → 17
declaration sites (0 forbidden, 17 allowed literals), 0 failures; `tools/check-mirror-dupes.py` →
17 duplicate declaration(s), 17 ratchet row(s), 0 failures; `tools/check-dead-file-coverage.py` →
`lwlib-utils.md: 18 range row(s) cover lwlib/lwlib-utils.c (291 lines)` and the same 18 over
`lwlib/lwlib-utils.h` (63 lines), 0 failures; `tools/check-citations.py` → 0 drift, 0 range, 0
stale; `tools/check-citation-text.py femacs/lwlib` → 0 of every class; `tools/check-port-lines.py`
→ the `lwlib/lwlib-utils.c` row is now stale (informational: the port is no longer under ten
percent) and 0 files are new; `tools/check-ledger-ticks.py` → 0 failures.

## Deviations

1. **Two gates on one file.**  `use_lucid` is the file gate taken from the build (`## Decisions`),
   `use_cairo` the C text's own `#ifdef USE_CAIRO`; the cairo half carries both as
   `all (feature = "use_lucid", feature = "use_cairo")` (P-86 clause 6: an arm flip must set every
   sub-gate).  `lwlib-utils.o` is in the object list of *both* toolkit lists (`MOTIF_OBJS` too), so
   a Motif-only reading of the same file would need the `use_motif` arm; the Lucid condition is the
   one this port's gate is written as, and the frozen configuration compiles neither.
2. **`<setjmp.h>` has no include edge** (C 23): nothing in this file names a type that header
   declares, and the mirror that carries them is `lisp.rs` (its own note at 1060), which C 24
   includes.  Recorded rather than dropped silently.
3. **The cairo/fontconfig surface is declared in the unit** (P-88 clause 1: no mirror in this tree
   carries `<cairo.h>`, `<cairo-ft.h>`, `<cairo-xlib.h>` or `<fontconfig/fontconfig.h>`, and
   `include/lwlib-utils.rs` leaves `cairo_t`/`cairo_scaled_font_t`/`FcChar8` to its user).
   `ceil` comes from the `lib/include/math.rs` mirror, which is the frozen target's `<math.h>`;
   `lround` and `sscanf` are declared at their use sites with the reference header's line (P-72
   rule 4), the `src/hbfont.rs` 585-590 shape, because those mirrors do not carry them.
4. **The X11 widget surface is C's spelling, not a measurement** (P-86 clause 3(2)): this machine
   has no X11 header at all.  `WidgetRec`/`CorePart`/`WidgetClassRec`/`CoreClassPart`/`CompositeRec`
   carry only the members this file reads, in C's relative order; `XEvent` is a union with `type`,
   `xexpose` and a 192-byte padding member; `XtIsComposite`/`XtIsWidget` are declared as entry
   points and their class variables are carried beside them (the `XtIsShell` shape `lwlib-Xlw.rs`
   chose for the same macros).  Every one of those offsets is **UNVERIFIED** (`## Owed` 1).
5. **Five declaration defects the clang probe caught** (`## Evidence` ③) were fixed rather than
   carried: four `const` qualifications and one return type.  The port's call sites keep C's
   spellings - Rust coerces `*mut T` to `*const T` at the argument position - so the fix is in the
   declarations only.
6. **Uninitialised locals start at C's zero value** (PORTING.md 0.1-13): `event` (C 36, written
   member by member at C 38-47), `dpi`/`result`/`pixel_size`/`font_matrix`/`ctm`/`extents` (C 156,
   166-167, 176, 183-185, 193 - the fontconfig out-parameters) and `root`/`x`/`y`/`width`/`height`/
   `border_width`/`depth` (C 221-223, filled by `XGetGeometry` and then read).  Named here because
   a reviewer cannot see the zeroing in C's text.
7. **`pub` and `match` are Rust keywords**: the two C locals of those names are spelled `pub_`
   (the header mirror's `XftFontClose` metavariable already uses that spelling) and `r#match`
   (the token compares as the C name, PORTING.md 0.2).
8. **C's implicit conversions are spelled**: `Dimension`/`c_uint` → `c_int` at C 45-46,
   `double` → `c_int` at C 195-198 (`lround`'s `c_long`), `double` → `c_ushort`/`c_short` at
   C 282-289, and `int` → `double` at C 161-162.  No semantic change.
9. **The mirror's `_decls` container** (`femacs/lwlib/include/lwlib-utils.rs`, P-39) groups C 4's
   declaration with C 9-16's and C 45-60's; the two typedefs C 6-7 declares follow the container -
   the one reordering the container forces.  The container's item order inside it is C's.

## Owed

1. **The `<X11/*>` surface's real shapes**: with an X11 toolchain, measure `CorePart`,
   `WidgetClassRec`, `CoreClassPart`, `CompositePart` and `XEvent`, and replace the placeholder
   records/offsets (P-88 clause 2).  What is missing on this machine is the X11 headers
   (xorgproto/XQuartz); cairo and fontconfig were measured (`## Evidence` ③).
2. **`XftFont`/`XftColor`/`XGlyphInfo` are the mirror's own records** and were measured from the
   reference header's declarations (the clang probe copies C 23-41 verbatim), not from `<Xft.h>`,
   which is an X11-based header this machine does not have; a future X11 round may measure the
   Xft header's own definitions and record the difference.
3. **The line-drift tolerance row is owed** (the integrator's file,
   docs/ports/line-drift-tolerance.tsv): `lwlib-utils`, tolerance **1.56**, basis `total`, reason =
   C 291 vs Rust 740 (154.3%): the whole file is transcribed, and the difference is the fixed cost
   of a unit whose two halves each declare a header surface no mirror in this tree carries - 96
   declarations with one `#[cfg (feature = …)]` line and one `/// C:` citation apiece, the 24-line
   X11 widget record set, the 40-line cairo/fontconfig record set, and the per-block C-order
   comments (the `lwlib-Xlw` row is the same class at 78.6%).
4. **The `docs/baseline/port-lines-pending.tsv` row** for `lwlib/lwlib-utils.c` is stale after this
   round (informational); the integrator's `tools/check-port-lines.py --write-pending` retires it.
5. **`femacs/src/haikuterm.rs` 30 cites this mirror by line number** ("the
   `lwlib/include/lwlib-utils.rs` 48-49 reading of the same dead region"): the `_decls` container
   moved that comment to **61-62**.  That file is not this lane's to edit (its own comment is prose,
   not a machine-checked citation).
6. **`docs/port-map.md`'s target-line column** for `femacs/lwlib/lwlib-utils.rs` still says 3.

## Not claimed

- **Zero run evidence**: nothing was linked or run; no widget was refreshed, no cairo context drawn
  and no font opened.  The two rlibs are compile products of a type-check.
- **The X11 half was never measured** (no X11 header here) and the cairo half was measured against
  *this* machine's cairo 1.18.4 / fontconfig 2.18.3, not against the reference's X11-era libraries;
  `cairo_xlib_surface_create`'s `Display`/`Drawable`/`Visual` remain C's spelling.
- **The frozen object is the 336-byte empty one**, so it says nothing about the body: the arm
  readings are the only compile evidence, and the cairo half's bodies are compiled by the arm but
  checked by no token gate (they are dead text in the frozen configuration).
- **No clang layout probe for the X11 records** and none for `<Xft.h>`; and no behavioural
  equivalence claim for either arm.
