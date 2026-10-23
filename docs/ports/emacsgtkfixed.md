# emacs/src/emacsgtkfixed.c (210 lines)

> **P-86's class, first arm-split file: the frozen configuration compiles nothing here.**  The
> build system puts `emacsgtkfixed.o` in `GTK_OBJ` only in the GTK3 branch (`configure.ac` 3756
> `AC_DEFINE ([HAVE_GTK3], [1])` beside 3757 `GTK_OBJ=emacsgtkfixed.o`) and the object list takes
> `GTK_OBJ` only when the toolkit compiles (3818-3819 empties it, 3826 `AC_DEFINE ([USE_GTK], [1])`
> beside 3827 `GTK_OBJ="gtkutil.o $GTK_OBJ"`; `emacs/src/Makefile.in` 182/452).  Both macros are
> `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` (HAVE_GTK3 1385, USE_GTK 3861) and
> `docs/baseline/frozen-build-sources.txt` lists no `emacsgtkfixed.o`, so `extract-abi.py`'s live
> checklist for the file is empty and its checkbox sections cannot be coverage evidence.  What
> follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence` readings and the
> `## Deviations`.  Port: `src/emacsgtkfixed.rs`, **both arms of C 25's `#ifdef HAVE_PGTK`
> carried**.

## Subrs (DEFUN) — 0

## Functions defined — 8

`EMACS_FIXED` (C 58-63), `emacs_fixed_class_init` (65-75), `emacs_fixed_init` (77-83),
`emacs_fixed_new` (85-92), `emacs_fixed_get_preferred_width` (94-110),
`emacs_fixed_get_preferred_height` (112-128), `XSetWMSizeHints` (137-198),
`XSetWMNormalHints` (204-208).  The last two are C's non-`static` definitions, i.e. exported
symbols (PORTING.md 0.2/P-52) and carry `#[unsafe(no_mangle)] pub unsafe extern "C"`.

## Macros defined — 0

`G_DEFINE_TYPE` (C 56) is an *invocation* of GLib's macro, not a definition; the port carries it
as C's text and owes the expansion (`## Owed` 6).

## Types and their fields — 1

`struct _EmacsFixedPrivate` (C 41-44, one `struct frame *f`).  The two records the *header*
defines (`struct _EmacsFixed`, `struct _EmacsFixedClass`) belong to
`src/include/emacsgtkfixed.rs`; this unit restates them for the arm that mirror withholds
them from (`## Deviations` 2).

## File-scope objects — 0

## Decisions

- **The file gate is `all (feature = "use_gtk", feature = "have_gtk3")`** (`configure.ac` 3826 /
  3756), on every item *and on C 23-30's include graph* (P-86 ⑧) - the same pair
  `src/gtkutil.rs` 123 gates the very same `"emacsgtkfixed.h"` edge with.  `HAVE_GTK3` is what
  makes the file GTK3-only: in a GTK2 build `GTK_OBJ` never receives `emacsgtkfixed.o` (the
  GTK2 branch leaves it empty at 3735) even though `USE_GTK` is defined there.
- **C 25's `#ifdef HAVE_PGTK` is the file's one inner condition and both arms are carried**
  (P-86 ①): the `#ifdef` arm (C 26/101-104/119-122) is `have_pgtk`, the `#else` arm (C 27-28/
  105-109/123-127/131-210) is `not (have_pgtk)`.  A GTK3-on-X11 build has `HAVE_X_WINDOWS`
  (configure.ac 3138-3142's `x11` case) and `include/xterm.rs`'s records are gated with it, so
  the X11 arm's flip must set `have_x_windows` as well - the `docs/ports/gtkutil.md` Evidence ②
  arm-A `--cfg` set.  `pgtkterm.h` and `xterm.h` are two `include!` edges at C 25-29's position.
- **The GTK surface has one home, `include/gtk.rs`** (P-90 ②): this unit reaches it at C 30, the
  position where `"emacsgtkfixed.h"` 24's `<gtk/gtk.h>` enters C's chain, and the names that
  mirror does not carry yet stand here at their C position (P-88 ①) - `GTypeInstance`,
  `GTypeClass`, `G_TYPE_CHECK_INSTANCE_CAST`, `G_TYPE_INSTANCE_GET_PRIVATE`,
  `g_type_check_instance_cast`, `g_type_instance_get_private`, `g_type_class_add_private` - each
  quoting its `gobject/gtype.h` line.  They are owed back to that mirror (`## Owed` 1).
- **The X11 names C 28's chain reaches are spelled here too** (P-88 ①, owed to `include/xlib.rs`
  and `include/xterm.rs`): `PMinSize`, `PResizeInc`, `XSizeHints` (with its unnamed aspect
  record), `XA_WM_SIZE_HINTS`, `XA_WM_NORMAL_HINTS`, `XChangeProperty`,
  `x_display_info_for_display`, `x_top_window_to_frame`.  X11 *is* on this machine, so each value
  and every offset is a clang measurement (`## Evidence` ③) and `XSizeHints` carries eighteen
  `const _: () = assert!` layout bindings.
- **`"emacsgtkfixed.h"`'s mirror is consumed at the crate root, not through a module.**  A mirror's
  text is written for crate-root scope (it names `c_void`, which `include/lisp.rs` imports there);
  wrapping the `include!` in a `mod` leaves every such name unresolved (measured).  Its one
  top-level prototype, `emacs_fixed_new` (C 56 of the header), therefore collides with this unit's
  *definition* of that function - `E0428` - which is P-39's documented class; the remedy is a
  change *inside* that mirror (`## Owed` 4).
- **C 32-36's `#if GNUC_PREREQ (4, 7, 0) && ! GLIB_CHECK_VERSION (2, 35, 7)` gate carries no arm.**
  Its body is two `#pragma GCC diagnostic` lines, which have no Rust spelling and silence a
  diagnostic Rust cannot emit; C's text is kept as a comment (the `src/pgtkterm.rs` 69,
  `src/pdumper.rs` 224, `src/timefns.rs` 79 and `src/ccl.rs` 913 readings).  The condition is
  **decided, not guessed**: `GNUC_PREREQ (4, 7, 0)` is false because the frozen table has
  `__GNUC__` 4 / `__GNUC_MINOR__` 2 (`docs/baseline/config/darwin-arm64.macros.tsv` 6264/6259),
  and `! GLIB_CHECK_VERSION (2, 35, 7)` is false for the GLib 2.90.0 this machine has.  The tree's
  registered **`glib_check_version_lt_2_36`** (`src/gfilenotify.rs` 298-305's
  `#if ! GLIB_CHECK_VERSION (2, 36, 0)`) is **a different condition** (2.36.0, not 2.35.7) and must
  not be reused for this line; no feature is needed while no arm is written.

## C line ranges

The rows are contiguous on purpose: `docs/port-map.md` binds this ledger to `src/emacsgtkfixed.c`
*and* to `src/emacsgtkfixed.h` (both map to a file named `emacsgtkfixed`), and
`tools/check-dead-file-coverage.py` reads each range against both references - so **every row opens
on the blank line that precedes its block and the union covers C 1-210 without a gap** (the header's
own 1-60 falls inside the same ranges; its blocks are the ledger of
`docs/ports/include-emacsgtkfixed.md`'s kind, and this file is its only consumer).

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's declaration block | dropped (P-86 ① / 0.1-19; the port opens with `// femacs: port of …`) |
| 20-22 | the blank, `#include <config.h>`, the blank | ungated |
| 23-31 | `lisp.h`, `frame.h`, C 25-29's `#ifdef HAVE_PGTK` / `"pgtkterm.h"` / `#else` / `"xterm.h"` / `#endif`, `"emacsgtkfixed.h"`, the blank | file gate + `have_pgtk` (C 26) / `not (have_pgtk)` (C 28); the GTK surface is `include/gtk.rs` |
| 32-40 | the GNOME-bug-683906 comment and the `GNUC_PREREQ (4, 7, 0) && ! GLIB_CHECK_VERSION (2, 35, 7)` gate around two `#pragma GCC diagnostic` lines (32-36), then the two `typedef`s of the header's records (38-39) | no arm for the pragmas (they have no Rust spelling; the condition is false here - see `## Decisions`); the typedefs are Rust `type` aliases |
| 41-46 | `struct _EmacsFixedPrivate { struct frame *f; }` | file gate |
| 47-57 | the two `static` forward declarations of the preferred-size callbacks (47-52, dropped per 0.1-12), C 53-55's `#ifndef HAVE_PGTK` declaration of `emacs_fixed_get_type`, C 56's `G_DEFINE_TYPE` | file gate; the declaration lives in a module + glob import, the macro's expansion is Owed 6 |
| 58-63 | `EMACS_FIXED` (C 61's `G_TYPE_CHECK_INSTANCE_CAST`) | file gate |
| 64-75 | the blank and `emacs_fixed_class_init` (C 72-73 the two class members, C 74 `g_type_class_add_private`) | file gate |
| 76-83 | the blank and `emacs_fixed_init` (C 80 `G_TYPE_INSTANCE_GET_PRIVATE`, C 82 `priv->f = 0`) | file gate |
| 84-92 | the blank and `emacs_fixed_new` (C 88 `g_object_new`, C 91 `GTK_WIDGET`) | file gate; `#[unsafe(no_mangle)]` (P-52) |
| 93-110 | the blank and `emacs_fixed_get_preferred_width` (C 101-109's `#ifdef HAVE_PGTK` arms) | file gate + `have_pgtk` / `not (have_pgtk)` |
| 111-128 | the blank and `emacs_fixed_get_preferred_height` (C 119-127's arms) | file gate + the same pair |
| 129-136 | the two blanks, C 131's `#ifndef HAVE_PGTK` and the "Override the X function …" comment | `not (have_pgtk)` |
| 137-198 | `XSetWMSizeHints` (C 145 `long data[18]`, C 146-163 the eighteen assignments, C 165-194 the `PMinSize`/`PResizeInc` block, C 175-179 the commented-out lines, C 196 `XChangeProperty`) | `not (have_pgtk)` |
| 199-208 | the blank, the "Override this X11 function …" comment and `XSetWMNormalHints` | `not (have_pgtk)` |
| 209-210 | the blank and C 210's `#endif` (C 131's) | `not (have_pgtk)` |

Every line of C 1-210 is inside a row (no gaps at all), which is also what the header's 60 lines
need.

## Findings

1. **The lead "#GTK3/GDK are on this machine" is falsified, and it is what the tree already
   records.**  Measured: `ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` ->
   `No such file or directory`; `which pkg-config` -> nothing; `ls /opt/homebrew/opt | grep -i gtk`
   -> `gtk-doc` only; `find /opt/homebrew /usr/local -name gtkfixed.h` -> nothing.  GLib/GObject
   (2.90.0), cairo and X11 *are* present, and `include/gtk.rs`'s own note says the same.  So no GTK
   or GDK record can be measured, every value of theirs in the port is the mirror's placeholder
   (UNVERIFIED, P-88 ②), and the arm flip below is a rustc reading of the port against the mirrors,
   not a build against GTK.
2. **The X11 arm cannot be flipped on this target *by design*.**  `include/systime.rs` 27 carries
   `const _: () = core::assert! (!cfg! (feature = "have_x_windows") || HAVE_X_WINDOWS);` and the
   frozen configuration has `HAVE_X_WINDOWS` `undef`
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1720), so the mirror itself refuses the flip -
   `docs/ports/xterm.md` calls it "the mirror refusing to be flipped before its target exists".  My
   X11-arm flip therefore reports one `E0080` from that line (`## Evidence` ②), and no reading of
   this arm on this target can exit 0.
3. **`include/xterm.rs`'s three INLINE bodies are not `unsafe`, so 10 `E0133`s are the arm's first
   block.**  `SELECTION_EVENT_DISPLAY` (mirror 1436-1441), `x_make_truecolor_pixel` (1503-1521) and
   `x_mutable_colormap` (1532-1536) are `pub fn` with raw-pointer dereferences in their bodies;
   Rust 2024 makes that a hard error, not a lint.  Measured: making the three `unsafe fn` in a
   *copy* of the mirror removes all ten (`## Evidence` ④).  `include/gtk.rs` and
   `docs/ports/xterm.md` 475 already record the neighbourhood ("with `have_x_windows` on the
   mirrors themselves do not compile").
4. **`include/gtk.rs`'s `GtkWidgetClass` has no `get_preferred_height`.**  C 73 writes it;
   the mirror's record carries `get_preferred_width` (gtkutil.c's member) and `style_updated`
   (xfns.c's).  Reading: one `E0609` whose span is the unit's own field write, caused by the
   record's member set - the `src/gtkutil.rs` `no field new_size_p on type frame` class, and
   `docs/ports/include-gtkutil.md` already names this exact addition as a change *inside*
   `include/gtk.rs`.
5. **`include/emacsgtkfixed.rs` declares `emacs_fixed_new` at its top level, so the unit that
   *defines* it gets `E0428`.**  This is P-39's class verbatim ("`lisp.rs` crate 根的
   `unsafe extern "C" { pub fn … }` … 会 `E0428`"); the remedy P-39 prescribes (and
   `include/xterm.rs` 212's `xterm_bitmap_decls` already practises) is to move that prototype into
   a `pub mod …_decls { … } use …_decls::*;` *inside the mirror*, so the definition shadows it.
   Measured on a copy: with that one change the `E0428` is gone (`## Evidence` ④).
6. **The mirror's two records and its prose disagree.**  Its head note says the two header records
   are "deferred rather than invented" (they hold GTK records by value, so no honest shape exists
   here), and then the file declares both under `#[cfg (feature = "have_pgtk")]` with
   `*mut c_void` members - although `emacsgtkfixed.h` 39-50 is inside *no* `#ifdef` (its 30-33
   `HAVE_PGTK` block holds only the two type macros).  Both halves are wrong for the X11 arm, which
   is why this unit restates them there (`## Deviations` 2, `## Owed` 4).
7. **`include/pgtkterm.rs` does not compile under `have_pgtk` either** (13 mirror-side errors in the
   PGTK arm, `## Evidence` ②b, `expected item after doc comment` at its 776 among them).  That is
   consistent with its own note (the pgtk records are owed) and with `docs/ports/pgtkterm.md`.

## Evidence

**① The frozen configuration.**  `cd femacs && CARGO_TARGET_DIR=/tmp/egf-lane cargo run -q -p xtask
-- build-tu src/emacsgtkfixed.c` -> `built src/emacsgtkfixed.rs -> ../build/femacs/emacsgtkfixed.o`,
**exit 0**, and the object is **336 bytes** (an empty object: every item of the port is behind the
file gate, and the one ungated edge - `include/xlib.rs` - is spelled ungated only so the
materializer does not give its single expansion slot to the pgtk-gated copy;
`include/xlib.rs`'s every item carries `have_x_windows` itself, measured).

**② The arm flips.**
a) X11 arm, final revision, the tree's own probe: `cargo run -q -p xtask -- probe
src/emacsgtkfixed.rs /tmp/egf-lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="use_gtk"'
--cfg 'feature="have_gtk3"' --cfg 'feature="have_x_windows"'` -> **exit 1, 13 errors, 10 warnings,
no rlib emitted.**  The classes: **10 × `E0133`** (`include/xterm.rs` 1436/1503/1532 - Finding 3),
**1 × `E0609`** (`get_preferred_height`, `include/gtk.rs` - Finding 4), **1 × `E0428`**
(`emacs_fixed_new`, `include/emacsgtkfixed.rs` - Finding 5), **1 × `E0080`**
(`include/systime.rs` 27 - Finding 2).  Twelve of the thirteen spans are mirror copies; the
thirteenth is the unit's own `get_preferred_height` field write, whose cause is the mirror's member
set.  All ten warnings are mirror-side too (`gtk.rs` 3, `xterm.rs` 2, `frame.rs` 2, `lisp.rs` 1,
`emacsgtkfixed.rs` 1, `stdbit.in.rs` 1).
b) PGTK arm (`--cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' --cfg 'feature="have_pgtk"'`):
**13 errors again, every one mirror-side** - 5 × `cannot find type Window`, 2 × `XrmDatabase`,
2 × `atimer`, 1 × `XRectangle`, 1 × `no field get_preferred_height`, 1 × `emacs_fixed_new`
redefined, 1 × `expected item after doc comment` (`include/pgtkterm.rs` 776); 0 in the unit's own
text.
c) The file gate alone (`--cfg 'feature="use_gtk"'`, the command the task names): **exit 0 and an
18,984-byte rlib, but it is vacuous** - every item of the unit is gated by
`all (use_gtk, have_gtk3)`, so nothing of the port is compiled (the same shape as ① 's 336-byte
object).  It is recorded here so the reading is not mistaken for an arm that closed.

**① ②a/b are the tree's own `build-tu`/`probe`; the numbers of b) and c) come from a `/tmp`
re-materialization of the same graph, because `probe` is budgeted** (`probe` 4 runs were spent in
total: three on two earlier revisions and one on the final one; see the report's budget line).  The
`/tmp` re-materialization was validated byte-for-byte against the tree's own
`build/probes/emacsgtkfixed/` before it was used (45 of 46 files identical; the one difference was
`include/systhread.rs`, which a sibling lane edited at 13:19 *while* this round ran - a live edit,
not a reproduction difference).

**③ clang measurements** (`X11 *is* on this machine`, `/opt/homebrew/include`, xorgproto 2025.1 +
libX11; a 30-line C probe in `/tmp/egf-measure/probe.c`, deleted at the end of the round):
`sizeof (XSizeHints)` **80**, align **8**; offsets `flags@0 x@8 y@12 width@16 height@20
min_width@24 min_height@28 max_width@32 max_height@36 width_inc@40 height_inc@44 min_aspect@48
max_aspect@56 base_width@64 base_height@68 win_gravity@72`, and the two aspects' `y` at +4;
`PMinSize` **16**, `PResizeInc` **64**, `USPosition` 1, `USize`-family values as `Xutil.h` 102-109
writes them; `XA_WM_NORMAL_HINTS` **40** (`Xatom.h` 48), `XA_WM_SIZE_HINTS` **41** (49);
`PropModeReplace` **0** (`X11/X.h` 475, already in `include/xlib.rs`); `sizeof (long)` **8**.

**④ The control reading that isolates the four mirror fixes.**  On a `/tmp` copy of the
materialized graph, four changes were made *outside* this unit - the header mirror's prototype into
a `pub mod emacsgtkfixed_decls { … } use …;` (Finding 5), `GtkWidgetClass` plus
`get_preferred_height` (Finding 4), `include/systime.rs` 27's assertion relaxed (Finding 2), and
`SELECTION_EVENT_DISPLAY`/`x_make_truecolor_pixel`/`x_mutable_colormap` made `unsafe fn`
(Finding 3).  The same rustc command as ②a then reads **exit 0, 0 errors, a 14,173,824-byte rlib**:
the port's own text is closed, and the 13 errors are exactly those four mirror-side changes.  This
is a */tmp* reading of a patched copy - it is not a reading of the tree.

**⑤ The gates.**  `python3 tools/extract-abi.py --check emacs/src/emacsgtkfixed.c
src/emacsgtkfixed.rs` -> **FAIL**, with exactly **one `- ` row**: `line count drifted
141.4%: C 210 vs Rust 507` (the integrator's `docs/ports/line-drift-tolerance.tsv` row; nothing
else is missing).  `check-mirror-dupes.py` -> **17 duplicate declaration(s), 17 ratchet row(s),
0 failure(s)**, none of them naming this unit.  `check-declarations.py` -> `10 declaration sites
(0 forbidden, 10 allowed literals), 0 failures`.  `check-citations.py` -> `676 file(s) scanned,
2 drift, 0 range, 912 external, 35 allowed, 0 stale` - the two drifts are `src/ftfont.rs`
67/68, another lane's; this unit's citations are all `external` (X11/GLib headers have no file in
the reference tree), so it contributes 0 drift.  `check-dead-file-coverage.py` -> `61 ledger(s)
bound (a C range table), 0 failure(s)` before this ledger's table lands.

## Deviations

1. **The records' member spellings are the header mirror's**: `_EmacsFixed`'s `container` and
   `r#priv` and `_EmacsFixedClass`'s `parent_class` are `*mut c_void` there, so this unit's field
   reads/writes carry `as` casts (`(*fixed).r#priv as *mut EmacsFixedPrivate`) and C's implicit
   pointer conversions are spelled (`klass as gpointer`, `data.as_ptr () as *const c_uchar`).  The
   two by-value GTK members cannot be measured here (no GTK tree), so their shape is UNVERIFIED
   (P-88 ②), as is the offset of `priv`.
2. **The X11 arm restates the header's two records**, gated `not (have_pgtk)` so that the mirror's
   `have_pgtk` copies and these never coexist in one compilation (the mirror withholds them from
   the arm that needs them - Findings 6).  `## Owed` 4 asks for the gate to be dropped; then this
   copy is deleted.
3. **`include/gtk.rs`'s `GtkWidgetClass` is used as it stands** (P-90 ②'s one-name-one-mirror), so
   C 73's member write is the one diagnostic in the unit's own text (Finding 4).  Shadowing it with
   a second record was rejected: that is exactly the silent-shadowing hazard P-90 ②/`check-mirror-
   dupes` exist for, and this unit's text is otherwise error-free.
4. **C's usual arithmetic conversions are spelled**: `data[1] = (*hints).x as c_long;` and the two
   increment remainders (`data[5] % (*hints).width_inc as c_long`), and
   `x_top_window_to_frame (dpyinfo, w as c_int)` narrows C's `Window` to the prototype's `int`.
5. **C 82's `priv->f = 0` keeps C's literal** (`0 as *mut frame`): the `null` family of
   `docs/ports/token-aliases.tsv` covers C's `NULL`, not this literal, and the body-token gate
   requires the `0` (the `src/alloc.rs` 1111 `val = 0 as *mut c_void;` spelling).
6. **`sizeof (EmacsFixedPrivate)` is `core::mem::size_of::<EmacsFixedPrivate> () as gsize`**: the
   prototype's parameter is `gsize`, which is not `usize` to rustc even where both are 64-bit.
7. **The unnamed `struct { int x; int y; }` C shares between `min_aspect` and `max_aspect`** is
   named `XSizeHints_aspect` (Rust has no nested record definition; P-67's rule, the
   `include/dispextern.rs` 356 `<union>_<member>` spelling).
8. **`PropModeReplace` is imported inside the one function that uses it** (`use
   x11_constants::PropModeReplace;`, the `src/xfns.rs` 1033 spelling) - it is the mirror's, so it
   is not restated here, and a crate-root `use` would need `have_x_windows` in its own gate.
9. **C's chain edges are spelled** (P-86 ⑧): `include/xlib.rs` ungated (the P-89 workaround: the
   materializer gives a file one expansion slot and the pgtk-gated copy would otherwise claim
   `xlib.rs` and leave the X11 arm without `x11_constants`), plus `include/dispextern.rs`,
   `include/systime.rs`, `include/font.rs` and `include/character.rs` at their headers' positions
   (`"termhooks.h"` 27/28, `"xterm.h"` 1131's `*mut font`, and the character helpers the mirrors'
   inline bodies call).
10. **`emacs_fixed_new`'s parameter is `*mut frame`** (a complete type in `include/frame.rs`), not
    the mirror's `*mut c_void`: the header has only the forward tag, P-55's `*mut c_void` is for
    tags no mirror defines, and the ABI is identical.

## Owed by the integrator (nothing here is claimed as done)

1. **`src/include/gtk.rs`: `GtkWidgetClass` needs `get_preferred_height`** (C 73;
   `docs/ports/include-gtkutil.md` already names this addition).  Until it lands the X11 and PGTK
   arms both carry one `E0609` whose span is this unit's own field write.
2. **`src/include/xterm.rs`: `SELECTION_EVENT_DISPLAY` (1436), `x_make_truecolor_pixel`
   (1503) and `x_mutable_colormap` (1532) need `unsafe` bodies or `unsafe fn`** - their ten
   `E0133`s block every `have_x_windows` arm, not just this one.
3. **`src/include/systime.rs` 27's assertion** is the mirror's intended refusal to be flipped
   on a target without `HAVE_X_WINDOWS`; the X11 arm's flip reading will carry its `E0080` until an
   X11 target is frozen (or the flip is done through `xtask dead-arms`, which also flips the frozen
   constant).
4. **`src/include/emacsgtkfixed.rs`**: (a) its `emacs_fixed_new` prototype must go into a
   `pub mod …_decls { … } use …_decls::*;` (P-39's shape, `include/xterm.rs` 212's precedent) or
   this unit's definition is `E0428`; (b) its two records' `#[cfg (feature = "have_pgtk")]` gate is
   wrong (`emacsgtkfixed.h` 39-50 is unconditional) and its head note already says these records
   should be *deferred*; when the gate is dropped, delete this unit's `not (have_pgtk)` copy
   (`## Deviations` 2); (c) `include/pgtkterm.rs` 776's `expected item after doc comment` is its
   own.
5. **`docs/ports/line-drift-tolerance.tsv`**: a row for this unit (C 210 vs Rust 507 = 141.4%
   drift).  `extract-abi --check` reads FAIL with that one `- ` row and nothing else.
   **DONE (status 3.493, integrator):** the `total`/`1.43` row is appended, and
   `extract-abi.py --check emacs/src/emacsgtkfixed.c src/emacsgtkfixed.rs` is now
   **OK with zero `-` rows**.
6. **`G_DEFINE_TYPE`'s expansion (C 56)** is owed: it defines `emacs_fixed_get_type` (declared in
   this unit via a module + glob import, `## Evidence` ④'s control shows the pattern), forward-
   declares `emacs_fixed_init`/`emacs_fixed_class_init` (C 65-83) and registers the class with
   `GTK_TYPE_FIXED`/`g_type_register_static_simple`.  PORTING.md has no entry for an
   item-defining macro of a third-party header (§11 owes one) - the `src/gtkutil.rs` 313-320
   treatment of the sibling invocation.
7. **No feature registration is owed**: `use_gtk` (configure.ac 3826), `have_gtk3` (3756),
   `have_pgtk` (3176) and `have_x_windows` (mirror-side) are all in `xtask/Cargo.toml`
   (212/232/252/681), and C 33's version gate needs none (see `## Decisions`).
8. **`docs/port-map.md`'s row for this file** still reads the 11-line stub's count (428); it is
   regenerated by `tools/gen-port-map.py`, which is outside this lane's write scope.
9. **The integrator's standing steps**: the tolerance row (5 - **done, 3.493**), a `docs/status.md`
   entry (**done, 3.493**) and
   `python3 tools/feature-coverage.py --write` (this unit adds sites for `use_gtk`; it registers no
   new feature name).  `python3 tools/check-port-lines.py` reads `53 file(s) under ten percent,
   69,407 reference line(s); 53 ratchet row(s), 0 new, 0 stale` with this unit at 507/210 lines, so
   the pending ratchet needs no row for it.

## Not claimed

- **Zero run evidence**: nothing in this file was ever executed - no `GtkFixed` subclass was
  registered, no widget created, no size hint set, no `XChangeProperty` issued.  The X11 and PGTK
  arms reach rustc at most, and on this machine they do not even get to an rlib (Findings 2-5).
- **GDK and GTK are unmeasured here** (Finding 1): every GDK/GTK record in the port is the mirror's
  placeholder, and this unit's own two records are UNVERIFIED, as are the offset of `priv` and the
  class record's member order.
- **The X11 arm's flip is not a target**: it is the frozen configuration's `#else` arm only in the
  sense that `HAVE_PGTK` is undef; `HAVE_X_WINDOWS` is undef too, which is what
  `include/systime.rs` 27 asserts.
- **The PGTK arm's 13 errors are not a claim about this unit**: they are `include/pgtkterm.rs`'s
  and `include/gtk.rs`'s outstanding names (Finding 7).
- **`C 175-179`'s commented-out lines and C's own comments** are carried as text; no claim is made
  that their content is right for any GTK version, and none of it is compiled.
