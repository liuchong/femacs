# emacs/src/pgtkim.c (311 lines)

> **P-86's class: the frozen configuration compiles nothing here.**  `pgtkim.o` sits in `PGTK_OBJ`
> (`emacs/src/Makefile.in` 325 `PGTK_OBJ=@PGTK_OBJ@`, filled by `configure.ac` 3875-3878 -
> `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` under
> `if test "$window_system" = "pgtk"`), and `docs/baseline/frozen-build-sources.txt` (106 lines)
> lists no pgtk object for this target, so `extract-abi.py`'s checklist for the file is empty and
> its checkbox section cannot be coverage evidence.  What follows is P-86 ②'s substitute: the
> **C line-range table**, the `## Evidence` readings, the `## Findings` and the `## Deviations`.
> Port: `src/pgtkim.rs` (605 lines), the **whole file, C 1-311**, every item under the file
> gate.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1

## Functions defined — 15

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

(The five counts are `python3 tools/extract-abi.py emacs/src/pgtkim.c`'s reading; the item lines are
deliberately not carried, because a `- [x]` here would be a claim `check-ledger-ticks.py` binds to
the port's text while the coverage evidence for a P-86 file is the table below - the
`docs/ports/gfilenotify.md` shape.  The unit *is* complete: all 15 functions, the one subr and the
`DEFVAR_LISP`-owned `Vpgtk_use_im_context_on_new_connection` are in `src/pgtkim.rs`.)

## Decisions

- **The file gate is `feature = "have_pgtk"` (P-86 ⑧), on every item *and on the include graph*.**
  The criterion is the reference's own: `configure.ac` 3171-3178 is the `pgtk )` arm of the
  `window_system` case, which sets `term_header=pgtkterm.h` (3172), `with_gtk3=yes` (3173),
  `USE_X_TOOLKIT=none` (3174), `HAVE_PGTK=yes` (3175) and
  `AC_DEFINE([HAVE_PGTK], [1], [Define to 1 if you have pure Gtk+-3.])` (3176); `PGTK_OBJ` (3875-3878)
  is what puts `pgtkim.o` in the link.  The frozen `configure` leaves `HAVE_PGTK` `undef`
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1489), so nothing here is compiled on this target
  and this batch's `build-tu` object is the empty one (Evidence ①).
- **The arm reading adds `have_gtk3` (Evidence ②), and *not* `use_gtk`.**  `have_gtk3` is implied by
  the same arm: `with_gtk3=yes` (3173) is what makes the GTK3 pkg check run and define
  `HAVE_GTK3` (`configure.ac` 3755-3756), and the GTK surface mirror gates three of the names the
  pgtk records use on it (`src/include/gtk.rs` 1119-1120 `GtkCssProvider`, 517-673 and
  1094/1586's GTK3-only entry points) - measured: without the flag the reading carries 3 extra
  `cannot find type GtkCssProvider` errors, with it they are gone.  `use_gtk` is the *GTK-in-X11*
  define (`configure.ac` 3820-3826, `HAVE_GTK=yes`), and the pgtk arm sets `USE_X_TOOLKIT=none`
  (3174) instead, so passing it would be a wider gate than the reference has.
- **The file's own text has no `#if` at all.**  Unlike `src/pgtkterm.c` (whose slice carries
  `GDK_WINDOWING_WAYLAND`/`HAVE_XWIDGETS` arms), `src/pgtkim.c` is live text to the preprocessor
  from C 21 to C 311: the pgtk build configuration is the only condition, and `#ifdef HAVE_PGTK`
  lives in the header (`src/pgtkterm.h` 29, mirrored by `include/pgtkterm.rs`'s per-item gates).
  So the file gate above is the whole gate mapping and no `## Gate mapping` table is needed.
- **`docs/port-map.md` binds this ledger to `src/pgtkim.c` alone** - the reference has no
  `src/pgtkim.h` - so this is the single-reference shape and there is no header mirror of its own
  to `include!` (P-90 ②); the header it *does* include is the next bullet's.
- **The include graph, and why the unit carries an edge the C `#include` list does not have.**
  C 21 is `<config.h>` and C 23 is `"pgtkterm.h"`; nothing else.  The port reaches
  `include/pgtkterm.rs` (C 23) and, through it, `include/gtk.rs` for `"pgtkterm.h"` 31's
  `<gtk/gtk.h>` - the surface's single home, whose own C 31 edge is still owed, so this unit carries
  the stand-in edge at the C position that makes the names visible (the `src/pgtkterm.rs` 81 and
  `src/pgtkfns.rs` C 28-chain shape).  **It also carries `include!("include/lisp.rs")` *ahead of*
  the mirror**, which C reaches through `"pgtkterm.h"` 23-26 (`dispextern.h`, `frame.h`,
  `character.h`, `font.h`): `macro_rules!` resolve in textual order, and `include/pgtkterm.rs`
  expands `dispextern.rs`/`frame.rs`/`character.rs`/`font.rs`/`sysselect.rs` *before* its own
  `include!("lisp.rs")`, so a unit that reached the mirror first would call `eassert!`/`eassume!`/
  `XUNTAG!`/`XSETPVECTYPESIZE!` before their definitions (measured: 44 `cannot find macro` errors,
  every one anchored in those mirrors - Findings 2).  `../lib/include/string.rs` is C 99-100's
  `strlen` (`lisp.h` 30's `<string.h>`, which `include/lisp.rs` does not carry yet); `sprintf`
  (C 59) arrives through `include/lisp.rs` 35's `<stdio.h>` mirror.
- **The third-party toolchain is on this machine only in part** (Evidence ⑤): GLib/GObject and cairo
  are present and are what `include/gtk.rs` cites; **GTK, GDK and Pango are not**, so every value of
  the Pango segment below is a placeholder, no layout is asserted for its records, and its
  citations name the header alone.  The arm flip is still a real compile of this unit's statements
  (it needs no system header: the declarations are the port's own and the symbols are wanted only at
  link time), which is why the reading is `text-only` for the *toolkit* and a real compile for the
  *unit* (P-86 ③).
- **The frozen `build-tu` object is the port's own, not a reference object.**  There is no
  `pgtkim.o` in the frozen build to compare against (`docs/baseline/frozen-build-sources.txt`), so
  Evidence ① proves "the gate is on every item" and nothing about the reference's behaviour.

## C line ranges

The rows are contiguous and ascend; `docs/port-map.md` binds this ledger to `src/pgtkim.c` alone, so
the table covers C 1-311.  Seventeen blank lines (C 20, 22, 24, 40, 47, 54, 63, 150, 164, 184,
200, 212, 224, 268, 276, 284, 295) carry no row, which is what `check-dead-file-coverage.py` expects
of a blank between blocks.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block | dropped (P-86 ①/0.1-19; the port opens with the `// femacs: port of src/pgtkim.c @ fd7afced403.` line) |
| 21 | `#include <config.h>` | ungated (the config mirror, the `src/pgtkfns.rs` 21 shape) |
| 23 | `#include "pgtkterm.h"` | file gate; the mirror edge, plus the stand-in edges to `include/gtk.rs` (`pgtkterm.h` 31) and the ahead-of-the-mirror `include/lisp.rs` (see `## Decisions`) |
| 25-39 | `im_context_commit_cb` (the "commit" handler) | file gate |
| 41-46 | `im_context_retrieve_surrounding_cb` (C 44 the `""` / `-1` arguments) | file gate |
| 48-53 | `im_context_delete_surrounding_cb` (C's two unread parameters) | file gate |
| 55-62 | `make_color_string` (C 59-60's `sprintf` and the three promoted `guint16`s) | file gate |
| 64-149 | `im_context_preedit_changed_cb` (C 81-85 the attribute-list comment, C 91/142 the `do … while`, C 101-102 the `continue`, C 108-137 the four `pango_attr_iterator_get` casts) | file gate |
| 151-163 | `im_context_preedit_end_cb` | file gate |
| 165-169 | `im_context_preedit_start_cb` (empty body) | file gate |
| 170-183 | `pgtk_im_focus_in` (C 177-179 the `gtk_widget_get_window (FRAME_GTK_WIDGET (f))` argument) | file gate |
| 185-199 | `pgtk_im_focus_out` | file gate |
| 201-211 | `pgtk_im_filter_keypress` (C 207's `gboolean` test, C 208/210's `true`/`false`) | file gate |
| 213-223 | `pgtk_im_set_cursor_location` (C 220's `GdkRectangle` aggregate, C 221's `&area`) | file gate |
| 225-267 | `pgtk_im_use_context` (C 236/281 the two `g_object_unref` sites, C 245-260 the six `g_signal_connect` sites) | file gate |
| 269-275 | `pgtk_im_init` (C 274's `Vpgtk_use_im_context_on_new_connection` read) | file gate |
| 277-283 | `pgtk_im_finish` | file gate |
| 285-294 | `DEFUN ("pgtk-use-im-context", Fpgtk_use_im_context, Spgtk_use_im_context, 1, 2, 0, …)` | file gate |
| 296-311 | `syms_of_pgtkim` (C 299's `defsubr`, C 301-304's four `DEFSYM`s, C 306-309's `DEFVAR_LISP`, C 310's `= Qt`), with the `Vpgtk_use_im_context_on_new_connection` definition C 306-310 owns | file gate |

## Findings

1. **`include/pgtkterm.rs`'s prototype for `pgtk_im_set_cursor_location` is not C's, and
   `include/gtk.rs`'s `GdkRectangle` note repeats the misquote.**  The mirror's row reads
   `/// C 667-668: extern void pgtk_im_set_cursor_location (struct frame *, int, int, GdkRectangle);`
   and declares `(f, x, y, area: GdkRectangle)`, while `src/pgtkterm.h` 667-668 is
   `(struct frame *, int, int, int, int)` and C 213-215's definition is
   `(struct frame *f, int x, int y, int width, int height)` - the same call `src/pgtkterm.c` 3071
   makes.  `include/gtk.rs` 1670-1671 then justifies carrying `GdkRectangle` by that 4th parameter.
   This unit defines C's signature (Rust lets the definition shadow the glob-imported declaration,
   PORTING.md 0.1-10), so nothing here depends on the misquote; the mirror row and the
   `GdkRectangle` note are owed to `docs/ports/include-pgtkterm.md` (Owed 3).
2. **`macro_rules!` order: the pgtk header mirror expands five reference-header mirrors (`dispextern.h`, `frame.h`,
   `character.h`, `font.h`, `sysselect.h`) before
   `lisp.rs`.**  `include/pgtkterm.rs`'s include list is `dispextern.rs`, `frame.rs`,
   `character.rs`, `font.rs`, `sysselect.rs`, then `lisp.rs`, and those first five call `eassert!`/
   `eassume!`/`XUNTAG!`/`XSETPVECTYPESIZE!` - defined in `lisp.rs` 387/402/503/3099.  A unit that
   reached the mirror without `lisp.rs` ahead of it measured 44 `cannot find macro` errors (working
   note of the flip, /tmp only).  Every landed consumer carries `lisp.rs` first
   (`src/pgtkterm.rs` 114, `src/gtkutil.rs`, `src/image.rs`, `src/xwidget.rs`), so this unit does
   too; the *mirror's* own order is still the more robust home for the fix and is named in Owed 6.
3. **`include/pgtkterm.rs` 772-776 does not parse under `have_pgtk`.**  The `prototypes` extern
   block ends with a five-line `///` comment (C 678's `pgtk_emacs_to_gtk_modifiers` note) and no
   item after it, which rustc rejects with `error: expected item after doc comment` (measured at the
   mirror's 776 in the materialised root).  The whole module is `#[cfg (feature = "have_pgtk")]`, so
   the frozen configuration never parses it and no gate sees it - this is the first reading that
   does (Owed 2).
4. **`include/xlib.rs` gates the three X11 types the pgtk records need on `have_x_windows`.**
   `pub type XrmDatabase` (196-197), `pub type Window` (846-847) and `XRectangle` are all
   `#[cfg (feature = "have_x_windows")]`, while `include/pgtkterm.rs` names them at 144, 193, 194,
   205, 289, 290 and 637 - measured: 7 `cannot find type` errors in the flip.  A pgtk build still
   compiles against `<X11/Xlib.h>`/`<X11/Xresource.h>` (that is where `Window`/`XrmDatabase`/
   `XRectangle` come from and why `include/pgtkterm.rs` reaches `xlib.rs` at all), so the gate looks
   too narrow for this arm; the mirror owner's call is Owed 5.
5. **`include/pgtkterm.rs` 340/348 spell C's bare incomplete tag.**  C 422/452 write
   `struct atimer *atimer_visible_bell;` / `*scale_factor_atimer;`, which C accepts by declaring the
   tag implicitly; the mirror writes `*mut atimer`, and no mirror in this graph provides `atimer`
   (measured: 2 `cannot find type` errors).  PORTING.md 0.1-2's rule for exactly this shape is
   `*mut c_void` (or the mirror's own edge to `include/atimer.rs`); Owed 4.
6. **My own first edition double-wrapped `G_CALLBACK!`, and the measurement is a mirror property.**
   `include/gtk.rs` 229-234's `g_signal_connect!` applies `G_CALLBACK!` inside the macro, so
   `g_signal_connect! (…, G_CALLBACK! (f), …)` becomes a cast of an `Option<unsafe extern "C" fn ()>`
   - measured: 6 `error[E0605]: non-primitive cast` in probe run 2.  The six call sites now pass the
   bare handler (the `src/gfilenotify.rs` 345 reading).  The same spelling appears at
   `src/pgtkterm.rs` 458-460; that file is another lane's and was **not** compiled here, so whether
   it errors is an inference from this measurement, not a reading.

## Evidence

**① The frozen configuration (`build-tu`, one run).**
`cd femacs && CARGO_TARGET_DIR=/tmp/pgtkim-lane cargo run -q -p xtask -- build-tu src/pgtkim.c`
-> `built src/pgtkim.rs -> ../build/femacs/pgtkim.o`, **exit 0**, and
`../build/femacs/pgtkim.o` is **336 bytes** - the empty object, which is the expected reading here
and not a proof of the new text: every item sits behind `have_pgtk`, so under the frozen feature set
rustc compiles `include/config.rs` and the `#![allow]` attribute only.  It is the port's own object;
the frozen build has no `pgtkim.o` to compare it with (see `## Decisions`).

**② The arm flip (`probe`, four runs - one over the three the mandate allowed).**  The last run:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/pgtkim-lane cargo run -q -p xtask -- probe src/pgtkim.rs \
    /tmp/pgtkim-lane/x.rlib --crate-type=lib -C panic=abort \
    --cfg 'feature="have_pgtk"' --cfg 'feature="have_gtk3"'
aborting due to 10 previous errors; 7 warnings emitted
error: rustc failed for src/pgtkim.rs (exit status: exit status: 1)      # no rlib written
```

**Every one of the 10 is anchored in `include/pgtkterm.rs`; none is in the unit's own text.**
They are the named mirror gaps of `## Findings` and `## Owed`:

| class | count | anchor |
| --- | --- | --- |
| `cannot find type Window` | 4 | `include/pgtkterm.rs` 144/193/289/290 (`include/xlib.rs` 846-847's `have_x_windows` gate) |
| `cannot find type XrmDatabase` | 2 | `include/pgtkterm.rs` 194/637 (`include/xlib.rs` 196-197's gate) |
| `cannot find type XRectangle` | 1 | `include/pgtkterm.rs` 205 (same gate) |
| `cannot find type atimer` | 2 | `include/pgtkterm.rs` 340/348 (bare incomplete tag; no mirror) |
| `expected item after doc comment` | 1 | `include/pgtkterm.rs` 776 (the dangling comment inside the `prototypes` extern block) |

Run 2 (the revision before the last two fixes) is the same command without `have_gtk3` and read
**60 errors**: those 10, plus the 3 `GtkCssProvider` errors the flag removes, plus 44 `cannot find
macro` errors and the unit's own 3 `E0381`s (below).  Run 3 read 13 (10 mirror + 3 `E0381`).

The set is stable across the tree's own movement: a **second, independent reading of the same 10**
was taken on a fresh materialisation (`build-tu` run 2, `build/femacs/units/pgtkim/_root.rs`,
compiled with the two `--cfg`s by `rustc --emit=metadata`) *after* another lane's
`src/include/systhread.rs` edit landed mid-flight - the edit that turns
`tools/check-mirror-dupes.py`'s reading from `17 duplicate declaration(s), 17 ratchet row(s),
0 failure(s)` into `27 duplicate declaration(s) … 10 failure(s)`, every failure in
`src/android.rs`/`src/sysdep.rs` and **none of them this unit's**.  The fresh reading
is the same 10 anchors, so nothing in that edit reaches this unit's errors.

**③ The unit's own statements are clean (`text-only`, P-86 ⑦'s "mask the gap" reading, /tmp only).**
A copy of the materialised root in `/tmp` with (a) the mirror's dangling doc comment dropped and
(b) the four missing types stubbed at the end of the root, compiled with the same two `--cfg`s:

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O _root.rs -o /tmp/pgtkim-lane/stub.meta \
    --crate-name pgtkim_stub --cfg 'feature="have_pgtk"' --cfg 'feature="have_gtk3"'
aborting due to 1 previous error; 7 warnings emitted
error[E0658]: use of unstable library feature `c_variadic`  --> include/lisp.rs:3756:51
```

That one error is **not this unit's and not new**: it is the `include/lisp.rs` 3756 row
(`verror`'s `core::ffi::VaList<'_>`) that `docs/ports/pgtkterm.md`'s Evidence ② already records as a
named mirror gap.  With the ten mirror gaps masked, **zero errors are anchored in the unit's own
statements**.  The reading is *not* an empty one (P-86 ⑦): runs 2 and 3 reported this unit's own
errors - run 2's `error[E0308]: mismatched types … expected bool, found i32` at
`build/probes/pgtkim/_root.rs:383` (C 142's `gboolean` test, `! pango_attr_iterator_next (iter)`,
fixed to `== 0`) and run 3's three `error[E0381]: used binding … isn't initialized` at
`_root.rs:333` (C 69-71's three uninitialised locals now carry C's zero value, PORTING.md 0.1-13) -
so rustc does type-check this unit's text and does report what it finds there.

**④ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/pgtkim.c
src/pgtkim.rs` -> `FAIL`, **exactly 1 `- ` line**:

```
- line count drifted 94.5%: C 311 vs Rust 605
```

That line is the integrator's (`## Owed` 1).  The check stopped being vacuous the moment the port
gained a `fn` (P-86 ⑩): its first reading carried four `- ` lines, three of them the per-function
token rows `im_context_preedit_changed_cb`/`continue`, `pgtk_im_use_context`/`g_callback` and
`syms_of_pgtkim`/`doc`, which are now the three rows appended to
`docs/ports/macro-body-notes.tsv` under `# src/pgtkim.c (311 lines)`.  Everything else the check
compares passes: the one subr (Lisp name, arity, interactive spec and docstring digest), the 15
function names, the `#[unsafe(no_mangle)]` rule for the seven non-`static` definitions, and the
"exported symbol C does not have" rule for `Vpgtk_use_im_context_on_new_connection`.

**⑤ What is on this machine, and what is not.**

```
$ ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0 \
        /opt/homebrew/include/pango-1.0 /opt/homebrew/include/gdk-pixbuf-2.0
ls: … four times: No such file or directory
$ ls -d /opt/homebrew/include/glib-2.0 /opt/homebrew/include/cairo
/opt/homebrew/include/glib-2.0        # GLib 2.90.0
/opt/homebrew/include/cairo           # cairo 1.18.4
$ which pkg-config
pkg-config not found
```

So the GLib/GObject and cairo half of the surface is quotable (and `include/gtk.rs` is where those
citations live); **the GTK, GDK and Pango half is not on this machine at all**, which is why every
value in the Pango segment of this unit is marked a placeholder, no size or offset is asserted for
`PangoColor`/`PangoAttribute`/`PangoAttrInt`/`PangoAttrColor`, and the missing toolchain is named in
`## Owed` 7.  **This also corrects the lane's own briefing, which stated that GTK3/GDK headers were
present under `/opt/homebrew/include/gtk-3.0`; they are not** (the four `ls` readings above), so
P-86 ③⑵'s `text-only` half applies to the toolkit surface while the unit itself is still really
compiled (Evidence ②/③).

**⑥ The other four gates.**

```
$ python3 tools/check-mirror-dupes.py                    # first reading, this unit's own revision
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)          # no FAIL/STALE on src/pgtkim.rs
$ python3 tools/check-mirror-dupes.py                    # after the systhread.rs edit above
27 duplicate declaration(s), 17 ratchet row(s), 10 failure(s)         # all 10 in src/android.rs /
                                                                     # src/sysdep.rs; 0 lines on pgtkim
$ python3 tools/check-declarations.py
10 declaration sites (0 forbidden, 10 allowed literals), 0 failures   # read 683 files (625 .rs …)
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 0 drift, 0 range, 731 external, 35 allowed, 0 stale
$ python3 tools/check-dead-file-coverage.py
info docs/ports/pgtkim.md: 19 range row(s) cover src/pgtkim.c (311 lines; 17 blank line(s) between
blocks need no row); 59 ledger(s) bound (a C range table), 0 failure(s)
```

The requirement this batch bears is "no `FAIL`/`STALE` row naming `src/pgtkim.rs`", and that
holds in both readings (the second one's 10 failures are the other lane's, reported above);
`grep -c pgtkim` over the gate's output is **0**.

## Deviations

1. **C 21's `<config.h>` is ungated** (`include!("include/config.rs")`), the `src/pgtkfns.rs` 21 /
   `src/pgtkterm.rs` 67 shape: it is the configuration the other mirrors read, and it produces no
   symbol.  Every *other* edge and item carries `have_pgtk` (P-86 ⑧).
2. **The Pango surface stands in this unit (P-88 ①), not in a mirror.**  `<pango/pango-attributes.h>`
   has no file in the reference tree, and `src/gtkutil.c` C 2892-2969 is the second consumer, so its
   long-term home is a mirror of its own - Owed 7.  Its four `PangoAttrType` members and
   `PANGO_UNDERLINE_NONE` are **placeholders** carrying the ordinal of the header's own member list,
   transcribed as a citation and not measured (P-88 ②), and no layout is asserted for the four
   records.
3. **C 69-71's three uninitialised locals get C's zero value** (`str`/`attrs` null, `pos` 0,
   PORTING.md 0.1-13); C relies on `gtk_im_context_get_preedit_string` filling all three.
4. **C 101-102's `continue` is written as the negated test.**  In C's `do { … } while
   (pango_attr_iterator_next (iter));` the `continue` jumps to the *condition*, not out of the loop;
   the port's `loop` evaluates that condition in a trailing `if pango_attr_iterator_next (iter) == 0
   { break; }`, so the rest of the body is nested under `if st < ed { … }`.  A literal `continue`
   would skip the `break` test and spin (`docs/ports/macro-body-notes.tsv`'s
   `im_context_preedit_changed_cb` row).
5. **The three `guint16` reads are promoted before the shift.**  C's `pac->color.red >> 8` is an
   `int` after the integer promotion and `%02x` reads an `unsigned int`, so the port writes
   `((*pac).color.red as c_int) >> 8` (the variadic argument's type is not promoted by Rust).
   `PangoColor`'s three members are spelled `c_ushort`, the spelling `include/gtk.rs` 366-368 already
   uses for `guint16`.
6. **The callbacks are `unsafe extern "C" fn`.**  `include/gtk.rs`'s `G_CALLBACK!` (141-148)
   transmutes the item's address into an `unsafe extern "C" fn ()`, and its own note requires that
   form; the `src/pgtkterm.rs` sibling spells its callbacks `unsafe fn`, which is recorded here
   rather than copied.
7. **C 245-260's `G_CALLBACK (f)` is not spelled at the call site**: the mirror's `g_signal_connect!`
   applies it, so the port names `g_signal_connect_data` and the six handlers
   (macro-body-notes row; Findings 6).
8. **`pgtk_im_filter_keypress` returns Rust `bool`** where C returns `bool` and the mirror's
   prototype says `-> bool`: C 208/210's `true`/`false` are the port's, and C 207's `gboolean` test
   is `!= 0`.
9. **C 306-310's `DEFVAR_LISP` variable is defined by this unit.**  C's storage is
   `globals.f_<vname>` (the macro's expansion) inside the one `struct emacs_globals globals` that
   alloc.c defines, and `include/globals.rs` 6681 mirrors the name as a *declaration* only; the unit
   that owns it defines it (`#[unsafe(no_mangle)] pub static mut … = lisp_h_Qnil ()`) - the
   `src/gnutls.rs` 1824 shape, status 3.29.
10. **C's zero-value default for a fresh context is C's own**: `pgtk_im_init` (C 272) writes `NULL`
    and C 274's `!NILP (Vpgtk_use_im_context_on_new_connection)` is the port's `! NILP (…)`; the
    variable's own initial value is `Qt` (C 310).

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`** (the integrator's file): append a `pgtkim` row for
   Evidence ④'s one `- ` line - C 311 total vs the port's 605 (94.5% measured; `total` basis, so a
   tolerance of 0.96 with the reason "the P-86 ①/⑩ scaffolding: the per-item gate and citations, the
   P-88 ① Pango segment, the include-edge notes").  **DONE (status 3.492, integrator):** the row is
   appended (`0.96`, `total` basis) and `extract-abi.py --check emacs/src/pgtkim.c src/pgtkim.rs`
   is now **OK with zero `-` rows**.
2. **`docs/ports/include-pgtkterm.md`**: `include/pgtkterm.rs` 772-776's dangling doc comment is a
   syntax error under `have_pgtk` (Findings 3).  Either the `prototypes` block gains the declaration
   it describes (`pgtk_emacs_to_gtk_modifiers`, blocked by `src/xwidget.rs`'s copy - its own Owed) or
   the comment becomes a `//` block.  Until then **no `have_pgtk` unit that includes this mirror can
   compile**, which is what keeps this unit's flip at 10 errors.
3. **`docs/ports/include-pgtkterm.md`**: the `pgtk_im_set_cursor_location` prototype (its 667-668
   row) should be C's four `int`s, and `include/gtk.rs` 1670-1671's `GdkRectangle` justification
   should be corrected with it (Findings 1).
4. **`docs/ports/include-pgtkterm.md`**: its 340/348 should not name the bare `atimer` tag - either
   `*mut c_void` per PORTING.md 0.1-2, or an edge to `include/atimer.rs` (Findings 5).
5. **`docs/ports/include-xlib.md`**: `Window`, `XrmDatabase` and `XRectangle` (its 196-197, 846-847)
   are gated `have_x_windows`, but the pgtk arm needs all three (Findings 4).  The gate the
   reference implies is `any (feature = "have_x_windows", feature = "have_pgtk")`.
6. **`docs/ports/include-pgtkterm.md`**: its include list expands those five reference headers before
   `lisp.rs` (Findings 2); carrying `include!("lisp.rs")` first, or moving the macro users under the
   macro definitions, would remove the constraint the unit works around today.
7. **`src/include/gtk.rs` / `include-gtk.md` (P-88 ⑤ move-out)**: the eight
   `gtk_im_context_*` entry points this unit declares at C 23's position (`gtk_im_context_reset`,
   `focus_in`, `focus_out`, `set_client_window`, `filter_keypress`, `get_preedit_string`,
   `set_surrounding`, `set_cursor_location`) belong in the GTK surface's single home; two of their
   neighbours (`gtk_im_multicontext_new`, `gtk_im_context_set_use_preedit`) are already there
   (its 1276-1284), so their move is what removes this unit's segment.
8. **`src/include/pango.rs` (new mirror) + `docs/ports/include-pango.md`**: the Pango
   surface this unit carries in-unit (Deviation 2) - `src/gtkutil.c` C 2892-2969 is the second
   consumer, so P-90 ① gives it a home of its own, and a Pango toolchain is what measures the five
   placeholder values and the four records' layouts.
9. **`docs/baseline/feature-coverage.tsv` / `xtask/Cargo.toml`**: **no new feature name is
   needed.**  `have_pgtk` is registered (feature-coverage 14, Cargo.toml 229) and so is `have_gtk3`
   (27, Cargo.toml 209); this unit adds no third gate.  The pair the arm reading uses is recorded in
   `## Decisions` and Evidence ②, and the `have_gtk3`-as-a-pgtk-subgate reading is worth an entry in
   `docs/status.md` with the rest of this batch.
10. **`tools/check-port-lines.py --write-pending`** (P-86 ④): the `src/pgtkim.c` row (311 reference
    lines, no longer under the ten-percent threshold) should take itself off
    `docs/baseline/port-lines-pending.tsv` 40 on the integrator's run.

## Not claimed

- **No run evidence.**  Nothing was linked, started or exercised: the flip produced no rlib (the
  mirror gaps abort it), and Evidence ③'s reading is a *compile* of a masked copy in `/tmp`, not of
  the tree.  No PGTK target exists here to run this against.
- **Not a GTK/Pango build.**  The arm flip compiles against this unit's own declaration segment;
  `include/gtk.rs`'s GLib/cairo numbers are header quotes and the Pango five are placeholders.  No
  GTK, GDK or Pango symbol was resolved and no `gtk_*`/`pango_*` call was made.
- **The flip is not a pass.**  10 errors remain, all in `include/pgtkterm.rs`; Evidence ③'s zero is
  an "own statements" reading *as measured by masking those ten*, not a green compile of the tree.
- **The Pango records' layout is unmeasured.**  `PangoColor`'s three `guint16`s, `PangoAttribute`'s
  prologue and the two derived records' payload offsets are C's documented shape, not a clang
  reading - P-88 ② forbids writing a number this machine cannot measure, and Owed 8 names the
  toolchain that would.
- **`src/pgtkterm.rs` 458-460** (Findings 6) was not compiled here; the claim that it shares this
  unit's double-`G_CALLBACK!` defect is an inference from this unit's measurement.

The integrator's bookkeeping for this unit (status 3.492): `docs/baseline/port-lines-pending.tsv`
was regenerated with `--write-pending` (61 -> 58 -> **53 ratchet rows, 0 stale**; this unit's row
left the list because 605 target lines are far past the ten-percent threshold);
`docs/baseline/feature-coverage.tsv` needs no change (the unit adds no feature name: `have_pgtk`
and `have_gtk3` were both registered already).
