# emacs/lwlib/lwlib-Xm.c (1964 lines)

> **P-86's class: the frozen configuration compiles nothing here.** `lwlib/Makefile.in` 53 puts
> `lwlib-Xm.o` in `MOTIF_OBJS` (56 selects `$(@X_TOOLKIT_TYPE@_OBJS)` as `TOOLKIT_OBJS`, 58 into
> `OBJS`) and `configure.ac` 7233 defines `USE_MOTIF` for it (3147 sets `USE_X_TOOLKIT=MOTIF`),
> so `extract-abi.py`'s live checklist is empty in this configuration and the checkboxes cannot be
> coverage evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the
> `## Evidence` readings, and the per-`#if` `## Deviations`.  Port: `lwlib/lwlib-Xm.rs`
> (**2,834 lines**, `md5 5beeb53c79b9c5b65e84b82bb8e85a5b`); header mirror
> `lwlib/include/lwlib-Xm.rs` (57 lines, `md5 45a5d4f61448a9a7abc272c6bad95104`; its
> `_decls` move is `## Deviations` 19).  Both arms of every `#if` are carried; the mirror moved its
> eleven declarations into a `_decls` module (P-39, the `include/lwlib-Xaw.rs` shape).

## Decisions

- **File gate `feature = "use_motif"`** (P-86 ⑧), the build's condition rather than one the C text
  spells: `lwlib/Makefile.in` 53 (`MOTIF_OBJS = lwlib-Xm.o`), `configure.ac` 7233
  (`AC_DEFINE ([USE_MOTIF], [1], [Define to 1 if using the Motif X toolkit.])`).
  `USE_MOTIF`, `USE_X_TOOLKIT` and `HAVE_X11` are `undef` in
  docs/baseline/config/darwin-arm64.macros.tsv, so the frozen configuration compiles none of this
  file and the port's frozen object is the 336-byte empty one.
- **Sub-gates** (P-86 ⑥, each of the C text's own conditions lower-cased): C 500's
  `#ifndef LESSTIF_VERSION` is `feature = "not_lesstif_version"`, C 709's
  `#ifdef XmNpositionIndex` is `feature = "xm_position_index"`, C 1546's `#ifdef ENERGIZE` is
  `feature = "energize"`, and C 1728's `#if XmVersion < 1002 || (defined LESSTIF_VERSION &&
  LESSTIF_VERSION < 84)` is `feature = "xm_version_lt_1002_or_lesstif_lt_84"` (a derived name for a
  compound condition, the `use_cairo_or_have_xft` precedent).  **None of the four is registered in
  `xtask/Cargo.toml`**, so no build configuration sets them and only a probe can
  (`## Owed` 2).
- **The one sub-gate the unit needs but the C text does not spell** is
  `feature = "use_x_toolkit"`: `include/lwlib-widget.rs` 96-109 gates `_widget_value.change` and
  `this_one_change` on it (lwlib-widget.h 87-101), and C 645, 654, 732, 757, 766 and 797 read those
  two fields.  A Motif build always defines `USE_X_TOOLKIT` (`configure.ac` 3147), so both green
  arms of the probe set it (`## Evidence` ②); `## Owed` 1 names the wiring.
- **This file is `text-only`** (P-86 ③⑵): this machine has no Xt and no Motif header
  (`/opt/homebrew/include/X11` carries `xorgproto` 2025.1's Xlib half only).  The Xlib records and
  scalars this file addresses (`XButtonPressedEvent`'s fifteen members, `XKeyEvent`'s
  `display`/`keycode`, `XEvent`, and the eleven event/mask constants) are **measured**
  (`## Evidence` ⑥); every Xt and Motif width, offset, enum value and resource name below is C's
  spelling with the value **UNVERIFIED** (`## Owed` 5 names the toolchain).
- **The `<X11/*>` and the twenty-four `<Xm/*>` headers stand in the unit** (P-88 ①: no mirror in
  this tree carries any of them and this file is their only consumer here);
  `lwlib/include/lwlib.rs` names `Widget`, `Boolean` and `XEvent` and leaves them to the unit that
  includes it.  C's own two include lists stand as the unit's comment.
- **C's 34 static forward declarations (C 77-121) are deleted**: Rust items are order-independent
  (PORTING.md 0.1-12 / checklist 10), and the ledger names them so the row is not a silent gap.
- **The two `#if 0` regions (C 123-128, C 131-153) are recorded, not carried** (dead in every
  configuration; the `src/character.rs` 91-97 precedent).
- **`dupstring` is declared in the unit**: C 956 calls it, `lisp.h` 5555 declares it, and the
  `lisp.rs` mirror does not carry it (`## Owed` 4).

## Gate mapping

| C condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`MOTIF_OBJS`, i.e. `USE_MOTIF`) | `feature = "use_motif"` (every item and every `include!` edge) | a Motif X toolkit configuration |
| `lwlib-widget.h` 87-101's `#ifdef USE_X_TOOLKIT` (the two fields C 645-797 read) | `feature = "use_x_toolkit"` (no site of its own: the mirror's gate, which this unit's arm sets) | Motif/Lucid, where `configure.ac` 3147 defines `USE_X_TOOLKIT` |
| C 500 `#ifndef LESSTIF_VERSION` ... C 504 | `feature = "not_lesstif_version"` | a build without LessTif (`LESSTIF_VERSION` undefined, this target's own reading) |
| C 709 `#ifdef XmNpositionIndex` ... C 712 | `feature = "xm_position_index"` | a Motif whose `<Xm/RowColumn.h>` defines `XmNpositionIndex` |
| C 1546 `#ifdef ENERGIZE` ... C 1644, and the initializer pair at C 1652-1663 | `feature = "energize"` | an Energize build |
| C 1728 `#if XmVersion < 1002 \|\| (defined LESSTIF_VERSION && LESSTIF_VERSION < 84)` ... C 1742 | `feature = "xm_version_lt_1002_or_lesstif_lt_84"` | Motif 1.1 or LessTif < 0.84 |

## C line ranges

The table is a **contiguous partition** of C 1-1964: `tools/check-dead-file-coverage.py` binds this
ledger to `lwlib/lwlib-Xm.h` as well (`ledger_for` falls back to `<stem>.md`), and the header's own
text lines sit at positions that are blank in the `.c`, so every blank line between blocks is
folded into the row that precedes it (the `lwlib-Xaw.md` `(wh)` practice, applied throughout).
C 1964 is the last line.

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's licence/declaration block (C 1-19) | dropped (P-86 ①) |
| 21-22 | `#include <config.h>` | ungated (every unit includes it) |
| 23-26 | `#include <unistd.h>`, C 24 `<stdio.h>` (`sprintf`), C 25 `<setjmp.h>` | file gate (C 23/25 have no edge of their own; C 24 is `lib/include/stdio.rs`) |
| 27-32 | `<X11/StringDefs.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/CoreP.h>`, `<X11/CompositeP.h>` (C 27-31) | file gate (the surface below) |
| 33-37 | `#include <lisp.h>` (C 33), `#include "lwlib-Xm.h"` (C 35), `#include "lwlib-utils.h"` (C 36) | file gate (`xmalloc`/`xstrdup`/`xfree`/`emacs_abort`; `lwlib-Xm.h`; `XtCompositeChildren`) |
| 38-62 | C 38-61's twenty-four `<Xm/…>` headers | file gate (the surface below; no Motif header in this tree) |
| 63-65 | `enum do_call_type { pre_activate, selection, no_selection, post_activate }` | file gate |
| 66-76 | `typedef struct _destroyed_instance { … } destroyed_instance` | file gate |
| 77-122 | C 77-121's static forward declarations (34 of them) | deleted (PORTING.md 0.1-12; the ledger names them) |
| 123-155 | C 123-128's `#if 0` (three declarations) and C 131-153's `#if 0` (`x_print_complete_resource_name`, C 136-151) | recorded, not carried (dead in every configuration) |
| 156-157 | `static destroyed_instance *all_destroyed_instances = NULL;` | file gate |
| 158-183 | `make_destroyed_instance`, C 176-182 `free_destroyed_instance` | file gate |
| 184-190 | `first_child` (C 184-189; C 81 declares it non-static) | file gate (exported) |
| 191-198 | `lw_motif_widget_p` | file gate (exported) |
| 199-218 | `resource_motif_string` | file gate |
| 219-261 | `destroy_all_children` | file gate |
| 262-268 | C 262-267 `xm_arm_callback`'s comment | file gate |
| 269-297 | `xm_arm_callback` | file gate (an `XtCallbackProc`) |
| 298-315 | C 298-314 `xm_update_label`'s comment | file gate |
| 316-364 | `xm_update_label` | file gate |
| 365-387 | `xm_update_list` | file gate |
| 388-398 | `xm_update_pushbutton` | file gate |
| 399-409 | `xm_update_cascadebutton` | file gate |
| 410-422 | `xm_update_toggle` | file gate |
| 423-463 | `xm_update_radiobox` | file gate |
| 464-468 | C 464-468 `make_menu_in_widget`'s two comments | file gate |
| 469-633 | `make_menu_in_widget` (C 500-504 the `#ifndef LESSTIF_VERSION` check, C 535-616 the create loop) | file gate / `not_lesstif_version` |
| 634-735 | `update_one_menu_entry` (C 709-712 the `#ifdef XmNpositionIndex` pair) | file gate / `xm_position_index` |
| 736-806 | `xm_update_menu` | file gate |
| 807-835 | `xm_update_text` (C 807-821), `xm_update_text_field` (C 823-834) | file gate |
| 836-901 | `xm_update_one_widget` | file gate (exported) |
| 902-987 | `xm_update_one_value` | file gate (exported) |
| 988-1003 | C 988-994 `activate_button`'s comment, `activate_button` (C 995-1002) | file gate (an `XtCallbackProc`) |
| 1004-1031 | C 1004-1006 `creation functions`, `dialog_key_cb` (C 1007-1030) | file gate (an `XtEventHandler`) |
| 1032-1294 | C 1032-1033 `dialogs`, `make_dialog` (C 1033-1293: the 24 `XtSetArg` groups, the two push-button loops, the separator/icon/text-field/radio/list arms) | file gate |
| 1295-1340 | `find_matching_instance` (C 1295-1330), `mark_dead_instance_destroyed` (C 1332-1339) | file gate |
| 1341-1412 | `recenter_widget` (C 1341-1375), `recycle_instance` (C 1377-1411) | file gate |
| 1413-1489 | `xm_create_dialog` (C 1439-1466 the `switch (name [0])`, C 1468-1478 the name decode) | file gate (exported) |
| 1490-1503 | C 1490-1492 `Create a menu bar …`, `make_menubar` (C 1493-1502) | file gate |
| 1504-1543 | `remove_grabs` (C 1504-1511), `make_popup_menu` (C 1513-1527), `make_main` (C 1529-1542) | file gate |
| 1544-1645 | C 1544 `Table of functions to create widgets`; C 1546-1644 the `#ifdef ENERGIZE` arm (`widget_maker`, the ten `create_*` declarations, `make_one`, the ten wrappers) | file gate / `energize` |
| 1646-1666 | `xm_creation_table` (C 1649-1662's rows, C 1652-1663 the inner `#ifdef ENERGIZE` pair) | file gate / `energize` |
| 1667-1700 | C 1667-1668 `Destruction of instances`, `xm_destroy_instance` (C 1675 the never-taken `0 &&` arm) | file gate (exported) |
| 1701-1749 | C 1701-1702 `popup utility`, `xm_popup_menu` (C 1728-1742 the `#if XmVersion < 1002 …` pair) | file gate / `xm_version_lt_1002_or_lesstif_lt_84` |
| 1750-1784 | `set_min_dialog_size` (C 1750-1757), `xm_pop_instance` (C 1759-1783) | file gate |
| 1785-1843 | C 1785-1787 `motif callback`, `do_call` (C 1817-1841 the `switch (type)`) | file gate |
| 1844-1862 | C 1844-1848 `xm_internal_update_other_instances`'s comment, the static (C 1849-1861) | file gate (an `XtCallbackProc`) |
| 1863-1904 | `xm_generic_callback` (C 1863-1870), `xm_nosel_callback` (C 1872-1887), `xm_pull_down_callback` (C 1889-1903) | file gate |
| 1905-1935 | C 1905-1910 the MenuShell comment; `xm_pop_down_callback` (C 1912-1922), `xm_pop_up_callback` (C 1924-1934) | file gate |
| 1936-1944 | C 1936-1937 `set the keyboard focus`, `xm_set_keyboard_focus` (C 1938-1943) | file gate (exported) |
| 1945-1958 | C 1945-1946 `Motif hack to set the main window areas.`, `xm_set_main_areas` (C 1946-1957) | file gate (exported) |
| 1959-1964 | C 1959-1964 `Motif hack to control resizing on the menubar.`, `xm_manage_resizing` | file gate (exported) |

## Findings

1. **The two `#if 0` regions are the only text not carried.**  C 123-128's three declarations are
   all also in `lwlib-Xm.h`, and C 131-153's function is the debug printer C's own text calls
   "handy to have available"; both are dead in every configuration (`tools/cinventory.py` skips
   `#if 0` unconditionally), so the rows above say `recorded`.
2. **The creation table's `#ifdef ENERGIZE` sits inside the initializer** (C 1652-1663).  Rust
   cannot put an attribute on an array element, so the port carries the two arms as two
   `#[cfg]`-gated definitions of `xm_creation_table` (4 rows without `energize`, 14 with) - exactly
   one exists in any configuration (`## Deviations` 5).
3. **`XmNallowShellResize` is spelled once, at C 1570.**  It is the one place the C text names a
   Motif resource Xt also names (`XtNallowShellResize` is what C 1064 and C 1538 use); the port
   declares both names and keeps each call site's own spelling - which is what the body-token check
   reported before the repair.
4. **C 1705's `dummy` is a *complete* record.**  `XButtonPressedEvent` is Xlib's own typedef for
   `XButtonEvent`, whose fifteen members the port declares in C's order, so its offsets are measured
   rather than inferred; `XKeyEvent`'s and the `XEvent` union's are measured in the same probe
   (`## Evidence` ⑥).  The Xt/Motif records are the opposite case (truncated, `UNVERIFIED`).
5. **The two `XtParent (widget)` reads at C 1918-1920 and C 1930-1932 stay two reads**: C writes the
   call twice in each condition and the port keeps both, so a widget whose parent changes between
   the two reads sees C's behaviour (`## Deviations` 18).

## Evidence

**① The frozen configuration (`build-tu`, 1 run).**
`cd femacs && CARGO_TARGET_DIR=/tmp/lxm-lane cargo run -q -p xtask -- build-tu lwlib/lwlib-Xm.c`
-> `built lwlib/lwlib-Xm.rs -> ../build/femacs/lwlib-Xm.o`, **exit 0**, `build/femacs/lwlib-Xm.o`
**336 bytes**, `md5 dbed68837694579185377dbb0821ab62`.  That is the expected reading and **not a
proof of the new text**: every item of the port sits behind the `use_motif` gate (P-86 ⑧'s "the
gate must go down to the include graph"), so the frozen configuration compiles only
`include/config.rs` and the crate's `#![allow]`.

**② The arm flips (P-86 ③⑴, ⑥): `probe`, 3 runs.**
Run 1, the same command as run 2 before the two mechanical repairs -> **exit 1, 19 errors + abort**,
all in this unit (`build/probes/lwlib-Xm/_root.rs`): nine `E0283` (an unannotated
`core::ptr::null_mut ()` in a variadic argument list), four `E0617` (`bool`/`i16`/`Position` passed
to a variadic function), one `E0600` (`(-1) as c_uchar`) and one `E0609`
(`(*children.offset (i as isize)).core` needed a second dereference).
Run 2, the file gate and the mirror's own toolkit gate:
`... -- probe lwlib/lwlib-Xm.rs /tmp/lxm-lane/x.rlib --crate-type=lib -C panic=abort
--cfg 'feature="use_motif"' --cfg 'feature="use_x_toolkit"'` -> **exit 0, 0 errors, 2 warnings, an
8,363,432-byte rlib**; both warnings are the mirrors' (`lib/include/stdbit.in.rs` 644 and
`src/include/lisp.rs` 3939 - the same two `lwlib-Xaw` reported).
Run 3, the file gate and **every** sub-gate:
`... --cfg 'feature="use_motif"' --cfg 'feature="use_x_toolkit"'
--cfg 'feature="not_lesstif_version"' --cfg 'feature="xm_position_index"'
--cfg 'feature="xm_version_lt_1002_or_lesstif_lt_84"' --cfg 'feature="energize"'` -> **exit 0, 0
errors, 2 warnings** (the same two), an 8,396,192-byte rlib.  That reading compiles the LessTif
check, the `XmNpositionIndex` pair, the Motif-1.1 menuPost block, all 13 Energize rows of the table
and the `#if 0`-neighbouring text.

**③ The mutation test (P-86 ⑦).**  A `/tmp` copy of run 3's materialized root with one type error
injected into **this unit's own text** (`let _mutant: c_int = c"boom".as_ptr ();` as the first
statement of `xm_update_one_widget`) compiled with the same `rustc` flags: control **exit 0, 0
errors**; mutant **exit 1, one `E0308` at that statement**.  The output changes, so run 3's green
reading is not vacuous.

**④ The `/tmp` harness reading on the current tree text** (not the tree's `probe`).  A `/tmp`
`materialize.py` reproduces xtask's `materialize` rule (one copy per file, first reach wins,
`xtask/src/main.rs` 1793-1930) over `lwlib/lwlib-Xm.rs` -> 28 files, 21,815 lines, and
the same `rustc` command as run 3 gives **exit 0, 0 errors, 2 warnings**.  This is the reading taken
*after* the two body-token repairs (`first_child`, `make_one`), i.e. against the text `## Evidence`
②③ were not measured on; it is a `/tmp` harness's, not the tree's `probe`.

**⑤ The text gates.**
`python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xm.c lwlib/lwlib-Xm.rs` first
reported four function rows - `first_child does not mention 0, compositewidget`,
`xm_create_dialog does not mention break, case, switch`, `make_one does not mention
xmnallowshellresize`, `do_call does not mention break, case, default, switch`.  The first and third
were repaired in the port (C's cast type and index literal kept; C 1570's own resource name used);
the two `switch` rows are recorded in `docs/ports/macro-body-notes.tsv` (PORTING.md 2.2/P-04, the
`sfnt_validate_gs` / `xaw_create_dialog` precedent).  What remains is the line-count drift row,
**44.3%: C 1964 vs Rust 2834**, whose tolerance row is the integrator's (`## Owed` 3).
`python3 tools/check-declarations.py` -> **0 failures** (17 pre-existing allowed literals, 0
forbidden).  `python3 tools/check-citations.py` -> **0 drift, 0 range, 0 stale** for this unit's two
files (the run's single `drift` is `src/dbusbind.rs`'s, pre-existing); this unit's new
citations name headers that are not in the tree (`<X11/*>`, `<Xm/*>`) and land in the tool's counted
`external` bucket.  `python3 tools/check-dead-file-coverage.py` -> **passed**, 0 failures, for this
ledger's 52 rows (it reads them against `lwlib/lwlib-Xm.c` *and*, via the `<stem>.md` fallback,
against `lwlib/lwlib-Xm.h` - `## Deviations` 17).

**⑥ The Xlib half, measured with clang** (`xorgproto` 2025.1, `clang -I/opt/homebrew/include
/tmp/lxm-lane/xprobe.c`):

```
XEvent 192 align 8; XButtonPressedEvent 96 (= XButtonEvent); XKeyEvent 96 align 8
XButtonEvent: type 0 serial 8 send_event 16 display 24 window 32 root 40 subwindow 48 time 56
               x 64 y 68 x_root 72 y_root 76 state 80 button 84 same_screen 88
XKeyEvent: display 24 keycode 84
Display* 8 Screen* 8 Window 8 KeySym 8 Bool 4 Time 8
ButtonPress 4 ButtonRelease 5 EnterNotify 7 MotionNotify 6 KeyPressMask 1
Button1Mask 256 Button2Mask 512 Button3Mask 1024 Button4Mask 2048 Button5Mask 4096
```

**⑦ Budget as run** (the lane's limit): `build-tu` **1 of 3**, `probe` **3 of 3** (run 1 is the error
list of `## Evidence` ②, runs 2-3 the two green arms), plus one `/tmp` harness compile and one
control/mutant pair (`## Evidence` ③④) that are not `probe` invocations; no `cargo test`, no `--all`,
no `gate-fast` and no `check-all-units.sh` was run, and `/tmp/lxm-lane` is removed after the readings
are recorded (what is cited here is the text above, not the directory).

## Deviations

1. **The Xt/Motif surface is C's spelling, not a measurement** (P-86 ③⑵): `WidgetRec`'s and
   `CorePart`'s members are carried in C's relative order with the members between them omitted, so
   their offsets are knowingly not the real ones (`CompositePart`'s two members are complete, its
   offset inside `WidgetRec` is not); `XtResource`, `Arg`, `XtTranslations`, `XmString`,
   `XmPushButtonCallbackStruct`, the eleven widget classes, the 61 `XmN*` resource names, the 20
   enum values and the 31 entry points are the same class.  `## Owed` 5 is the measurement.
2. **`XmPushButtonCallbackStruct` carries its two read members only**: C 272 reads `reason` and
   `event`, and the members after them (Motif's `widget`/`doit`/...) are omitted, so the record's
   `sizeof` is not Motif's - its two offsets are the first two, so they are.
3. **The eleven Xlib scalars and the three Xlib records are measured**, not placeholders
   (`## Evidence` ⑥); `XButtonPressedEvent` is Xlib's own typedef for `XButtonEvent`, and the port
   spells the alias with a `pub type`.
4. **C's 34 static forward declarations (C 77-121) are deleted** (PORTING.md 0.1-12 / checklist 10) -
   Rust resolves items order-independently - and the ledger's row names all 34, so the deletion is
   not a silent gap.
5. **The creation table's initializer pair becomes two `#[cfg]`-gated definitions** (C 1652-1663);
   the rows are C's own, in C's order.
6. **C 1439's `switch (name [0])` and C 1817's `switch (type)` become `match`** with C's shared
   bodies as `b'E' | b'e' => ...` arms and constant arms; C has no `default:` in the first and one in
   the second, so the port adds an empty `_ =>` arm there and keeps C's `default:`'s `emacs_abort ()`
   here (PORTING.md 2.2; the two `docs/ports/macro-body-notes.tsv` rows).
7. **`overrideTrans` (C 490) is the port's `&CStr` of the same bytes** and `override` is spelled
   `override_` (a Rust keyword, the `lwlib-Xaw.rs` reading); C's `unsigned char type` locals are
   `r#type` (C 484/507, C 1898).
8. **Locals C leaves uninitialised start at zero** (PORTING.md 0.1-13): `children`, `menu`,
   `result`/`form`/`row`/`icon`/`icon_separator`/`message_label`/`separator`, `widget`,
   `widget_to_manage`, `focus`, `dead_one`, `total_buttons`, `wv`, `trans`, `pos_list`, `dummy`, and
   every `Arg` array (C 240, 276, 324, 481, 640, 793, 878, 940, 1056, 1496, 1534, 1565, 1793).
   Named here because a reviewer cannot see the zeroing in C.
9. **C's Boolean truth tests are spelled**: `if (! menu)` -> `if menu.is_null ()`,
   `if (! instance)` -> `if instance.is_null ()`, `if (type != XmMENU_BAR && ...)`,
   `if (! cur->call_data)` -> `!(*cur).call_data.is_null ()`, `if (ac)` -> `if ac != 0`,
   `if (up)` -> `if up != 0`, `if (button)` -> `if !button.is_null ()`, `if (name [3])` ->
   `*name.offset (3) != 0`, `if (cur->button_type)` -> `if (*cur).button_type != 0`, and the five
   `strcmp (...)` tests become `!= 0`; C 1675's `if (0 && XtClass (widget) == ...)` is carried
   verbatim as `if 0 != 0 && ...`.
10. **C's `bool` fields are `true`/`false`** (`val->edited`, `cur->selected`) where C writes
    `False`/`True`/`1`, and the places C relies on an implicit conversion are spelled:
    `XtSetSensitive (widget, (*val).enabled as Boolean)` (C 650, 851) and
    `(*cur).enabled as c_int as XtPointer` (C 447, 540).
11. **C's varargs default promotions are spelled**: `Dimension`/`Position`/`bool`/`c_int` arguments
    to `XtVaSetValues`/`XtVaGetValues` carry `as c_int` (C 419, 449, 1084-1088, 1374, 1756), C's
    `(XtArgVal)` casts are the `XtSetArg!` macro's own `as XtPointer`, `char` -> `c_int` at C 1468
    and 1476 (`name [1] - '0'`), `i` (`unsigned int`) -> `c_int` at C 711, and C 507's `type = -1` of
    an `unsigned char` is `(-1 as c_int) as c_uchar` (C's 255).
12. **`XmSTRING`/`XmPIXMAP` are declared `c_int`** (C's own type is `unsigned char`) and the three
    `XmMENU_*` values are `c_uchar` (C 510/512/1898-1900 compare them with an `unsigned char`).
13. **`XtGetSubresources`' trailing `NULL, 0`** (C 214) is carried as C writes it; the `NULL` in a
    variadic position is the typed `*mut c_void` constant rather than an unannotated
    `core::ptr::null_mut ()`, which rustc cannot infer (`## Evidence` ② run 1's `E0283`).
14. **The ENERGIZE `make_one`'s `(*fn) (...)`** (C 1574, 1578) is `(fn_.unwrap ()) (...)`: C's
    function-pointer parameter is a nullable `Option<unsafe extern "C" fn ...>` in the port (P-27),
    the `lwlib-Xaw.rs` reading of the same shape.
15. **The five `XmIs*` predicates and `XtIsShell`/`XtIsManaged` are macros in C** and are declared as
    functions here (the `lwlib-Xlw.rs` reading of `XtIsShell`); `XmCreateRadioBox`,
    `XmCreateScrolledList` and `XmCreateMainWindow` are convenience macros in C and the port declares
    the call the C text makes.
16. **`dupstring` is declared in the unit** (lisp.h 5555; C 956), because no mirror in this tree
    carries it (the `src/term.rs` 384-385 reading of the same entry).
17. **`tools/check-dead-file-coverage.py` binds this ledger to `lwlib/lwlib-Xm.h` too**, which is why
    the table above is a contiguous partition (the header's text lines sit where the `.c` is blank).
18. **C's two `XtParent (widget)` calls inside each of C 1918-1920 and C 1930-1932 are kept as two
    calls**, not folded into a local (see `## Findings` 5).
19. **The header mirror `lwlib/include/lwlib-Xm.rs` carries a `_decls` module** (P-39): the
    unit *defines* all eleven declarations it carries (`E0428` otherwise), so they live in
    `lwlib_Xm_decls` and are glob-imported - the `include/lwlib-Xaw.rs` / `include/lwlib-Xlw.rs`
    shape.
20. **`XmNpositionIndex`'s declaration is gated with its use** (`feature = "xm_position_index"`), so
    the resource name exists exactly where the C text's `#ifdef` is true.

## Owed

1. **`use_motif` must imply `use_x_toolkit`** (`## Decisions`): `include/lwlib-widget.rs` 96-109
   gates `_widget_value.change`/`this_one_change` on the latter and this unit reads them at six
   sites.  **Owner: the integrator** (the `--cfg` list `xtask` derives) - a Motif build without it
   is `E0609`, which is why both green arms of `## Evidence` ② set it.  (A `use_motif`-only probe
   was not run: the probe budget is 3 of 3.)
2. **The four sub-gate names are set by nothing but a probe**: `not_lesstif_version`,
   `xm_position_index`, `energize` and `xm_version_lt_1002_or_lesstif_lt_84` do not appear in
   `xtask/Cargo.toml` (grep), so no build configuration can turn them on and
   `docs/baseline/feature-coverage.tsv` does not know them.  **Owner: the integrator** (the
   derivation belongs where the other platform features are registered).
3. **The line-drift tolerance row is owed** (the integrator's file,
   `docs/ports/line-drift-tolerance.tsv`): `lwlib-Xm`, tolerance **0.45**, basis `total`, reason =
   C 1964 vs Rust 2,834 (44.3%): the whole file is transcribed, and the difference is the fixed cost
   of a unit whose third of the text is a surface no mirror in this tree carries (about 700
   declaration lines of the Xt and Motif halves), of the per-arm `#[cfg]` lines, and of the
   per-block C-line comments (`lwlib-Xaw` is the same class at 75.7%).
4. **`dupstring`'s home**: two units declare it locally (`src/term.rs` 384-385 and this one);
   the proper home is the `lisp.rs` mirror (lisp.h 5555 declares it).  **Owner: the lisp lane** -
   then this unit's copy goes away.
5. **The Xt/Motif surface's real shapes**: with an X11 toolchain (`xorgproto` + libXt and Motif or
   LessTif), measure `XmPushButtonCallbackStruct`, the eleven widget classes, the 61 `XmN*`
   resource-name spellings, the 20 enum values and the `WidgetRec`/`CorePart` offsets, and measure
   with the reference's own `configure` which of the four sub-conditions a Motif build takes.  What
   is missing on this machine is *Xt* and *Motif* (xorgproto 2025.1's Xlib half is present and
   measured, `## Evidence` ⑥).  P-90 clause 1 asks for a dedicated Xt mirror
   (`src/include/<sdk>.rs`); this unit's copy is one to move when it lands (P-88 clause 5).
6. **`docs/port-map.md`'s target-line column** for `lwlib/lwlib-Xm.rs` still says 3 (and
   `lwlib/include/lwlib-Xm.rs`'s 47), and **`docs/baseline/feature-coverage.tsv`'s row for
   `use_motif`** (line 90) is stale: no new *name* is added, but `tools/feature-coverage.py --write`
   is the integrator's reading to refresh.
7. **`docs/baseline/port-lines-pending.tsv`**'s row for `lwlib/lwlib-Xm.c` is stale after this round
   (informational); `tools/check-port-lines.py --write-pending` retires it.

## Not claimed

- **Zero run evidence**: nothing was linked or run; no widget was created, no callback fired, no
  dialog was popped up and no menu was posted.  The artifacts are compile products of a type-check.
- **No Xt or Motif header was consulted** (there is none on this machine): every Xt/Motif type,
  constant and layout above is C's spelling, the records' omitted members mean their offsets are
  knowingly not the real ones, and the 20 enum values are placeholders.
- **The frozen object is the 336-byte empty one**, so it says nothing about the body; `## Evidence`
  ②③ are type-checks, not behaviour, and **the reading taken after the two body-token repairs is a
  `/tmp` harness's (`## Evidence` ④), not the tree's `probe`**.
- **No behavioural equivalence** for any arm, and no separate reading for the sub-gate combinations
  the third probe did not isolate (`energize` with LessTif, `xm_position_index` alone, the Motif-1.1
  menuPost pair alone) - they were compiled together, in one arm.
- **`dupstring`, the five `XmIs*` predicates and the three convenience creators are declarations
  this unit makes**, so the arm's green reading does not say those names link to anything.
