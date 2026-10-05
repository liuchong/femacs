# emacs/src/widget.c (514 lines)

> **P-86's class: the frozen configuration compiles nothing here.**  `widget.o` joins the build
> only when the toolkit is not `none` - `configure.ac` 7227-7229 (`WIDGET_OBJ=widget.o` and
> `AC_DEFINE ([USE_X_TOOLKIT], [1], …)` under `if test "${USE_X_TOOLKIT}" != "none"`) through
> `src/Makefile.in` 298/499's `$(WIDGET_OBJ)` - and `docs/baseline/frozen-build-sources.txt` lists
> no `widget.o`, so `extract-abi.py`'s checklist has no live item and this ledger is P-86 clause
> 2's substitute: the **C line-range table** below, the `## Evidence` readings and the
> `## Deviations`.  Port: `femacs/src/widget.rs` (**1,464 lines**).

## Decisions

- **Gate.**  C's text has no `#ifdef` around its body (C 26-514 is unconditional); the condition is
  the build's, so the file gate is `feature = "use_x_toolkit"` (`USE_X_TOOLKIT`, `configure.ac`
  7229's `AC_DEFINE`) on **every item and on every `include!` edge** (P-86 clauses 1 and 8).  The
  arm reading sets the file gate plus the two sub-gates the mirrors under it need
  (`have_x_windows` for `include/xterm.rs`/`include/xlib.rs`, `have_x11` for
  `include/widgetprv.rs`'s three records - P-86 clause 6).
- **`text-only` (P-86 clause 3(2)), with a measured Xlib half.**  This machine has **no Xt and no
  Xaw header at all**: `/opt/X11/include/X11` does not exist, and `/opt/homebrew/include/X11`
  carries `xorgproto` 2025.1's `<X11/Xlib.h>`, `<X11/Xutil.h>`, `<X11/Xresource.h>`,
  `<X11/Xregion.h>`, `<X11/X.h>` and `<X11/cursorfont.h>` but no `<X11/Intrinsic.h>`,
  `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`, `<X11/Shell.h>`, `<X11/ShellP.h>`, `<X11/ObjectP.h>` or
  `<X11/StringDefs.h>`.  So the Xlib records and the eleven macro values the file names are
  **clang-measured** (Evidence ③) while every Xt width, offset, enum value and resource-type
  string is C's spelling and **UNVERIFIED** (the `include/gtk.rs` `placeholder, not GDK's value`
  practice, status 3.481).
- **The X11/Xt surface stands in the unit** (P-88 clause 1): no header in the reference tree
  declares any of it, this file is its first consumer, and `include/xlib.rs` mirrors only the Xlib
  half.  The dedicated SDK mirror P-90 clause 1 asks for is owed (`## Owed` 1), and this unit's
  copy is the one to move then (P-88 clause 5 / P-90 clause 3).
- **The two widget mirrors do not carry C's records yet.**  `include/widget.rs` 154-157 spells
  `EmacsFrame`/`EmacsFrameClass` as `*mut c_void`, and `include/widgetprv.rs` 114-166 keeps the
  three records with the by-value `CorePart`/`CoreClassPart` replaced by `*mut c_void`.  A pointer
  where C has an inline part cannot express C 264-265/296-299/331-346's `ew->core.*` accesses or
  C 106-144's thirty-two-value `CoreClassPart` initializer, so the unit declares C's own **tags**
  (`_EmacsFrameRec`, `_EmacsFrameClassRec`, which no mirror claims) plus the two Xt parts, and
  converts at each use site (P-72 clause 3).  `## Owed` 2-4 carries the mirror fixes.
- **The four entry points `include/widget.rs` 159-164 declares at its top level** (C 146-155,
  320-324, 474-491, 507-514) are defined here.  P-39's fix is the mirror's `_decls` module
  (`include/emacsgtkfixed.rs` 49-56, status 3.496 ②⑺); the unit records it (`## Owed` 2) and the
  arm reading shows the four `E0428` in the mirror copy rather than working around them.
- **No feature name is new**: `use_x_toolkit` (`femacs/xtask/Cargo.toml` 703) and `have_x_windows`
  (260) are registered; `docs/baseline/feature-coverage.tsv` needs a regeneration (`## Owed` 9).

## C line ranges

Rows are ascending and cover every non-blank line of the reference.  **`tools/check-dead-file-coverage.py`
binds this ledger to `src/widget.h` as well** (`ledger_for` falls back to `<stem>.md`, and that
header's own ledger is `docs/ports/include-widget.md`), so C 35, 44, 53, 55 and 89 - blank in
`src/widget.c` but text in `src/widget.h` (`#ifndef XtNfont`, `#ifndef XtCForeground`,
`#define XtNvisualBell`, `#define XtCBellVolume`, `/* structures */`) - are folded into the rows
that surround them and marked `(wh)` below.  The blank lines between blocks that need no row:
18, 20, 25, 105, 145, 156, 165, 175, 185, 197, 209, 222-223, 271, 319, 325, 333, 355, 371,
380-381, 411, 417, 443, 472, 492, 505-506 (28 lines, the tool's own reading below).

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block | dropped (P-86 clause 1 / 0.1-19) |
| 19 | `/* Emacs 19 face widget ported by Fred Pierresteguy */` | carried, C's own text (the `// femacs: port of …` note follows it) |
| 21-24 | the "censored by the Communications Decency Act" banner | carried, C's own text |
| 26 | `#include <config.h>` | ungated (every unit includes it) |
| 27-35 | `widget.h`, `<stdlib.h>`, `lisp.h`, `sysstdio.h`, `xterm.h`, `frame.h`, and 35 (wh) | file gate (P-86 clause 8); `character.rs` joins at C 33 for `dispextern.h` 25's own include (`## Deviations` 13) |
| 36-44 | `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/cursorfont.h>`, `widgetprv.h`, `<X11/ObjectP.h>`, `<X11/Shell.h>`, `<X11/ShellP.h>`, `../lwlib/lwlib.h`, and 44 (wh) | file gate; the two Emacs-side headers are `include!`ed at their C positions, the five X11/Xt headers stand as the in-unit surface (P-88 clause 1) |
| 45-51 | the six `static` prototypes | dropped (PORTING.md 0.1-12 / P-91 clause 1); C's own text is carried as a comment at its position |
| 52-55 | 53 and 55 (wh), and C 54's `#define offset(field)` | file gate; `macro_rules! offset` (body: `offset_of!`, C's tag `_EmacsFrameRec`) |
| 56-86 | `static XtResource resources[]` | file gate; `static mut resources: [XtResource; 12]`, C's twelve rows |
| 87-89 | C 88's `#undef offset`, and 89 (wh) | recorded (Rust's `macro_rules!` has no `#undef` and the macro has exactly the twelve call sites above) |
| 90-104 | C's commented-out actions table and translations string | carried, C's own text |
| 106-144 | `static EmacsFrameClassRec emacsFrameClassRec = { { … } };` | file gate; `CoreClassPart`'s thirty-two columns, field-list form |
| 146-155 | `emacsFrameClass` | file gate; `#[unsafe(no_mangle)]` |
| 157-164 | `get_default_char_pixel_size` | file gate |
| 166-174 | `pixel_to_char_size` | file gate |
| 176-184 | `char_to_pixel_size` | file gate |
| 186-196 | `round_size_to_char` | file gate |
| 198-208 | `get_wm_shell` | file gate |
| 210-221 | `#if 0` block holding `mark_shell_size_user_specified` | dead text C itself excludes; carried as a comment, C's own lines |
| 224-270 | `set_frame_size` (C 227-260 is C's widget-hierarchy note) | file gate |
| 272-318 | `update_wm_hints` | file gate |
| 320-324 | `widget_update_wm_size_hints` | file gate; `#[unsafe(no_mangle)]` |
| 326-332 | `update_various_frame_slots` | file gate |
| 334-354 | `update_from_various_frame_slots` | file gate |
| 356-370 | `EmacsFrameInitialize` | file gate; `unsafe extern "C" fn` (the class record takes its address) |
| 372-379 | `resize_cb` | file gate; `unsafe extern "C" fn` (C 402 hands it to Xt) |
| 382-410 | `EmacsFrameRealize` | file gate; `unsafe extern "C" fn` |
| 412-416 | `EmacsFrameDestroy` | file gate; `unsafe extern "C" fn` |
| 418-442 | `EmacsFrameResize` | file gate; `unsafe extern "C" fn` |
| 444-471 | `EmacsFrameQueryGeometry` | file gate; `unsafe extern "C" fn` |
| 473 | `/* Special entry points */` | carried, C's own text |
| 474-491 | `EmacsFrameSetCharSize` | file gate; `#[unsafe(no_mangle)]` |
| 493-504 | `EmacsFrameExpose` | file gate; `unsafe extern "C" fn`; C 503's `flush_frame` is this unit's helper (`## Deviations` 12) |
| 507-514 | `widget_store_internal_border` | file gate; `#[unsafe(no_mangle)]` |

## Findings

1. **`XtNumber (resources)` is 12**, not 11: C 57-85's table has twelve rows (geometry, iconic,
   emacsFrame, minibuffer, unsplittable, internalBorderWidth, interline, foreground, cursorColor,
   barCursor, visualBell, bellVolume).  The port computes the count from the table's own type
   (`size_of::<[XtResource; 12]> () / size_of::<XtResource> ()`), which is C's `(sizeof (arr) /
   sizeof (arr[0]))` - a `static`'s initializer cannot name the static itself (`E0013`), so no
   `XtNumber!` macro can do it.
2. **The twelve `resource_offset` values are not C's.**  C's `offset (field)` is `offsetof
   (EmacsFrameRec, emacs_frame.field)`, i.e. `offsetof (EmacsFramePart, field)` plus the *size of
   the inline `CorePart`* - and this unit's `CorePart` carries only the four members C 264-265/
   296-299/331-346 read.  The offsets are therefore computed against the port's record, not C's;
   `## Owed` 1 and `## Not claimed` name the consequence.
3. **`sizeof wmshell->wm.wm_hints` is 56 and `sizeof (long)` is 8**, so C 317's `memcmp` length is
   48.  Measured on this machine's `XWMHints` (Evidence ③); the port writes `size_of_val
   (&(*wmshell).wm.wm_hints) - size_of::<c_long> ()`, which keeps C's two tokens.
4. **`include/frame.rs`'s `flush_frame!` cannot type-check in any unit**, twice over: its body
   assigns `FRAME_RIF! (f)` (`include/termhooks.rs` 425, whose `rif` the mirror degrades to
   `*mut c_void`) to `*mut redisplay_interface`, and then calls the interface member with a `*mut
   frame` while `include/dispextern.rs` 3717 declares that member as `*mut c_void` and keeps the
   record behind `dispextern_face_records`, a switch no configuration sets.  `src/androidterm.rs`
   130-145 measured the same first failure.  The unit writes C's own body (frame.h 1916-1919) as a
   unit-local `flush_frame` (`## Deviations` 12); C 503's text stays `flush_frame (f);`.
5. **`include/dispextern.rs` omits `dispextern.h` 25's `#include "character.h"`** (`MAX_CHAR`,
   `CHAR_VALID_P`, `CHARACTERP`, `STRING_CHAR`, `UNIBYTE_TO_CHAR`, `string_char_and_length`,
   `BYTES_BY_CHAR_HEAD`, `raw_prev_char_len` in its and `include/buffer.rs`'s ungated text - 12 of
   the arm's errors).  The unit adds the edge at C 33, the `src/dispnew.rs` 47 reading of the same
   C line (`## Deviations` 13), which removes all twelve.
6. **The four `E0428` the arm leaves are P-39's shape**: `include/widget.rs` 161-164 declares
   `emacsFrameClass`, `EmacsFrameSetCharSize`, `widget_store_internal_border` and
   `widget_update_wm_size_hints` at its top level, and this file defines them (C 146-155, 320-324,
   474-491, 507-514).  The fix is the mirror's `_decls` module (`include/emacsgtkfixed.rs` 49-56,
   status 3.496 ②⑺), and rustc attributes the four errors to the mirror's copy (`## Owed` 2).
7. **`include/xterm.rs` 774's `pub font: *mut font`** names a type that mirror does not reach
   (`xterm.h` names `struct font` as an incomplete tag; `include/font.rs` is the record's home).
   `src/xfns.rs` and `src/frame.rs` mask it by including `font.rs` themselves; this unit's C text
   has no `font.h`, so it reports the gap instead of adding a non-C edge (`## Owed` 5).
8. **`tools/check-dead-file-coverage.py` binds this ledger to `src/widget.h` too**, because
   `ledger_for` falls back to `<stem>.md` and the header's own ledger is the separate
   `docs/ports/include-widget.md`; five of that header's text lines (35, 44, 53, 55, 89) fall on
   line numbers blank in `src/widget.c`, so the table folds them into the surrounding rows
   (`(wh)` in `## C line ranges`).  Nothing about the `.c` port changes; the header is covered by
   its own ledger's checklist.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/widget-lane cargo run -q -p xtask -- build-tu src/widget.c`
→ `built src/widget.rs -> ../build/femacs/widget.o`, **exit 0**, and `build/femacs/widget.o` is
**336 bytes** - the empty object the file gate leaves (the reference compiles nothing from this
file either).  The materialized `build/femacs/units/widget/_root.rs` carries **0**
`// [xtask dedupe]` lines, i.e. no gated edge stole another file's expansion slot (P-89).

**② The arm (status 3.499's reading).**  `probe` was run **4** times (budget 3): the first three
were the offset/`flush_frame` fix, the `character.rs` edge (17 → 5 errors) and the check itself,
and the fourth re-ran the reading after the `wm_hints` token fix of Evidence ④ - the four readings
and this one are the same command, and the last two print the same five errors.  The unit's
revision is `femacs/src/widget.rs` md5 `32d1b85f2f860c067648ca5a3c73719e` (1,464 lines).

    cd femacs && CARGO_TARGET_DIR=/tmp/widget-lane cargo run -q -p xtask -- probe src/widget.rs \
      /tmp/widget-lane-probe.rlib --crate-type lib -C panic=abort \
      --cfg 'feature="use_x_toolkit"' --cfg 'feature="have_x_windows"' --cfg 'feature="have_x11"'

→ **exit 1, 5 errors, and not one of them is located in this unit**: **4 × `E0428`** whose primary
span is `include/widget.rs`'s copy 161-164, each with the note that names this file's definition
(`_root.rs` 997/1381/1457/…) - Finding 6 - and **1 × `E0425`** (`cannot find type font`) at
`include/xterm.rs`'s copy 774 - Finding 7.  **No error's primary span is `_root.rs`'s own text**: the only `_root.rs` locations
the log prints are the four `E0428`s' `previous definition of … here` notes and five
`unused doc comment` **warnings** on this unit's `unsafe extern "C"` blocks - the house warning
class `docs/ports/haiku.md`'s Evidence also reports; 11 warnings in all, none in a statement.
Before the
`character.rs` edge of Finding 5 the same command printed **17** errors (12 more, all in
`include/dispextern.rs`'s and `include/buffer.rs`'s copies); that reading is what measured the
omission.  The arm's `_root.rs` also has **0** dedupe lines.

**③ The measurements (clang, this machine's `xorgproto` 2025.1 under `/opt/homebrew/include`).**

    clang -I/opt/homebrew/include -o /tmp/wprobe.bin /tmp/wprobe.c && /tmp/wprobe.bin
    XSetWindowAttributes sizeof=112 event_mask=72 bit_gravity=32
    XExposeEvent sizeof=64 x=40 y=44 width=48 height=52 count=56
    XSizeHints sizeof=80 width_inc=40 height_inc=44 min_width=24 base_width=64
    XWMHints sizeof=56
    Window=8 Region=8 XID=8
    CWEventMask=2048 CWBitGravity=16 CWWidth=4 CWHeight=8 InputOutput=1
    CopyFromParent=0 NorthWestGravity=1 SubstructureNotifyMask=524288 PropertyChangeMask=4194304
    USSize=2 NULLQUARK=0
    PMinSize=16 PResizeInc=64 PBaseSize=256 PWinGravity=512

(The probe includes `<X11/Xlib.h>`, `<X11/Xutil.h>`, `<X11/Xresource.h>` and `<X11/Xregion.h>`;
`Pixel` is **not** in this machine's Xlib — it is Xt's `typedef unsigned long Pixel`, which is why
the unit substitutes `c_ulong`, the same width `include/widgetprv.rs` 139-142 already writes.)

**④ The inventory gate.**

    python3 tools/extract-abi.py --check emacs/src/widget.c femacs/src/widget.rs

→ `FAIL` with **one** problem line, `line count drifted 184.8%: C 514 vs Rust 1,464` - the
tolerance row `docs/ports/line-drift-tolerance.tsv` needs and this lane may not write
(`## Owed` 8).  The four other lines the first run printed are closed: `macro offset does not
mention emacsframerec`, `function EmacsFrameSetCharSize does not mention 0, 1` and `function
EmacsFrameExpose does not mention xexpose` are three rows this port added to
`docs/ports/macro-body-notes.tsv` (each with its C line and reason), and `function update_wm_hints
does not mention wm_hints` was a real defect of this port, fixed by keeping C's
`sizeof wmshell->wm.wm_hints` spelling (Finding 3).

**⑤ The other gates on the same worktree.**
`check-mirror-dupes` **17 duplicate / 17 ratchet / 0 failure** (no row names this unit);
`check-declarations` **7 sites (0 forbidden, 7 allowed literals) / 0 failures**;
`check-citations` **676 files, 0 drift, 0 range, 0 stale** (this unit's one `lisp.h 30` drift was
fixed in the same round); `check-citation-text` reports nothing for `femacs/src/widget.rs`;
`check-dead-file-coverage` binds this table with **0 failures**.

## Deviations

1. **`EmacsFrameRec`/`EmacsFrameClassRec` are declared under C's tags** (`_EmacsFrameRec`,
   `_EmacsFrameClassRec`) because `include/widget.rs` 154-155 spells `EmacsFrame` as `*mut
   c_void` and `include/widgetprv.rs` 150-158 keeps the typedef with `core`/`core_class` as
   pointers.  C's `EmacsFrame` (the mirror's pointer) is converted at each use site - `(*(ew as
   *mut _EmacsFrameRec))` - which is P-72 clause 3's shape; the mirror fix is `## Owed` 3-4.
2. **The Xt surface's values are placeholders or C's spelling, all UNVERIFIED**: `XtVersion` and
   `XtExposeNoCompress` carry `0` with the `placeholder, not Xt's value` marker;
   `XtGeometrySuccess`…`XtGeometryDone` are numbered `0`…`4` (C 470's ternary needs only
   distinct values); `Pixel`/`Boolean`/`Cardinal`/`Dimension`/`Position`/`String`/`XtPointer`/
   `XtArgVal`/`XtValueMask`/`XtGeometryMask`/`XtEnum`/`XtVersionType`/`Time` keep the widths Xt's
   documentation gives (and the two `include/widgetprv.rs` already substitutes); the seven `XtR*`
   and eight `XtN*`/`XtC*` strings are the names the X resource-manager convention gives those
   resources.  No Xt header was read.
3. **The two Xt parts are members-only.**  `CorePart` carries `width`, `height`,
   `background_pixel` and `border_pixel` (the members C 264-265/296-299/331-346 read or write), and
   `WMShellRec` carries `core` and `wm` (C's `CompositePart`/`ShellPart` sit between them and are
   not readable here).  Their offsets are therefore **not** Xt's - the `src/haiku.rs` 87-120
   reading of the same situation - and Finding 2 is the consequence for `resources[]`.
   `CoreClassPart`'s thirty-two member *names* and their order are C's own comments in the
   initializer (C 107-142), which is the only member-name evidence on this machine.
4. **C's implicit conversions are spelled**: `FRAME_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT`/`FRAME_TEXT_*`
   results narrowed to `Dimension` with `as Dimension` (C 182-183, 264-265, 341); `Dimension`
   widened to `c_int` in the arithmetic of C 296-299 and in C 352/428's `frame_size_history_extra`
   arguments; `CWWidth`/`CWHeight`/`CWEventMask`/`CWBitGravity` cast to `XtGeometryMask`/
   `XtValueMask` (C 396-397, 461, 466); `InputOutput` to `c_uint` and `CopyFromParent` to `*mut
   Visual` (C 398); `(XtArgVal) {…}` compound literals written as `as XtArgVal` (C 302-307);
   `(XtPointer) False`/`(XtPointer) 0`/`(XtPointer) 4` as `as XtPointer` (C 60-85); `false`/`true`
   for the `bool` parameters C spells `0`/`1` (C 487-488).
5. **`EmacsFrame`'s untyped pointer is converted at each dereference** (`include/widget.rs`
   154-155, P-55); C's `ew->emacs_frame.frame` opens every function and is written
   `(*(ew as *mut _EmacsFrameRec)).emacs_frame.frame`.
6. **`XEvent` is opaque in the mirror** (`include/xlib.rs` 1164), so C 499-501's `event->xexpose`
   member is reached through `(*(event as *mut XExposeEvent))`; `XExposeEvent` is the unit's
   measured record.
7. **`f->output_data.x` is a `*mut c_void` union member** (`include/frame.rs` 35-45), so C 338's
   `struct x_output *x = f->output_data.x` is written `(*f).output_data.x as *mut x_output` (the
   `src/frame.rs` 5473 / `src/emacsgtkfixed.rs` 399 shape).
8. **C's uninitialized locals are zeroed** (PORTING.md 1.5): `char_width`/`char_height` (C 190-191),
   the eight locals of `update_wm_hints` (C 275-284) and `ok_width`/`ok_height` (C 453); each first
   read follows the C call that fills it.
9. **`#if 0` and `#undef`**: C 210-221 is dead text C itself excludes and is carried as a comment
   with C's own lines; C 88's `#undef offset` has no Rust counterpart, and the macro has exactly
   the twelve call sites above.
10. **C 45-51's six `static` prototypes are dropped** (PORTING.md 0.1-12, P-91 clause 1) and C's
    text is carried as a comment; the six are defined at their own C positions, which is why
    `resize_cb` (C 378) may call `EmacsFrameResize` (C 418-442) before its definition.
11. **`update_wm_hints`'s `return memcmp (…)`** (C 316-317, an `int` in a `bool` return) is written
    `0 != memcmp (…)`.
12. **C 503's `flush_frame (f)` is a unit-local helper** rather than `include/frame.rs`'s
    `flush_frame!` (Finding 4).  C's own body is written (frame.h 1916-1919) with the one
    conversion the mirror's `*mut c_void` member needs, and the record the body reads is a
    members-only unit-side copy with `flush_display` asserted at offset 72 - the
    `src/androidterm.rs` 111-145 shape.
13. **`include/character.rs` is `include!`ed at C 33** for `dispextern.h` 25's own
    `#include "character.h"`, which `include/dispextern.rs`'s `include!` list omits (Finding 5).
    The `src/dispnew.rs` 47 line does exactly this; the alternative (patched mirror) is not this
    lane's to make.
14. **The Xlib names `include/xlib.rs` does not carry** are declared in this unit at their C
    headers' positions: `XSetWindowAttributes`, `XExposeEvent`, `XSizeHints` (+ its two aspect
    records), `Region`/`_XRegion`, `Time`, `SubstractionNotifyMask`… see ⑤ of Evidence for the
    measured values (`SubstructureNotifyMask`, `CWEventMask`, `CWBitGravity`, `CWWidth`,
    `CWHeight`, `InputOutput`, `NorthWestGravity`, `USSize`, `NULLQUARK`).  Their home is that
    mirror (`## Owed` 6), and `CopyFromParent` plus `PropertyChangeMask`/`StructureNotifyMask` come
    from its `x11_constants` module through `include/xterm.rs`'s own `use`, not a second copy.
15. **`NULL`** is declared in this unit (`*mut c_void`, the `lib/include/stddef.in.rs` 75-80
    spelling) because `<stddef.h>` is not in this file's C include list.

## Owed

1. **The Xt/Xaw SDK mirror** (P-90 clause 1): `femacs/src/include/<sdk>.rs` with the first line
   `// femacs: mirror of the <SDK>'s <header> …`, carrying `Widget`/`WidgetClass`/`WidgetRec`/
   `WidgetClassRec`/`Boolean`/`Cardinal`/`Dimension`/`Position`/`String`/`Pixel`/`XtPointer`/
   `XtArgVal`/`XtValueMask`/`XtGeometryMask`/`XtEnum`/`XtVersionType`/`ArgList`/`XtResource`/
   `XtWidgetGeometry`/`XtGeometryResult`/`CorePart`/`CoreClassPart`/the fourteen proc typedefs/
   `XtEventHandler`/`WMShellRec`/`WMShellPart`/`WMShellWidget` and the Xt entry points
   (`XtParent`, `XtIsWMShell`, `XtCreateWindow`, `XtAddRawEventHandler`, `XtVaSetValues`,
   `XtInheritSetValuesAlmost`, `XtInheritAcceptFocus`, `XtInheritDisplayAccelerator`), with every
   layout **measured** on an Xt toolchain and the real `XtVersion`/`XtExposeNoCompress`/`XtR*`/
   `XtN*` values read from `StringDefs.h`.  `src/xfns.rs`'s `USE_X_TOOLKIT` block, `src/xterm.rs`
   775 and the four `lwlib/` units will want the same surface; this unit's copy is the one to move
   and delete then (P-88 clause 5).
2. **`include/widget.rs`**: the four entry-point prototypes (159-164) into a `_decls` module with a
   glob import, P-39's shape (`include/emacsgtkfixed.rs` 49-56, status 3.496 ②⑺), so the four
   `E0428` of Findings 6 disappear; and the Xaw half's `EmacsFrame`/`EmacsFrameClass` aliases
   re-spelled as `*mut _EmacsFrameRec`/`*mut _EmacsFrameClassRec` once the records are C's.
3. **`include/widgetprv.rs`**: `EmacsFrameRec.core` and `EmacsFrameClassRec.core_class` spelled
   `CorePart`/`CoreClassPart` **by value** (the `lwlib/include/xlwmenuP.rs` 12-19 convention: keep
   C's spelling and let the including unit supply `CorePart`), and `EmacsFramePart`'s
   `Boolean`/`Pixel` widths confirmed against an X11 header.  With 1-3 in place, this unit's tags,
   its `CorePart`/`CoreClassPart` and the two `*mut _EmacsFrameRec` conversions all go away.
4. **`docs/baseline/layout-excluded-mirrors.txt`** needs the `widgetprv.rs` row that mirror's own
   ledger already asks for, if its three records are to be measured rather than substituted.
5. **`include/xterm.rs` 774**: `pub font: *mut font` must either reach `include/font.rs` through
   its own `include!` list or degrade to `*mut c_void` with C's `/// C:` line (P-72 clause 3) - the
   record is `xterm.h`'s and `C` leaves the tag incomplete.  This unit has no `font.h` in its C
   include list, so `src/xfns.rs`/`src/frame.rs`'s workaround is not C's own shape.
6. **`include/xlib.rs`**: `XSetWindowAttributes`, `XExposeEvent`, `XSizeHints` (+ `XSizeHints`'s
   two aspect records), `Region`/`_XRegion`, `Time`, `SubstructureNotifyMask`, `CWEventMask`,
   `CWBitGravity`, `CWWidth`, `CWHeight`, `InputOutput`, `NorthWestGravity`, `USize`… the
   constants and records of Deviations 14 belong in that mirror (P-72 clause 1), measured where
   this unit has them measured.
7. **`include/dispextern.rs`**: the `character.rs` edge for `dispextern.h` 25, and the two halves
   of Finding 4 - `redisplay_interface`'s gate (`dispextern_face_records` is set by no
   configuration) and its `flush_display` member's `*mut c_void` against `flush_frame!`'s `*mut
   frame` argument.  `include/frame.rs`'s `flush_frame!` comment should then name the two
   constraints it cannot satisfy.
8. **`docs/ports/line-drift-tolerance.tsv`**: the `widget.c` row (`C 514 vs Rust 1,464`,
   184.8 %) - Evidence ④; the file is not this lane's to write.
9. **`docs/baseline/feature-coverage.tsv`** needs `python3 tools/feature-coverage.py --write`
   (this unit's `use_x_toolkit`/`have_x_windows` sites are not in the current table), and
   **`docs/port-map.md`**'s `src/widget.c` row needs `tools/gen-port-map.py` (it still reads 18
   port lines).
10. **`docs/ports/macro-body-notes.tsv`** already carries this unit's three rows (Evidence ④);
    no further row is owed for it.

## Not claimed

- **Zero run evidence**: nothing was linked and nothing ran.  There is no X toolkit on this machine
  and no Xt header to compile the arm against, so no widget, frame, size hint or event was ever
  exercised.
- **Every Xt width, offset, enum value and resource-type string in the unit is C's spelling, not a
  measurement** (Deviations 2-3): `XtVersion`/`XtExposeNoCompress` are placeholders, the
  `XtGeometryResult` numbers are placeholders, `CorePart`/`CoreClassPart`/`WMShellRec` carry only
  the members this file names, and `resources[]`'s twelve `resource_offset` values are computed
  against the port's record (Finding 2) - they are **not** C's byte offsets.
- The Xlib half is measured (Evidence ③) but only for **this** machine's `xorgproto` 2025.1; a
  different Xlib could lay the three records out differently, and no assertion is written here
  because `xtask layout-asserts` has no baseline for them.
- **The arm's five remaining errors are not this unit's** (Findings 6-7): they live in
  `include/widget.rs`'s and `include/xterm.rs`'s copies, and the `E0428` four are this unit's own
  definitions meeting the mirror's prototypes, which is P-39's defect rather than a silent success.
- The frozen object is the 336-byte empty one, so it says nothing about the body - the arm reading
  of Evidence ② is the only compile evidence, and **it does not run the code**.
