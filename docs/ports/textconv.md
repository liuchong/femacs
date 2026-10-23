# emacs/src/textconv.c (2640 lines) - whole file, C 1-2640

> **One of P-86's class: the frozen configuration compiles nothing here.**  `textconv.o` joins the
> build only through `WINDOW_SYSTEM_OBJ` (`emacs/src/Makefile.in` 304/468), and `configure.ac`
> 7521-7526 appends it under `test "$window_system" = "x11" || test "$REALLY_ANDROID" = "yes"` -
> the same `if` that `AC_DEFINE`s `HAVE_TEXT_CONVERSION` (7524).  The frozen target has no window
> system at all (`build/emacs-ref/src/Makefile` 304 is `WINDOW_SYSTEM_OBJ=`, empty;
> `docs/baseline/config/darwin-arm64.macros.tsv` 1645/1678 record `HAVE_TEXT_CONVERSION` and
> `HAVE_WINDOW_SYSTEM` as `undef`; `docs/baseline/frozen-build-sources.txt` lists no `textconv.o`),
> so `extract-abi.py`'s checklist for this file is *not* empty - the file's body sits under no
> `#if` of its own - but the file gate is the build's, and this ledger carries both P-86 2's
> **C line-range table** and the readings.
>
> Port: `src/textconv.rs` (**3048 lines**, md5 `7dc3259b2fc8a11f8cdd1e7fe4812db9`), carrying **C 1-2640 with
> both arms of every `#if`**.  The reference `emacs/src/textconv.c` is md5
> `d7770484b5b17f5d32dc0580c910ff7f` (unchanged; `emacs/` is read-only).
>
> **This ledger replaces the previous revision's conclusion** ("not compiled by the frozen
> configuration, so the port is the empty unit plus the note that names the gate").  What that
> revision measured (not in the frozen source list, empty object, no live checklist entry) is
> unchanged and still true; P-86 1 replaces the conclusion - transcribe the text, both arms, no
> stub - exactly as `docs/ports/xselect.md`, `docs/ports/w32menu.md` and `docs/ports/w32font.md`
> record for their own pairs.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`WINDOW_SYSTEM_OBJ`, `configure.ac` 7521-7526: `window_system = x11` or `REALLY_ANDROID = yes`) | `feature = "have_text_conversion"` on every item **and on the include graph** | an X11 or Android configuration; `HAVE_TEXT_CONVERSION` is `undef` in the frozen table (macros.tsv 1645), which is why the frozen object is empty |
| C 45 `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` | `all (feature = "have_text_conversion", feature = "have_android", feature = "not_android_stubify")` for the recorded text | the arm's body is C 46-52's `#if 0`, dead in **every** configuration; `HAVE_ANDROID` is `undef` (macros.tsv 1240), `ANDROID_STUBIFY` is `undef` (macros.tsv 78) |
| C 55 `#ifndef TEXTCONV_DEBUG` | `not (feature = "textconv_debug")` is the live arm (the `((void) 0)` fallback); `feature = "textconv_debug"` is C's other arm, which is the `#if 0` body above | `TEXTCONV_DEBUG` is not defined by the frozen table, so the fallback is live; the name is this lane's and is not registered (`## Owed` 2) |
| C 1742 `#ifdef HAVE_ANDROID` ... C 2312 `#endif` | `all (feature = "have_text_conversion", feature = "have_android")` on each of the fourteen items | an Android configuration (`REALLY_ANDROID = yes`), where the file gate holds too |

`tools/check-cond-eval.py src/textconv.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence 5): the four are C 45, 46, 55 and 1742, and the frozen evaluator and clang
agree on all of them, so this file is on `docs/baseline/config/cond-eval-verified.txt`.

## C line ranges

Rows are ascending and contiguous at the reference's block boundaries, so every line of the
reference is accounted for (the blank lines the gate excludes carry no row of their own;
`tools/check-dead-file-coverage.py` is the machine reading).

| C | block | arm |
| --- | --- | --- |

| 1-19 | the reference's opening block (the title, the five-line notice and the URL) | dropped (PORTING.md 0.1-19) |
| 20-32 | the "String conversion support." comment (the CJK/Android rationale) | file gate; carried as prose |
| 33-34 | `#include <config.h>` | ungated (every unit includes it) |
| 35-42 | the five headers (`textconv.h`, `buffer.h`, `syntax.h`, `blockinput.h`, `keyboard.h`) | file gate on every edge (P-86 8); `textconv.h`'s own chain and surface are carried in place (`## Deviations` 1) |
| 43-60 | the debug macros: C 45's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` around C 46-52's `#if 0`, and C 55's `#ifndef TEXTCONV_DEBUG` | `all (have_text_conversion, have_android, not_android_stubify)` for the recorded `#if 0` (dead in every configuration); `not (feature = "textconv_debug")` for the live `((void) 0)` fallback arm |
| 61-65 | the "The window system's text conversion interface." comment and `text_interface` | file gate |
| 66-69 | the "How many times text conversion has been disabled." comment and `suppress_conversion_count` | file gate |
| 70-80 | the `enum textconv_batch_edit_flags` comment and its two members | file gate; constants over `c_int` (PORTING.md 1.3) |
| 81-116 | the `copy_buffer_text` comment and function | file gate |
| 117-118 | the "Conversion query." banner | file gate |
| 119-132 | the `get_mark` comment and function | file gate |
| 133-155 | the `select_window` comment and function | file gate |
| 156-164 | the `restore_selected_window` comment and function | file gate; its address goes to `record_unwind_protect` (P-12) |
| 165-444 | the `textconv_query` comment and function | file gate; C 233/243's `goto escape1` is the `'escape1` block and C 299/426's `switch`es are `match`es (`macro-body-notes.tsv`) |
| 445-474 | the `sync_overlay` comment and function | file gate |
| 475-513 | the `record_buffer_change` comment and function | file gate |
| 514-524 | the `reset_frame_conversion` comment and function | file gate |
| 525-579 | the `reset_frame_state` comment and function | file gate |
| 580-598 | the `detect_conversion_events` comment and function | file gate |
| 599-785 | the `really_commit_text` comment and function | file gate |
| 786-812 | the `really_finish_composing_text` comment and function | file gate |
| 813-968 | the `really_set_composing_text` comment and function | file gate |
| 969-1026 | the `really_set_composing_region` comment and function | file gate |
| 1027-1127 | the `really_delete_surrounding_text` comment and function | file gate |
| 1128-1240 | the `locate_and_save_position_in_field` comment and function | file gate; C 1169's `goto exit` is the `'exit` block (`macro-body-notes.tsv`) |
| 1241-1264 | the `really_request_point_update` comment and function | file gate |
| 1265-1322 | the `really_set_point_and_mark` comment and function | file gate |
| 1323-1449 | the `really_replace_text` comment and function | file gate |
| 1450-1458 | the `complete_edit` comment and function | file gate; its address goes to `record_unwind_protect_ptr` (P-12) |
| 1459-1469 | the `struct complete_edit_check_context` comment and its two members | file gate; measured 16/8 (0/8) |
| 1470-1507 | the `complete_edit_check` comment and function | file gate; its address goes to `record_unwind_protect_ptr` (P-12) |
| 1508-1642 | the `handle_pending_conversion_events_1` comment and function | file gate; C 1567's `switch` is a `match` and C 1576's arm-exit `break` a labelled block (`macro-body-notes.tsv`) |
| 1643-1653 | the `decrement_inside` comment and function | file gate; its address goes to `record_unwind_protect_ptr` (P-12) |
| 1654-1741 | the `inside` object (C's function-local `static int`, C 1667) and the `handle_pending_conversion_events` comment and function | file gate; the static is spelled at file scope (`## Deviations` 3) |
| 1742-1743 | C 1742's `#ifdef HAVE_ANDROID` line | `all (have_text_conversion, have_android)`; `HAVE_ANDROID` is `undef` (macros.tsv 1240) |
| 1744-1776 | the `get_conversion_field` comment and function | `all (have_text_conversion, have_android)` |
| 1777-1799 | the `start_batch_edit` comment and function | `all (have_text_conversion, have_android)` |
| 1800-1820 | the `end_batch_edit` comment and function | `all (have_text_conversion, have_android)` |
| 1821-1846 | the `commit_text` comment and function | `all (have_text_conversion, have_android)` |
| 1847-1872 | the `finish_composing_text` comment and function | `all (have_text_conversion, have_android)` |
| 1873-1905 | the `set_composing_text` comment and function | `all (have_text_conversion, have_android)` |
| 1906-1941 | the `set_composing_region` comment and function | `all (have_text_conversion, have_android)` |
| 1942-1971 | the `textconv_set_point_and_mark` comment and function | `all (have_text_conversion, have_android)` |
| 1972-1992 | the `delete_surrounding_text` comment and function | `all (have_text_conversion, have_android)` |
| 1993-2012 | the `request_point_update` comment and function | `all (have_text_conversion, have_android)` |
| 2013-2035 | the `textconv_barrier` comment and function | `all (have_text_conversion, have_android)` |
| 2036-2072 | the `replace_text` comment and function | `all (have_text_conversion, have_android)` |
| 2073-2200 | the `get_extracted_text` comment and function | `all (have_text_conversion, have_android)`; C 2137/2163's two `goto finish` are the `'finish` block |
| 2201-2313 | the `get_surrounding_text` comment and function and C 2312's `#endif /* HAVE_ANDROID */` | `all (have_text_conversion, have_android)`; C 2262's `goto finish` is the `'finish` block |
| 2314-2325 | the `conversion_disabled_p` comment and function | file gate |
| 2326-2328 | the "Window system interface." banner | file gate |
| 2329-2355 | the `report_selected_window_change` comment and function | file gate |
| 2356-2380 | the `report_point_change` comment and function | file gate |
| 2381-2407 | the `disable_text_conversion` comment and function | file gate |
| 2408-2434 | the `resume_text_conversion` comment and function | file gate |
| 2435-2444 | the `register_textconv_interface` comment and function | file gate |
| 2445-2448 | the `postponed_buffers` comment and object | file gate |
| 2449-2501 | the `check_postponed_buffers` comment and function | file gate; C's two `continue`s are labelled blocks (`src/androidterm.rs` 279-283's shape) |
| 2502-2503 | the "Lisp interface." banner | file gate |
| 2504-2581 | the `DEFUN ("set-text-conversion-style")` docstring and subr | file gate; `DEFUN!` (docstring compared with the reference) |
| 2582-2640 | `syms_of_textconv`: the six `DEFSYM`s, the three `DEFVAR_LISP`s (and the three `V*` objects), `defsubr` and `staticpro` | file gate; `macro-body-notes.tsv` covers the `doc` marker |

## Subrs (DEFUN) — 1

- [x] set-text-conversion-style  (Fset_text_conversion_style, 1/2, doc 2dfc6a9e3ef6, line 2504)

## Functions defined — 32

- [x] copy_buffer_text  (line 86, static)
- [x] get_mark  (line 123, static)
- [x] select_window  (line 138, static)
- [x] restore_selected_window  (line 159, static)
- [x] textconv_query  (line 187)
- [x] sync_overlay  (line 449, static)
- [x] record_buffer_change  (line 482, static)
- [x] reset_frame_conversion  (line 518)
- [x] reset_frame_state  (line 529)
- [x] detect_conversion_events  (line 582)
- [x] really_commit_text  (line 611, static)
- [x] really_finish_composing_text  (line 791, static)
- [x] really_set_composing_text  (line 820, static)
- [x] really_set_composing_region  (line 973, static)
- [x] really_delete_surrounding_text  (line 1032, static)
- [x] locate_and_save_position_in_field  (line 1133, static)
- [x] really_request_point_update  (line 1246, static)
- [x] really_set_point_and_mark  (line 1273, static)
- [x] really_replace_text  (line 1328, static)
- [x] complete_edit  (line 1453, static)
- [x] complete_edit_check  (line 1475, static)
- [x] handle_pending_conversion_events_1  (line 1514, static)
- [x] decrement_inside  (line 1646, static)
- [x] handle_pending_conversion_events  (line 1658)
- [x] conversion_disabled_p  (line 2319)
- [x] report_selected_window_change  (line 2333)
- [x] report_point_change  (line 2366)
- [x] disable_text_conversion  (line 2385)
- [x] resume_text_conversion  (line 2411)
- [x] register_textconv_interface  (line 2438)
- [x] check_postponed_buffers  (line 2454)
- [x] syms_of_textconv  (line 2583)

The fourteen items below are **outside the frozen checklist** (the frozen preprocessor leaves C
1742-2312 dead) and are carried as P-86 1 requires, with `all (feature = "have_text_conversion",
feature = "have_android")` on each - they are ticked here because the port carries them, and their
coverage evidence is the C line-range table above, not this list (P-86 2):

- [x] get_conversion_field  (line 1749, inside `#ifdef HAVE_ANDROID`)
- [x] start_batch_edit  (line 1785, inside `#ifdef HAVE_ANDROID`)
- [x] end_batch_edit  (line 1806, inside `#ifdef HAVE_ANDROID`)
- [x] commit_text  (line 1831, inside `#ifdef HAVE_ANDROID`)
- [x] finish_composing_text  (line 1857, inside `#ifdef HAVE_ANDROID`)
- [x] set_composing_text  (line 1889, inside `#ifdef HAVE_ANDROID`)
- [x] set_composing_region  (line 1914, inside `#ifdef HAVE_ANDROID`)
- [x] textconv_set_point_and_mark  (line 1948, inside `#ifdef HAVE_ANDROID`)
- [x] delete_surrounding_text  (line 1976, inside `#ifdef HAVE_ANDROID`)
- [x] request_point_update  (line 1998, inside `#ifdef HAVE_ANDROID`)
- [x] textconv_barrier  (line 2021, inside `#ifdef HAVE_ANDROID`)
- [x] replace_text  (line 2043, inside `#ifdef HAVE_ANDROID`)
- [x] get_extracted_text  (line 2089, inside `#ifdef HAVE_ANDROID`)
- [x] get_surrounding_text  (line 2212, inside `#ifdef HAVE_ANDROID`)

## Macros defined — 1

- [x] TEXTCONV_DEBUG  (line 56, body a29646b15239) - both arms: the live `((void) 0)` fallback is
  `#[cfg (not (feature = "textconv_debug"))] macro_rules! TEXTCONV_DEBUG` and C's other arm is the
  `#if 0` body recorded at C 45-53.

## Types and their fields — 1

- [x] complete_edit_check_context: struct window* w, bool check

## File-scope objects — 3

- [x] static struct textconv_interface text_interface  (line 64, static)
- [x] static int suppress_conversion_count  (line 68, static)
- [x] static Lisp_Object postponed_buffers  (line 2447, static)

(`PENDING_POINT_CHANGE`/`PENDING_COMPOSE_CHANGE` are the members of C 73-77's
`enum textconv_batch_edit_flags`, which the frozen inventory does not list as a type; the port has
them as `c_int` constants at C 70's position.)

## Evidence

1. **The frozen reading.**  `CARGO_TARGET_DIR=/tmp/textconv-lane/cargo cargo run -q -p xtask --
   build-tu src/textconv.c` -> `built src/textconv.rs -> ../build/femacs/textconv.o`, exit 0, and
   `wc -c build/femacs/textconv.o` -> **336** bytes: the empty object P-86 8's criterion asks for
   (every item and every include edge is behind `feature = "have_text_conversion"`, which the frozen
   configuration does not set).
2. **The arm readings.**  `cargo run -q -p xtask -- probe src/textconv.rs /tmp/textconv-lane/x.rlib
   --crate-type=lib -C panic=abort --cfg 'feature="have_text_conversion"' --cfg
   'feature="have_android"' --cfg 'feature="not_android_stubify"'` -> exit 0, **0 errors** (`x.rlib`
   14,497,856 bytes) - all three `#if`s flipped at once.  `... --cfg 'feature="have_text_conversion"'`
   alone -> exit 0, **0 errors** (`x_base.rlib` 14,466,696 bytes) - the Android block gated out and
   the rest still compiling.  No error in either reading comes from another unit or from a mirror.
3. **The endianness-free clang measurements of the types this unit declares**
   (`/tmp/textconv-lane/wprobe.c`, `/tmp/textconv-lane/cprobe.c`, compiled with
   `-DHAVE_TEXT_CONVERSION=1 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
   -I emacs/src -I emacs/lib`): `struct window` 528/8 with `last_point` at **352** and
   `ephemeral_last_point` at **360** (i.e. `last_point + sizeof (ptrdiff_t)`); `struct
   textconv_interface` 32/8 (0/8/16/24); `struct textconv_callback_struct` 48/8 (0/8/12/16/24);
   `offsetof (struct frame, conversion)` 560; `struct complete_edit_check_context` 16/8 (0/8) - the
   last one is a *reproduction* of C 1461-1466's two members (the reference's definition is
   file-local, so the frozen clang cannot be pointed at it), and it is the same 16/8 the port
   asserts.
4. **The token and inventory reading.**  `python3 tools/extract-abi.py --check
   emacs/src/textconv.c src/textconv.rs` -> one `- ` line left: `line count drifted 15.5%:
   C 2640 vs Rust 3048` (`## Owed` 5 is its row).  The five token notes it raised before the
   `macro-body-notes.tsv` rows were appended (the macro's `0`, and `case`/`default`/`goto`/`switch`/
   `uninit`/`doc` on `textconv_query`, `locate_and_save_position_in_field`,
   `handle_pending_conversion_events_1` and `syms_of_textconv`) are recorded there and resolve.
   **Appended to `docs/ports/macro-body-notes.tsv` by this lane**: five rows.
5. `python3 tools/check-cond-eval.py src/textconv.c` -> `4 conditions, 0 unknown, 0 disagreements
   [VERIFIED]`.
6. `python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range, ...` (the file's
   `/// C:` and `/* C: */` citations all resolve; the wrapped-citation shape that reported four
   drifts early in the round is gone).
7. `python3 tools/check-declarations.py` -> `19 declaration sites (2 forbidden, 17 allowed
   literals), 2 failures`, and **both failures are `src/xmenu.rs`** (another lane's in-flight
   file); `src/textconv.rs` contributes 0 sites and 0 findings.
8. `python3 tools/check-mirror-dupes.py` -> `32 duplicate declaration(s), 17 ratchet row(s), 15
   failure(s)`, naming `gmalloc.rs` and `xmenu.rs` (another lane's in-flight files) - **no row and no
   failure names `textconv.rs`** (the unit declares the names no mirror in its graph carries, and it
   declares them once).
9. `python3 tools/check-port-lines.py` -> the row `src/textconv.c 2640 24 264 known-unported target
   (plan batch F)` is now **stale** (the port is 3048 lines) - informational, and its
   regeneration is `## Owed` 6.
10. `python3 tools/check-dead-file-coverage.py` -> the reading this table is bound by (`## Owed` 7
   if the worktree moves under it).
11. `python3 tools/check-ledger-ticks.py` -> 0 findings from this ledger.

## Deviations

1. **`textconv.h` is carried in place, and its function prototypes are dropped.**  The header's
   mirror (`src/include/textconv.rs`) declares `register_textconv_interface` at its top level
   (outside its `textconv_decls` module) and this unit defines that function, which is `E0428`
   (measured on this checkout: two items, one name, one crate).  So the unit carries the three
   records, the two enums and `TEXTCONV_SKIP_CONVERSION_REGION` at C 35's position with `/// C:
   textconv.h <line>:` citations, and drops the prototypes of the functions it defines - C's
   prototype-plus-definition has no Rust counterpart (PORTING.md 0.1-12).  The two records' layout
   asserts are the mirror's clang readings, re-asserted here so this unit's build measures them too.
2. **`struct window`'s `ephemeral_last_point` (window.h 302) is reached through an accessor macro.**
   `include/window.rs` carries `last_point` and records `ephemeral_last_point` as the field it still
   owes (its 11), so the port writes `ephemeral_last_point! (w)` for C's `w->ephemeral_last_point`,
   defined as `*(&raw mut (*w).last_point).add (1)`: the two fields are adjacent `ptrdiff_t`s and
   Evidence 3 measures 352 and 360.  `## Owed` 4 makes this macro disappear.
3. **C 1667's function-local `static int inside` is at file scope.**  C's local static is one object
   with internal linkage; the port's `static mut inside: c_int = 0` is the same object (and the
   `record_unwind_protect_ptr (decrement_inside, &inside)` at C 1685 addresses the same storage).
4. **The `#if 0` debug region (C 45-53) is recorded, not transcribed** (the `src/androidfns.rs`
   1015-1022 shape): it is dead in every configuration, and its body is an `<android/log.h>` include
   and the `__android_log_print` spelling of the macro.  The `#ifndef TEXTCONV_DEBUG` guard's other
   arm *is* that body, so `feature = "textconv_debug"` has no item to carry - `## Owed` 2 records
   the name.
5. **C's `switch`es and `goto`s.**  C 299/426/1567's `switch`es are `match`es (C's `default:`
   becomes `_`, and a C `switch` arm's `break` has no Rust counterpart), C 233/243's `goto escape1`,
   C 1169's `goto exit` and C 2137/2163/2262's `goto finish` are labelled blocks
   (`'escape1:`/`'exit:`/`'finish:` keep C's label names), and C 1576's arm-exit `break` is
   `'end_batch_edit`.  Both loops whose C `continue` jumps to the `for` increment
   (`check_postponed_buffers`) use the labelled-block shape `src/androidterm.rs` 279-283 records.
   The two `for (last = &f->conversion.actions; *last; last = &(*last)->next) ;;` link walks are
   `last = &raw mut (*f).conversion.actions; while !(*last).is_null () { last = &raw mut (**last).next; }`.
6. **`ckd_add`/`ckd_sub` are written out** as `overflowing_add`/`overflowing_sub` plus the target
   clamp at each site (PORTING.md 2.6), because the frozen `lib/include/intprops.rs` keeps
   `INT_ADD_WRAPV`'s text and is not compiled.
7. **C's uninitialized locals** - `struct buffer *buffer UNINIT` (C 1519), `Lisp_Object c1, c2`
   (C 1136), `ptrdiff_t beg, end` (C 1136), `struct complete_edit_check_context context` (C 1519)
   and the `field_start`/`field_end`/`start_byte`/`end_byte` temporaries - are initialized to C's
   zero value and then assigned at C's own statements (PORTING.md 1.13: Rust has no partial
   initialization), which is marked at each site.
8. **`TEXTCONV_DEBUG`'s fallback macro is a name this lane introduces**
   (`not (feature = "textconv_debug")`); the macro does not evaluate its arguments, matching C's
   `((void) 0)`.  `## Owed` 2 asks for the feature's registration.
9. **`VA`/variadics are not involved, and neither is `alloca`**: the two macros C's text calls that
   PORTING.md covers elsewhere (`TEXTCONV_DEBUG` and `type_minimum`) are the only ones, and
   `TYPE_MINIMUM` is a `macro_rules!` over the Rust primitive as `src/androidselect.rs` 113-115
   records.
10. **The `#if defined HAVE_ANDROID` debug macro's string literal** (`"EmacsInputConnection"`,
    C 50) is *not* carried, because the region it lives in is the `#if 0` of Deviation 4; if a later
    round transcribes it, the literal needs a `declarations-allowlist.tsv` row (it is C's own
    string, and `docs/baseline/declarations-allowlist.tsv` has no row for it today).

## Owed

1. **`src/include/textconv.rs`: move `register_textconv_interface` into `textconv_decls`.**
   That is the one declaration that keeps its own defining unit from `include!`ing the mirror
   (Deviation 1).  With it moved, this unit's C 35 surface section (records, enums, constant) can be
   deleted in favour of `include!("include/textconv.rs")` + `use textconv_decls::*;`, the shape
   `src/androidterm.rs` 152 uses.  `src/include/**` is not this lane's to write.
2. **Register `textconv_debug` in `xtask/Cargo.toml`** (the `trace_selection` /
   `have_sys_types_h` situation `docs/ports/xselect.md` records), so C 55's guard has a registered
   name; until then the `not (...)` arm is live in every configuration and the port compiles.
3. **`src/include/lisp.rs` owes seven declarations** this unit now writes in place:
   `record_unwind_protect_excursion` (lisp.h 4826), `insert_from_string` (4335-4336), `del_range_1`
   (4346), `safe_del_range` (4351), `save_restriction_save` (4924), `save_restriction_restore`
   (4925) and `find_field` (4920-4921); `src/include/keyboard.rs` owes
   `reading_key_sequence` (keyboard.h 252).  When they land, this unit's `unsafe extern "C"` block
   at C 81's position shrinks to nothing (one declaration per name, P-26).
4. **`src/include/window.rs` owes `ephemeral_last_point`** (window.h 302, under
   `#ifdef HAVE_TEXT_CONVERSION`), which retires the `ephemeral_last_point!` accessor macro
   (Deviation 2).
5. **`docs/ports/line-drift-tolerance.tsv` needs a row, and this lane may not write it.**
   `extract-abi.py --check`'s only remaining `- ` line is the 15.5% drift.  Suggested row (append,
   the table is append-only):
   `textconv	0.16	C 2640 vs the port's 3048 (15.5%): the `have_text_conversion` file gate on every item and on the whole include graph (P-86 8), both arms of the four C conditions (C 45/46/55/1742, the Android half carried as P-86 1 requires), the `/// C:` line and the in-unit `textconv.h` surface, the `ephemeral_last_point!` accessor and the seven in-unit lisp.h declarations (Owed 3-4), and C's comment blocks carried as prose	total`
6. **`docs/baseline/port-lines-pending.tsv` is stale for this target** (`src/textconv.c 2640 24 264
   known-unported target (plan batch F)`): regenerating it is `tools/check-port-lines.py
   --write-pending`, and `docs/baseline/**` is not this lane's to write.
7. **`docs/port-map.md`'s row for `src/textconv.c`** (target lines 24 -> 3048) is a
   generated value (`tools/gen-port-map.py`), and that file is not this lane's to write.
8. **Re-verify this lane's five `macro-body-notes.tsv` rows survive the next whole-file rewrite of
   that shared file**: the row above them (`aligned_alloc`) had to be re-appended after one such
   rewrite by a concurrent lane.  Regression shape: `extract-abi.py --check` prints the five token
   notes again.
9. **`docs/ports/include-textconv.md` should record that its own unit now exists**: the mirror
   ledger says "No unit includes this mirror yet" and names `keyboard.rs` as the one committed
   caller; the state this round leaves is "its own unit carries the surface in place, and its
   `register_textconv_interface` is the reason" (Owed 1).
