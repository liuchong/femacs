# emacs/src/termopts.h (29 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_TERMOPTS_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 2
- [x] extern bool interrupt_input  (line 24)
- [x] extern bool interrupts_deferred  (line 27)

## Decisions

- **The whole header is a guard, two comments and two declarations.**  C 20-21 is the guard,
  C 23 and C 26 are C's own comments, and C 24/C 27 are the two `extern bool` objects.  There is
  no other `#if` (`tools/check-cond-eval.py src/termopts.h` -> `1 conditions, 0 unknown,
  0 disagreements [VERIFIED]`), so the mirror carries both declarations as live text.
- **The two declarations sit in a module the mirror glob-imports (`termopts_objs`), not at the
  crate root.**  The storage is the unit's: `src/keyboard.c` 348/351 define both objects, and
  `src/keyboard.rs` 314/316 define them at *its* crate root.  A crate-root declaration of a
  name the including unit also defines at the crate root is `E0428` (measured on this tree), so the
  mirror uses the `atimer_decls` / `globals.rs`-`subrs` shape: the unit's definition shadows the
  glob import, and a unit without a definition resolves the name through it.  This is the
  "which side is the definition" rule the lane brief states; here the definition is the *unit's*, so
  the module is the mirror's.
- **`bool` is C's `<stdbool.h>` bool** (one byte, PORTING.md 1.1); neither object is a `DEFVAR_*`
  holder, so `tools/check-globals.py` has nothing to say about them.
- **No unit `include!`s the mirror yet**, and there is no `src/termopts.c` at all — the header's
  objects are keyboard.c's, and C's readers reach them through this header (`src/sysdep.c`
  1296/1542, `src/process.c` 6041, `src/xdisp.c`, …).  Two of the port's units declare the names at
  their own crate roots today: `src/sysdep.rs` 404 (inside an `unsafe extern "C"` block) and
  966, and `src/process.rs` 6317-6318.  They are not in this mirror's include graph, so
  nothing here shadows them; when those units adopt the mirror their copies must retire first
  (the 3.143/3.153 use-site class), because a crate-root copy plus the glob import is fine but a
  *second crate-root declaration* is `E0428`.
- **`src/keyboard.rs` 13-16's note that this header "has no mirror" is now stale** — the
  mirror exists and is shaped so that the unit can include it without moving its definitions.  The
  file belongs to another lane, so the sentence is left for its owner to refresh (recorded here so
  the discrepancy is not silent).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/termopts.h src/include/termopts.rs`
  -> `OK emacs/src/termopts.h -> src/include/termopts.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/termopts.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (the guard).
- `python3 tools/extract-abi.py emacs/src/termopts.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 0 types, 2 file-scope objects (C 24, C 27).
- `rg -c "interrupt_input|interrupts_deferred" src/*.rs src/include/*.rs` ->
  `sysdep.rs:11`, `keyboard.rs:17`, `process.rs:3`, `include/termopts.rs:4`, `eval.rs:2`,
  `include/blockinput.rs:3`, `thread.rs:1`, `include/thread.rs:1`.  `rg -n 'include/termopts' femacs`
  -> no unit `include!`s the mirror.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  Its module plus glob import are what let the crate hold `interrupt_input` without a crate-root definition next to `keyboard.rs`'s.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 576 .rs files / 592 map targets / 0 findings (the file count includes mirrors other lanes landed in the same session).
