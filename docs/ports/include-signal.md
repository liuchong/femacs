# emacs/nt/inc/signal.h (39 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] SIG2STR_MAX  (line 31, body ce6ec6b75307)
- [x] _NT_INC_SIGNAL_H_  (line 24, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/signal.h, complete for this target: the include guard, `SIG2STR_MAX` (C's
`(5 + 1 + 10 + 1 + 2)` kept as an expression, so every one of its tokens is visible) and the two
prototypes.

**Which arm**: the file has no `#if`/`#else` pair to choose between - only `# ifndef SIG2STR_MAX`
around the constant - and it is not on `docs/baseline/config/cond-eval-verified.txt`, so the
extractor resolves nothing in it and the whole file is live text.

**Includes**: C 37's `#include_next <signal.h>` resolves to MinGW's own header in a MinGW build,
not to the frozen target's, so the port's `<signal.h>` mirror is deliberately not pulled in (the
`nt/inc/sys/include/time.rs` decision for `<sys/time.h>`); the system header supplies everything
else the units need.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead, and `src/comp.c`, `src/filelock.c` and `src/w32term.c`
are among the units whose `#include <signal.h>` resolves here.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/signal.h
nt/inc/include/signal.rs` -> `OK`.
