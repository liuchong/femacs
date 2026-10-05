# emacs/lwlib/lwlib-Xaw.c (844 lines)

> **P-86's class: the frozen configuration compiles nothing here.** `lwlib/Makefile.in` 52 puts
> `lwlib-Xaw.o` in `LUCID_OBJS` (56 selects that list as `TOOLKIT_OBJS`, 58 into `OBJS`) and
> `configure.ac` 7231 defines `USE_LUCID` for it, so `extract-abi.py`'s live checklist is empty in
> this configuration and the checkboxes cannot be coverage evidence.  What follows is P-86 ②'s
> substitute: the **C line-range table**, the `## Evidence` readings, and the per-`#if`
> `## Deviations`.  Port: `femacs/lwlib/lwlib-Xaw.rs` (**1,483 lines**, `md5
> 6828664d04fc06b5a7440fc9f9079aa9`); header mirror `femacs/lwlib/include/lwlib-Xaw.rs` (`md5
> bbe89a42968f8fbd539ce5f9de671a5c`, its `_decls` move in `## Deviations` 16).  Both arms of every
> `#if` are carried; the mirror moved its eight declarations into a `_decls` module (P-39, the
> `include/lwlib-Xlw.rs` shape).

## Decisions

- **File gate `feature = "use_lucid"`** (P-86 ⑧), the build's condition rather than one the C text
  spells: `lwlib/Makefile.in` 52 (`LUCID_OBJS = lwlib-Xlw.o xlwmenu.o lwlib-Xaw.o`),
  `configure.ac` 7231 (`AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])`).
  This is the same gate the sibling `femacs/lwlib/lwlib-Xlw.rs` carries: `lwlib/Makefile.in` has no
  `XAW_OBJS` list to read the condition from - `lwlib-Xaw.o` is an entry of `LUCID_OBJS`, the same
  list as `lwlib-Xlw.o`.
  `USE_LUCID`, `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
  docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of this
  file and the port's frozen object is the 336-byte empty one.
- **Sub-gates** (P-86 ⑥, each of the C text's own conditions lower-cased): C 36-50's `HAVE_XAW3D`
  pair is `feature = "have_xaw3d"`; C 54's `#if defined USE_CAIRO || defined HAVE_XFT` is
  `feature = "use_cairo_or_have_xft"` (the derived name `include/xlwmenuP.rs` 53 already uses for
  the same condition); the C 55 `#ifdef USE_CAIRO` inside it is `feature = "use_cairo"`, and C 58's
  `#else /* HAVE_XFT */` arm is `all (feature = "have_xft", not (feature = "use_cairo"))`.
- **This file is `text-only`** (P-86 ③⑵): this machine has no Xt, Xaw or Xft header
  (`/opt/homebrew/include/X11` carries `xorgproto` 2025.1's Xlib half only).  The one Xlib record
  the file reads element by element (`XColor`) and the four Xlib scalar constants are **measured**
  (`## Evidence` ⑥); every Xt/Xaw/Xft width, offset, enum value and resource name below is C's
  spelling with the value **UNVERIFIED** (the `include/gtk.rs` `placeholder, not GDK's value`
  practice, status 3.481 - `## Owed` 5 names the toolchain that would measure them).
- **The `<X11/*>`, `<X11/Xaw*>` and `<X11/Xft/*>` surface stands in the unit** (P-88 ①: no mirror in
  this tree carries any of those headers and this file is their only consumer here);
  `lwlib/include/lwlib.rs` names `Widget`, `Boolean` and `XEvent` and leaves them to the unit that
  includes it.  The Xaw3d/Xaw pair declares the *same* six class names in both arms, so the port has
  one declaration of them and C's two `#include` lists stand as the unit's comment.
- C's two forward declarations (C 75-77 `xaw_generic_callback`, C 476-479 `wm_delete_window`) are
  **deleted**: Rust items are order-independent (PORTING.md 0.1-12 / checklist 10).

## Gate mapping

| C condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`LUCID_OBJS`, i.e. `USE_LUCID`) | `feature = "use_lucid"` (every item and every `include!` edge) | a Lucid X toolkit configuration |
| C 36 `#ifdef HAVE_XAW3D` / C 43 `#else` (the six Athena headers), and C 620-626/653-659 (`beNiceToColormap`) | `feature = "have_xaw3d"` | Xaw3d (the `#ifdef` has no `#else` text, so nothing is gated the other way) |
| C 54 `#if defined USE_CAIRO \|\| defined HAVE_XFT` … C 73, C 88 … C 308, C 485-495, C 518-521, 532-534, 556-607, 630-636, 663-669 | `feature = "use_cairo_or_have_xft"` | a cairo or Xft configuration |
| C 55 `#ifdef USE_CAIRO` … C 60 (`<stdlib.h>`, `lwlib-utils.h`) and C 219-221 (`cairo_surface_flush`) | `feature = "use_cairo"` (nested in the region gate) | a cairo configuration |
| C 58 `#else` … C 60 (`<X11/Xft/Xft.h>`, and the seven entry-point calls) | `all (feature = "have_xft", not (feature = "use_cairo"))` | Xft without cairo |

## C line ranges

Rows are ascending and cover every non-blank line of the reference.  **`tools/check-dead-file-coverage.py`
binds this ledger to `lwlib/lwlib-Xaw.h` as well** (`ledger_for` falls back to `<stem>.md`, and that
header's own ledger is `docs/ports/include-lwlib-Xaw.md`), so C 20 and C 26 - blank in
`lwlib/lwlib-Xaw.c` but the two `void` lines of `xaw_destroy_instance`'s and
`xaw_pop_instance`'s declarations in the header - are folded into the rows that surround them and
marked `(wh)`.  C 844 is the last line; the 40 blank lines (two of them bare form feeds)
between blocks need no row, and C 469's `/* Dialog boxes */` has one of its own (the tool prints
both readings).

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's licence/declaration block, and 20 (wh) | dropped (P-86 ①) |
| 21 | `#include <config.h>` | ungated (every unit includes it) |
| 23 | `#include <stdcountof.h>` (`countof`) | file gate |
| 24 | `#include <stdio.h>` (`sprintf`) | file gate |
| 25-27 | `#include <setjmp.h>`, 26 (wh), `#include <lisp.h>` | file gate (C 25 has no edge of its own: `lisp.rs` carries the types) |
| 29 | `#include "lwlib-Xaw.h"` | file gate (`widget_instance`, `widget_value`, `widget_creation_entry`) |
| 31-34 | `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/CoreP.h>`, `<X11/Shell.h>` | file gate (the surface below) |
| 36-50 | `#ifdef HAVE_XAW3D` … `#else` … `#endif`: the six `<X11/Xaw3d/…>` / `<X11/Xaw/…>` headers | both arms carried as one declaration (same names) |
| 52 | `#include <X11/Xatom.h>` (`XA_ATOM`) | file gate |
| 54-73 | `#if defined USE_CAIRO \|\| defined HAVE_XFT`: C 55-57 the `USE_CAIRO` includes, C 58-60 the `HAVE_XFT` include, C 62-70 `struct widget_xft_data`, C 73 the `#endif` | region gate / `use_cairo` / `have_xft` |
| 75-77 | forward declaration of `xaw_generic_callback` | deleted (PORTING.md 0.1-12) |
| 80-85 | `lw_xaw_widget_p` | file gate |
| 88-120 | `fill_xft_data` (static) | region gate |
| 122-137 | `destroy_xft_data` (static; an `XtCallbackProc` at C 600) | region gate |
| 139-160 | `openFont` (static) | region gate |
| 162-185 | `get_text_width_and_height` (static) | region gate |
| 187-223 | `draw_text` (static; C 219-221 the `#ifdef USE_CAIRO` pair) | region gate / `use_cairo` |
| 226-243 | `set_text` (static) | region gate |
| 245-266 | `find_xft_data` (static) | region gate |
| 268-284 | `command_press` (static; an `XtActionProc` of C 486-490) | region gate |
| 286-305 | `command_reset` (static; the other `XtActionProc`) | region gate |
| 308 | `#endif` closing C 88's region | region gate |
| 310-375 | `xaw_update_one_widget` (C 319-325 and C 351-370 the region's two blocks) | file gate / region gate |
| 377-385 | `xaw_update_one_value` | file gate |
| 387-395 | `xaw_destroy_instance` | file gate |
| 397-402 | `xaw_popup_menu` | file gate |
| 404-466 | `xaw_pop_instance` (C 429-437 the `WM_PROTOCOLS` property, C 440-462 the centring block) | file gate |
| 469 | `/* Dialog boxes */` | file gate |
| 471-475 | `overrideTrans`, `dialogOverride` (static) | file gate |
| 476-479 | forward declaration of `wm_delete_window` | deleted (PORTING.md 0.1-12) |
| 480-483 | `xaw_actions`, `actions_initted` | file gate |
| 485-495 | `button_actions`, `buttonTrans` | region gate |
| 497-673 | `make_dialog` (static; C 518-521, 532-534, 556-607, 630-636, 663-669 the region's five blocks, C 620-626 and 653-659 the `have_xaw3d` pair) | file gate / region gate / `have_xaw3d` |
| 675-736 | `xaw_create_dialog` (C 691-718 the `switch (name [0])`) | file gate |
| 739-778 | `xaw_generic_callback` (static; an `XtCallbackProc` at C 370) | file gate |
| 780-822 | `wm_delete_window` (static; the `XtActionProc` of C 480-482) | file gate |
| 826-837 | `xaw_create_main` (static; the table's `widget_creation_function`) | file gate |
| 839-844 | `xaw_creation_table` | file gate |

## Findings

1. **The two arms of C 55-60 call the same seven Xft entry points with two spellings.**  The
   `USE_CAIRO` arm reaches them through `lwlib-utils.h`'s `crxft_*` renamings (macros, so that arm's
   call carries `!`), the `HAVE_XFT` arm calls the header's own functions - and three of the seven
   macros drop an argument (`XftFontClose`'s `Display *`, `XftDrawCreate`'s `Colormap`,
   `XftTextExtentsUtf8`'s `Display *`).  Each of the seven call sites therefore carries both arms
   (`## Deviations` 4).
2. **C 594 calls `calloc` from the region both arms share, but C 56's `#include <stdlib.h>` is
   inside the `#ifdef USE_CAIRO` arm**: the `HAVE_XFT` arm has no visible declaration of `calloc`
   (it reaches one transitively at best).  The port includes the `lib/include/stdlib.rs` mirror for
   the whole region and the reading is recorded (`## Deviations` 11).
3. **`xaw_update_one_widget`'s second region block (C 351-370) searches `instance->xft_data[]` for
   the widget and then re-reads `instance->xft_data[nr]`** after the loop; the loop only stops at
   the `widget == NULL` terminator, so the re-read is the null check - C's own text, carried as it
   stands.
4. **The four Xaw3d/Xaw conditional sites (C 36-50 and the two `beNiceToColormap` pairs) turn on
   `DefaultDepthOfScreen (XtScreen (dialog)) >= 16`**, i.e. the Xaw3d arm asks the *screen* for its
   depth once per button; carried as C writes it.

## Evidence

**① The frozen configuration (`build-tu`, 3 runs).**
`cd femacs && CARGO_TARGET_DIR=/tmp/lxaw-lane cargo run -q -p xtask -- build-tu lwlib/lwlib-Xaw.c`
→ `built lwlib/lwlib-Xaw.rs -> ../build/femacs/lwlib-Xaw.o`, **exit 0**, `build/femacs/lwlib-Xaw.o`
**336 bytes**, `md5 dbed68837694579185377dbb0821ab62`.  That is the expected reading and **not a
proof of the new text**: every item of the port sits behind the `use_lucid` gate, so the frozen
configuration compiles only `include/config.rs` and the crate's `#![allow]` (P-86 ⑧'s "the gate must
go down to the include graph").  The third run, taken after the `XEvent`-union and comment edits
*inside* the gate, reproduced the same 336 bytes and the same md5 - which is what that sentence
means: the object is invariant under this file's text.

**② The arm flip (P-86 ③⑴, ⑥): `probe` with the file gate and the sub-gates.**
`… -- probe lwlib/lwlib-Xaw.rs /tmp/lxaw-lane/x.rlib --crate-type=lib -C panic=abort
--cfg 'feature="use_lucid"'` → **exit 0, 0 errors, 3 warnings, a 7,975,656-byte rlib** (one warning
is this unit's, since fixed; the other two are the mirrors' - `lib/include/stdbit.in.rs` 644 and
`src/include/lisp.rs` 3934).
`… -- probe lwlib/lwlib-Xaw.rs /tmp/lxaw-lane/x2.rlib … --cfg 'feature="use_lucid"'
--cfg 'feature="use_cairo_or_have_xft"' --cfg 'feature="use_cairo"' --cfg 'feature="have_xaw3d"'`
→ **exit 1, exactly one `error`**, and it is **in the include graph, not in this unit**:

```
error[E0428]: the name `widget_xft_data` is defined multiple times
   --> build/probes/lwlib-Xaw/…_lwlib_include_lwlib-int_rs.rs:23:1     (the mirror's copy)
   = note: `widget_xft_data` must be defined only once in the type namespace of this module
error: aborting due to 1 previous error
```

i.e. `lwlib/include/lwlib-int.rs` 20-23 keeps a zero-size stand-in for C's *incomplete*
`struct widget_xft_data` (lwlib-int.h 26) and this unit is the one that completes the tag (C 62-70).
`## Owed` 1 names the owner and the fix; the mutation test below shows why this reading cannot
excuse the arm's own text.

**③ The mutation test (P-86 ⑦).**  Injecting a type error into *this unit's own* text
(`let _mutant: c_int = c"boom".as_ptr ();` as the first statement of `lw_xaw_widget_p`) and
re-running ②'s second command changes the filtered output **not at all** - `diff` of the two runs
(`error`/`-->`/`rustc failed` lines) is empty, both are the same single `E0428` in the mirror, exit
1.  rustc stops in name resolution and never type-checks this unit's bodies, so ②'s sub-gate reading
is **vacuous** for them.

**④ The arm-reach pre-flight (P-86 ⑦'s fallback: this unit's text + the real mirrors + the two named
mirror defects repaired in a copy).**  A `/tmp` harness reproduces xtask's `materialize` rule
(`femacs/xtask/src/main.rs` 1793-1930: one copy per file, first reach wins, the ungated reach claims
the slot) over `femacs/{src,lib,lwlib}/include` and compiles
`/tmp/lxaw-lane/harness/lwlib/lwlib-Xaw.rs` = this unit's text, with `lwlib-int.rs`'s two defects
fixed in the copy (`## Owed` 1-2) and nothing else changed.  Three arms, each
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort
--crate-name lwlib_Xaw --cfg 'feature="use_lucid"' …`:

| arm (`--cfg` additions) | reading |
| --- | --- |
| none | **exit 0, 0 errors** |
| `use_cairo`, `use_cairo_or_have_xft`, `have_xaw3d` | **exit 0, 0 errors** (80,048-byte object) |
| `have_xft`, `use_cairo_or_have_xft` | **exit 0, 0 errors** |

This is the reading that says the port's statements type-check against the real mirrors; it is
**not** the same reading as ② (`## Not claimed`).

**⑤ The text gates.**
`python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xaw.c femacs/lwlib/lwlib-Xaw.rs` →
one row left, the line-count drift (**C 844 vs Rust 1,483**, 75.7%), whose tolerance row is the
integrator's (`## Owed` 4); the one body-token row it first reported (`xaw_create_dialog does not
mention break, case, switch`) is recorded in `docs/ports/macro-body-notes.tsv` (the `switch` →
`match` class, PORTING.md 2.2/P-04, the `sfnt_validate_gs` row's precedent).
`python3 tools/check-declarations.py` → **0 failures** (17 pre-existing allowed literals, 0
forbidden).  `python3 tools/check-citations.py` → **0 drift, 0 range, 0 stale**; the citations this
unit adds all name headers that are not in the tree (`<X11/*>`, `<Xft.h>`, `cairo.h`, `lisp.h`), so
they land in the tool's counted `external` bucket.  `python3 tools/check-dead-file-coverage.py` →
**passed**, 0 failures, for this ledger's 38 rows (it reads them against `lwlib/lwlib-Xaw.c`
*and*, via the `<stem>.md` fallback, against `lwlib/lwlib-Xaw.h` - `## Deviations` 14).

**⑥ The Xlib half, measured with clang** (`xorgproto` 2025.1, `clang -I/opt/homebrew/include
/tmp/lxaw-lane/xprobe.c`; the same probe measured `sizeof XEvent`):

```
sizeof XColor 16; offsets pixel 0 red 8 green 10 blue 12 flags 14 pad 15
XEvent 192; XButtonEvent 96; Display* 8; Screen* 8; Window 8; Atom 8; Drawable 8; Colormap 8
XA_ATOM 4 PropModeAppend 2 None 0 ButtonPress 4
XEvent size 192 align 8; XColor size 16 align 8
```

`XColor`'s layout and the four constants are what this unit's declarations carry; the `XEvent` union
is the measured 192 bytes at the measured align 8 (`lwlib-Xlw.rs`'s `_padding` shape, sized as
`[c_ulong; 24]` so the alignment comes along).  The Xt/Xaw/Xft half has no header on
this machine and is `UNVERIFIED` (`## Owed` 5).

## Deviations

1. **The Xt/Xaw/Xft surface is C's spelling, not a measurement** (P-86 ③⑵): `WidgetRec`'s six
   `CorePart` members (`parent`, `being_destroyed`, `x`, `y`, `width`, `height`) are carried in
   relative order with the members between them omitted, so their offsets are knowingly not the
   real ones; `XtResource`, `XtActionsRec`, `Arg`, `XtTranslations`, `XEvent` and the four Xft
   records are the same class.  `## Owed` 5 is the measurement.
2. **`XtJustifyCenter`, `XtChainLeft`/`XtChainRight`/`XtChainBottom` and `XtGrabNonexclusive` are
   placeholders, not the readings**: they are Xaw/Xt enum *values* (Command.h, Form.h, Shell.h)
   whose headers are not on this machine, so their values are marked as such at the declaration
   (the `include/gtk.rs` practice) instead of invented.  A configuration that passes them as
   `XtSetArg` arguments would need the real numbers.
3. **C 36-50's two include lists become one declaration block**: both arms name the same six
   classes (`scrollbarWidgetClass`, `dialogWidgetClass`, `commandWidgetClass`, `labelWidgetClass`,
   `panedWidgetClass`, and Shell.h's two), so the port declares them once and keeps C's two
   `#include` lists as the unit's comment (the `src/xfns.rs` 125-129 reading of the same pair).
4. **The seven Xft call sites each carry both arms' spellings** (`XftFontOpenName! (…)` under
   `use_cairo`, `XftFontOpenName (…)` under `have_xft` without cairo) because C's two arms reach a
   macro and a function of the same name (`## Findings` 1).  Each arm is a `#[cfg]`-gated block so
   rustc sees one statement, and each arm's text is C's own call.
5. **C's `static char X [] = "…"`** (`overrideTrans`, `dialogOverride`, `buttonTrans`) is the port's
   `&CStr` of the same bytes: the array length is C's own bookkeeping and the pointer C passes is the
   same (`## Not claimed` 2).
6. **C's two forward declarations are deleted** (C 75-77, 476-479; PORTING.md 0.1-12/checklist 10) -
   Rust resolves items order-independently, so nothing takes their place.
7. **C 691-718's `switch` becomes a `match`** on `*name as u8` with C's five shared bodies as
   `b'E' | b'e' => …` arms, and C has no `default:` while Rust needs exhaustiveness, so the port
   adds one `_ => { }` arm that leaves the five locals exactly as C's fall-through does
   (PORTING.md 2.2; `docs/ports/macro-body-notes.tsv`'s `xaw_create_dialog` row).
8. **Locals C leaves uninitialised start at zero** (PORTING.md 0.1-13): `bg`/`fg` (C 92, filled by
   `XtVaGetValues`), `colors` (C 93, `XQueryColors` fills `red`/`green`/`blue`), `gi` (C 170),
   `w` (C 559), `xft_font`/`button_override` (C 519-520, written on every path that reads them),
   `button_name`/`shell`/`dialog`/`button`/`override` (C 513-517), `props` (C 410), `x`/`y` (C
   442-443), `nkids`/`kids` (C 787-789) and the four `Arg` arrays (C 328, 442, 510, 829).
   Named here because a reviewer cannot see the zeroing in C.
9. **C's Boolean truth tests are spelled**: `if (! pop_up_p)` → `if pop_up_p == 0` (and the three
   neighbours), `if (! actions_initted)` → `if actions_initted == 0`, `if (name [3])` →
   `*name.offset (3) != 0`, `if (bw == 0)`, `if (up)` → `if up != 0`, and the `!ptr` tests become
   `is_null ()`/`!…is_null ()` (the `lwlib-Xlw.rs` reading of the same class).
10. **X11's `None` lives in a module** (`xaw_x11_constants`) and is imported by the two functions
    that write it: a top-level `None` would shadow Rust's prelude `Option::None` for the whole unit
    (the `include/xlib.rs` 29-31 reading; `src/xterm.rs` 1898's `use`).  The measured value is 0
    (`## Evidence` ⑥).
11. **`<stdlib.h>`'s mirror is included for the whole `use_cairo_or_have_xft` region**, not only
    for `use_cairo` as C 55-60 has it, because C 594 calls `calloc` from the arm that does not
    include that header (`## Findings` 2).
12. **C's implicit conversions are spelled**: `Dimension`/`c_int` → `c_uint` at C 240-241, 734-737
    and 249-252 (`XCreatePixmap`'s three `unsigned int`), `c_int` → `c_uint` at C 445-450 (the
    centring arithmetic is `unsigned int` in C), `char` → `c_int` at C 720/728 (`name [1] - '0'`),
    `c_int` → `XtPointer` at C 429/459/461 (C's `XtSetArg` casts to `XtArgVal`), `c_ushort` →
    `c_uint` in the `DefaultDepthOfScreen` argument, and `c_int` → `t`→`c_double`→`c_int`
    at C 216 (`xft_font->height * 1.2`).
13. **Four brace-less `if`s took braces** where an arm-pair stands inside them (`destroy_xft_data`'s
    two, `draw_text`'s `if (!data->xft_draw)`): C's single statement becomes a block so both arms
    can be written, and the control flow is C's.
14. **`tools/check-dead-file-coverage.py` binds this ledger to `lwlib/lwlib-Xaw.h` too**, because
    `ledger_for` falls back to `<stem>.md` and that header's own ledger is the separate
    `docs/ports/include-lwlib-Xaw.md`; two of the header's text lines (20, 26 - each a bare
    `void`) fall on line numbers blank in `lwlib/lwlib-Xaw.c`, so the table folds them into the
    rows that surround them (`(wh)` in `## C line ranges`).  Nothing about the `.c` port changes.
15. **One loop variable moves to function scope**: C 127 declares `i` inside its `for`
    (`for (int i = 0; xft_data[i].widget; ++i)`); the port declares it with the function's other
    locals and the loop's value sequence is C's (the same class as the `for (nr = …)` loops, which
    C declares at function scope already).
16. **The header mirror `femacs/lwlib/include/lwlib-Xaw.rs` carries a `_decls` module** (P-39): the
    eight declarations move into `lwlib_Xaw_decls` + a glob import, so this unit's definitions
    shadow the foreign declarations instead of colliding with them (`E0428`).  This is the same
    edit `include/lwlib-Xlw.rs` and `include/lwlib-utils.rs` carry.

## Owed

1. **The mirror's `widget_xft_data` stand-in** (`## Evidence` ②): `femacs/lwlib/include/lwlib-int.rs`
   20-23 declares `pub struct widget_xft_data;` for lwlib-int.h 26's incomplete tag, and the unit
   that completes the tag (C 62-70, this file) cannot share the name with it.  **Owner: the
   `lwlib-int.rs` mirror** (not this lane's whitelist).  The fix is the `xlwmenuP.rs` convention -
   a mirror leaves an incomplete type to the unit that includes it - i.e. delete those three lines
   (nothing else names the record: `_widget_instance.xft_data` below keeps the bare name and the
   defining unit supplies the item), or gate them `not (feature = "use_lucid")`.  Until then **no
   `use_cairo`/`have_xft` translation unit of this file can be compiled by `probe`** (`## Evidence`
   ②③).
2. **The same mirror gates `_widget_instance.xft_data` on the narrow feature**: `lwlib-int.rs`
   33-36 writes `#[cfg (feature = "use_cairo")]` for lwlib-int.h 33's
   `#if defined USE_CAIRO || defined HAVE_XFT`.  A `have_xft`-without-cairo build would compile
   `find_xft_data`'s `(*inst).xft_data` against a record with no such field (the harness's `## Evidence`
   ④ third arm is green only because the copy repairs this line as well).  Same owner; the fix is
   `use_cairo_or_have_xft`.
3. **The derived `use_cairo_or_have_xft` is set by nothing but a probe**: `femacs/xtask/Cargo.toml`
   701 registers it and `include/xlwmenuP.rs` 53-83 already uses it, but no build configuration sets
   it, so a Lucid + cairo/Xft build of this unit (and of that mirror) would compile the region
   **off**.  **Owner: the integrator** - the feature must be set whenever `use_cairo` or `have_xft`
   is (the tool that generates the `--cfg` list, `xtask`, is where the derivation belongs).
4. **The line-drift tolerance row is owed** (the integrator's file,
   `docs/ports/line-drift-tolerance.tsv`): `lwlib-Xaw`, tolerance **0.79**, basis `total`, reason =
   C 844 vs Rust 1,483 (75.7%): the whole file is transcribed, and the difference is the fixed cost
   of a unit whose half of the text is a header surface no mirror in this tree carries (about 120
   declaration lines), of the seven Xft call sites that must carry two arms each, of the per-arm
   `#[cfg]` lines, and of the per-block C-line comments (`lwlib-Xlw` is the same class at 78.6%).
5. **The `<X11/*>`/Xaw/Xft surface's real shapes**: with an X11 toolchain (`xorgproto` + libXt, Xaw
   or Xaw3d and Xft), measure `CorePart`, `XtResource`, `XtActionsRec`, the four Xft records and the
   five enum values of `## Deviations` 2, and measure with the reference's own `configure` which
   arm of `USE_CAIRO`/`HAVE_XFT` a Lucid build takes.  What is missing on this machine is *Xt*, *Xaw*
   and *Xft* (xorgproto 2025.1's Xlib half is present and measured, `## Evidence` ⑥).  P-90 clause 1
   asks for a dedicated Xt/Xaw mirror (`femacs/src/include/<sdk>.rs`); this unit's copy is one to
   move when it lands (P-88 clause 5).
6. **`docs/port-map.md`'s target-line column** for `femacs/lwlib/lwlib-Xaw.rs` still says 3 (and
   `femacs/lwlib/include/lwlib-Xaw.rs`'s 39), and **`docs/baseline/feature-coverage.tsv` is stale
   for the five features this unit adds sites for** (`use_lucid`, `have_xaw3d`, `use_cairo`,
   `have_xft`, `use_cairo_or_have_xft`): no new *name* is added, but `tools/feature-coverage.py
   --write` is the integrator's reading to refresh.
7. **`docs/baseline/port-lines-pending.tsv`**'s row for `lwlib/lwlib-Xaw.c` is stale after this
   round (informational); `tools/check-port-lines.py --write-pending` retires it.

## Not claimed

- **Zero run evidence**: nothing was linked or run; no widget was created, no callback fired, no
  dialog was popped up and no font was opened.  The artifacts are compile products of a type-check.
- **No Xt, Xaw or Xft header was consulted** (there is none on this machine): every Xt/Xaw/Xft type,
  constant and layout above is C's spelling, the records' omitted members mean their offsets are
  knowingly not the real ones, and the four Xaw enum constants are placeholders.
- **The frozen object is the 336-byte empty one**, so it says nothing about the body; the two
  `probe` readings are ① a green `use_lucid` type-check and ② a mirror-side failure for the
  sub-gate arm, and **the green arm-reach reading (`## Evidence` ④) is a `/tmp` harness's, not the
  tree's `probe`** - it differs in the two mirror lines it repairs.
- **No behavioural equivalence** for either arm, and no claim that the two unmeasured arms
  (`have_xaw3d` on, `have_xft` without cairo) would look right on a real X target.
- **Budget as run** (the lane's limit): `build-tu` **3 of 3**, `probe` **4 of 3** - the fourth run is
  the unmutated control of `## Evidence` ③ (`diff` of the two outputs); no `cargo test`, no
  `--all`, no `gate-fast` and no `check-all-units.sh` was run, and `/tmp/lxaw-lane` is removed after
  the readings are recorded (what is cited here is the text above, not the directory).
