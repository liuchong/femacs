# emacs/lib/assert.in.h (27 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **There is nothing to carry, and that is measured.**  Every line of the template is preprocessor
  text: an unguarded `#include_next <assert.h>` (C 25), a `#pragma GCC system_header` behind
  `#if __GNUC__ >= 3` (C 20-22), `@PRAGMA_COLUMNS@` (C 23) and two comments.  `extract-abi.py`
  reports 0 subrs, 0 functions, 0 macros, 0 types and 0 file-scope objects; the tool's own reading is
  what makes "nothing" a decision rather than an omission (the `arg-nonnull.rs` and `c++defs.rs`
  shape).
- **The one condition cannot change the answer.**  `tools/check-cond-eval.py lib/assert.in.h`
  reports `1 conditions, 0 unknown, 1 disagreements`: clang sees `__GNUC__` undefined in this
  configuration while the evaluator's macro table does not.  Either arm carries the same single
  pragma, and a pragma is not a declaration, so there is no P-51 arm to name and no value to
  measure - the disagreement is recorded here instead of being papered over.
- **No include guard, by the template's own instruction.**  C 17-18 says the include must not be
  guarded because `<assert.h>` defines `assert` on every inclusion; the port therefore has no guard
  name to carry either (contrast `root-uid.rs`, whose `ROOT_UID_H_` is a guard name).
- **Where C's `assert` is needed, the unit mirrors it.**  `lib-src/ebrowse.rs` and
  `include/android-asset.rs` each carry their own `macro_rules! assert` with the frozen `NDEBUG`
  body; this template hands them nothing, and the two spellings are the same construct because both
  mirror the same C macro from `<assert.h>`.
- **No unit includes this mirror.**  In the reference the template is a gnulib snippet the build
  machinery turns into `lib/assert.h`; the frozen build has no such generated header
  (`build/emacs-ref/lib/assert.h` does not exist) and no source file names it, so the mirror stands
  alone - recorded rather than assumed.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/assert.in.h femacs/lib/include/assert.in.rs`
  -> `OK emacs/lib/assert.in.h -> femacs/lib/include/assert.in.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py lib/assert.in.h`
  -> `1 conditions, 0 unknown to the evaluator, 1 disagreements`.
- `python3 tools/extract-abi.py emacs/lib/assert.in.h` -> this ledger: 0 items in all five sections.
- Consumer evidence (a unit that mirrors C's `assert` macro itself):
  `build-tu lib-src/ebrowse.c` -> see the status entry's reading.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 0 failures; `check-globals` 283 units / 0; `check-verbatim` 6 trees
  / 4607 files / 0; `check-target-names` 0 findings.
