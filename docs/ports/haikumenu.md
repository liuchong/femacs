# emacs/src/haikumenu.c (845 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with the Haiku platform objects under
> `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and
> `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
> `docs/baseline/frozen-build-sources.txt` lists no `haikumenu.o` and `extract-abi.py`'s checklist
> for the file is empty (P-86 ②).  What follows is that clause's substitute: the **C line-range
> table**, the `## Evidence` readings and the `## Deviations`.
>
> Port: `src/haikumenu.rs` (**1188 lines**), which carries the file **whole, C 1-845** - no
> slice.  The 14 functions (6 `static`), the 2 `DEFUN`s, the 3 file-scope objects (`menu_item_selection`,
> `menu_timer_timespec`, `popup_activated_p`) and the 4 `DEFSYM`s of the reference stand at C's own
> line, in C's own order.
>
> **Acquisition note - this port is `text-only` (P-86 ③⑵).**  There is no Haiku SDK and no Haiku
> toolchain on this machine (the measurement is `docs/ports/haikuterm.md` Evidence ②, same
> machine), so there is **no arm flip on a Haiku target** and nothing here is compiled or linked
> against a Haiku system.  The `<InterfaceKit.h>`/`<SupportDefs.h>` surface this file's headers
> pull in is already mirrored by `include/haiku_support.rs` (landed before this unit); every Haiku
> width, constant and enum value this port spells (`HAIKU_INFO_ALERT`, `HAIKU_IDEA_ALERT`,
> `MENU_BAR_RESIZE`, the `haiku_alert_type`, `int32`) is declared by that mirror and is
> **UNVERIFIED** (P-88 ①/②).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`HAIKU_OBJ`, i.e. `HAVE_HAIKU` <- `HAVE_BE_APP`, configure.ac 3097-3114) | `feature = "have_haiku"` (on every item *and* on C 21-29's include graph, P-86 ⑧) | a Haiku app-kit configuration |
| this file's **own** `#if`/`#ifdef` | **none**: `grep -n '#if' src/haikumenu.c` is empty - the whole body is one live region, so there is no sub-gate to carry | - |
| `HAVE_WINDOW_SYSTEM` / `HAVE_EXT_MENU_BAR` - not gates of this file: `lisp.h` 3925-3936 defines both from `HAVE_HAIKU` | `feature = "have_window_system"`, `feature = "have_ext_menu_bar"` (both set by the arm pre-flight; a Haiku configuration always has them) | every Haiku configuration |

`HAIKU_OBJ`/`HAVE_HAIKU` is the only build condition, and it is the *build's*, not one the C text
spells.  `tools/check-cond-eval.py src/haikumenu.c` reads the file as
`0 conditions, 0 unknown to the evaluator, 0 disagreements [VERIFIED]` - an empty condition set
is what "no `#if` of its own" looks like to that gate.

## C line ranges

The rows are **contiguous** - each block's range also carries the blank line(s) that follow it -
so that the table covers C 1-845 with no gaps; `check-dead-file-coverage.py` requires the last
range to reach the reference's final line.

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block (C 1-17) and its blank line | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 18-20 | `#include <config.h>` (C 19) | ungated (every unit includes it) |
| 21-30 | the include list: `lisp.h`, `frame.h`, `keyboard.h`, `menu.h`, `buffer.h`, `blockinput.h`, `haikuterm.h`, `haiku_support.h` | file gate |
| 31-35 | the three file-scope objects: `menu_item_selection` (C 31), `menu_timer_timespec` (C 32), `popup_activated_p` (C 34) | file gate |
| 36-187 | `digest_menu_items` (`static`; C 76 the `while`, C 93 the `menu_items_n_panes == 1` early exit, C 113 the `strcmp`, C 144-181 the four `BMenu_add_item` call shapes) | file gate |
| 188-302 | `haiku_dialog_show` (`static`; C 218 the item loop, C 270 the boundary, C 281 the `BAlert_go`) | file gate |
| 303-336 | `haiku_popup_dialog` (C 316 the `unuse_menu_items` unwind, C 327 the `haiku_dialog_show` call) | file gate |
| 337-348 | `haiku_menu_show_help` (`static`; its address is handed to `BMenu_run` at C 424) | file gate |
| 349-356 | `haiku_process_pending_signals_for_menu_1` (`static`; address handed to `internal_catch_all` at C 374) | file gate |
| 357-365 | `haiku_process_pending_signals_for_menu_2` (`static`; address handed to `internal_catch_all` at C 374-375) | file gate |
| 366-379 | `haiku_process_pending_signals_for_menu` (`static`; address handed to `BMenu_run` at C 426) | file gate |
| 380-504 | `haiku_menu_show` (C 393 the `SAFE_NALLOCA`, C 421-423 the `BMenu_run`, C 438-488 the selection walk) | file gate |
| 505-528 | `free_frame_menubar` (C 510-512 the three written frame fields, C 515-516 the menubar) | file gate |
| 529-537 | `initialize_frame_menubar` | file gate |
| 538-730 | `set_frame_menubar` (C 564-565 the `alloca`, C 573-606 the shallow arm, C 607-725 the deep arm, C 651-654 the four `alloca`s) | file gate |
| 731-755 | `run_menu_bar_help_event` | file gate |
| 756-762 | `menu-or-popup-active-p` (`DEFUN`, 0/0, C 757's `SKIP` docstring) | file gate |
| 763-794 | `haiku-menu-bar-open` (`DEFUN`, 0/1, interactive `"i"`) | file gate |
| 795-833 | `haiku_activate_menubar` (C 805-818 the `saved_menu_event` arm, C 819-831 the tracking arm) | file gate |
| 834-845 | `syms_of_haikumenu`; C 837-840 the four `DEFSYM`s, C 842-843 the two `defsubr`s | file gate |

## Evidence

**① The frozen reading (P-86 ⑧'s judgement: the file gate must reach the include graph).**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/hkmenu-lane cargo run -q -p xtask -- build-tu src/haikumenu.c
built src/haikumenu.rs -> ../build/femacs/haikumenu.o
exit 0
$ ls -l build/femacs/haikumenu.o
-rw-r--r--@ 1 liu  staff  336 Oct  5 18:07 build/femacs/haikumenu.o
$ md5 build/femacs/haikumenu.o
MD5 (build/femacs/haikumenu.o) = dbed68837694579185377dbb0821ab62
```

**exit 0** and a **336-byte empty object** - `include/config.rs` is the only item the frozen
configuration compiles, and every one of C 21-29's includes and every item this file declares sits
behind `have_haiku`.  (Placing the include edges outside the gate is what makes the frozen compile
fail: P-86 ⑧'s `E0428 Emacs_Cursor`/`cannot find macro` measurement.)  The first run of the same
command read the same 336 bytes / same md5 on the pre-port stub text; the second (above) is the
reading of the final port, so the frozen configuration's object is unchanged by the port.

**② The arm reading.**  `xtask probe` with the file gate and the two gates a Haiku configuration
always has (`lisp.h` 3925-3936 makes `HAVE_WINDOW_SYSTEM` and `HAVE_EXT_MENU_BAR` follow
`HAVE_HAIKU`; `docs/ports/haikufns.md`'s Gate mapping row states the same three-cfg pre-flight):

```
$ cd femacs && RUSTUP_TOOLCHAIN=nightly-2026-10-01 CARGO_TARGET_DIR=/tmp/hkmenu-lane \
    cargo run -q -p xtask -- probe src/haikumenu.rs /tmp/hkmenu-lane/haikumenu-arm.rlib \
    --crate-type=lib -C panic=abort \
    --cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_ext_menu_bar"'
exit 1
```

**exit 1, one error, none in this unit's own text**:

* **the one error is in a mirror copy**: `error[E0080]: evaluation panicked: assertion failed:
  !cfg!(feature = "have_haiku") || HAIKU` at `include/systime.rs` 28 - the `HAIKU` constant's
  assertion, the same gap `docs/ports/haikuterm.md` Owed 6 / `docs/ports/haikuselect.md` Owed 5 /
  `docs/ports/haikufont.md` Owed 7 name; no Haiku configuration can reach a full crate until that
  mirror round lands.  4 warnings, all mirror-side (`unused doc comment` in `include/lisp.rs`
  and `include/haikuselect.rs`, `unused attribute` in `include/frame.rs` 1426/1431).

The lane's prescribed two-cfg command (`have_haiku`+`have_window_system` only) reads **5 errors**
instead: the same `include/systime.rs` 28 assertion, plus `E0425 cannot find value
FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT` in this unit's own `FRAME_EXTERNAL_MENU_BAR!` write macro
(C 512).  That constant is declared under
`all (have_window_system, have_ext_menu_bar)` in `include/frame.rs` (`FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT`
is under `have_window_system` alone), and a Haiku configuration always defines `HAVE_EXT_MENU_BAR`
(lisp.h 3931-3934) - so the third `--cfg` is the pre-flight's, not a change to the port.  Two
earlier readings of this unit's own text were also taken and are recorded here because they are
what the first two runs measured:

| run | cfgs | exit | errors | in the unit | in mirrors |
| --- | --- | ---: | ---: | ---: | ---: |
| 1st (initial text) | `have_haiku` + `have_window_system` | 1 | 6 | 3 (`E0308` at `BMenu_run`'s assignment, `E0530` at `digest_menu_items`'s third parameter, two parse errors) + `E0425` (the cfg gap) | 1 (`include/systime.rs` 28) |
| 2nd (cast + braces fixed) | same two | 1 | 5 | 1 (`E0530`) + `E0425` (the cfg gap) | 1 (same) |
| 3rd (final text, `menu_items_used_` + braces) | the three above | 1 | **1** | **0** | 1 (same) |

The three self-inflicted defects were: C's brace-less single `if`/`else` bodies (C 152-166,
C 169-181) need a Rust block; `BMenu_run`'s `void *` result needs C's implicit conversion spelled
as a cast; and C's third parameter of `digest_menu_items` is named `menu_items_used`, which Rust
refuses to shadow a `static` with (E0530).  All three are in the port's text and were retired
before the reading above (Deviations 1, 7, 10).

**③ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/haikumenu.c
src/haikumenu.rs` → **one row and `FAIL`**:

```
- line count drifted 40.6%: C 845 vs Rust 1188
FAIL emacs/src/haikumenu.c -> src/haikumenu.rs
```

Nothing else fails: no missing function, `DEFUN`, macro, type, field or file-scope object, and no
missing body token (P-78's token comparison reports no row).  The one row is the expected
`line-drift-tolerance.tsv` row, which is the integrator's (`## Owed` 1).

**④ The other gates (repo root, on this revision), one line each.**
`check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`,
and this unit has no finding (the C title/recital prose is not transcribed). -
`check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)`; this file
has none. - `check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 1604 external, 36
allowed, 0 stale` - this unit's citations are clean. -
`check-ledger-ticks.py` → `234 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried
by another file, 356 row(s) with no function tick ...`: this ledger carries no ticked-function
section (the class's checklists are empty, P-86 ②), so nothing is bound here. -
`check-dead-file-coverage.py` → `131 ledger(s) bound (a C range table), 0 failure(s)`, with
`info docs/ports/haikumenu.md: 20 range row(s) cover src/haikumenu.c (845 lines; 0 blank line(s)
between blocks need no row)`.  **Not run here** (outside this lane's budget):
`gate-fast`, `check-all-units.sh`, `cargo test`, `xtask dead-arms`, `extract-abi --all`.

## Deviations

1. **C's brace-less `if`/`else` bodies need a Rust block.**  C 151-166 and C 168-181 pass a single
   `BMenu_add_item (…)` statement as the body of four arms; Rust requires braces, so the port
   carries each call in `{ }`.  Same statements, same order, same call shapes.
2. **`USE_SAFE_ALLOCA` is `USE_SAFE_ALLOCA! (sa_avail, sa_count)`** (C 47, C 388): C's macro
   *declares* its two names and a Rust `macro_rules!` cannot (its `let`s are hygienic), so the call
   site names them - `include/lisp.rs`'s documented spelling.
3. **`sizeof *menus`/`sizeof *panes` (C 58/60) is written out** as
   `core::mem::size_of::<*mut c_void> ()`, and the pointers are cast to `*mut c_void` for the
   libc `memset` prototype.
4. **`alloca` (C 564-565, C 651-654) is the port's stack pool** `sa_stack_alloc` (PORTING.md P-05's
   decision), with the byte size C's `sizeof` gives (`previous_menu_items_used * sizeof (Lisp_Object)`,
   `(subitems + 1) * sizeof *submenu_start`, and the three `subitems *` arrays).
5. **`FRAME_MENU_BAR_LINES`/`FRAME_MENU_BAR_HEIGHT`/`FRAME_EXTERNAL_MENU_BAR`/`FRAME_HAIKU_MENU_BAR`
   writes are unit-local two-argument macros.**  C writes all four (C 510-512, C 516) and the
   mirrors' accessors are read-only functions; a Rust function cannot be an assignment target, so
   the same name is redefined in this unit with a `($f, $v)` arm (the `src/haikufns.rs` 350-410
   shape).  The `!` in an invocation is the port's spelling of C's assignment (`NAME! (f, v)` for
   `NAME (f) = v`).
6. **`FRAME_OUTPUT_DATA (f)->menu_bar_open_p` (C 521/523, C 786, C 817, C 828) goes through the
   mirror's member-named accessor** `haiku_output_bits! (menu_bar_open_p, …)`: `include/haikuterm.rs`
   folds the four `bool_bf` members (`hourglass_p`, `menu_bar_open_p`, `dirty_p`, `complete_p`) into
   one `bits: u8` byte (PORTING.md 1.3/P-80), so the member name is the macro's first argument.
7. **`digest_menu_items`'s third parameter is `menu_items_used_`, not `menu_items_used`.**  C's
   parameter shadows the file-scope `menu_items_used` (`include/keyboard.rs`'s `keyboard_objs`);
   Rust refuses a parameter that shadows a `static` (measured: `E0530: function parameters cannot
   shadow statics`), so the port keeps the name with the trailing underscore the token comparison
   strips (the `src/alloc.rs` 4333-4334 `finalizers_` / `PORTING.md` 0.2 shape).  The parameter's
   four read sites (C 49, C 50, C 76, C 144) are the only ones renamed.
8. **`int rc` (C 774, 798) receives a `bool`.**  `haiku_support.h` 711/743 declare
   `BMenuBar_start_tracking`/`be_replay_menu_bar_event` as returning `bool`, and C stores the result
   in an `int`; the port keeps C's `int rc` and its `if (rc == 0)` tests and writes the conversion
   as `as c_int` at the two call sites (C 780, C 808-809, C 822).
9. **`menu_item_selection`'s `volatile` qualifier has no Rust counterpart.**  C 31 is
   `static Lisp_Object *volatile menu_item_selection;`; the port keeps C's type (`*mut Lisp_Object`)
   and its accesses (`is_null` tests and the `aref_addr` equality at C 463) and records only the
   dropped qualifier.
10. **`BMenu_run`'s `void *` result is cast to `Lisp_Object *`** at C 421-423: C's implicit
    conversion from `void *` is `as *mut Lisp_Object` in Rust.
11. **C's mixed-width comparisons are spelled with an explicit cast.**  C 690/693 compare the
    `int` statics `menu_items_used`/`previous_menu_items_used` with the `ptrdiff_t` `i`, and C 747
    compares `(mb_idx + MENU_ITEMS_ITEM_HELP)` with `ASIZE (vec)`; the port keeps C's order and
    writes `as ptrdiff_t` / `as isize` at exactly those comparisons.
12. **`Lisp_Object vals[10]` (C 194) is zero-initialized** with `Qnil ()` (PORTING.md 0.1-13's
    "initialize the whole object first" rule).  C writes `vals[nb_buttons]` and reads `vals[val]`
    after a run of writes; Rust's definite-assignment analysis cannot prove the read is covered, so
    the array starts all-`Qnil` - a value the C never reads (the read only happens for
    `val < nb_buttons`, which only counts the written slots).
13. **The address-taken `block_input` is a unit-local C-ABI copy** (`mod block_input_abi`, C 251
    and C 425): `include/blockinput.rs` 52-56 spells the INLINE as a Rust-ABI `unsafe fn`, which a
    C function pointer cannot take (Owed 3).  `unblock_input` is already `extern "C"` in the mirror
    and needs no copy.
14. **The 15 use-site declarations** below (P-72 rule 4) are declared here because the mirrors C
    21-29 reach do not carry them yet: `decode_window_system_frame`, `check_window_system`,
    `adjust_frame_size` (frame.h), `process_pending_signals`, `quit`, `internal_catch_all`,
    `record_unwind_protect_void`, `record_unwind_save_match_data`, `menu_bar_items`, `safe_run_hooks`
    (lisp.h), `unuse_menu_items`, `menu_separator_name_p`, `show_help_echo`,
    `kbd_buffer_store_help_event`, `timer_check` (keyboard.h), `request_sigio`, `unrequest_sigio`
    (dispextern.h).  Their move into those mirrors is owed (Owed 4).
15. **The `count` shadow at C 579 and the `i` shadow at C 581** are kept as Rust shadows of the
    outer `count` (C 566) and `i` (C 567); the shallow arm's `count`/`i` are the ones C's own block
    declares, and the deep arm's are the outer ones, exactly as in C.
16. **Line count**: 1188 Rust lines for 845 C lines is 140.6% (40.6% drift) - the
    `line-drift-tolerance.tsv` row Owed 1 names.  The port's shape is the family's: one item per
    C construct with the C citation on each line, the use-site declaration block, and the
    unit-local macro block.

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`: one row, `haikumenu`, tolerance >= 0.406, basis
   `total`** (Evidence ③ is the reading).  Not in this lane's writable set.
2. **`include/systime.rs` 28's `HAIKU` assertion** stays the arm probe's one error, so no Haiku
   configuration can compile a whole crate until the constant lands.  Owner:
   `docs/ports/include-systime.md` (the same item `docs/ports/haikuterm.md` Owed 6 /
   `docs/ports/haikuselect.md` Owed 5 / `docs/ports/haikufont.md` Owed 7 name).
3. **`include/blockinput.rs`'s `block_input` is Rust-ABI**, and C takes its address at C 251 and
   C 425 (both `void (*) (void)` arguments of `BAlert_go`/`BMenu_run`).  Owner:
   `docs/ports/include-blockinput.md` - the mirror needs the C-ABI form P-26/P-39 ask for; this
   unit works around it with the `block_input_abi` module (Deviations 13).  The same item
   `docs/ports/haikuselect.md` Owed 1 names.
4. **The 15 use-site declarations (Deviations 14) are owed to their mirrors** (P-72 rule 4):
   frame.h's three, lisp.h's six, keyboard.h's five and dispextern.h's two.  Each line of the block
   in `src/haikumenu.rs` names the prototype and the header line it was read from.
5. **`docs/baseline/port-lines-pending.tsv`'s row for `src/haikumenu.c` is now stale**: the tree
   reads `stale src/haikumenu.c: no longer under the threshold` for the row
   `src/haikumenu.c 845 11 845 known-unported target` - 1,188 Rust lines for 845 C lines is 141%,
   not 1%.  `python3 tools/check-port-lines.py --write-pending` is the integrator's step (the
   `docs/ports/haikufns.md` Owed 5 precedent), and `docs/port-map.md`'s row's target count is
   regenerated with it.
6. **A Haiku round**: with `<support/SupportDefs.h>`/`<InterfaceKit.h>`/`<InterfaceKit.h>`'s kit
   headers in hand, confirm the values this unit uses from the mirror - `HAIKU_INFO_ALERT`,
   `HAIKU_IDEA_ALERT` (C 215-216), `MENU_BAR_RESIZE` (C 553), the `haiku_alert_type` parameter of
   `BAlert_new`, and `int32` (C 197) - and the `BMenu_*`/`BAlert_*`/`BBMenuBar_*` prototypes'
   parameter lists.  Nothing here is measured against a Haiku system.
7. **No `macro-body-notes.tsv` row is owed.**  `extract-abi.py --check` (Evidence ③) reports no
   missing body token; the four `if`-block braces and the trailing-underscore parameter of
   Deviations 1/7 are shapes the report already accounts for.
8. **This ledger's `## C line ranges` table is machine-read by `check-dead-file-coverage.py`**; a
   future slice that moves an item must keep the table ascending and complete (last row to 845).

## Not claimed

- **No Haiku-target compile or link.**  There is no Haiku SDK or toolchain on this machine, so
  Evidence ②'s reading is a `/tmp`/`build`-tree harness reading on this host, not a Haiku build;
  nothing here runs.
- **No runtime or device behaviour.**  The menu bar's creation, the popup loop, the dialog, the
  help-event store and the menubar activation are transcribed, not exercised.
- **Every Haiku width, constant and value this port names is C's spelling, not a measurement**, and
  the records/constants are **UNVERIFIED** - Owed 6 is the round that measures them.
- **`xtask gate-fast`, `check-all-units.sh`, `cargo test`, `xtask dead-arms` and
  `extract-abi --all` were not run** (this lane's budget names them as out of scope), and the C++
  half of the backend (`haiku_support.cc`, the `B*` implementations) is out of reach here.
- **The `_root.rs` line numbers of Evidence ② are the unit's own** only because every `include!` is
  replaced by one line; an error reported against a `_Users_…` copy is a mirror's, and the one such
  error is named there.
