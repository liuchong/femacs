# emacs/src/pgtkselect.c (1975 lines) — the whole file

> **One of P-86's class: the frozen configuration compiles nothing here.**  `pgtkselect.o` sits in
> `PGTK_OBJ` (`emacs/src/Makefile.in` 325, `PGTK_OBJ=@PGTK_OBJ@`; `configure.ac` 3878 fills it —
> `PGTK_OBJ="pgtkfns.o pgtkterm.o pgtkselect.o pgtkmenu.o pgtkim.o xsettings.o"` — in the branch
> `window_system = pgtk`, whose `AC_DEFINE` is `configure.ac` 3176's `HAVE_PGTK`), and
> `docs/baseline/frozen-build-sources.txt` (106 source lines) lists no pgtk object, so the file is not
> in this target's source list at all and the text can never be *compiled* here.  As with
> `pgtkterm.c`, `pgtkfns.c` and `xselect.c`, what the earlier `status.md` 3.31 note measured (not in
> the frozen source list, empty checklist, 336-byte object) is unchanged and still true; P-86 ①
> replaces the *conclusion* — "the port is the empty unit plus the note that names the gate" — with
> **transcribe the text, both arms, no stub**.  Port: `femacs/src/pgtkselect.rs` (2329 lines, from
> the 11 lines of the placeholder), carrying **C 1-1975**.
>
> **The file gate is `have_pgtk`** (`HAVE_PGTK`, `configure.ac` 3176), on every item *and on the
> include graph* (P-86 ⑧ — the reading Evidence ① records is a 336-byte empty object for exactly
> that reason).  **This file has no `#if`/`#ifdef` of its own to carry arms for**: the whole 1975
> lines sit under one build condition, and `python3 tools/check-cond-eval.py src/pgtkselect.c` reads
> `3 conditions, 0 unknown, 0 disagreements [VERIFIED]` — the three conditions being the file's own
> `#if 0` blocks (C 41-44, C 435-439, C 864-882), which are dead in *every* configuration
> (`docs/status.md` 3.55) and are therefore recorded at their positions, with no arm of their own.
> The file is on `docs/baseline/config/cond-eval-verified.txt` 345.  P-86 ⑥'s "file gate + every
> sub-gate" therefore reduces to the file gate here; the two arm readings below are the gate alone
> (narrow) and the gate plus the implications a real pgtk build has
> (`have_window_system`, `use_gtk`, `have_gtk3`).
>
> **The GTK/GDK surface's single home is `femacs/src/include/gtk.rs`** (P-90 ①②).  C's text does not
> itself `#include <gtk/gtk.h>`: the names arrive through C 23's `"pgtkterm.h"`, whose 31 is
> `#include <gtk/gtk.h>` under its own `HAVE_PGTK`.  That edge is owed by `include/pgtkterm.rs`
> (`## Owed` of `docs/ports/include-pgtkterm.md`), so — the `src/pgtkfns.rs` C 28 stand-in shape —
> this unit carries the edge itself at C 23's chain.  What the mirror does *not* carry is the GDK
> **selection** surface this file is made of: the nine atoms/values, `GdkPropMode`,
> `GtkTargetEntry`/`GtkTargetList` and the seventeen entry points in
> `femacs/src/pgtkselect.rs` 118-292 are declared in-unit at C's `#include` position (PORTING.md
> P-88 ①) and their move into `include/gtk.rs` is owed (P-90 ③; `## Owed` 1/2).
>
> **Acquisition: this batch is `text-only` (P-86 ③⑵).**  Measured on this machine: `ls -d
> /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0` → both `No such file or directory`;
> `pkg-config` is not installed; no `gtk/gtk.h` or `gdk/gdk.h` under `/opt/homebrew`, `/usr/local` or
> `/Library`.  So there is no GTK/GDK tree to compile the arm against, every GDK citation below names
> the header alone (no line), every one of its values is a placeholder, and no layout assertion is
> written for `GtkTargetEntry` (the `include/gtk.rs` 20-30 rule).  **GLib and cairo are present**
> (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0), so `G_MAXUINT32` quotes the header *and the line*
> (`glib/gtypes.h` 98).  The three records this file defines are Emacs's own and their layouts are
> clang readings on this machine (Evidence ⑦).  The missing toolchain is named in `## Owed`.
>
> **Two mirror gaps, both named:** `include/gtk.rs` declares `GdkEventProperty` and
> `GdkEventSelection` as zero-sized placeholders (pgtkterm.h 571-572 only ever took a pointer to
> them), while C 953-975 and C 1791-1801 *read* `event->window`/`->atom`/`->state` and
> `event->selection`/`->property` — six errors in both arm readings, all six caused by those two
> records (`## Owed` 3); and C's `dpyinfo->display` is the mirror's anonymous-union alias `gdpy`
> (`include/pgtkterm.rs` 177-180, `## Deviations` 2).  A third gap the arm reads without: C 1129's
> `gdk_atom_intern_static_string ("ATOM_PAIR")` in `pgtk_get_window_property` needs the same GDK
> surface (declared in-unit here).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`PGTK_OBJ` ⇔ `window_system = pgtk` ⇔ `HAVE_PGTK`) | `feature = "have_pgtk"`, on every item *and* on C 20-27's include graph | a pure GTK3 (pgtk) build |
| C 41-44 `#if 0` (the `expect_property_change` forward declaration) | no arm: C's own dead text in every configuration (`docs/status.md` 3.55), recorded, dropped as a forward declaration (PORTING.md 0.1-12) | never |
| C 435-439 `#if 0` (`static int prop_location_identifier;`) | no arm: dead text, recorded | never |
| C 864-882 `#if 0` (`expect_property_change`'s definition) | no arm: dead text, recorded | never |

There is no other condition in the file: `grep -n '^\s*#' src/pgtkselect.c` finds only C 20-27's seven
`#include`s, C 116's `#define LOCAL_SELECTION`, and those three `#if 0`s.

## C line ranges

Every non-blank line of the reference is inside exactly one row (blank lines between blocks need no
row; `tools/check-dead-file-coverage.py` is the machine check).  "file gate" means
`#[cfg (feature = "have_pgtk")]` on the port's items and on the include edge that carries it.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the licence head | dropped (PORTING.md 0.1-19: no licence block in the port) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 22-27 | the six Emacs includes (`lisp.h`, `pgtkterm.h`, `termhooks.h`, `keyboard.h`, `atimer.h`, `blockinput.h`) | file gate; the mirror graph (`lisp.rs`, `pgtkterm.rs`, `gtk.rs` at pgtkterm.h 31, `termhooks.rs`, `keyboard.rs`, `atimer.rs`, `blockinput.rs`) |
| 29-31 | the "deliberately does not implement INCR" note | file gate (comment) |
| 33-35 | "Advance declaration of structs." + `struct selection_data;` / `struct prop_location;` | file gate; the two tags are defined at C 323-337 / C 424-433 (P-72 ③) |
| 37-56 | the ten static forward declarations, with C 41-44's `#if 0` | file gate; dropped (PORTING.md 0.1-12), the `#if 0` recorded |
| 58-67 | `pgtk_nalloc` | file gate |
| 69-114 | `frame_for_pgtk_selection` | file gate |
| 116-117 | `#define LOCAL_SELECTION` | file gate |
| 119-142 | `symbol_to_gdk_atom`, `gdk_atom_to_symbol` | file gate |
| 146-233 | `pgtk_own_selection` | file gate |
| 235-312 | `pgtk_get_local_selection` | file gate |
| 314-321 | `pgtk_decline_selection_request` | file gate |
| 323-337 | `struct selection_data` | file gate (layout measured, Evidence ⑦) |
| 339-358 | `struct pgtk_selection_request` | file gate (layout measured, Evidence ⑦) |
| 360-363 | the "Stack of selections" comment + `selection_request_stack` | file gate |
| 365-380 | `pgtk_push_current_selection_request` | file gate |
| 382-391 | `pgtk_pop_current_selection_request` | file gate; `unsafe extern "C" fn` (its address goes to `record_unwind_protect_void`) |
| 393-420 | `pgtk_selection_request_lisp_error` | file gate; `unsafe extern "C" fn` (same) |
| 422-433 | the "Keep a list of the property changes that are awaited." comment + `struct prop_location` | file gate (layout measured, Evidence ⑦) |
| 435-439 | `#if 0` / `static int prop_location_identifier;` / `#endif` | recorded (dead in every configuration) |
| 441-445 | `property_change_reply`, `property_change_reply_object`, `property_change_wait_list` | file gate |
| 447-456 | `set_property_change_object` | file gate |
| 459-525 | `pgtk_reply_selection_request` | file gate |
| 529-662 | `pgtk_handle_selection_request` | file gate |
| 664-718 | `pgtk_convert_selection` | file gate |
| 722-774 | `pgtk_handle_selection_clear` | file gate |
| 776-783 | `pgtk_handle_selection_event` (non-`static`) | file gate |
| 785-842 | `pgtk_clear_frame_selections` (non-`static`) | file gate |
| 844-854 | `waiting_for_other_props_on_window` | file gate |
| 856-882 | the `expect_property_change` comment + C 864-882's `#if 0` definition | recorded (dead in every configuration) |
| 884-903 | `unexpect_property_change` | file gate |
| 905-915 | `wait_for_property_change_unwind` | file gate; `unsafe extern "C" fn` (its address goes to `record_unwind_protect_ptr`) |
| 917-947 | `wait_for_property_change` | file gate |
| 949-978 | `pgtk_handle_property_notify` (non-`static`) | file gate |
| 980-989 | `pgtk_display_selection_waiting_message` | file gate; `unsafe extern "C" fn` (its address goes to `start_atimer`) |
| 991-1000 | `pgtk_cancel_atimer` | file gate; `unsafe extern "C" fn` (its address goes to `record_unwind_protect_ptr`) |
| 1003-1078 | the `reading_selection_*` statics + `pgtk_get_foreign_selection` | file gate |
| 1080-1097 | "Subroutines of `pgtk_get_window_property_as_lisp_data`" + `pgtk_size_for_format` | file gate |
| 1099-1183 | `pgtk_get_window_property` | file gate |
| 1185-1239 | `pgtk_get_window_property_as_lisp_data` | file gate |
| 1243-1386 | the conversion-table comment + `selection_data_to_lisp_data` | file gate |
| 1388-1402 | `cons_to_gdk_long` | file gate |
| 1404-1544 | `lisp_data_to_selection_data` | file gate |
| 1546-1578 | `clean_local_selection_data` | file gate |
| 1580-1604 | `DEFUN ("pgtk-own-selection-internal", …)` | file gate |
| 1606-1676 | `DEFUN ("pgtk-get-selection-internal", …)` | file gate |
| 1678-1719 | `DEFUN ("pgtk-disown-selection-internal", …)` | file gate |
| 1721-1745 | `DEFUN ("pgtk-selection-owner-p", …)` | file gate |
| 1747-1783 | `DEFUN ("pgtk-selection-exists-p", …)` | file gate |
| 1785-1801 | the `pgtk_handle_selection_notify` comment + the function (non-`static`) | file gate |
| 1804-1855 | the "Drag and drop support" banner + `DEFUN ("pgtk-register-dnd-targets", …)` | file gate |
| 1857-1869 | `DEFUN ("pgtk-drop-finish", …)` | file gate |
| 1871-1881 | `DEFUN ("pgtk-update-drop-status", …)` | file gate |
| 1883-1975 | `syms_of_pgtkselect` (the 17 `DEFSYM`s, the 8 `defsubr`s, the 6 `DEFVAR_*` variables and the two `staticpro`s) | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/pgtks-lane/cargo cargo run -q -p xtask -- build-tu src/pgtkselect.c`
→ `built src/pgtkselect.rs -> ../build/femacs/pgtkselect.o`, **exit 0**,
`build/femacs/pgtkselect.o` **336 bytes**, `md5 dbed68837694579185377dbb0821ab62`.  That is an empty
object: every item sits behind `have_pgtk`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  **This is not a proof of the new text**; it is the
P-86 ⑧ reading that says the gate is on the include graph as well as on the items.
**Budget: the mandate allowed three `build-tu` runs and this batch used three** - the first two
while the port was being written (the second re-materialised after an edit) and the third after the
citation pass, whose reading is the one recorded here (identical: comments do not reach codegen).

**② The arm readings (three, `text-only`, P-86 ③⑵).**  The `have_pgtk` arm cannot be compiled on a
GTK target here (no GTK tree), so the readings below are the unit's own text compiled with the arm's
cfgs by rustc, on the root `build-tu` materialises
(`build/femacs/units/pgtkselect/_root.rs`; `xtask`'s `materialize` rewrites the `include!` lines in
place, so its line numbers are the port's own):

```
$ rustc --edition 2024 --crate-type=lib --emit=metadata -O --crate-name unit_probe \
    build/femacs/units/pgtkselect/_root.rs -o /tmp/pgtks-lane/narrow.meta --cfg 'feature="have_pgtk"'
exit 1; 14 lines matching ^error  (13 errors + rustc's `aborting due to 13 previous errors`)

$ ... + --cfg 'feature="have_window_system"' --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"'
exit 1;  8 lines matching ^error  (7 errors + the abort line)
```

| # | error | span | cause (owner) |
| --- | --- | --- | --- |
| 1-4 | `E0609 no field window/atom/state on GdkEventProperty` (narrow 4, wide 4) | `femacs/src/pgtkselect.rs` 1318/1324/1325/1327 (C 957-964) | **mirror**: `include/gtk.rs` 1660-1672 holds `GdkEventProperty` as `_private: [u8; 0]` — C's member reads have no field (`## Owed` 3) |
| 5-6 | `E0609 no field selection/property on GdkEventSelection` (narrow 2, wide 2) | `femacs/src/pgtkselect.rs` 2161/2165 (C 1796-1800) | **mirror**: `include/gtk.rs` 1664-1670's placeholder — the same gap (`## Owed` 3) |
| 7 | `E0658 c_variadic` (both) | `include/lisp.rs` 3761 | **mirror**: `verror`'s `VaList` parameter (the row `docs/ports/pgtkterm.md` and `docs/ports/pgtkfns.md` record) |
| 8-10 | `E0425 cannot find type XrmDatabase` ×2, `XRectangle` ×1 (**narrow only**) | `include/pgtkterm.rs` 199/210/642 | **mirror/config**: `include/xlib.rs`'s X11 records are `have_x_windows`-only (status 3.542), and `include/pgtkterm.rs`'s own copy names them without that arm; the wide reading removes them (`include-pgtkterm.md`) |
| 11-13 | `E0425 cannot find type GtkCssProvider` ×3 (**narrow only**) | `include/pgtkterm.rs` 289/290/291 | **mirror/config**: `include/gtk.rs` gates that GTK3 class on `have_gtk3`, which a real pgtk build always has (the second reading is what shows it gone) |

**Nothing in either reading is an error of this slice's own statement text**: the six unit-span rows
are the two mirror records' missing members, and the seven mirror-span rows are the two other
mirrors' own gaps.

**③ The mutation test (the reading is not vacuous).**  One type error injected into the unit's own
text (`pgtk_nalloc`, C 61-67: `let bad: *mut c_void = 5i32; return bad;` for C 66's `return`):

```
exit 1;  9 lines matching ^error  (8 errors + the abort line)
error[E0308]: mismatched types
    --> build/femacs/units/pgtkselect/_root.rs:353:26
353 |   let bad: *mut c_void = 5i32;   /* MUTATION (injected type error) */
```

Exactly **one** new error, naming the injected line; the other seven are byte-identical to the wide
reading.  Restored, and verified: `shasum` of the root is `dc871b752e6ab55e9cc94e86b5b1faa94c3e33e5`
before and after, identical.

**④ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/pgtkselect.c
femacs/src/pgtkselect.rs` → `FAIL`, and after the fourteen `macro-body-notes.tsv` rows this batch
appended (one append, at the end of the file) the whole output is the expected lone line:

```
- line count drifted 17.9%: C 1975 vs Rust 2329
```

The measured drift is **(2329 − 1975) / 1975 = 17.92 %**; the tolerance row is the integrator's
(P-86 ⑩'s pairing rule).  Before this batch's rows the same command printed the drift line plus
thirteen `- ` token rows, and a rehearsal against a scratch notes file holding the first twelve of
the fourteen rows below still printed two of them (`wait_for_property_change_unwind`'s `0` and
`Fpgtk_register_dnd_targets`'s `continue`) — which is why the block has fourteen.  They are all this
file's own sanctioned spelling:

| function | token(s) | why the token cannot reappear |
| --- | --- | --- |
| `pgtk_own_selection`, `pgtk_selection_request_lisp_error`, `pgtk_clear_frame_selections`, `Fpgtk_disown_selection_internal`, `Fpgtk_selection_exists_p` | `display` | C's `dpyinfo->display` is the anonymous union's second member; `include/pgtkterm.rs` 177-180 keeps the first member's name `gdpy` and records the alias (`## Deviations` 2) |
| `pgtk_handle_selection_request`, `pgtk_handle_selection_clear`, `pgtk_clear_frame_selections`, `pgtk_get_foreign_selection`, `Fpgtk_disown_selection_internal` | `cons_to_integer` | C's `CONS_TO_INTEGER (…, guint32, …)` is the direct `cons_to_unsigned (…, G_MAXUINT32 as uintmax_t) as guint32` (`## Deviations` 3) |
| `pgtk_handle_selection_request` | `goto` | C's six `goto DONE` and the `DONE:` label become the `'done` loop (PORTING.md 2.2) |
| `pgtk_reply_selection_request`, `Fpgtk_register_dnd_targets` | `continue` | the port's for-loop spelling carries the stepper after the body, so C's jump-to-step is the block's `break` (PORTING.md 2.2) |
| `pgtk_size_for_format` | `switch`, `case`, `default` | the `switch` becomes a `match` with `_ => emacs_abort ()` (PORTING.md 2.2) |
| `pgtk_get_window_property` | `typeof` | C 1158's GNU `typeof (ldata)` cast is the port's `data as *mut c_ulong` |
| `syms_of_pgtkselect` | `doc` | the seven `DEFVAR_*` calls' `doc:` keyword is the port's third macro argument |
| `wait_for_property_change_unwind`, `Fpgtk_selection_exists_p` | `0` | C's pointer-null literal is `std::ptr::null_mut ()` / `.is_null ()` |

`docs/ports/macro-body-notes.tsv` is append-only for the porter and shared with the other lanes, so
the fourteen rows were appended once, in one block, as the then-last lines of the file (1523-1536;
the other lanes have appended behind them since), each with its reason; nothing else in that file was
touched.

**⑤ The citation shape.**  PORTING.md asks for "a `// C <line>` citation per block and a `/// C:`
line per declaration you place in the unit", and that is what this port carries: one citation per
item (on its signature line: the 26 functions, the eight `DEFUN!`s' block comments,
`syms_of_pgtkselect`), one per declaration (each struct field, each of the nine statics, each of the
in-unit GDK constants and records) and one `/* C a-b */` marker per C comment block.  The port does
**not** carry the per-statement line numbers `src/xselect.rs` does: they are not part of the
requirement, no gate checks them, and a hand pass over 554 of them at this size could not be
confirmed line by line - the finer map is this ledger's `## C line ranges` table instead.  Every
citation the port does carry was verified against the reference (the 23 rows a naive
first-identifier check reports are all item signatures, whose cited C line is the item's own
`static void` / `void` line).

**⑥ The self-checks (whole-tree last lines).**

```
python3 tools/check-cond-eval.py src/pgtkselect.c
  -> src/pgtkselect.c: 3 conditions, 0 unknown, 0 disagreements [VERIFIED]
python3 tools/check-declarations.py
  -> 17 declaration sites (0 forbidden, 17 allowed literals), 0 failures        (no pgtkselect row)
python3 tools/check-mirror-dupes.py
  -> 18 duplicate declaration(s), 17 ratchet row(s), 1 failure(s)              (the one failure is
     `femacs/src/gmalloc.rs`; no pgtkselect row -> no new failure for this file)
python3 tools/check-citations.py
  -> 676 file(s) scanned, 0 drift, 0 range, 1522 external, 36 allowed, 0 stale (one `external` row
     is this file's: `femacs/src/pgtkselect.rs:192 glib/gtypes.h 98`)
python3 tools/check-dead-file-coverage.py
  -> 116 ledger(s) bound (a C range table), 0 failure(s); this file's row is
     `info docs/ports/pgtkselect.md: 55 range row(s) cover src/pgtkselect.c (1975 lines; 65 blank
      line(s) between blocks need no row)`
python3 tools/check-ledger-ticks.py
  -> 233 ledger(s) bound, 0 failure(s)                                        (no pgtkselect row)
python3 tools/check-port-lines.py
  -> 21 file(s) under ten percent, 23,883 reference line(s); 21 ratchet row(s), 0 new, 0 stale
python3 tools/check-target-names.py
  -> 629 .rs files, 591 map targets, 0 finding(s)
```

**⑦ The layout readings (`clang`, our own three records).**  The three records this file *defines*
have only pointer/integer members, so their layout is measurable on this machine even though GDK is
not here (`GdkAtom` is `typedef struct _GdkAtom *GdkAtom;`, a pointer).  Probe
(`/tmp/pgtks-lane/layout.c`, `clang -o /tmp/pgtks-lane/layout … && /tmp/pgtks-lane/layout`):

```
prop_location          sizeof=48  id=0 display=8 window=16 property=24 desired_state=32 arrived=36 next=40
selection_data         sizeof=64  data=0 size=8 format=16 type=24 nofree=32 property=40 wait_object=48 next=56
pgtk_selection_request sizeof=48  last=0 dpyinfo=8 request=16 converted_selections=24 conversion_fail_tag=32 converted=40
sizeof_GdkAtom=8 sizeof_bool=1
```

Those are the numbers in the port's `const _:` assertions.  **No layout assertion is written for
`GtkTargetEntry`**: its three members are GTK's declaration, not a measured record (`## Owed` 4).

## Deviations

1. **The three `#if 0` blocks carry no arm.**  C 41-44 (a forward declaration of
   `expect_property_change`), C 435-439 (its counter) and C 864-882 (its definition) are dead in every
   configuration (`docs/status.md` 3.55, the `alloc.rs`/`android.rs`/`character.rs` reading), so each
   is recorded at its C position in the port and not transcribed; C 41-44 is dropped anyway as a
   forward declaration (PORTING.md 0.1-12).  This is why the file has **no sub-gate** and only the
   file gate (P-86 ⑥).
2. **`dpyinfo->display` is the mirror's `gdpy`.**  C's anonymous
   `union { GdkDisplay *gdpy; GdkDisplay *display; };` (pgtkterm.h 154-161) is one pointer at one
   offset; `include/pgtkterm.rs` 177-180 keeps C's *first* member as the field name and records the
   alias in its own ledger, so this port reads `(*dpyinfo).gdpy` at C 170/176-190, 415, 811, 835, 1193,
   1712, 1781 and 1823.  The `display` token therefore cannot reappear (the notes rows above).
3. **C's `CONS_TO_INTEGER (x, guint32, var)` is spelled with the direct unsigned helper.**  lisp.h
   4188-4195 expands the macro, for an unsigned `type`, to
   `var = cons_to_unsigned (x, TYPE_MAXIMUM (type))`; the port's `CONS_TO_INTEGER!` trait has only the
   signed/unsigned 64-bit impls (`include/lisp.rs` 4042-4043), so the five `guint32` sites (C 577-578,
   745-746, 808, 833, 1033, 1711 - six `CONS_TO_INTEGER` sites) are written `cons_to_unsigned (…, G_MAXUINT32 as uintmax_t) as
   guint32` — C's own helper, C's own bound and C's own truncating store (the `src/charset.rs` 1178 /
   `docs/ports/inotify.md` Deviations 6-7 precedent).  The `cons_to_integer` token cannot reappear.
4. **C's `continue`s are the block exits of the port's loops.**  In `pgtk_reply_selection_request`
   (C 481) and `pgtk_register_dnd_targets` (C 1843) the port's `while`/`FOR_EACH_TAIL!` spelling puts
   the stepper *after* the body, where C's `for` increment runs *before* the next test, so C's
   `continue` is the `'step`/`'body` block's `break` — the same set of statements, in the same order,
   with the step still executed exactly once per iteration (`docs/ports/macro-body-notes.tsv`).
5. **Twelve local declarations get a zero initializer C does not write** (PORTING.md 1.5), each
   carrying a `// 1.5` marker in the port: two C only ever assigns (`pgtk_own_selection`'s `gtargets`
   C 159, `pgtk_get_foreign_selection`'s `delayed_message` C 1024), one C fills by pointer
   (`pgtk_handle_selection_request`'s `cs` C 599, `std::mem::zeroed ()`), and nine whose addresses C
   passes to a callee (`local_selection_time` C 534 and C 732, `time` C 792, `timestamp` C 1693,
   `length`/`actual_format`/`actual_type` C 1103/1107, and the three out-parameters
   `actual_type`/`actual_format`/`actual_size` C 1187-1189).
6. **`pgtk_handle_selection_request`'s `goto DONE` becomes the `'done` loop and the `DONE:` section
   follows it** (PORTING.md 2.2), with **C's own order kept**: C 647-648's `if (pushed) … converted =
   true;` runs *before* C 650-653's reply/decline (the opposite of `xselect.c`, so the port does not
   copy that file's order).
7. **`G_MAXUINT32` is a reading, not a memory** (`glib/gtypes.h` 98, GLib 2.90.0 on this machine);
   GDK's atoms and `GdkPropMode` values are **placeholders**, and the port says so at each one
   (`// placeholder, not GDK's value`).  GDK's predefines happen to equal X11's for three of the
   atoms, but nothing here claims that: they are UNVERIFIED until a GTK toolchain is present.
8. **C 1129's `gdk_atom_intern_static_string ("ATOM_PAIR")`** (and C 1135/1321/1332's other three)
   are the port's `c"ATOM_PAIR".as_ptr ()`: the same pointer, spelled as the C string literal the
   header's `const gchar *` parameter takes.

## Owed

1. **The GDK selection surface moves into `include/gtk.rs`.**  The nine atoms/values, `GdkPropMode`,
   `GtkTargetEntry`, `GtkTargetList` and the seventeen entry points
   (`femacs/src/pgtkselect.rs` 118-292) are this unit's in-unit segment (P-88 ①); P-90 ①② make the
   surface's single home `include/gtk.rs`, so they move there in the next mirror round (P-88 ⑤'s
   move-back shape) and this segment disappears.  Owner: the mirror round.  The GDK values stay
   UNVERIFIED until then.
2. **`include/pgtkterm.rs`'s C 31 edge** (`#include <gtk/gtk.h>`) would put `include/gtk.rs` on every
   pgtk unit's graph and retire this unit's stand-in edge at C 23's chain
   (`femacs/src/pgtkselect.rs` 90-91).  Owner: `docs/ports/include-pgtkterm.md` (its measured blocker
   set is the unlanded copies in `src/image.rs`/`src/xwidget.rs`).
3. **`include/gtk.rs`'s two event records need their first member-reader's members.**
   `GdkEventProperty` (gtk.rs 1660-1672) must carry `type`/`window`/`send_event`/`atom`/`time`/`state`
   and `GdkEventSelection` (gtk.rs 1664-1670) `type`/`window`/`send_event`/`selection`/`target`/
   `property`/`time`/`requestor`, because C 957-964 and C 1796-1800 read five of them — the status
   3.481 `GdkEventKey` shape.  Until then this unit's arm has six `E0609`s (Evidence ②, rows 1-6) and
   the arm cannot be called clean on the GTK side.  Owner: the mirror's owner (`include/gtk.rs`).
4. **A GTK/GDK toolchain has to be present** to (a) measure `GtkTargetEntry`'s layout, (b) replace
   every placeholder value above with GDK's own, and (c) compile the arm against real headers rather
   than the mirror alone.  Named absence: no `gtk/gtk.h`/`gdk/gdk.h` under `/opt/homebrew`,
   `/usr/local`, `/Library`, and no `pkg-config`.  Owner: the environment.
5. **`docs/ports/line-drift-tolerance.tsv`** needs one row for this file: `pgtkselect`, tolerance
   ≥ 0.18, basis `total` — C 1975 vs the port's 2329 (Evidence ④).  That book is not on the porter's
   writable list (P-86 ⑩).  Owner: the integrator.
6. **`docs/port-map.md` 510's Rust count (11) and `docs/port-inventory.md` 577** still say the
   placeholder's line count (the port is 2329); both are generated (`tools/gen-port-map.py`,
   `tools/gen-port-inventory.py`).  Owner: the integrator.
7. **`docs/baseline/port-lines-pending.tsv` needs nothing.**  The ratchet reads `0 new, 0 stale`
   (Evidence ⑥): the port is above the ten-percent threshold, so by the book's own rule this file has
   dropped out of it.  Recorded so the integrator does not go looking for a row.
8. **`include/frame.rs` 861's `FRAME_PGTK_P`** returns `false` unconditionally (the frozen, non-pgtk
   arm) while a pgtk build's `FRAME_PGTK_P` is `FRAME_OUTPUT_DATA (f)->type == output_pgtk`; the port
   keeps C's token and the reading stands as the mirror's (the `docs/ports/pgtkfns.md` Owed 5 row for
   `FRAME_SCALE_FACTOR` is the same class).  Owner: `docs/ports/include-frame.md`.

## Not claimed

- **No behaviour was executed anywhere.**  There is no GTK tree to compile the arm against and no
  PGTK target to run it on; the two arm readings are rustc name-resolution/type readings on a
  `text-only` configuration (P-86 ③⑵).  Nothing here says a selection request, a MULTIPLE conversion,
  a drag-and-drop registration or a `syms_of_pgtkselect` run has ever worked.
- **No symbol-level or ABI claim.**  The frozen object is 336 bytes of nothing, so no symbol of this
  file exists in any built artifact; `check-symbols.py`/`nm` comparisons are not part of this batch's
  evidence and could not be.
- **The GDK/GDK values are not verified.**  Every GDK atom, mode and prototype in the in-unit segment
  is a placeholder read from no header (no GDK header exists here).  `G_MAXUINT32` is the one value
  measured, and it is GLib's, not GDK's.
- **The three `#if 0` blocks are not ported** — by C's own decision, not this port's (Deviations 1).
- **The line-count ratio is not a completeness claim.**  P-86 ②'s coverage evidence is the C
  line-range table above (machine-checked by `check-dead-file-coverage.py`), and every non-blank
  reference line is in exactly one row; the 17.9 % drift is a spelling budget, not a measure of work
  left.

- **One declaration left this unit (status 3.725).**  `gdk_window_get_display` (gdk/gdkwindow.h),
  declared here from the start for C 958's call, is `include/gtk.rs`'s row now that the pgtkfns
  eleventh slice needed it there; the local copy is dropped under P-26's one spelling per construct
  and the call site is unchanged.  Measured: this unit's own probe (`have_pgtk +
  have_window_system`) reads **9 errors / 8 warnings** on both sides of the drop, so the change is
  neutral for this file, and `check-mirror-dupes.py` returns to 16/16/0.

- **A second declaration left this unit (status 3.727).**  `gtk_get_current_event_time`
  (gtk/gtkmain.h), declared here for C 157's call, is `include/gtk.rs`'s row now that the pgtkfns
  thirteenth slice needed it there; the local copy is dropped under P-26's one spelling per
  construct with the C 157 call site unchanged.  Measured: this unit's own probe (`have_pgtk +
  have_window_system`) reads **9 errors / 8 warnings** on both sides of the drop, and
  `check-mirror-dupes.py` returns to 16/16/0.
