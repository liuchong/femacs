# emacs/src/haikuselect.c (1450 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with the Haiku platform objects under
> `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and
> `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
> `docs/baseline/frozen-build-sources.txt` lists no `haikuselect.o` and `extract-abi.py`'s
> checklist for the file is empty (P-86 ②).  What follows is that clause's substitute: the
> **C line-range table**, the `## Evidence` readings and the `## Deviations`.
>
> Port: `femacs/src/haikuselect.rs` (**1660 lines**), which carries the file **whole, C 1-1450** -
> no slice.  The 15 functions (7 `static`), the 9 `DEFUN`s, the 4 file-scope objects, the 4
> `DEFVAR_*` variables and the 15 `DEFSYM`s of the reference stand at C's own line, in C's own
> order.
>
> **Acquisition note - this port is `text-only` (P-86 ③⑵).**  There is no Haiku SDK and no Haiku
> toolchain on this machine (the measurement is `docs/ports/haikuterm.md` Evidence ②, same
> machine), so there is **no arm flip on a Haiku target** and nothing here is compiled or linked
> against a Haiku system.  The `<SupportDefs.h>`/`Errors.h` scalars and error codes this file's
> headers pull in are declared at their use site, each with one `/// C:` line and the standing
> **UNVERIFIED** mark (P-88 ①/②); `docs/ports/haikuselect.h`'s mirror (`include/haikuselect.rs`)
> is the one header of this file that *does* have a mirror, and the `be_*` prototypes ride on it.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`HAIKU_OBJ`, i.e. `HAVE_HAIKU` <- `HAVE_BE_APP`, configure.ac 3097-3114) | `feature = "have_haiku"` (on every item *and* on C 21-29's include graph, P-86 ⑧) | a Haiku app-kit configuration |
| this file's **own** `#if`/`#ifdef` | **none**: `grep -n '#if' src/haikuselect.c` is empty - the whole body is one live region, so there is no sub-gate to carry | - |
| `HAVE_WINDOW_SYSTEM` / `HAVE_EXT_MENU_BAR` - not gates of this file: `lisp.h` 3925-3936 defines both from `HAVE_HAIKU` | `feature = "have_window_system"`, `feature = "have_ext_menu_bar"` (both set by the arm pre-flight; a Haiku configuration always has them) | every Haiku configuration |

`HAIKU_OBJ`/`HAVE_HAIKU` is the only build condition, and it is the *build's*, not one the C text
spells.  `tools/check-cond-eval.py src/haikuselect.c` reads the file as
`0 conditions, 0 unknown to the evaluator, 0 disagreements [VERIFIED]` - an empty condition set
is what "no `#if` of its own" looks like to that gate.

## C line ranges

The rows are **contiguous** - each block's range also carries the blank line(s) that follow it -
so that the table covers C 1-1450 with no gaps; `check-dead-file-coverage.py` requires the last
range to reach the reference's final line, and `## Owed` 8 records the second reference it binds
to this ledger.

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 18-19 | `#include <config.h>` | ungated (every unit includes it) |
| 20-30 | the include list: `lisp.h`, `blockinput.h`, `coding.h`, `haikuselect.h`, `haikuterm.h`, `haiku_support.h`, `keyboard.h`, `<stdlib.h>` | file gate |
| 31-36 | `haiku_dnd_frame` | file gate |
| 37-39 | `haiku_dnd_follow_tooltip` | file gate |
| 40-43 | `haiku_dnd_allow_same_frame` | file gate |
| 44-45 | `static void haiku_lisp_to_message (Lisp_Object, void *);` | dropped (PORTING.md 0.1-12) |
| 46-60 | `haiku_get_clipboard_name` (`static`) | file gate |
| 61-77 | `haiku-selection-timestamp` (`DEFUN`) | file gate |
| 78-139 | `haiku-selection-data` (`DEFUN`) | file gate |
| 140-145 | `haiku_unwind_clipboard_lock` (`static`) | file gate |
| 146-200 | `haiku-selection-put` (`DEFUN`) | file gate |
| 201-218 | `haiku-selection-owner-p` (`DEFUN`) | file gate |
| 219-221 | the comment and return-type lines over `haiku_message_to_lisp` | file gate |
| 222-414 | `haiku_message_to_lisp` (C 246 and C 344 the two `switch (type_code)` statements) | file gate |
| 415-455 | `lisp_to_type_code` (`static`) | file gate |
| 456-792 | `haiku_lisp_to_message` (`static`; C 477 and C 718 the two `FOR_EACH_TAIL`s, C 509 the `continue`, C 535 the `goto decode_normally`) | file gate |
| 793-798 | `haiku_should_quit_drag` (`static`) | file gate |
| 799-805 | `haiku_unwind_drag_message` (`static`) | file gate |
| 806-848 | `haiku_report_system_error` (`static`; C 808 the `switch (code)`) | file gate |
| 849-930 | `haiku-drag-message` (`DEFUN`) | file gate |
| 931-1027 | `haiku-roster-launch` (`DEFUN`) | file gate |
| 1028-1061 | `haiku-write-node-attribute` (`DEFUN`) | file gate |
| 1062-1087 | `haiku-send-message` (`DEFUN`) | file gate |
| 1088-1125 | `haiku_dnd_compute_tip_xy` (`static`) | file gate |
| 1126-1134 | `haiku_note_drag_motion_1` (`static`) | file gate |
| 1135-1140 | `haiku_note_drag_motion_2` (`static`) | file gate |
| 1141-1172 | `haiku_note_drag_motion` | file gate |
| 1173-1198 | `haiku_note_drag_wheel` (C 1183 the `ie->kind` read, C 1188 the write) | file gate |
| 1199-1204 | `init_haiku_select` | file gate |
| 1205-1222 | `haiku_handle_selection_clear` | file gate |
| 1223-1249 | `haiku_selection_disowned` (C 1229 the `switch (id)`) | file gate |
| 1250-1257 | `haiku_start_watching_selections` | file gate |
| 1258-1261 | the page break (C 1258) and the "Notification support." banner (C 1260) | file gate |
| 1262-1298 | `haiku_notifications_notify_1` (`static`) | file gate |
| 1299-1371 | `haiku-notifications-notify` (`DEFUN`, `MANY`) | file gate |
| 1372-1450 | `syms_of_haikuselect` (C 1375/1381/1387/1392 the four `DEFVAR_*`s, C 1402-1437 the 15 `DEFSYM`s, C 1439-1447 the nine `defsubr`s) | file gate |

## Evidence

**① `build-tu` (the frozen configuration).**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/hks-lane cargo run -q -p xtask -- build-tu src/haikuselect.c
built src/haikuselect.rs -> ../build/femacs/haikuselect.o
exit 0;  build/femacs/haikuselect.o = 336 bytes
```

336 bytes is **the empty object the file gate leaves**: the frozen arm compiles nothing of this
file, exactly as the reference's own frozen configuration compiles nothing of it.

**② The arm probe (`have_haiku` plus the two follow-on gates).**

```
$ cd femacs && RUSTUP_TOOLCHAIN=nightly-2026-10-01 CARGO_TARGET_DIR=/tmp/hks-lane \
    cargo run -q -p xtask -- probe src/haikuselect.rs /tmp/hks-lane/haikuselect-arm.rlib \
    --crate-type=lib -C panic=abort \
    --cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_ext_menu_bar"'
exit 1
```

- **errors: 1**, and it is a **mirror copy**, not the unit:
  `build/probes/haikuselect/_Users_…_femacs_src_include_systime_rs.rs:28` =
  `femacs/src/include/systime.rs` 28's
  `const _: () = core::assert! (!cfg! (feature = "have_haiku") || HAIKU);` (`E0080`, `HAIKU` is
  undefined) - the item `docs/ports/haikuterm.md` Owed 6 names.
- **errors in the unit itself (`_root.rs`): 0.**
- warnings: 3, all in mirror copies (`lib/include/stdbit.in.rs` 644 `unused doc comment`,
  `include/lisp.rs` 3939 `unused doc comment`, `include/frame.rs` 1360 `unused attribute`); none
  is in the unit's statements.
- toolchain: `rustc 1.101.0-nightly (21b707e3f 2026-09-30)`.  Nightly is needed because
  `include/lisp.rs` 3761 declares `verror` with `core::ffi::VaList`, and `c_variadic` only left
  the unstable list in 1.99 (on this machine's default `rustc 1.98.1` that one declaration is an
  `E0658` in a **mirror copy**, before any of this unit's text is reached - measured, and not a
  finding of this file).  The three `--cfg`s are the file gate plus the two gates a Haiku
  configuration always has; without `have_window_system` the two `Emacs_Cursor` definitions
  (`include/dispextern.rs` 105 and `include/haikugui.rs` 60) collide in the crate root.

**③ The mutation control (P-86 ⑦: the reading above is not vacuous).**  Injecting one type error
into the unit's own text - `femacs/src/haikuselect.rs` 627, `let mut sizet_data: size_t = 0;`
replaced by `= c"injected_control";` - changes the reading from **1 error to 2**, and the new one
names the injected line:

```
error[E0308]: mismatched types
   --> _root.rs:627:32
627 |   let mut sizet_data: size_t = c"injected_control";
    |                       ------   ^^^^^^^^^^^^^^^^^^^ expected `usize`, found `&CStr`
```

`_root.rs`'s line numbering is the unit's own (every `include!` is replaced by one line), so the
error is the unit's line 627 and not a mirror's.  Restored, and
`shasum -a 256 femacs/src/haikuselect.rs` → `ad38d105a7b4a747572870d09304f709a735e378584442a5be83cb9798871bac`
before and after the injection.

**④ `extract-abi.py --check`.**

```
$ python3 tools/extract-abi.py --check emacs/src/haikuselect.c femacs/src/haikuselect.rs
- line count drifted 14.5%: C 1450 vs Rust 1660
FAIL emacs/src/haikuselect.c -> femacs/src/haikuselect.rs
```

**One row, and it is the line-count row**: 14.5% is the measured drift (C 1450 → Rust 1660), and
the tolerance row for it is the integrator's (`## Owed` 3).  The five `does not mention …` rows
the first run reported (`haiku_message_to_lisp` case/default/float, `haiku_lisp_to_message`
case/continue/default/double/float/goto, `haiku_report_system_error` case/default,
`haiku_selection_disowned` case, `syms_of_haikuselect` doc) are the recorded spelling
differences of `## Deviations` 5/6/8 and now sit in `docs/ports/macro-body-notes.tsv`
(`## Deviations` 11).

**⑤ The five text gates (whole-tree last lines; whether this lane's files appear).**

| gate | whole-tree last line | this lane's files |
| --- | --- | --- |
| `check-declarations.py` | `18 declaration sites (1 forbidden, 17 allowed literals), 1 failures` (the one finding is `femacs/oldXMenu/insque.rs`, another lane's file) | no finding |
| `check-mirror-dupes.py` | `21 duplicate declaration(s), 17 ratchet row(s), 4 failure(s)` | no finding (all four name `femacs/src/gmalloc.rs`; pre-existing) |
| `check-dead-file-coverage.py` | `116 ledger(s) bound (a C range table), 0 failure(s)` | this ledger is bound twice - to `src/haikuselect.c` (`37 range row(s) cover src/haikuselect.c (1450 lines)`) and, by the naming rule `## Owed` 8 records, to `src/haikuselect.h` - and reports no failure for either |
| `check-ledger-ticks.py` | `233 ledger(s) bound …, 0 failure(s), …, 357 row(s) with no function tick, 0 row(s) without a ledger` | no finding (this ledger has no `- [x]` ticks) |
| `check-citations.py` | `676 file(s) scanned, 0 drift, 0 range, 1522 external, 36 allowed, 0 stale` | no finding |

`df -h /` over the lane's runs: `1.8Ti` size, `4.4Gi`-`4.5Gi` available (`12Gi` used,
`73%`-`74%` capacity; the scratch `/tmp/hks-lane` is deleted at the end).

The five whole-tree readings were taken with **other lanes' files landing in the working tree
while this lane ran** (`femacs/src/gmalloc.rs`, `femacs/src/pgtkselect.rs`,
`femacs/src/w32uniscribe.rs`, `femacs/oldXMenu/*.rs`, `docs/ports/pgtkselect.md`,
`docs/ports/macro-body-notes.tsv`'s later rows): `check-mirror-dupes.py`'s four failures all name
`femacs/src/gmalloc.rs`, `check-declarations.py`'s one failure names
`femacs/oldXMenu/insque.rs`, and the bound/external counts move with those lanes' ledgers and
citations.  **No gate reports a finding against `femacs/src/haikuselect.rs`,
`docs/ports/haikuselect.md` or this lane's five `docs/ports/macro-body-notes.tsv` rows**
(`grep -c haikuselect` on the two failing gates' output: 0).

This lane's command budget: `build-tu` **2** (one failed run on the text before the three
missing `#[cfg]`s were added, one reading), `probe` **2** (Evidence ② and an identical repeat on
the restored file), `cargo test`/`gate-fast`/`xtask --all`/`xtask dead-arms`/`check-all-units.sh`
not run, git untouched.

## Deviations

1. **C 44's `static` forward declaration is dropped** (PORTING.md 0.1-12): Rust has no prototypes,
   so `haiku_lisp_to_message`'s definition stands where C defines it (C 457).
2. **C 219-221's comment is carried as a `//` comment** over `haiku_message_to_lisp`'s definition,
   and C 1258-1261's page break plus its "Notification support." banner the same way.
3. **`block_input`'s address is taken** (C 929, `be_drag_message`'s `void (*) (void)` argument).
   `include/blockinput.rs` 24 spells `block_input` as a Rust-ABI `unsafe fn` (it is an `INLINE`),
   which a C function pointer cannot take, so the unit defines the C-ABI copy C's own
   address-taking would emit, in the module `block_input_abi` (the name is kept inside the module
   because a second crate-root `block_input` beside the mirror's would be an `E0428`).  The mirror
   round that owes the C-ABI form is `## Owed` 1.
4. **Uninitialized C locals are initialized** (E0381): `dat`, `message`, `pbuf`, `buf`,
   `buf_size`, `name`, `count`, `type_code`, `msg`, `point_x`, `point_y`, `max_x`, `max_y`, `x`,
   `y`, `team_id`, `cargs`, `r#type`, `file`, `canonical`, `tem`, `t1`, `t2`, `data` and the value
   locals are zeroed or nil-initialized, and each first read follows the C call that fills it (the
   `src/haiku.rs` 31/… Finding 3 shape).
5. **C's `switch` statements become `match` on a labelled `'switch` block** (PORTING.md
   2.2/P-04), so a C `break` is `break 'switch` and C's `default:` is the `_` arm.  C 1223-1248's
   `switch (id)` has **no** `default:`, so its `_` arm exists only because Rust requires one; it
   `break`s exactly like the C `case`s do.
6. **`haiku_lisp_to_message`'s two `goto`s become labelled blocks.**  C 509's `continue;` is the
   `'body` block's `break 'body` inside `FOR_EACH_TAIL!`, because the port macro's stepper sits at
   the *end of the body* (lisp.h 5909-5912's `for` increment is what C's `continue` reaches);
   C 535-536's `goto decode_normally` and C 787-788's `default: decode_normally:` label are the
   `'decode_normally` block, whose end is C's label (the `src/androidmenu.rs` 462/470 shape).
7. **C 249-332 and C 347-386's multi-character character constants** (`'MSGG'`, `'BOOL'`, …) are
   unit constants named by the literals' own spelling, with C's value
   `(c1 << 24) | (c2 << 16) | (c3 << 8) | c4`.
8. **The Haiku scalars and error codes C reaches through `<support/SupportDefs.h>` and
   `<support/Errors.h>` are declared in the unit** (P-88 ①/②): `int8`, `int16`, `int64`, `uint32`
   and `B_OK`, `B_BAD_VALUE`, `B_ENTRY_NOT_FOUND`, `B_PERMISSION_DENIED`, `B_LINK_LIMIT`,
   `B_BUSY`, `B_NO_MORE_FDS`, `B_FILE_ERROR`, `B_NO_MEMORY`, `B_ALREADY_RUNNING`.  Their widths
   and **values are the SDK's documented ones, not measurements** (`## Not claimed`), and
   `int32`/`status_t`/`team_id` come from the existing `include/haiku_support.rs` mirror.
9. **`haiku_note_drag_motion_1` (C 1126-1133) falls off the end of its C body**; the port spells
   the `return Qnil ()` its `Lisp_Object` return type needs (the value is discarded at C 1165).
10. **Unit-local `TYPE_MAXIMUM!`/`TYPE_MINIMUM!`**: `<intprops.h>`'s two range macros reach C
    through `lisp.h` 42, but the frozen `lib/include/intprops.rs` keeps C's own text and is not
    compiled, so the unit writes the Rust primitive at the use site (the `src/androidselect.rs`
    112-115 shape) and spells C's `TYPE_RANGED_FIXNUMP (type, x)` as
    `TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (type), x, TYPE_MAXIMUM! (type))`.
11. **`docs/ports/macro-body-notes.tsv`: five rows appended at the end of the file (one append)**:
    `haiku_message_to_lisp` → `case,default,float`, `haiku_lisp_to_message` →
    `case,continue,default,double,float,goto`, `haiku_report_system_error` → `case,default`,
    `haiku_selection_disowned` → `case`, `syms_of_haikuselect` → `doc`.  They record the
    `switch`/`goto`/keyword spellings of Deviations 5/6/8 and the `DEFVAR_*!` macro's positional
    docstring, not a semantic gap.
12. **Line count.** C 1,450 vs Rust 1,660 (14.5%): Rust needs a line per item's gate attribute,
    per `#[unsafe(no_mangle)]`, and per `unsafe`/cast that C leaves implicit.  The
    `docs/ports/line-drift-tolerance.tsv` row is the integrator's (`## Owed` 3).
13. **C's implicit conversions are spelled** (PORTING.md 1.1): `INT_TO_INTEGER (timestamp)`'s
    `int64` → `EMACS_INT`, the `make_fixnum`/`make_int`/`make_uint`/`make_float` argument
    widening of `haiku_message_to_lisp`, `len`'s `ptrdiff_t` → `ssize_t` at C 194 and
    `ssize_t` → `EMACS_INT`/`ptrdiff_t` at C 110/115, `XFIXNAT`'s `EMACS_INT` → `uint32` at
    C 497/500, and the `int` → `EMACS_INT` promotions of `haiku_dnd_compute_tip_xy` (C 1107-1122),
    where C's `int` truncates again on each assignment.
14. **C's `type` and `ref` identifiers are Rust keywords**, so they keep their names as the raw
    identifiers `r#type` (C 937, 1130 and `haiku_notifications_notify_1`'s local, C 1268) and
    `r#ref` (C 176, 486, 551).
15. **The four `DEFVAR_*` variables this file owns** (`haiku_signal_invalid_refs`,
    `Vhaiku_drag_track_function`, `Vhaiku_lost_selection_functions`,
    `Vhaiku_drag_wheel_function`) are defined here, where C's storage is
    `globals.f_<vname>`; `include/globals.rs`'s `variables` declaration is shadowed (the
    `src/haikufns.rs` 4150-4182 shape).
16. **`ie->kind` is the mirror's `input_event::bits` unit**, so C 1183/1186/1188's read and write
    go through a unit-local `input_event_bits!` (P-80's member-named shape, the
    `src/pgtkterm.rs` 191 copy).
17. **`haiku_note_drag_motion_1`/`haiku_note_drag_motion_2`/`haiku_should_quit_drag`/
    `haiku_unwind_drag_message`/`haiku_unwind_clipboard_lock` are `unsafe extern "C" fn`** (C's
    `static`), because their addresses are taken (C 187, 921, 929, 1165).
18. **`*(buf as *const bool)`** at C 259 reads C's own `bool`; the value written on the producing
    side is a Rust `bool` (1 byte, 0 or 1), so the read is C's, and the ledger records the
    assumption rather than widening it to a byte test.

## Owed

1. **`include/blockinput.rs`'s `block_input` is Rust-ABI**, and C takes its address at C 929
   (`be_drag_message`'s C function pointer).  Owner: `docs/ports/include-blockinput.md` - the
   mirror needs the C-ABI form P-26/P-39 ask for (this unit works around it with the
   `block_input_abi` module, `## Deviations` 3).
2. **A Haiku round**: with `<support/SupportDefs.h>`/`<support/Errors.h>` in hand, confirm the
   widths of `int8`/`int16`/`int64`/`uint32` and the **values** of the ten `B_*` codes this unit
   declares (`## Deviations` 8).
3. `docs/ports/line-drift-tolerance.tsv`: one row, `haikuselect`, tolerance >= 0.145, basis
   `total` (`## Deviations` 12).  Not in this lane's writable set.
4. **`docs/baseline/port-lines-pending.tsv`'s row is now stale**: the tree reads
   `stale    src/haikuselect.c: no longer under the threshold` for the row
   `src/haikuselect.c 1450 11 145 known-unported target (plan batch F)` - 1,660 Rust lines for
   1,450 C lines is 114%, not 1%.  `python3 tools/check-port-lines.py --write-pending` is the
   integrator's step (the `haikufns.md` Owed 5 precedent), and the `docs/port-map.md` row's target
   count (`femacs/src/haikuselect.rs | 11`) is regenerated with it.
5. **`include/systime.rs` 28's `HAIKU` assertion** stays the arm probe's one error, so no Haiku
   configuration can compile a whole crate until the constant lands.  Owner:
   `docs/ports/include-systime.md` (the same item `docs/ports/haikuterm.md` Owed 6 names).
6. The five `macro-body-notes.tsv` rows are **declarations of a spelling difference**, not proof
   that the bodies match; the body comparison itself is `extract-abi --check`'s reading above.
7. This ledger's `## C line ranges` table is machine-read by `check-dead-file-coverage.py`; a
   future slice that moves an item must keep the table ascending and complete.
8. **`check-dead-file-coverage.py`'s ledger lookup binds this ledger to the header row too.**
   Its `ledger_for` turns `femacs/src/include/haikuselect.rs` into the candidates
   `docs/ports/src-haikuselect.md` and `docs/ports/haikuselect.md` - not the header's own
   `docs/ports/include-haikuselect.md`, which is the name the rule's docstring and
   `check-ledger-ticks.py` use for `src/include/` targets.  The table above is therefore written
   contiguously (it covers every line 1-1450, blank lines included), so the mis-binding is a
   no-op pass rather than a spurious gap.  Owner: `tools/check-dead-file-coverage.py`'s
   `ledger_for` (or the `include-*.md` convention it contradicts); measured with this ledger in
   place: `116 ledger(s) bound (a C range table), 0 failure(s)`, the `src/haikuselect.h` row
   among them.

## Not claimed

- **No Haiku-target compile or link.**  There is no Haiku SDK or toolchain on this machine, so
  Evidence ②'s reading is a `/tmp`/`build`-tree harness reading on this host, not a Haiku build;
  nothing here runs.
- **No runtime or device behaviour.**  The clipboard, the drag-and-drop loop, the roster launch,
  the node attribute write and the notification call are transcribed, not exercised.
- **Every Haiku width and value below `## Deviations` 8 is C's spelling, not a measurement**, and
  the records/constants are **UNVERIFIED** - `## Owed` 2 is the round that measures them.
- **`xtask gate-fast`, `check-all-units.sh`, `cargo test`, `xtask dead-arms` and
  `extract-abi --all` were not run** (this lane's budget names them as out of scope), and the C++
  half of the backend (`haiku_select.cc`, the `be_*` implementations) is out of reach here.
- **The `_root.rs` line numbers of Evidence ②/③ are the unit's own** only because every
  `include!` is replaced by one line; an error reported against a `_Users_…` copy is a mirror's,
  and the one such error is named there.
