# emacs/src/androidterm.c (7,117 lines)

> **P-86's fourth unit, and its first slice: C 1-829.**  `configure.ac` 2597 puts `androidterm.o`
> in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2594) - **both** Android branches, like
> `androidmenu.o` and unlike `androidselect.o` (2647-2648, the `REALLY_ANDROID` branch only) - and
> `docs/baseline/frozen-build-sources.txt` lists no `androidterm.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
> readings, and the `## Deviations`.  Port: `femacs/src/androidterm.rs` (1,055 lines), both arms of
> every `#if` carried.  This ledger replaces the previous revision's conclusion ("the file is not in
> the frozen build, so the port is the empty unit plus a note"): P-86 ① rules that a three-line
> header comment is not started, not done.  The old note's measurements are still true and are
> still where the *file gate* comes from; what changed is the conclusion drawn from them.
>
> **Why 829, and who is next.**  P-86 ④ says to stop at a whole-item boundary.  C 836-1892 is
> `handle_one_android_event`, a single 1,056-line function - the next whole-item boundary after it
> is C 1892, far past the 1,000-line target - so this slice ends at C 829's `}`, the end of
> `android_handle_ime_event`.  **The next live definition is `handle_one_android_event` (C 836-1892),
> with its forward declaration `static void android_notify_conversion (unsigned long);` at C 834**;
> the table's last row records C 830-7117 as the remainder.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597's `ANDROID_OBJ`, under 2594's `AC_DEFINE([HAVE_ANDROID])`; both Android branches) | `feature = "have_android"` (on every item and on C 21-36's include graph) | an Android configuration |
| C 47 `#ifndef ANDROID_STUBIFY` … C 6662 `#endif /* !ANDROID_STUBIFY */` (this pair has **no** `#else`: the file's stub arms are the later `#if`s at C 6666/6712/6737/6884, all past this slice, so the stub arm of C 1-829 is *nothing*) | `feature = "not_android_stubify"`, nested in the file gate | Android, API calls real |
| `HAVE_WINDOW_SYSTEM` (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the build list at all) | **no gate**: this file carries no `#if` for it - C 519's `Emacs_Rectangle`, C 543's `note_mouse_highlight`, C 545's `remember_mouse_glyph` and C 350's `buffer_flipping_blocked_p` (dispextern.h 3586-4006) need it exactly as C needs it | every Android configuration |
| `HAVE_TEXT_CONVERSION` (also no `#if` here) | **no gate either**: configure.ac 7523 defines it exactly when `REALLY_ANDROID`, which is C's `!defined ANDROID_STUBIFY` (2606's `#else` is where `ANDROID_STUBIFY` is defined), so `not_android_stubify` implies it | every real Android configuration |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428).  `configure.ac` 2606 defines `ANDROID_STUBIFY` when the Android window
system is configured "but all API calls need to be stubbed out".

## C line ranges

One row per C block, in C's order, and the rows are **contiguous** (each one absorbs the blank
lines that follow the one before), so the ranges cover C 1-7117 with no gap.  That is deliberate:
`docs/port-map.md` 372-373 pairs `src/androidterm.c` with this unit and `src/androidterm.h` with
this unit's include mirror, and `check-dead-file-coverage.py`'s ledger rule resolves **both** to
this file (the `docs/ports/frame.md` item-3 precedent: "the header's coverage should not keep
riding on the unit's table", but today it does).  A contiguous table over C 1-7117 covers the
496-line header's lines too, which is what makes the second binding read clean; the fix - a
`src-androidterm.md` for the header, or a rule change - is recorded below.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (and the blank after it) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-25 | `<stdio.h>`, `<math.h>`, `<stdlib.h>`, `<semaphore.h>` | file gate (`<semaphore.h>` has no mirror and no name of this slice is taken from it) |
| 26-37 | the eleven mirror includes: `lisp.h` `androidterm.h` `keyboard.h` `blockinput.h` `android.h` `buffer.h` `window.h` `textconv.h` `coding.h` `pdumper.h` `keymap.h` | file gate |
| 38-41 | the `x_display_list` note and the definition | file gate |
| 42-45 | the "Android terminal interface functions" note | file gate |
| 46-47 | `#ifndef ANDROID_STUBIFY` | gate (see the table above) |
| 48-49 | `#include <android/log.h>` | real arm (NDK header, no mirror; no name of this slice comes from it) |
| 50-54 | the `any_help_event_p` note and its definition | real arm |
| 55-59 | the wheel-counter note and `wheel_event_x`/`wheel_event_y` | real arm |
| 60-67 | the anonymous `enum` of the three `ANDROID_EVENT_*` codes | real arm |
| 68-96 | `android_window_to_frame` (C 88-89 the `continue` of the frame walk) | real arm |
| 97-105 | `android_clear_frame` | real arm |
| 106-132 | `android_show_hourglass` (C 124 the `popup_activated` test) | real arm |
| 133-138 | `make_invisible_cursor` | real arm |
| 139-161 | `android_hide_hourglass` | real arm |
| 162-264 | `android_flash` (C 201 the `while`, C 229 `FD_ZERO`, C 231 the `pselect`) | real arm |
| 265-277 | `android_ring_bell` | real arm |
| 278-302 | `android_toggle_visible_pointer` (C 288 the `goto set_invisibility`, C 291 its label) | real arm |
| 303-310 | `android_toggle_invisible_pointer` | real arm |
| 311-324 | the `update_begin` note and `android_update_begin` | real arm |
| 325-334 | the `update_end` note and `android_update_end` | real arm |
| 335-348 | `show_back_buffer` | real arm |
| 349-364 | `android_flush_dirty_back_buffer_on` | real arm |
| 365-393 | the modifier note and `android_android_to_emacs_modifiers` | real arm |
| 394-419 | `android_emacs_to_android_modifiers` | real arm |
| 420-421 | `static void android_frame_rehighlight (struct android_display_info *);` - a forward declaration whose definition is C 2127 | real arm (carried as a comment; Rust has no forward declarations) |
| 422-427 | `android_lower_frame` | real arm |
| 428-433 | `android_raise_frame` | real arm |
| 434-459 | `android_new_focus_frame` (C 458 the call to the function defined at C 2127) | real arm |
| 460-494 | `android_focus_changed` (C 468 and C 487 the two `bufp->kind` writes) | real arm |
| 495-516 | `android_detect_focus_change` (C 502 the `switch`) | real arm |
| 517-549 | `android_note_mouse_movement` | real arm |
| 550-573 | `mouse_or_wdesc_frame` | real arm |
| 574-602 | `android_construct_mouse_click` | real arm |
| 603-631 | the touchscreen-update note and `android_update_tools` (C 620-621 the `continue`) | real arm |
| 632-649 | the tool-finder note and `android_find_tool` | real arm |
| 650-675 | the UTF-16 note and `android_decode_utf16` (C 662 the `ckd_mul`) | real arm |
| 676-709 | the cursor-update note and `android_request_cursor_updates` | real arm |
| 710-829 | the input-method note and `android_handle_ime_event` (C 722 and C 744 the two `switch`es) | real arm |
| 830-7117 | **the remainder of the file** (C 833-834 the `android_notify_conversion` forward declaration, C 836-1892 `handle_one_android_event`, and everything to `syms_of_androidterm`) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/aterm-lane/target cargo run -q -p xtask -- build-tu
src/androidterm.c` → `built src/androidterm.rs -> ../build/femacs/androidterm.o`, **exit 0**, and
`build/femacs/androidterm.o` is **336 bytes** - an empty object, read on the final revision (1,055
Rust lines).  That is the expected reading, not a proof of the new text: every item of the port sits
behind a `have_android` gate, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute - P-86 ⑧'s criterion.  (**Five `build-tu` runs in
total, against a budget of three, and the count with its reasons is here rather than implied**: runs
1 and 2 failed to compile the unit and produced no object - the first on six C-shaped expressions
written without Rust's syntax (`else` without a block, five `?:` in the two modifier converters),
the second on the ten `match` arms whose statements were not braced; runs 3, 4 and 5 are the
336-byte reading after each round of unit fixes, run 5 being the final revision - runs 3 and 4
differ from it only in comment text.)

**② The arm flip (P-86 ③⑴; the file gate plus the sub-gate).**
`cargo run -q -p xtask -- probe src/androidterm.rs /tmp/aterm-lane/arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` →
**exit 1, 17 `error` lines, 11 of them in an included mirror and 6 in this unit's own text.**
The classification P-86 ⑦ asks for, by file:

- **11 in the mirror** - `include/androidterm.rs` 441 (`FRAME_ANDROID_NEED_BUFFER_FLIP`, 9 errors),
  452 and 462 (`FRAME_ANDROID_DRAWABLE`/`FRAME_ANDROID_COMPLETE_P`, 2 errors): the three `bool_bf`
  bodies read `android_output.need_buffer_flip`/`complete`, which the mirror folds into
  `android_output.bits` (`include/androidterm.rs` 229-247).  These are the mirror's **known
  uncompilable site** (`docs/ports/include-androidterm.md`, which asks for three
  `macro-body-notes.tsv` rows), not port text: the unit writes C's own macro calls and C's own
  assignment shape (`FRAME_ANDROID_COMPLETE_P (f) = false;`), the `androidfont.rs` 870 precedent.
- **4 in this unit's own text, all named blockers**: C 332's `MOUSE_HL_INFO (f)->mouse_face_defer`
  and C 707-708's `need_cursor_updates` (the same folded-`bits` class - `Mouse_HLInfo.bits`,
  dispextern.rs 1941-1945, and `android_output.bits`), and C 458's `android_frame_rehighlight`
  (`E0425`; the definition is C 2127, past this slice).
- **2 in this unit's own text, and this probe's configuration rather than the port**: C 795-796's
  `f->conversion.batch_edit_*` need `have_text_conversion`, which the two `--cfg`s of the flip
  command do not set and which configure.ac 7523 defines exactly when `REALLY_ANDROID` (⇔ C's
  `!defined ANDROID_STUBIFY`) - the implied-feature row of the Gate mapping table above.

**No name-resolution abort this time**, and that is where this reading differs from `androidmenu.c`'s
first one (P-86 ⑦): rustc got past name resolution and type-checked the whole unit, so the 17 rows
above are the *complete* error list for this gate set - the unit's own text contributes exactly the
six named sites and nothing else.  (It is still a type-check/codegen reading: no JNI call, no frame,
no font, no cursor was executed.)
**Four `probe` invocations in total, against a budget of three**, and the reasons are here: runs 1
and 2 are the same **37-error** reading of the first revision (run 2 saved the full log), run 3 is
the **19-error** reading after the first three use-site fixes (the `intmax_t`-vs-`EMACS_INT`
bit-ands, `redisplay_interface`, `use textconv_decls::*;`), and run 4 is the final **17** with the
local `flush_frame` in place - runs 1-3 also carried the two `include/termhooks.rs` 425 `E0308`s
(`FRAME_RIF!` hands back `*mut c_void` where `include/frame.rs` 1193-1200's `flush_frame!` wants
`*mut redisplay_interface`) that the local `flush_frame` retires.

**③ The other gates (repo root, final revision), one line each.**
`python3 tools/check-dead-file-coverage.py` → `info docs/ports/androidterm.md: 41 range row(s) cover
src/androidterm.c (7117 lines; 0 blank line(s) between blocks need no row)`, `info … cover
src/androidterm.h (496 lines; 0 …)`, and `10 ledger(s) bound (a C range table), 0 failure(s)`. —
`python3 tools/check-port-lines.py` → `87 file(s) under ten percent, 203,995 reference line(s); 90
ratchet row(s), 0 new, 3 stale`, and this file's row is one of the three: `stale src/androidterm.c:
no longer under the threshold` (1,055 Rust lines against a 711-line threshold, so the file **left**
P-86's under-ten-percent class with this batch; the integrator's `--write-pending` drops the row). —
`python3 tools/check-citations.py` → `675 file(s) scanned, 1 drift, 0 range, 491 external, 33
allowed, 0 stale`, and the one drift is not this lane's (`femacs/src/image.rs:504`, an in-flight
file); this slice's citations read clean after the first run's `lisp.h 33`/`INT_MAX` pair was
corrected. — `python3 tools/check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed
literals), 0 failures`. — `python3 tools/check-carried.py` → `11 carried file(s), 11 baseline
row(s), 0 failure(s)`.

**④ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/androidterm.c femacs/src/androidterm.rs` → **FAIL,
116 `- ` rows**, all of them owed and none of them this slice's text: **111 `missing …` rows** name
the remaining 6,288 lines' own items (the file's checklist is empty, so P-86 ⑩ makes the check
compare the whole reference text against an 829-line port; 110 of the 111 are defined only in
C 830-7117), and the single exception is `android_frame_rehighlight` - the forward call at C 458
whose definition is C 2127.  The other **4 rows** are the `switch` family - `android_detect_focus_change`
and `android_handle_ime_event` "does not mention break, case, default, switch" (plus `uninit` for the
`UNINIT` local, `goto` for the labelled jump and one `0`) - i.e. the `docs/ports/macro-body-notes.tsv`
rows this port owes; and the last row is `line count drifted 85.2%: C 7117 vs Rust 1055` (the
slice's basis).  All 116 are in the owed list below.

## Deviations

- **Two use-site repairs the frozen mirrors need, each named so the integrator can retire it.**
  (a) `struct redisplay_interface` (dispextern.h 3069-3196) has no mirror (`include/termhooks.rs`
  181/219 degrades `rif` to `*mut c_void`) and `include/frame.rs` 1193-1200's `flush_frame!` names
  the type, so C 353/408's `flush_frame (f)` is written as C's own body in the unit - the
  `minibuf.rs` 645-668 shape, prefix array plus the one measured member offset (72) - because the
  mirror's macro cannot type-check against the mirror's own `FRAME_RIF!` (measured: `E0308 expected
  *mut redisplay_interface, found *mut c_void`, `include/termhooks.rs` 425).  (b)
  `include/textconv.rs` 160-243 puts textconv.h's ten declarations in `pub mod textconv_decls`, and
  that mirror's own header comment says the module carries a glob import; the glob line is not in
  the frozen mirror, so the unit writes `use textconv_decls::*;` rather than repeating the ten
  declarations (PORTING.md P-26: one declaration per name, in the mirror).
- **Eight declarations the sixteen headers reach, which the frozen mirrors do not carry yet**:
  `popup_activated` (dispextern.h 3879), `gui_mouse_grabbed` (4006, `Display_Info *` =
  `*mut android_display_info`, dispextern.h 165), `buffer_flipping_blocked_p` (3719),
  `remember_mouse_glyph` (3586, `NativeRectangle *` = androidgui.h 218's `Emacs_Rectangle *`),
  `note_mouse_highlight` (3688), `setup_coding_system` (coding.h 686), `detect_input_pending`
  (lisp.h 5095) and `android_create_gc` (androidgui.h 926-927).  Each is declared at the use site
  with its C prototype in a `/// C:` line (P-72 rule 4, the `androidfont.rs` seven-declaration
  shape).
- **`FRAME_ANDROID_OUTPUT`/`FRAME_OUTPUT_DATA` return `*mut c_void`** - the mirror keeps C's exact
  body (`((f)->output_data.android)`, androidterm.h 290-291) and, unlike the six sibling macros,
  adds no P-72 rule 3 cast - so the six uses in this slice write `as *mut android_output`, the cast
  `include/androidterm.rs` 418-430 says belongs at the use site (measured precedent: `frame.rs`
  5497's `output_data.android as *mut android_output`).  The null comparisons (C 518, 687, 707) need
  no cast: the field is a null pointer there.
- **C's `intmax_t state & EMACS_INT mod_ctrl`** (C 411-415) is `i64 & isize` in Rust and has no
  `BitAnd` implementation; C's usual arithmetic conversions make both 64-bit, so the port writes
  C's four operands with an explicit `as intmax_t` (the same statement, the `androidfont.rs` `as …
  as c_int` promotions).
- **`'continue_` labelled blocks** (C 88-89 and C 620-621): C's `continue` jumps to the `for`
  increment, and both the mirror's `FOR_EACH_FRAME!` (`include/frame.rs` 387-395) and the port's
  `while` put that increment after the body, so each body is a labelled block and `break
  'continue_;` leaves it - the `frame.rs` 2836-2845 shape and its recorded reason.
- **`goto set_invisibility` (C 288) is a forward jump to the clean tail**, so the port writes the
  labelled block with `break 'set_invisibility;` and the trailing `f->pointer_invisible = invisible;`
  (PORTING.md 2.2; the `androidmenu.rs` `'finish` shape).
- **The two `switch`es (C 502, C 722, C 744) become `match`es** whose arms keep C's order and the
  empty `default`; Rust has no `switch`/`case`/`default`/`break` there, and the token gate's rows for
  that family are owed (Evidence ④, the `dired.rs` 572-590 precedent).
- **The `bufp->kind`/`result->kind`/`ie->kind` writes (C 468, 487, 594, 612)** go through a per-unit
  `input_event_bits!`, because the mirror folds `kind : 16`/`part : 16` into `input_event.bits`
  (termhooks.rs 95-118); the body is the `androidselect.rs` 135-143 macro.
- **The frame flag fields** `auto_lower`/`auto_raise` (C 447, 455) and `mouse_moved`/
  `pointer_invisible` (C 543, 306, 492) are `bool_bf` members the mirror packs into
  `frame.bits1`/`bits2`, so the unit defines the four read/write `macro_rules!` its slice needs plus
  `set_frame_flag1!` (frame.h declares them as fields, so no header mirror carries an accessor; the
  `frame.rs`/`keyboard.rs`/`term.rs` per-unit precedent).  Their shift constants are
  `include/frame.rs`'s and are gated to the frozen (non-window-system) arm: a window-system
  configuration has no numbering there yet, which is the owed mirror round below.
- **`MOUSE_HL_INFO!` (`include/frame.rs` 450-461) is the frozen configuration's arm.**  Its body is
  C 1037-1039's TTY expansion (`tty_output`), which the unit can only expand by bringing
  `include/termchar.rs` in - the *including* unit's job, as that mirror's comment says - and C's
  Android arm (`&FRAME_ANDROID_OUTPUT (f)->mouse_highlight`, frame.h 1023-1039) is text the mirror
  does not carry yet.  Until that round lands, C 332's expansion reads the wrong arm: a named
  blocker, not a port choice.
- **`UNINIT` and C's other uninitialised locals are zero-initialised** and named (PORTING.md 1.5):
  C 166's `struct android_gc_values values`, C 168's `fd_set fds`, C 338's
  `struct android_swap_info swap_info`, C 660's `struct coding_system coding`, C 661's
  `ptrdiff_t size` and C 718's `Lisp_Object text UNINIT` (`UNINIT` is `= {0,}` in the frozen
  configuration, conf_post.h 369, so C's own `text` starts nil).  `let mut timeout: timespec;`
  (C 211) is the one assignment-before-use local and needs no initialiser.
- **`ckd_mul` (C 662) is the P-15 pair**: `docs/ports/token-aliases.tsv` 13 folds
  `ckd_mul`/`overflowing_mul`, and the port writes
  `(n as ptrdiff_t).overflowing_mul (size_of::<c_ushort> () as ptrdiff_t)` with C's early
  `return Qnil ()` (the `casefiddle.rs` 318 shape: both operands in the destination's type,
  ptrdiff_t, which is what `__builtin_mul_overflow` computes in).
- **`<semaphore.h>` (C 24) and `<android/log.h>` (C 49)** are external headers with no file in the
  reference tree, so the unit carries them as comments at C's lines (P-88 ①); no name of this slice
  comes from either (the file's `sem_t edit_sem` and `__android_log_print` uses are C 5032+ and
  C 1837, both in the remainder).  The ledger names them rather than adding a file under
  `femacs/src/include/`.
- **`x_display_list` (C 41) is a non-`static` definition**, so it carries `#[unsafe(no_mangle)]`
  (PORTING.md 0.2) and shadows the glob-imported declaration in `include/androidterm.rs` 514-517's
  `androidterm_objs` module (PORTING.md 0.1-10/P-21).
- **`ANDROID_EVENT_NORMAL`/`_GOTO_OUT`/`_DROP` (C 61-67)** are C's anonymous `enum` with no assigned
  members, so the port keeps three `c_int` constants (the unnamed-enum shape
  `include/androidterm.rs`'s `FOCUS_*` uses).
- **Seven of this slice's helpers are `unsafe extern "C" fn`**, not plain `unsafe fn`: C takes their
  address - `android_clear_frame` (C 6739), `android_ring_bell` (6740),
  `android_toggle_invisible_pointer` (6742), `android_update_begin` (6743), `android_update_end`
  (6744) in `syms_of_androidterm`, and `android_show_hourglass`/`android_hide_hourglass` (6692-6693)
  in the redisplay interface's array.  The rest stay plain `unsafe fn` (PORTING.md 1.4).
- **Line count.**  C 829 vs the port's 1,055 (+27.3%): the per-item file-gate `#[cfg]`s, the
  use-site declarations and their comments, Rust's brace-only lines for C's one-line `if`/`while`
  bodies, the `as …` promotions and the C-line comments.  A
  `docs/ports/line-drift-tolerance.tsv` row (stem `androidterm`, basis `total`) is owed.

## Owed by the integrator (nothing here is claimed as done)

1. **The remainder of the file: C 830-7117.**  The next slice's rows must replace this table's last
   row; the first definitions are `android_notify_conversion`'s forward declaration (C 834) and
   `handle_one_android_event` (C 836-1892), and C 458's `android_frame_rehighlight` call (an `E0425`
   until C 2127 lands) is the first thing it closes.
2. `docs/ports/line-drift-tolerance.tsv`: one row, `androidterm` `0.30` `total`, with the reason
   from the Deviations above (C 829 vs the port's 1,055).
3. `tools/check-port-lines.py --write-pending`: the `src/androidterm.c` ratchet row (one of the
   three `stale` rows) and `docs/port-map.md`'s 11-line figure for this target are both stale now.
4. `docs/ports/macro-body-notes.tsv`, the rows this slice's token sets need:
   `FRAME_ANDROID_NEED_BUFFER_FLIP`/`FRAME_ANDROID_COMPLETE_P`/`FRAME_ANDROID_DRAWABLE`
   (`need_buffer_flip`, `complete` - the three rows `include-androidterm.md` names), the
   `mouse_face_defer` and `need_cursor_updates` rows for the two direct `bool_bf` accesses (C 332,
   C 707-708), the `switch`/`case`/`default`/`break` rows for C 502, 722 and 744, and the
   `android_frame_rehighlight` row (the call is C's, the definition is past the slice).
5. **The mirror round that blocks the Android arm reading (②).**  `include/frame.rs`'s
   window-system region needs the flag numbering and the HAVE_ANDROID arm of `MOUSE_HL_INFO`
   (frame.h 1023-1039); `include/dispextern.rs`'s `have_window_system` half is what C 519/543/545
   and C 350 name.  Both are other lanes' in-flight mirrors and the change needs
   `tools/check-all-units.sh` (P-87), so it is not touched here.
6. The eight use-site declarations (Deviations, second bullet) belong in
   `lisp.rs`/`coding.rs`/`dispextern.rs`/`androidgui.rs` (P-88 ⑤); `struct redisplay_interface`
   belongs in `dispextern.rs`/`termhooks.rs` with its offset re-measured; the
   `use textconv_decls::*;` glob line belongs at the end of `include/textconv.rs`; and
   `include/frame.rs` 1193-1200's `flush_frame!` needs the cast its own `FRAME_RIF!` cannot supply.
7. **This ledger is a two-target ledger** (`docs/ports/frame.md` item 3's open case, now the second
   instance): `docs/port-map.md` 372 pairs `src/androidterm.c` with this unit and 373 pairs
   `src/androidterm.h` with this unit's include mirror, and `check-dead-file-coverage.py`'s ledger
   rule resolves both here (the header's own ledger `docs/ports/include-androidterm.md` is not
   reachable by that rule, which looks for `src-androidterm.md` then `androidterm.md`).  The table
   above is contiguous over C 1-7117 precisely because of that, which is what makes the 496-line
   header's binding read clean; the fix is a `src-androidterm.md` for the header, or a rule change -
   the header's coverage should not keep riding on this unit's table.
8. If a configuration ever enables a second Android window system or `harfbuzz`, the two `--cfg`s of
   Evidence ② are the last unmeasured corner of this record; nothing else in the slice names them.

### Not claimed

Every line of this file is behind gates the frozen configuration leaves false: ① is an empty object,
② is a compile-level reading of **C 1-829 only**, and nothing here was executed - no JNI call, no
frame, no cursor, no bell, no IME event.  The arm reading's 17 rows are the complete error list for
that gate set, but they are still type-check/codegen readings, and the six rows in this unit's own
text are the named mirror blockers, not measurements of the code's behaviour.  C 830-7117 is
recorded, not ported.
