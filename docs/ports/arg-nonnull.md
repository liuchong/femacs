# emacs/lib/arg-nonnull.h (26 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The header declares **one** macro, `_GL_ARG_NONNULL` (C 20-26), and this mirror carries it as a
  disposition rather than a definition: span-wide attribute spellings have no Rust counterpart,
  so the name owns a row in `docs/ports/excluded-macros.txt`
  (`_GL_ARG_NONNULL  syntax  attribute (lib/arg-nonnull.h); no Rust counterpart`), the same
  disposition the mirrors that copy `_GL_ARG_NONNULL` prototypes already record
  (`include/timespec.rs` 15, `include/getopt-core.rs` 34, `include/getopt.in.rs` 13) and the two
  units that spell it at a definition (`getdelim.rs` 24, `canonicalize-lgpl.rs` 44).
- **The frozen target takes C 22, and both arms are the same disposition.** `tools/check-cond-eval.py
  lib/arg-nonnull.h` reports 2 conditions, 0 unknown, 0 disagreements; C 21's
  `__GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 3) || defined __clang__` is true for the
  target's clang through its `defined __clang__` term, so C 22's
  `__attribute__ ((__nonnull__ params))` is live and C 24's empty arm is dead text (P-08).  Neither
  arm can produce a Rust declaration — one is an attribute, the other is nothing — so the exclusion
  row is the whole port either way, and no `#[cfg (feature = "…")]` carrier is needed (P-51).
- C 20's `#ifndef _GL_ARG_NONNULL` is a whole-file name guard; the port has no counterpart for it
  (PORTING.md 0.1-4: `xtask`'s include-graph dedup does the same job).
- **No translation unit includes this mirror, and that is the reference's own shape.** In the
  reference `arg-nonnull.h` is a gnulib module snippet the build machinery installs
  (`emacs/lib/gnulib.mk.in` 3159-3171); nothing in `src/` or `lib/` spells
  `#include "arg-nonnull.h"`, because the macro reaches its users through `system.h`'s aggregate
  text.  The port's aggregates name what they take from each other directly (PORTING.md 0.1-4/P-61),
  so the mirror stays unincluded and the file is intentionally a single commentary block.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/arg-nonnull.h lib/include/arg-nonnull.rs`
  → `OK emacs/lib/arg-nonnull.h -> lib/include/arg-nonnull.rs` (0 problem lines).
- `python3 tools/check-cond-eval.py lib/arg-nonnull.h`
  → `2 conditions, 0 unknown to the evaluator, 0 disagreements`.
- `python3 tools/extract-abi.py lib/arg-nonnull.h` → this ledger: 0 functions, 0 macros to mirror
  (the header's single macro is excluded, so it is reported by the inventory gate instead), 0
  types, 0 file-scope objects.
- Consumer build (a unit whose own prototype spells the dropped attribute, i.e. the exclusion's
  call-site side): `build-tu lib/getdelim.c` → `built lib/getdelim.rs -> ../build/femacs/getdelim.o`.
- Gates on the same worktree: `check-globals.py` 283 units / 0 missing; `check-declarations.py`
  13 sites (0 forbidden, 13 allowed literals) / 0 failures; `check-mirror-dupes.py` 0 failures;
  `check-verbatim.py` 6 trees, 4607 files, 0 failures.
