# emacs/lwlib/lwlib.c (1330 lines)

> **P-86 file: the frozen configuration compiles nothing here, and the ledger is the C range
> table rather than ticked boxes.** `docs/baseline/frozen-build-sources.txt` 97 lists `lwlib/`
> with `oldXMenu/` as "the X11 widget libraries; this target has no window system",
> `build/emacs-ref/src/Makefile` 304 spells `WINDOW_SYSTEM_OBJ=` (empty) and
> `build/emacs-ref/lwlib/` carries a `Makefile` and no objects, so `extract-abi.py`'s checklist
> for this file is empty and a checkbox section could not be coverage evidence (P-86 ②).  What
> follows is the substitute: the **gate mapping**, the **C line-range table**, `## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/lwlib-lane/cargo cargo run -q -p xtask -- build-tu lwlib/lwlib.c`
→ `built lwlib/lwlib.rs -> ../build/femacs/lwlib.o`, **exit 0**, and `build/femacs/lwlib.o` is
**336 bytes** - an empty object (`otool -l` prints `size 0x0` for `__text`, `nm` no symbol).  That is
the expected reading and not a proof of the new text: every item of the port sits behind a
`use_x_toolkit` gate, so under the frozen feature set rustc compiles only `include/config.rs` and the
`#![allow]` attribute.  **The same reading is the measurement that the frozen configuration does
*not* compile this reference file** - `docs/baseline/frozen-build-sources.txt` 97 lists `lwlib/` with
`oldXMenu/` as excluded, `build/emacs-ref/src/Makefile` 304 has `WINDOW_SYSTEM_OBJ=` (empty) and
`build/emacs-ref/lwlib/` carries a `Makefile` and no object - so the frozen object is the empty one
on both sides and the arm flips below, not this reading, are this file's evidence (the situation
`docs/ports/lwlib-Xlw.md` records for its own file).

**② The arm flips (`xtask probe`, the whole file per configuration).**

| configuration | reading |
| --- | --- |
| `use_x_toolkit` + `use_lucid` | **exit 0**, `/tmp/lwlib-lane/lwlib_lucid.o` **17,200 bytes**, 0 errors |
| `use_x_toolkit` + `use_motif` | **exit 0**, `lwlib_motif.o` **17,848 bytes**, 0 errors |
| `use_x_toolkit` + `use_lucid` + `use_xaw` | **exit 0**, `lwlib_xaw.o` **17,824 bytes**, 0 errors |
| `use_x_toolkit` alone (C 52-54's configuration) | **exit 1, two errors**: `error: At least one of USE_LUCID or USE_MOTIF must be defined.` (C's own `#error` text, from the port's `compile_error!`) and one `E0308` in `lw_window_is_in_menubar` - the arm C never compiles has no `return`, and C refuses the configuration rather than giving it one |

Each probe is
`cd femacs && CARGO_TARGET_DIR=/tmp/lwlib-lane/cargo cargo run -q -p xtask -- probe lwlib/lwlib.rs
<out> --crate-type lib --emit obj -C panic=abort --cfg 'feature="use_x_toolkit"' --cfg …`.
The only warnings in the three good runs are pre-existing and not this unit's: `unused doc comment` on
`lib/include/stdbit.in.rs:644` and `src/include/lisp.rs:3939`.

**③ The defects the flips found, and their fixes** (the lane's "该修" class; every one is in this
unit's own text):
1. **Every `#[unsafe(no_mangle)]` definition was missing its `#[cfg (feature = "use_x_toolkit")]`**:
   the frozen build compiled all 28 of them against gated types - **252 errors**, all `E0425` on
   `LWLIB_ID`/`Widget`/`Boolean`/`c_int`/`strcmp`.  The gate is now on each of them and the frozen
   object is the 336-byte empty one again.
2. **`0xDEADBEEF` as a bare literal is out of `i32`'s range in the two `memset` calls**
   (**2 × `literal out of range for i32`**, a `deny`-by-default lint); C's `int` argument is now
   spelled as the conversion rustc asks for (`0xDEADBEEF_u32 as c_int`).
3. **A function-local `static` table of `*const c_char` is not `Sync`** (**2 × E0277** on
   `lw_separator_p`); the two read-only `separator_names` tables are `const` items of C's name.
4. Two more were fixed from a reading rather than a compiler message: `*type` for the `r#type`
   parameter (7 sites - the Rust keyword is not a place expression), and `False` for the two fields
   `include/lwlib-widget.rs` carries as Rust `bool` (`edited`, `free_toolkit_data`).

**④ The interface change P-39 forces, and one defect it produced and then fixed.**
`include/lwlib.rs`'s 23 entry points and `include/lwlib-int.rs`'s four were top-level
`unsafe extern "C"` declarations until this round, and a unit that *defines* those names is `E0428`
against them (measured on a 5-line probe: declaration + `#[unsafe(no_mangle)]` definition in one
scope is "the name is defined multiple times").  Both mirrors now carry the
`lwlib_decls`/`lwlib_int_decls` module + glob import the four sibling mirrors in this directory
already use.  The first version ended the module with a plain `use …_decls::*;`, which the
`xlwmenu.rs` flip immediately caught (**6 × `E0425 cannot find function lw_separator_p`**):
`xlwmenu.rs` `include!`s `include/lwlib-int.rs` **inside its own `xlwmenu_decls` module**, and a
private glob import is not re-exported to a downstream glob.  The two mirrors now end with
`pub use …_decls::*;`.

The consumers of the changed mirrors then read:

| unit | flip | reading |
| --- | --- | --- |
| `lwlib/lwlib-Xlw.rs` (includes **both** mirrors) | `use_lucid` | **exit 0**, 5,240 bytes, 0 errors |
| `lwlib/lwlib-utils.rs` (includes `lwlib.rs`) | `use_lucid` | **exit 0**, 3,008 bytes, 0 errors |
| `lwlib/xlwmenu.rs` (includes `lwlib-int.rs` in a module) | `use_lucid` + `use_x_toolkit` | **exit 0**, 36,424 bytes, 0 errors (with `use_lucid` alone: the 6 `lw_separator_p` errors of the paragraph above, plus `widget_value` tail-field errors that are that file's own `use_x_toolkit` gate) |
| `src/widget.rs` (includes `lwlib.rs` at its crate root) | `use_x_toolkit` | exit 1, ~40 × `E0425`, **every one of them a name of that unit's own unmeasured X11/Xt surface** (`XEvent`, `Region`, `XrmQuark`, `CWWidth`, `x_output`, …) and **none from the two mirrors** - the file's own `## Owed`, not this round's change |

**⑤ The six units the mirrors sit in, re-built under the frozen configuration** (`build-tu` for
`lwlib/lwlib-Xlw.c`, `lwlib/lwlib-utils.c`, `lwlib/xlwmenu.c`, `lwlib/lwlib-Xm.c`, `lwlib/lwlib-Xaw.c`,
`src/widget.c`): all **exit 0**.  These readings are weak on purpose and are recorded as such: those
units' `include!`s of the two mirrors are themselves `#[cfg]`-gated, so the frozen build never
expands them (the objects are the empty ones they were before).  The strong readings are ④'s probes.

## Deviations` and `## Owed`.  Port: `femacs/lwlib/lwlib.rs`
> (**1,641 lines**), every `#if`/`#ifdef` carried with both arms.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`lwlib/Makefile.in` 58's `OBJS`, reached only under `configure.ac` 7226-7229's `USE_X_TOOLKIT != none`; 7296's `LIBXMENU='$(lwlibdir)/liblw.a'`) | `feature = "use_x_toolkit"` (on every item and on C 23-50's include graph) | an X toolkit configuration (any of the two below) |
| C 33-35 `#if defined (USE_LUCID)` | `feature = "use_lucid"` | the Lucid/Athena toolkit |
| C 36-38 `#if defined (USE_MOTIF)` | `feature = "use_motif"` | the Motif toolkit |
| C 39-41 `#else /* not USE_MOTIF */` … `#define USE_XAW` … `#endif` | `feature = "use_xaw"` (the derived macro, written as the feature the C 43-50 and per-function arms are gated on) | Lucid without Motif |
| C 44 `#ifdef HAVE_XAW3D` … `#include <X11/Xaw3d/Paned.h>` … `#else` … `#include <X11/Xaw/Paned.h>` … `#endif` | `feature = "have_xaw3d"` (the two arms name the same one entry point this file needs) | Xaw3d |
| C 52-54 `#if !defined (USE_LUCID) && !defined (USE_MOTIF)` … `#error` … `#endif` | `compile_error!` under `all (feature = "use_x_toolkit", not (feature = "use_lucid"), not (feature = "use_motif"))` | a toolkit build with no toolkit chosen (which C refuses) |
| C 884-886 `#ifdef USE_MOTIF` … `extern Widget first_child (Widget);` … `#endif`, C 903-909 (the Motif half of `lw_raise_all_pop_up_widgets`'s test), C 1118-1122's Motif arm, C 1173-1178, C 1190-1195, C 1202-1204, C 1212-1214 | `feature = "use_motif"` | the Motif toolkit |

`use_x_toolkit` rather than one toolkit's name (the `lwlib-Xlw.rs`/`lwlib-Xm.rs` choice) is the
file's own condition: `lwlib.o` is in `OBJS` for **both** toolkits, and
`femacs/lwlib/include/lwlib-widget.rs` gates `widget_value`'s `change`, `this_one_change`,
`toolkit_data` and `free_toolkit_data` - which the readers in this file use - on
`feature = "use_x_toolkit"`, so any narrower gate would leave those fields out while the code that
reads them is compiled.  `USE_TOOLKIT`, `USE_LUCID`, `USE_MOTIF`, `USE_XAW` and `HAVE_XAW3D` are all
`undef` in `docs/baseline/config/darwin-arm64.macros.tsv` (1689, 3863, 3865, 3873), which is why
the frozen object below is the empty one.

## C line ranges

Rows are contiguous and ascending; the blank lines between blocks are the ones the table leaves
out (`tools/check-dead-file-coverage.py` reads the reference for them).  **The header shares this
ledger**: `docs/port-map.md` binds `lwlib/lwlib.c` *and* `lwlib/lwlib.h` to it, so the eight rows
above whose text names a position in the header (`21-22`, `23-26`, `27-32`, `43-51`, `52-55`,
`56-59`, `64-95`, `96-126`) extend over the header's own non-blank lines and the gate reads the
table against both references (the `docs/ports/lwlib-utils.md` arrangement, whose 63-line header is
inside its table the same way); `docs/ports/include-lwlib.md` is the header's *mirror* ledger.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's opening block (title, the copyright lines, the warranty and copying text) | dropped (P-86 clause 1) |
| 21-22 | `#include <config.h>` (the header's `LWLIB_H` guard is its 21-22) | ungated (every unit includes it) |
| 23-26 | `setjmp.h`, `lisp.h`, `c-strcase.h` (the header's `<X11/Intrinsic.h>` is its 24) | file gate (`setjmp.h` has no edge of its own: nothing here names a type it declares, and `lisp.rs` carries them) |
| 27-32 | `sys/types.h`, `stdio.h`, `lwlib-int.h`, `lwlib-utils.h`, `<X11/StringDefs.h>` (the header's widget-value comment runs to its 43) | file gate (`sys/types.h` has no edge of its own; `<X11/StringDefs.h>` is the surface declared in the unit) |
| 33-42 | `#if defined (USE_LUCID)` … `#include "lwlib-Xlw.h"` … `#endif`, `#if defined (USE_MOTIF)` … `#include "lwlib-Xm.h"` … `#else` … `#define USE_XAW` … `#endif` | lucid / motif / xaw (see the gate table) |
| 43-51 | `#if defined (USE_XAW)` … the `HAVE_XAW3D` pair … `#include "lwlib-Xaw.h"` … `#endif` (the header's `enum menu_separator` starts at its 51) | xaw gate, the pair under `have_xaw3d` |
| 52-55 | `#if !defined (USE_LUCID) && !defined (USE_MOTIF)` … `#error` … `#endif` (the header's first two `SEPARATOR_*` members are its 53-55) | the `compile_error!` under not(lucid), not(motif) |
| 56-59 | `#ifndef max` … `#define max(x, y) ((x) > (y) ? (x) : (y))` … `#endif` (the header's separator members run to its 69) | file gate (the port's `macro_rules! max`) |
| 60-62 | the `all_widget_info` comment and its definition | file gate |
| 64-95 | the 21 static forward declarations (the header's entry-point prototypes start at its 76) | deleted (PORTING.md 0.1-12 / checklist 10) |
| 96-126 | `free_widget_value_tree` (static) (the header's `lw_separator_p` prototype is its 126-127) | file gate |
| 127-155 | `copy_widget_value_tree` (static) | file gate |
| 157-183 | `allocate_widget_info` (static) | file gate |
| 185-193 | `free_widget_info` (static) | file gate |
| 195-203 | `mark_widget_destroyed` (static; Xt takes its address as the destroy callback) | file gate |
| 205-220 | `allocate_widget_instance` (static) | file gate |
| 222-227 | `free_widget_instance` (static) | file gate |
| 229-249 | `get_widget_info` (static) | file gate |
| 251-257 | `lw_get_widget_info` | file gate |
| 259-281 | `get_widget_instance` (static) | file gate |
| 283-290 | `lw_get_widget_instance` | file gate |
| 292-304 | `find_instance` (static) | file gate |
| 307-313 | `safe_strcmp` (static) | file gate |
| 316-332 | `#if 0` … the `printf` form of `EXPLAIN` … `#else` … `((void) 0)` … `#endif` | the `#if 0` arm is dead text (not transcribed); the live arm is the port's macro |
| 335-485 | `merge_widget_value` (static; C 443-447 its inner `#if 0`) | file gate; the inner `#if 0` is dead text |
| 488-511 | `name_to_widget` (static) | file gate |
| 513-533 | `set_one_value` (static; C 520-531 the three toolkit arms) | file gate / lucid, motif, xaw |
| 535-547 | `update_one_widget_instance` (static) | file gate |
| 549-560 | `update_all_widget_values` (static) | file gate |
| 562-612 | `lw_modify_all_widgets` | file gate |
| 615-629 | `initialize_widget_instance` (static) | file gate |
| 632-640 | `find_in_table` (static) | file gate |
| 642-673 | `dialog_spec_p` (static) | file gate |
| 675-724 | `instantiate_widget_instance` (static; C 680-707 the three tables and the two dialog creators, C 697-699 the empty Lucid arm) | file gate / lucid, motif, xaw |
| 726-739 | `lw_register_widget` | file gate |
| 741-748 | `lw_get_widget` | file gate |
| 750-768 | `lw_make_widget` | file gate |
| 770-779 | `lw_create_widget` | file gate |
| 782-824 | `destroy_one_instance` (static; C 803-820 the three-link if-then-else chain) | file gate / lucid, motif, xaw |
| 826-840 | `lw_destroy_widget` | file gate |
| 842-859 | `lw_destroy_all_widgets` | file gate |
| 861-866 | `lw_destroy_everything` | file gate |
| 868-882 | `lw_destroy_all_pop_ups` | file gate |
| 884-886 | `#ifdef USE_MOTIF` … `extern Widget first_child (Widget);	/* garbage */` … `#endif` | motif gate |
| 888-919 | `lw_raise_all_pop_up_widgets` (C 903-909 the Motif half of the test) | file gate / motif |
| 921-954 | `lw_pop_all_widgets` (static; C 931-952 the three arms) | file gate / lucid, motif, xaw |
| 956-960 | `lw_pop_up_all_widgets` | file gate |
| 962-966 | `lw_pop_down_all_widgets` | file gate |
| 968-983 | `lw_popup_menu` (C 971-982 the three arms) | file gate / lucid, motif, xaw |
| 985-1009 | `get_one_value` (static; C 993-1004 the three arms) | file gate / lucid, motif, xaw |
| 1011-1031 | `lw_get_some_values` | file gate |
| 1033-1044 | `lw_get_all_values` | file gate |
| 1046-1057 | `lw_get_widget_value_for_widget` | file gate |
| 1059-1062 | the `lwlib_updating` comment and its definition (static) | file gate |
| 1064-1101 | `lw_internal_update_other_instances` | file gate |
| 1103-1112 | `lw_get_widget_id` | file gate |
| 1114-1123 | `lw_set_keyboard_focus` (C 1118-1122 the `#if`/`#else` pair) | motif arm / not-motif arm |
| 1125-1143 | `show_one_widget_busy` (static) | file gate |
| 1145-1163 | `lw_show_busy` | file gate |
| 1165-1179 | `lw_refigure_widget` (C 1170-1172 the Xaw arm, C 1173-1178 the Motif `if`/`else`) | xaw gate, motif gate |
| 1181-1196 | `lw_window_is_in_menubar` (C 1187-1189 the Lucid arm, C 1190-1195 the Motif arm) | lucid arm / motif arm |
| 1198-1205 | `lw_set_main_areas` | motif gate |
| 1207-1215 | `lw_allow_resizing` | motif gate |
| 1217-1330 | `lw_separator_p` (C 1230-1252 and C 1275-1297 the two function-local `separator_table` types and tables) | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/lwlib-lane/cargo cargo run -q -p xtask -- build-tu lwlib/lwlib.c`
→ `built lwlib/lwlib.rs -> ../build/femacs/lwlib.o`, **exit 0**, and
`build/femacs/lwlib.o` is **336 bytes** - an empty object (two `nm`/`otool` reads: no `__text`).
That is the expected reading for this file and not a proof of the new text: every item of the port
sits behind a `use_x_toolkit` gate, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  **It is also the measurement that says the
frozen configuration does *not* compile this reference file** - `docs/baseline/frozen-build-sources.txt`
97 lists `lwlib/` as excluded, `build/emacs-ref/src/Makefile` 304 has `WINDOW_SYSTEM_OBJ=` empty and
`build/emacs-ref/lwlib/` holds no object - so the frozen object is the empty one on both sides and
the readings below, not this one, are the port's evidence (the same situation
`docs/ports/lwlib-Xlw.md` records).

**② The arm flips (`xtask probe`, the whole file per arm).**
`cd femacs && CARGO_TARGET_DIR=/tmp/lwlib-lane/cargo cargo run -q -p xtask -- probe lwlib/lwlib.rs
/tmp/lwlib-lane/lwlib_lucid.o --crate-type lib --emit obj -C panic=abort
--cfg 'feature="use_x_toolkit"' --cfg 'feature="use_lucid"'`
→ **exit 0**, `/tmp/lwlib-lane/lwlib_lucid.o` **17,200 bytes**, and **0 errors in this unit**: the two
warnings the run prints are `unused doc comment` on `lib/include/stdbit.in.rs:644` and
`src/include/lisp.rs:3939`, both pre-existing and neither this file's.  *(The Motif and Xaw flips are
recorded below as they are run.)*

**③ The three source defects the flips found, and their fixes** (all in this unit's own text, the
class the lane rules call "该修"):
1. every `#[unsafe(no_mangle)]` definition was missing its `#[cfg (feature = "use_x_toolkit")]`, so
   the frozen build compiled 28 exported definitions against gated types (**252 errors**, all
   `E0425` on `LWLIB_ID`/`Widget`/`Boolean`/`strcmp`/…); the gate is now on each of them.
2. `0xDEADBEEF` as a bare literal is out of `i32`'s range in the two `memset` calls
   (**2 × "literal out of range for `i32`**); the port spells C's conversion to `int` explicitly
   (`0xDEADBEEF_u32 as c_int`).
3. a function-local `static` table of `*const c_char` is not `Sync` (**2 × E0277**); the two
   read-only `separator_names` tables are `const` items of C's name.
   Two more were fixed without a compiler reading: `*type` for the `r#type` parameter (7 sites) and
   `False` for the two `bool` fields `include/lwlib-widget.rs` carries as Rust `bool`
   (`edited`, `free_toolkit_data`).

**④ The interface change P-39 forces, and the units it touches.**  `include/lwlib.rs`'s 23 entry
points and `include/lwlib-int.rs`'s four were top-level `unsafe extern "C"` declarations until this
round; a unit that *defines* them is `E0428` against them (measured on a 5-line probe).  Both
mirrors now carry the `lwlib_decls`/`lwlib_int_decls` module + glob import the four sibling mirrors
in this directory already use, and the seven units whose include graph reaches them are re-built
(`## Evidence` ⑤).

## Deviations

1. **The file gate is `use_x_toolkit`**, not a toolkit name (see the gate table).  Consequence:
   `use_x_toolkit` alone also fires C 52-54's `compile_error!`, which is C's own answer for a
   toolkit configuration with no toolkit chosen - and that configuration then reports a second
   error, `lw_window_is_in_menubar` returning nothing, because C's text gives that function a
   `return` per toolkit arm and never compiles the armless case.  The port could silence it with a
   port-only `return` under the refusing gate, which this lane did **not** write: C refuses the
   configuration and the port refuses it with C's own message, the second error being the shape of
   the text C never builds.
2. **`use_xaw` is a derived feature name** (C 39-41's `#define USE_XAW`), not a configuration
   constant - there is no `USE_XAW` in `docs/baseline/config/darwin-arm64.macros.tsv`.  The same
   reading the tree already uses for `use_cairo_or_have_xft`; it is **not** registered in
   `femacs/xtask/Cargo.toml`'s `[features]` table yet (`## Owed` 1).
3. **`destroy_one_instance`'s if-then-else chain** (C 803-820) gates each link with its own `#if`,
   and Rust cannot put an attribute on an `else`: the port spells C's chain as C's `else` chains do
   - the first link the preprocessor leaves in place, the rest nested under
   `not (feature = …)` of the arm just taken - which reproduces C's control flow in every one of the
   four configurations and keeps C 818-820's empty compound statement.
4. **`lw_raise_all_pop_up_widgets`'s split condition** (C 903-909) is held in one local the port
   names (`managed_p`) with one `#[cfg]` spelling per arm - the `xlwmenu.rs` `frobbed_ok` shape for
   a condition the preprocessor splits.
5. **`lw_window_is_in_menubar`'s one `return`** is spelled as one `return` per arm (C's text is only
   well formed when at most one of `USE_LUCID`/`USE_MOTIF` is defined - its `;` sits inside the
   first arm - which is exactly what C 52-54's `#error` requires); the two arms are otherwise C's
   expression, `menubar_widget`'s pointer truth test written out (the `xlwmenu.rs` Deviations 7
   spelling).
6. **C 60-94's static forward declarations are deleted** (PORTING.md 0.1-12); Rust items are
   order-independent, and the table's `64-94` row is the record.
7. **The `#if 0` regions are not transcribed** (C 316-329's `printf` form of `EXPLAIN`, C 443-447
   inside `merge_widget_value`): dead in every configuration, recorded here and in the unit's
   comment (the `src/character.rs` 91-97 / `lwlib-Xlw.md` 1 precedents).
8. **C's chained assignments** (`wv->name = wv->value = wv->key = (char *) 0xDEADBEEF;`,
   `copy->lname = copy->lkey = Qnil;`) are one assignment per line, in C's right-to-left order (the
   same stores to the same three slots).
9. **`safe_strcmp`'s** `!!s1 ^ !!s2` and `(s1 && s2) ? … : s1 ? False :
   !!s2` are C's three-way test written with Rust's `if`: C's `&&`/`?:` chain over two pointers has
   no Rust spelling that keeps the token order, and the two `bool`-to-`Boolean` sites carry C's
   values (`True`/`False`) exactly.
10. **The two `separator_names` tables are `const`, not `static`**: `*const c_char` is not `Sync`,
    which an immutable `static` requires, and C's `static` here is read-only.  Same names, same
    order, same member values.
11. **`#ifndef max` is the port's `macro_rules! max`** with C's fold, in the unit (the
    `src/alloc.rs` 129 shape); `EXPLAIN`'s live arm (`((void) 0)`) is a `macro_rules!` of C's name
    expanding to `()`.  The token rule reads `max`'s `?`/`:` as punctuation and says nothing about
    it; `EXPLAIN`'s `0` is the one row this file needs in `docs/ports/macro-body-notes.tsv`
    (`dialog_spec_p`'s `switch`/`case`/`default` is the other), both appended by this lane.
12. **The `<X11/*>` surface stands in this unit** (P-88 clause 1): no mirror in this tree carries
    `<X11/Intrinsic.h>`, `<X11/Xlib.h>`, `<X11/StringDefs.h>`, `<X11/Xaw{,3d}/Paned.h>`, and this
    machine has none of them, so every width, offset and resource name is C's spelling with the
    value **UNVERIFIED** (the `lwlib-Xaw.rs` /`lwlib-Xm.rs` reading).  `XtWidgetBeingDestroyedP`
    comes from `include/lwlib-utils.rs` and is *not* redeclared here.
13. **The two mirrors on this unit's include graph end their P-39 module with
    `pub use …_decls::*;`, not a private `use`**: `lxwmenu.rs` `include!`s `include/lwlib-int.rs`
    inside its own `xlwmenu_decls` module, and a private glob import is not re-exported to a
    downstream glob (`## Evidence` ④; the `E0425` the `xlwmenu.rs` flip caught).  The four sibling
    mirrors in this directory keep the private form because their consumers `include!` them at a
    crate root.
14. **Two names the mirrors do not carry are declared in the unit**: `dupstring` (`lisp.h` 5555; the
    `lwlib-Xm.rs` 2828 reading) and `strnlen` (`<string.h>`; the `lib-src/pop.rs` 307 shape).

## Owed

1. **The integrator, not this lane, owns three baselines this file needs** (the P-86 ⑩ situation,
   same as `docs/ports/androidmenu.md`):
   - `docs/ports/line-drift-tolerance.tsv`: a row for stem `lwlib`.  The port is **1,641 lines**
     against C's 1,330 (**+23.4%**, the 5% default allows 1,396), and the difference is the same
     fixed cost every sibling in this directory declares: the in-unit `<X11/*>` surface (~90 lines,
     a citation per declaration), the per-item `#[cfg]` line (79 of them), the per-block C-line
     comment, and the spelled-out casts/`match` arms.  Measured neighbours: `lwlib-Xlw` 0.80,
     `lwlib-Xm` 0.45, `lwlib-utils` 1.56.
   - `docs/baseline/port-lines-pending.tsv`: its `lwlib/lwlib.c 1330 3 133 known-unported target`
     row is now stale (informational, not a failure) and `--write-pending` is the integrator's call.
   (The third book, `docs/ports/macro-body-notes.tsv`, **is** in this lane's whitelist - append
   only - and the two rows `dialog_spec_p` and `EXPLAIN` need are there.)
2. **`use_xaw` is not in `femacs/xtask/Cargo.toml`'s `[features]` table** (generated from the
   tree), and `docs/baseline/feature-coverage.tsv` has no row for it.  Until the table is
   regenerated the only way to compile the Xaw arms is `--cfg 'feature="use_xaw"'` on the command
   line, which is what the probe above does.
3. **`src/xfns.rs` is not probed this round.**  It includes `lwlib/include/lwlib.rs` at its crate
   root (C 99), so the P-39 change reaches it the same way it reaches `src/widget.rs` - whose flip
   (④) shows that path resolving and fails only on its own unmeasured surface.  `xfns.rs`'s arm is a
   slice of its own lane, not a condition on this change; the reading is owed, not claimed.
4. **`first_child`'s declaration is this unit's own** (C 884-886): `lwlib-Xm.c` defines it under
   `use_motif` and `lwlib-Xm.h` does not declare it, so nothing else can supply it.
5. **`docs/ports/macro-body-notes.tsv` line 1554** (the `textconv` lane's `TEXTCONV_DEBUG` row,
   landed concurrently) says "the `EXPLAIN` row above is the same shape": this lane's `EXPLAIN` row
   is now in that file but *below* it.  The reference is satisfied and the wording is that lane's.
6. **The X11 surface is owed a mirror** (P-90 clause 1): `src/widget.rs`'s comment already names the
   `<Xt/Xaw>` mirror that `src/xfns.rs`, `src/xterm.rs` and the `lwlib/` units would share, and this
   unit's copy is the one to move then.

## Not claimed

- **Nothing here is behaviourally verified.**  No X11/Xt/Motif/Xaw header is on this machine and the
  frozen configuration compiles neither side of this file, so the readings are: the frozen object
  (empty, ①), the three arm flips and the refusing configuration (②), the consumers of the two
  mirrors this round changed (④) and the frozen re-builds of the six units the mirrors sit in (⑤).
  There is no differential run, no layout assertion for any type in this file (the layout probe
  reaches `src/include`/`lib/include` only), and no live widget was ever created.
- **Every X11/Xt/Xaw width, offset, enum value and resource name in this port is C's spelling with
  the value `UNVERIFIED`** (the port has no `<X11/*>` mirror; the surface stands in this unit).
  The one measured number is the `XEvent` union's 192-byte size, which is the frozen target's own
  Xlib and which `lwlib-Xm.md` measured.
- **`use_xaw` is not registered** in `femacs/xtask/Cargo.toml`'s `[features]` table (`## Owed` 2), so
  the Xaw arm is only compilable through a command-line `--cfg`, which is what ② uses.
- **The line-count reading is a failure in `extract-abi.py --check` until the integrator writes the
  tolerance row** (`## Owed` 1): the port is 1,641 lines against C's 1,330.

