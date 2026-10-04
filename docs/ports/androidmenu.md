# emacs/src/androidmenu.c (860 lines)

> **P-86's first unit: the frozen configuration compiles nothing here.** `configure.ac` 2598 puts
> `androidmenu.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2604), and
> `docs/baseline/frozen-build-sources.txt` lists no `androidmenu.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
> readings, and the `## Deviations`.  Port: `femacs/src/androidmenu.rs` (1,011 lines), both arms of
> every `#if` carried.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`ANDROID_OBJ`, i.e. `HAVE_ANDROID`) | `feature = "have_android"` (on every item and on C 22-27's include graph) | an Android configuration |
| C 29 `#ifndef ANDROID_STUBIFY` … C 827 (real JNI arm) | `all (feature = "have_android", feature = "not_android_stubify")` | Android, API calls real |
| C 829 `#else` … C 837 (stub `popup_activated`) | `all (feature = "have_android", not (feature = "not_android_stubify"))` | Android, API calls stubbed |
| C 839-860 (`DEFUN`, `init_androidmenu`, `syms_of_androidmenu`) | `feature = "have_android"` | an Android configuration |
| C 850 `#ifndef ANDROID_STUBIFY` (inside `init_androidmenu`) | `feature = "not_android_stubify"` (nested in the file gate) | Android, API calls real |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6430).  `configure.ac` 2606 defines `ANDROID_STUBIFY` when the Android window
system is configured "but all API calls need to be stubbed out".

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 22-27 | `lisp.h`, `androidterm.h`, `android.h`, `blockinput.h`, `keyboard.h`, `menu.h` | file gate |
| 29 | `#ifndef ANDROID_STUBIFY` | gate (see the table above) |
| 31 | `#include <android/log.h>` | real arm (NDK header, no mirror) |
| 33-41 | `popup_activated_flag` (static), `current_menu_serial` | real arm |
| 43-47 | `popup_activated` | real arm |
| 49-68 | `struct android_emacs_context_menu`, `menu_class` | real arm |
| 70-119 | `android_init_emacs_context_menu` (C 89-101 the two `FIND_METHOD*` macros, C 103-115 their seven uses, C 117-118 the `#undef`s) | real arm |
| 121-125 | `android_unwind_local_frame` | real arm |
| 127-143 | `android_push_local_frame` | real arm |
| 145-171 | `struct android_dismiss_menu_data`, `android_dismiss_menu` | real arm |
| 173-212 | `android_process_events_for_menu` | real arm |
| 214-241 | `struct android_menu_subprefix`, `android_free_subprefixes` | real arm |
| 243-593 | `android_menu_show` (C 347-348 + C 382-383 the `goto next_item`/label pair, C 502/515/519 the three `goto finish`, C 590-592 the `finish:` block) | real arm |
| 595-651 | `struct android_emacs_dialog`, `dialog_class`, `android_init_emacs_dialog` (C 631-650 the two macros) | real arm |
| 653-790 | `android_dialog_show` | real arm |
| 792-827 | `android_popup_dialog` | real arm |
| 829-837 | `#else`: `popup_activated` returning 0 | stub arm |
| 839-845 | `DEFUN ("menu-or-popup-active-p", …)` | file gate |
| 847-854 | `init_androidmenu` (C 850-853 the nested `#ifndef`) | file gate / real arm (inner block) |
| 856-860 | `syms_of_androidmenu` | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/androidmenu cargo run -q -p xtask -- build-tu src/androidmenu.c`
→ `built src/androidmenu.rs -> ../build/femacs/androidmenu.o`, **exit 0**, and
`build/femacs/androidmenu.o` is **336 bytes** - an empty object.  That is the expected reading,
not a proof of the new text: every item of the port sits behind a `have_android` gate, so under
the frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute
(measured on the final revision with xtask's own materialization rule reproduced in `/tmp`: 46
files, exit 0, 336-byte object - the two `build-tu` runs above were on earlier revisions whose only
differences are the header comment and lines *inside* the gates).

**② Flips of the arm (P-86 ③⑴; the file gate plus the sub-gate).**
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort
--cfg 'feature="have_android"' -o /tmp/androidmenu_x.o build/femacs/units/androidmenu/_root.rs`
→ **exit 1, 11 `error` lines (10 errors + the abort line)**: 1 × `E0428 the name Emacs_Cursor is
defined multiple times` (`include/androidgui.rs` 53 vs `include/dispextern.rs` 82 - the collision
`docs/ports/include-androidterm.md`'s "Known interaction for a future Android target" and
`docs/ports/include-androidgui.md`'s layout-check section already record) and 9 × `E0425` for
`window`/`WINDOWP`/`XWINDOW` (frame.rs 469/470/939), `coding_system` ×4 (termhooks.rs 215/216/
354/357) and `buffer` ×2 (keyboard.rs 231/232) - names whose owning mirror (`window.rs`,
`coding.rs`, `buffer.rs`) C's header include list does not reach either (the P-72 rule-4 class
androidterm.rs's own comment records for `struct window *`).  **Every one of the 10 is in the
included mirrors; 0 are in the unit's own text.**

With the sub-gate as well
(`--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`, which is the pair that
selects the real arm) the reading is **exit 1, the same 10 errors, 0 of them in the unit's own
text**.

**②a The flip's "0 in our text" is vacuous - measured.**  Injecting a deliberate type error into
the unit's own text (`return popup_activated_flag + c"…".as_ptr ();`) changes the arm compile's
output *not at all*: still `error: aborting due to 10 previous errors`, the injected error nowhere
in the log.  rustc emits the mirror errors in name resolution and never reaches type-checking of
this unit's bodies.  The arm's own compilation evidence is therefore **②b**, not ②.

**②b Pre-flight: the unit's text type-checked against the real mirrors** (a `/tmp` harness - the
port's own text with its `include!` graph replaced by the real frozen mirrors where they compile
standalone (`config.rs`, `lisp.rs`, `android.rs`, `blockinput.rs`, `menu.rs`) and by stubs for the
three pieces that do not (keyboard.h's `menu_items`/`MENU_ITEMS_*`, androidterm.h's
`x_display_list`/`FRAME_ANDROID_WINDOW!`, `frame`)):
`rustc --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` → **exit 0, 0
errors**.  This is what compiled the arm's 500 lines; the same harness found ten defects of the
port in the first pass (a chained assignment C writes as `a = b = x` - Rust's assignment yields
`()`; `static mut x: T;` without a value; `if c` with a bare statement body; `record_unwind_*`'s
`unsafe extern "C" fn` parameter vs a Rust fn item; `FRAME_ANDROID_WINDOW`'s type).  With the
unit's *frozen* feature set the harness's stub environment compiles clean too (0 errors).

**③ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/androidmenu.c femacs/src/androidmenu.rs` →
2 `- ` lines: `- function android_menu_show does not mention goto` and
`- line count drifted 17.6%: C 860 vs Rust 1011`, then `FAIL`.
C's `goto` keyword has no Rust counterpart (PORTING.md 2.2: a forward jump is a labeled block or a
state machine), and the repo's practice for exactly this token is a per-function row in
`docs/ports/macro-body-notes.tsv` (204 rows there; `mpz_get_str`'s `goto` is the closest shape);
the line count is the structural expansion the tolerance table exists for (P-86's per-item
`#[cfg]` gates, Rust's `((*(*env)).X.unwrap ()) (…)` for C's `(*env)->X (…)`, and the
`unsafe extern` blocks C gets from its headers).  **Both rows are owed by the integrator** (see
below); everything else in the checklist is satisfied, including the seven function bodies whose
C tokens are compared.

**④ Other gates (repo root, final revision).**
`check-cond-eval.py src/androidmenu.c` → `src/androidmenu.c: 2 conditions, 0 unknown, 0
disagreements [VERIFIED]` / `2 conditions, 0 unknown to the evaluator, 0 disagreements` (the tool
takes paths under `emacs/`; `emacs/src/androidmenu.c` is not a form it accepts). —
`check-citations.py` → `675 file(s) scanned, 0 drift, 0 range, 350 external, 33 allowed, 0 stale` —
`check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures` —
`check-target-names.py` → `628 .rs files, 591 map targets, 0 finding(s)` —
`check-ledger-ticks.py` → `235 ledger(s) bound …, 0 failure(s)` —
`check-mirror-dupes.py` → `23 duplicate declaration(s), 23 ratchet row(s), 0 failure(s)` —
`check-port-lines.py` → `95 file(s) under ten percent, 245,578 reference line(s); 96 ratchet
row(s), 0 new, 1 stale`, the stale row being `src/androidmenu.c: no longer under the threshold`
(this unit's row retires; `--write-pending` is the integrator's step).

## Deviations

- **`#ifndef ANDROID_STUBIFY` → `not_android_stubify`, and the file gate → `have_android`.**  C's
  text has no `#ifdef HAVE_ANDROID`; the absence of the file from `ANDROID_OBJ` is the gate, so
  the port writes it per item *and* on the include graph (which is what makes the frozen
  configuration compile nothing here, matching `frozen-build-sources.txt`).  C's
  `!defined (ANDROID_STUBIFY)` takes the registered `not_android_stubify`, in the conjunction
  `all (have_android, …)` because that feature alone does not imply an Android configuration
  (the `alloc.rs` 6430 usage site nests it the same way).  Both arms are carried; the stub arm is
  reachable only with `have_android` set and `not_android_stubify` clear.
- **The flip needs *two* `--cfg`s.**  P-86 ③⑴ prescribes flipping "the file's gate"; for this file
  a single `--cfg 'feature="have_android"'` selects the *stub* arm (C 829-837, three lines), and
  the real arm needs `--cfg 'feature="not_android_stubify"'` too.  Both readings are in Evidence ②
  so the difference is visible rather than assumed.
- **`ANDROID_DELETE_LOCAL_REF` (android.h 340-342) is expanded in the unit under C's own name.**
  `docs/ports/excluded-macros.txt` records it as "no Rust text" because "the NDK's
  `JNINativeInterface` has no mirror" - a premise `include/jni.rs` (status 3.154, all 230 entries,
  verified by `spikes/jni-probe.sh`) has superseded.  The unit therefore defines
  `macro_rules! ANDROID_DELETE_LOCAL_REF` with C's expansion and calls it at C's 14 use sites, so
  C's token survives and the body check compares it; the per-site spelling without the macro (the
  first draft) failed that comparison (`- function … does not mention
  android_delete_local_ref, jobject`).  The integrator should retire the stale row (or move the
  macro into `include/android.rs`, which has the same `#ifndef ANDROID_STUBIFY` live arm).
- **C's `(jboolean)` casts in variadic calls are `as jboolean as c_int`.**  rustc rejects a
  variadic argument narrower than `c_int` (`E0617 can't pass u8 to variadic function`, measured),
  and C's own default argument promotions pass an `int` for exactly those arguments, so the
  second cast is C's promotion.  `jobject`/`jclass`/`jmethodID` keep C's cast spellings (C
  `(jobject) old` is `old as jobject`).
- **`ckd_add` (C 440).**  C's `ckd_add (&item_id, i + MENU_ITEMS_ITEM_VALUE, 0)` stores the
  `size_t` sum into an `int` and flags the non-representable case; the port keeps both halves - C's
  own `+ 0` as `overflowing_add (0)` (the `ckd_add`/`overflowing_add` family of
  `docs/ports/token-aliases.tsv`) and the representability test - because `overflowing_add` alone
  cannot express the narrowing store (the `buffer.rs` 3602 shape).
- **`__func__` (C 331).**  C's `__func__` is this function's own name as a function-scope array;
  the port keeps the name (`let __func__: *const c_char = c"android_menu_show".as_ptr ();`), which
  is also what makes the C token comparable.
- **`goto` (C 347-348, 502, 515, 519).**  `goto next_item` is the labeled block `'next_item`
  (whose `break` is the jump, C's label text kept) and the three `goto finish` are `break` out of
  the `'finish: loop` whose tail is C's `finish:` block - PORTING.md 2.2's forward-jump shape, one
  copy of the label's statement, no early `return`.  The `goto` keyword itself has no Rust
  counterpart: the per-function row in `docs/ports/macro-body-notes.tsv` is owed.
- **C's function-local macros.**  `FIND_METHOD`/`FIND_METHOD_STATIC` are `macro_rules!` inside the
  same function; `macro_rules!` scope ends with the block, so C 117-118's `#undef`s have no
  counterpart.  C's dialog pair keeps C's asymmetry: C 638-646's `FIND_METHOD_STATIC` body has no
  `eassert` (C 647 is the blank line its last backslash continues into) while C 631-637's
  `FIND_METHOD` has one - the port keeps both as written.
- **Three C `static` functions are `unsafe extern "C" fn`.**  `android_unwind_local_frame`,
  `android_dismiss_menu` and `android_free_subprefixes` are passed to
  `record_unwind_protect_void`/`_ptr`, whose parameter is `unsafe extern "C" fn`; a Rust fn item is
  the Rust ABI and does not unify (`E0308`, measured in the pre-flight).  The `callproc.rs`
  `call_process_kill` is the precedent.
- **Uninitialised locals (PORTING.md 1.5/0.1-13).**  Named here: `data` (C 259,
  `android_dismiss_menu_data`, zeroed because C stores its two members later),
  `popup_activated_flag` and `current_menu_serial` (C 36/41, C's tentative definitions, i.e. zero),
  `id` in `android_menu_show` (C 258, address taken at C 511) and in `android_dialog_show` (C 659,
  address taken at C 762), `error_name` in `android_popup_dialog` (C 797, address taken at C 817),
  and the two static class records `menu_class`/`dialog_class` (C 68/610, C's tentative
  definitions).  `android_popup_dialog`'s `contents` parameter is reassigned at C 811, so the port
  rebinds it (`let mut contents = contents;`, the `term.rs` shape).
- **Seven declarations the frozen mirrors do not carry** (P-72 rule 4): `quit` (lisp.h 4786),
  `process_pending_signals` (lisp.h 3880), `memory_full` (lisp.h 4432, `AVOID` → `-> !`),
  `record_unwind_protect_void` (lisp.h 4825), `menu_separator_name_p` (keyboard.h 473),
  `check_window_system` (frame.h 1504), `unuse_menu_items` (keyboard.h 385).  The unit declares
  them at the use site (the `alloc.rs` `mark_haiku_display` / `android-asset.rs` `ESPIPE` shape);
  each should move into its mirror when that mirror is completed.
- **NDK values measured, not assumed** (this machine's SDK: NDK
  `~/Library/Android/sdk/ndk/29.0.14206865`, the same one `spikes/jni-probe.sh` defaults to; the
  task's 25.1/25.2/26.1 are also installed).  `<android/log.h>` 74-89 gives
  `ANDROID_LOG_FATAL == 7` and its 112 declares
  `int __android_log_print(int prio, const char *tag, const char *fmt, ...)`;
  `android.rs`'s `android_get_current_api_level!` names `__ANDROID_API__`, which the NDK defines as
  `__ANDROID_MIN_SDK_VERSION__` (measured 24 for `--target=aarch64-linux-android24`).  Both are
  declared in the unit because `<android/log.h>` has no mirror and the macro's own comment leaves
  `__ANDROID_API__` to its expansion site, which this unit is the first of.
- **Line count.**  C 860 vs Rust 1,011 (17.6%): P-86's per-item `#[cfg]` gates (both arms carried),
  Rust's expression verbosity for the JNI vtable calls and the seven mirror-gap declarations.  A
  `docs/ports/line-drift-tolerance.tsv` row (stem `androidmenu`, tolerance ≥ 0.18, basis `total`) is
  owed.
- **`--check` is no longer vacuous for this file.**  `empty_unbuilt` (extract-abi 1362) exempts
  only a port with *no* `fn` at all; this unit has them, so the full comparison - seven function
  bodies, `defsubr`, invented-symbol and line-count checks - applies and is what ③ reports.

## Owed by the integrator (nothing here is claimed as done)

1. `docs/ports/macro-body-notes.tsv`: one row, `android_menu_show` → `goto` (PORTING.md 2.2's
   labeled-block spelling; the `mpz_get_str` row is the precedent).
2. `docs/ports/line-drift-tolerance.tsv`: one row, `androidmenu` `0.18`–`0.20`, `total`, with the
   reason from the Deviations above.
3. `tools/check-port-lines.py --write-pending`: the `src/androidmenu.c` row is already stale.
4. The mirror rows this unit proved are missing or stale: `include/jni.rs` exists, so
   `docs/ports/excluded-macros.txt`'s `ANDROID_DELETE_LOCAL_REF` row and `include/android.rs`'s
   comment ("`JNINativeInterface` has no mirror") should be rewritten; the seven declarations above
   belong in `lisp.rs`/`keyboard.rs`/`frame.rs`; the `Emacs_Cursor` pair (`androidgui.rs` 53 vs
   `dispextern.rs` 82) and the `window`/`buffer`/`coding_system` names need the `cfg`s
   `docs/ports/include-androidterm.md` already asks for - until then an Android configuration
   cannot compile any translation unit, which makes every `have_android` flip vacuous (Evidence
   ②a).
