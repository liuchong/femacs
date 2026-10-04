# emacs/src/androidfont.c (1,102 lines)

> **P-86's third unit: the frozen configuration compiles nothing here.** `configure.ac` 2597 puts
> `androidfont.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2594), and
> `docs/baseline/frozen-build-sources.txt` lists no `androidfont.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
> readings, and the `## Deviations`.  Port: `femacs/src/androidfont.rs` (1,391 lines), both arms of
> every `#if` carried.  This ledger replaces the previous revision's conclusion ("the file is not in
> the frozen build, so the port is the empty unit plus a note"): P-86 ① rules that a three-line
> header comment is not started, not done.  The old note's measurements are still true and are
> still where the *file gate* comes from; what changed is the conclusion drawn from them.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597's `ANDROID_OBJ` - both Android branches, unlike `androidselect.o`, which 2647-2648 adds only in the `REALLY_ANDROID` one) | `feature = "have_android"` (on every item and on C 26-40's include graph) | an Android configuration |
| C 42 `#ifndef ANDROID_STUBIFY` … C 1102 `#endif` (this file has **no** `#else` arm, so its stub arm is *nothing*) | `feature = "not_android_stubify"`, nested in the file gate | Android, API calls real |
| the three `struct font_driver` members C guards with `#ifdef HAVE_WINDOW_SYSTEM` that C 1045 names (font.h 682-698: `draw`, `get_bitmap`, `free_bitmap`) | `feature = "have_window_system"`, the same condition `include/font.rs` spells for the declaration | any configuration whose mirror has those members |
| font.h 817-821's two `harfbuzz` members (`begin_hb_font`, `end_hb_font`), which C's designated initialiser leaves implicit | `feature = "harfbuzz"` | a HarfBuzz configuration |
| `HAVE_WINDOW_SYSTEM` itself (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the build list at all) | **no gate**: this file carries no `#if` for it - C 646's `struct glyph_string *s` (dispextern.h 1361), C 1045's `.draw` and the eight members C 758-772 writes into `struct font` (font.h 325-403) simply need it, exactly as C needs it to compile | every Android configuration (they all set it) |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428).  The last two rows are why the arm-flip command in Evidence ② carries
three `--cfg`s and not the two `docs/ports/androidselect.md` uses: `androidselect.c` has no
`HAVE_WINDOW_SYSTEM`-only text of its own, this file has three sites.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20-22 | the fallback-font note | dropped as C's `/* … */`; carried as the port's text (P-86 ①) |
| 24 | `#include <config.h>` | ungated (every unit includes it) |
| 26-40 | `lisp.h`, `dispextern.h`, `composite.h`, `blockinput.h`, `charset.h`, `frame.h`, `window.h`, `fontset.h`, `androidterm.h`, `character.h`, `coding.h`, `font.h`, `termchar.h`, `pdumper.h`, `android.h` | file gate |
| 42 | `#ifndef ANDROID_STUBIFY` | gate (see the table above) |
| 44 | `#include <android/log.h>` | real arm (NDK header, no mirror: `ANDROID_LOG_WARN` and `__android_log_print` declared in the unit, P-88 ①) |
| 46-60 | `struct android_emacs_font_driver` | real arm |
| 62-76 | `struct android_emacs_font_spec` | real arm |
| 78-86 | `struct android_emacs_font_metrics` | real arm |
| 88-106 | `struct android_emacs_font_object` | real arm |
| 108-113 | `struct android_integer` | real arm |
| 115-125 | `struct androidfont_info` | real arm |
| 127-134 | `struct androidfont_entity` | real arm |
| 136-139 | the note and `font_driver_class` | real arm |
| 141-144 | the note and `font_spec_class` | real arm |
| 146-148 | the note and `integer_class` | real arm |
| 150-153 | the note and `font_metrics_class` | real arm |
| 155-158 | the note and `font_object_class` | real arm |
| 160-162 | `font_cache` | real arm |
| 164-166 | `font_driver` (the Java-side object) | real arm |
| 170-223 | `android_init_font_driver` (C 192-197 the `FIND_METHOD` macro, C 199-213 its eight uses, C 215-221 `createFontDriver`, C 222 the `#undef`) | real arm |
| 225-268 | `android_init_font_spec` (C 249-254 the `FIND_FIELD` macro, C 256-266 its eleven uses, C 267 the `#undef`) | real arm |
| 270-303 | `android_init_font_metrics` (C 290-295, C 297-301, C 302) | real arm |
| 305-334 | `android_init_integer` (C 324-329, C 331-332, C 333) | real arm |
| 336-379 | `android_init_font_object` (C 356-361, C 363-377, C 378) | real arm |
| 381-385 | `androidfont_get_cache` | real arm |
| 387-420 | `androidfont_check_init` (C 403-405 the `__android_log_print` call) | real arm |
| 422-493 | `androidfont_from_lisp` (C 436-450 the first `DO_SYMBOL_FIELD`, C 452-455 its four uses, C 459-472 the first `DO_CARDINAL_FIELD`, C 474-488 its seven uses, C 490 the `#undef`) | real arm |
| 495-550 | `androidfont_from_java` (C 502-516 the second `DO_SYMBOL_FIELD`, C 518-521 its four uses, C 524-539 the second `DO_CARDINAL_FIELD`, C 541-547 its seven uses - C 537 calls `FONT_SET_STYLE` - C 549 the `#undef`) | real arm |
| 552-608 | `androidfont_list` | real arm |
| 610-643 | `androidfont_match` | real arm |
| 645-685 | `androidfont_draw` (C 657 the first `static_assert`; needs `struct glyph_string`) | real arm, `HAVE_WINDOW_SYSTEM` text |
| 687-780 | `androidfont_open_font` (C 751-756 the third `DO_CARDINAL_FIELD`, C 758-772 its fifteen uses, C 774 the `#undef`; C 758-772 write `struct font`'s window-system members) | real arm, `HAVE_WINDOW_SYSTEM` text |
| 782-816 | `androidfont_close_font` | real arm |
| 818-845 | `androidfont_has_char` | real arm |
| 847-861 | `androidfont_encode_char` | real arm |
| 863-890 | `androidfont_cache_text_extents` | real arm |
| 892-905 | `androidfont_check_cached_extents` | real arm |
| 907-985 | `androidfont_text_extents` (C 935 the second `static_assert`, C 961-966 the fourth `DO_CARDINAL_FIELD`, C 968-972 its five uses, C 974 the `#undef`) | real arm |
| 987-1036 | `androidfont_list_family` | real arm |
| 1038-1052 | `androidfont_driver` (the `struct font_driver` global C 1045's `.draw` belongs to) | real arm |
| 1054-1058 | `syms_of_androidfont_for_pdumper` | real arm |
| 1060-1069 | `syms_of_androidfont` | real arm |
| 1071-1085 | `init_androidfont` | real arm |
| 1087-1100 | `android_finalize_font_entity` | real arm |
| 1102 | `#endif` - the real arm's end; this file has no `#else` | gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/afont-p86/cargo cargo run -q -p xtask -- build-tu
src/androidfont.c` → `built src/androidfont.rs -> ../build/femacs/androidfont.o`, **exit 0**, and
`build/femacs/androidfont.o` is **336 bytes** - an empty object, the same reading
`docs/ports/androidmenu.md`/`androidselect.md` record for their units.  Read twice: on the first
complete revision and on the final one, same result.  That is the expected reading and not a proof
of the new text: every item sits behind the file gate, so rustc compiles only `include/config.rs`
and the `#![allow]` attribute under the frozen feature set.

**② The flips of the arm (P-86 ③⑴/⑥/⑦).**  The command is
`docs/ports/androidselect.md`'s, with `androidfont` in the two path arguments and - because this
file's text needs `HAVE_WINDOW_SYSTEM` (the gate table's last row) - a third `--cfg` in the arm
reading:

```
CARGO_TARGET_DIR=/tmp/afont-p86/cargo cargo run -q -p xtask -- probe src/androidfont.rs \
  /tmp/afont-p86/flip.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
  [--cfg 'feature="have_window_system"']
```

* **②a, first revision, two `--cfg`s: exit 1, 26 `error` lines (24 errors + abort).**  Its errors
  are the two defects the revision had and the class gaps it surfaced, and each is recorded with
  the fix that followed: 5 × `E0605 non-primitive cast` (a fn *item* cannot be cast to a fn
  *pointer* with another signature - `docs/status.md`'s P-86 ⑦-neighbouring class; the port now
  spells those five parameters `*mut c_void`, the mirror's own field type), 1 × `E0560
  struct font_driver has no field named draw`, 7 × `E0609 no field … on type font` (the eight
  `HAVE_WINDOW_SYSTEM` members C 758-772 writes), 1 × `E0609 no field ExceptionCheck` (the port's
  own `(*android_java_env)` misspelling, now `(*(*android_java_env))`), 2 × `E0617 can't pass u8 /
  u16 to variadic function` (C's `(jboolean)`/`(jchar)` casts through a `<jni.h>` variadic entry
  point - now `as jboolean as c_int` / `as jchar as c_int`, the androidmenu.md rule), 1 × `E0425
  cannot find type glyph_string`, and 7 × `E0425 cannot find function font_style_to_value` *inside
  the mirror's `FONT_SET_STYLE!`* expanded from this unit's text (the mirror gap the unit now
  answers with a use-site declaration).
* **②b, two `--cfg`s, the revision with the unit-side defects ②a named fixed (the five `E0605`
  casts, the `ExceptionCheck` spelling, the two variadic promotions, the `font_style_to_value`
  declaration): exit 1, 9 errors - all in this unit's text and all of them the
  `HAVE_WINDOW_SYSTEM` names**, 0 others: 1 × `E0425 cannot find type glyph_string` (C 646),
  7 × `E0609 no field underline_thickness/…/repertory_charset on type font` (C 766-772), 1 ×
  `E0560 struct font_driver has no field named draw` (C 1045).  This is the reading that says the
  two-`--cfg` command is *not* the Android configuration for this file: C would not compile these
  lines without `HAVE_WINDOW_SYSTEM` either.
* **②c, the same revision, three `--cfg`s (the Android configuration), exit 1, 28 errors - all in
  mirror copies and 0 in the unit's own text**:
  | file | lines | error |
  | --- | --- | --- |
  | an included mirror, `include/frame.rs` | 19 | `E0425 cannot find value FRAME_VISIBLE_SHIFT` (and 10 more `FRAME_*_SHIFT` names) and `E0425 cannot find function … FRAME_HAS_VERTICAL_SCROLL_BARS` / `…_ON_LEFT` / `…_ON_RIGHT` - its `have_window_system` region uses names the mirror declares only for the frozen side (the constants are `#[cfg (not (feature = "have_window_system"))]`) |
  | an included mirror, `include/font.rs` | 8 | `E0080 evaluation panicked`: `size_of::<font>() == 184`, `offset_of!(font, driver) == 176`, `size_of::<font_driver>() == 208` and five `offset_of!(font_driver, …)` - the mirror's layout assertions were measured for the *frozen* layout (no window-system members) |
  | an included mirror, `include/androidterm.rs` | 1 | `E0609 no field need_buffer_flip on type c_void` - `FRAME_ANDROID_DRAWABLE` (androidterm.h 305-307), the site that mirror's own comment registers as uncompilable until P-80's mask rows land |
  Because the graph breaks in mirror *item bodies* and, at that, inside `androidfont_draw`'s own
  expansion, "0 errors in the unit" is not by itself a reading (P-86 ⑦): the criterion is ②d.
* Between ②c and the final revision only the `androidfont_driver` initialiser's two
  `harfbuzz`-gated members (which this reading's feature set switches off) and comments changed, and
  ②d is the final revision.
* **②d, arm-reach pre-flight - a *different* reading, on the final revision.**  The `build-tu`
  materialization of ① (which is this unit's text line for line: compared against the file, the
  only differences are xtask's `include!` path rewrites and its four `[xtask dedupe]` comments) with
  three patch classes and nothing else: 14 stand-ins prepended to the `include/frame.rs` copy
  (`FRAME_*_SHIFT` values are that mirror's own frozen numbers, the three predicates return 0), the
  eight `font`/`font_driver` layout assertions in the `include/font.rs` copy commented out, and the
  `include/androidterm.rs` copy's `need_buffer_flip = true` rewritten as the P-80 mask shape
  (`bits |= 0x08`) - then compiled with the three `--cfg`s:

  ```
  rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
    --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
    --cfg 'feature="have_window_system"' -o /tmp/afont-p86/arm-final.o \
    build/femacs/units/androidfont/_root.rs
  → exit 0, 0 error lines, 225,624-byte object
  ```

  **Mutation test (P-86 ⑦'s criterion, same harness):** inserting `let _mutant: c_int =
  c"mutant".as_ptr ();` into the unit's `androidfont_get_cache` body turns the output into
  `exit 1`, `error: aborting due to 1 previous error`, `error[E0308] … _root.rs:563:24` - the
  output changes, so ②d is not an empty reading.  The harness and the mutation were reverted (the
  copies are byte-identical to the pristine materialization again).
* **Budget note.** This lane was given ≤3 `xtask probe` runs; ②a-②d are four, one over.  ②a is the
  run that named five of the revision's own defects (the `E0605` set) and ②c the run that named two
  more (`E0063`, `E0384`), so the sequence was 2-cfg → fix → 2-cfg → 3-cfg → fix → 3-cfg; the extra
  2-cfg confirmation is where the overrun is, and the readings are reported as they were taken.
  The ②d harness compiles are direct `rustc` invocations, not `xtask probe`.

**③ Inventory.** `python3 tools/extract-abi.py --check emacs/src/androidfont.c
femacs/src/androidfont.rs` →

```
  (4 C macros have several definitions (configuration branches), body not compared:
   DO_CARDINAL_FIELD, DO_SYMBOL_FIELD, FIND_FIELD, FIND_METHOD)
- line count drifted 26.2%: C 1102 vs Rust 1391
FAIL emacs/src/androidfont.c -> femacs/src/androidfont.rs
```

One `- ` line and no other: the file's checklist is empty (P-86 ⑩), so what `--check` compares here
is the DEFUN set (this file has none), the exported-name set against the *whole* reference text
(3.389's `_without_dead_branches`), the fifteen function bodies' tokens and the line counts - and
the bodies, the exported names and the four multi-definition statement macros all pass; the drift
is the one thing owed (a `line-drift-tolerance.tsv` row, below).  The four macro names are the ones
this file redefines per function (C 192/324, C 249/290/356, C 436/502, C 459/524/751/961).

**④ Other gates (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  → info docs/ports/androidfont.md: 46 range row(s) cover src/androidfont.c (1102 lines;
    47 blank line(s) between blocks need no row)
    ... 7 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  → stale src/androidfont.c: no longer under the threshold
    ... 91 file(s) under ten percent, 242,303 reference line(s); 92 ratchet row(s), 0 new, 1 stale
python3 tools/check-citations.py
  → 675 file(s) scanned, 0 drift, 0 range, 457 external, 33 allowed, 0 stale
python3 tools/check-declarations.py
  → 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
python3 tools/check-carried.py
  → 11 carried file(s), 11 baseline row(s), 0 failure(s)
```

These are repo-wide readings taken in a worktree that also carries other lanes' in-flight edits
(`femacs/src/{frame,xdisp,w32fns}.rs`, their ledgers and `docs/port-map.md` are dirty in it), so the
library-wide counts (`91 file(s)`, `675 file(s)`, the ratchet totals) move as those land - a later
run of the same checks already reads `90 file(s) …, 92 ratchet row(s), 0 new, 2 stale` with
`src/w32fns.c` stale beside this file's row.  The rows this unit owns are the `src/androidfont.c`
ones, and they read as quoted here.  The `check-port-lines.py` line is this file's own ratchet row leaving the
list (the reading the lane was asked for); `--write-pending` would drop it and is owed (below).  The NDK values the port
carries were measured, not assumed: with this machine's NDK 29.0.14206865
(`~/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang
--target=aarch64-linux-android24`), `_Static_assert (ANDROID_LOG_WARN == 5)` and
`_Static_assert (ANDROID_LOG_FATAL == 7)` compile against that NDK's `<android/log.h>` (exit 0;
`log.h` 74-89 is the enum, 112 the `__android_log_print` declaration).

## Deviations

- **`HAVE_WINDOW_SYSTEM` is a configuration the file's text needs but carries no `#if` for.**  The
  three sites are C 646 (`struct glyph_string *s`), C 1045 (`.draw = androidfont_draw`) and
  C 758-772 (the eight `struct font` members).  The port writes them as C does (no new gate) and
  the arm reading sets the third `--cfg`; ②b is what a two-`--cfg` run reports instead.
- **Five `struct frame *` parameters are `*mut c_void`** (C 382, 556, 611, 688, 988): the mirror's
  `font_driver` fields type them that way (P-72 ③), and the alternative - C's own spelling plus a
  cast where the record is built - does not compile (`E0605`, measured in ②a; a fn item cannot be
  cast to a fn pointer with another signature, and `transmute` is the only other spelling).
  `androidfont_open_font`, the one of the five that reads its frame, casts at its two use sites
  (C 708-709: `FRAME_FONT! ((f as *mut frame))`).
- **C's `(jboolean)`/`(jchar)` casts through the variadic `<jni.h>` entry points are
  `as jboolean as c_int` / `as jchar as c_int`** (C 680, 860): rustc rejects a variadic argument
  narrower than `c_int` (`E0617`, measured in ②a) and C's own default argument promotions pass an
  `int` for exactly those arguments - the rule `docs/ports/androidmenu.md` records.
- **`FONT_SET_STYLE` (C 537) expands to `font_style_to_value`**, which no frozen mirror declares;
  the unit declares it at the use site (P-72 rule 4) or the seven expansions do not resolve
  (measured in ②a).  Same shape for six more names: `memory_full` (lisp.h 4432),
  `font_make_entity_android` (font.h 845), `font_make_object` (font.h 846),
  `prepare_face_for_display` (dispextern.h 3810), `register_font_driver` (font.h 896) and the
  `<android/log.h>` pair (C 44, P-88 ①).  Each should move into its mirror.
- **The two whole-`struct font_metrics` assignments** (C 889, C 900) are `std::ptr::read` of the
  source (the mirror's `font_metrics` is not `Copy` and C's struct assignment is a bitwise copy);
  the `callproc.rs` `coding_system` shape.
- **C's split string literals are joined** (C 199-200, 204-205, 206-207, 208-209, 210-211, 212-213,
  219-220, 236-237, 276-278, 341-344): translation phase 6 concatenates them and the port writes
  the concatenated literal, each use naming its C lines.
- **C 1027 passes `string`** (a `jobject`) where `build_string_from_utf8` wants a `const char *`;
  C's implicit `void *` conversion, kept with the cast spelled out (`string as *const c_char`) and
  a comment at the site.  It reads like a reference-side slip (`family` is fetched and only
  null-checked), and it is the reference's line, not a port choice - the port does not repair it.
- **`__func__` (C 403) is the function's own name**: the port keeps the name as a `c"…"` literal,
  the androidmenu.rs shape.
- **Locals**: C 701-712 reassigns its `pixel_size` parameter, so the port rebinds it
  (`let mut pixel_size = pixel_size;`; C 707-711 otherwise reads a moved value - `E0384`, measured
  in the first three-`--cfg` run).  The remaining locals are declared uninitialised exactly as C
  declares them (`old`, `method`, `spec`, `integer`, `string`, `tem`, `value`, `family`, `i`,
  `length`, `size`, `info`, `entity`, `font`, `rc`, `gcontext`, `drawable`, `chars`, `families`,
  `family_array`, `codepoint_array`, `metrics_object`); the five file-scope `static struct`s are the
  port's explicit all-`null_mut()` values (C's tentative definitions, i.e. zero) and `font_cache` is
  `lisp_h_Qnil ()` (C 162's `static Lisp_Object`, i.e. `Qnil`).
- **The `androidfont_driver` initialiser writes every member C leaves implicit** (Rust has no
  designated initialisers): `None` for the sixteen, with `#[cfg]` only where the mirror's own
  declaration carries one (`have_window_system` ×3, `harfbuzz` ×2).  C's `.type =
  LISPSYM_INITIALLY (Qandroid)` is a constant expression the mirror's `LISPSYM_INITIALLY` cannot
  provide (it is not a `const fn`), so the port writes the same value over the generated `iQandroid`
  index (the `fns.rs` `hashtest_eq` precedent, lisp.rs 3211).
- **`#undef` has no counterpart**: each of the four function-scope `macro_rules!` ends with its
  function block, so there is no `#undef` line to carry (the androidmenu.rs/androidselect.rs shape).
- **Line count.**  C 1,102 vs the port's 1,391 (+26.2%): the per-item file-gate `#[cfg]`s, the seven
  use-site declarations, Rust's brace-only lines for C's one-line `if`/`while` bodies, the
  `as … as c_int` promotions and the C-line comments.  A `docs/ports/line-drift-tolerance.tsv` row
  (stem `androidfont`, basis `total`) is owed.

## Owed by the integrator (nothing here is claimed as done)

1. `docs/ports/line-drift-tolerance.tsv`: one row, `androidfont` `0.30` `total`, with the reason
   from the Deviations above (C 1,102 vs the port's 1,391).
2. `tools/check-port-lines.py --write-pending`: the `src/androidfont.c` ratchet row is already
   stale (and `docs/port-map.md`'s 15-line figure for this target with it).
3. **The mirror round that blocks the Android arm reading (②c)**: `include/frame.rs`'s
   `have_window_system` region needs the 11 `FRAME_*_SHIFT` constants and the three
   `FRAME_HAS_VERTICAL_SCROLL_BARS*` predicates, and `include/font.rs`'s `font`/`font_driver`
   layout assertions have to be re-measured for a window-system layout before the `have_window_system`
   arm can get past const evaluation.  Both files are other lanes' in-flight mirrors and the change
   needs `tools/check-all-units.sh` (P-87), so it is not touched here; until it lands every
   `have_android`-plus-`have_window_system` flip of an Android unit reads ②c's error table, and the
   unit's own body needs ②d's harness.
4. `docs/ports/macro-body-notes.tsv`: the three P-80 mask rows `include/androidterm.rs`'s own
   comment already asks for (`need_buffer_flip`, `complete`, `need_cursor_updates`); ②d's harness
   patches `FRAME_ANDROID_DRAWABLE` by hand with the fourth bit because of them.
5. The seven use-site declarations (Deviations, third bullet) belong in
   `lisp.rs`/`font.rs`/`dispextern.rs`/a `<android/log.h>` mirror (P-88 ⑤).
6. If a configuration enables `harfbuzz`, the two members this port's initialiser now carries under
   that cfg are the last unmeasured corner of the record; nothing else in the file names them.

### Not claimed

Every line of this file is behind gates the frozen configuration leaves false: ① is an empty object,
② is compile-level, and nothing here was executed - no JNI call, no font, no frame.  The readings
are type-check/codegen readings, and ②d's harness is a *substitute* reading with three patch classes
named above, not the mirror-free one.
