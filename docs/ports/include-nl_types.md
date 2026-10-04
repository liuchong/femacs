# emacs/nt/inc/nl_types.h (25 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _NL_TYPES_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/nl_types.h, complete for this target: the include guard and the one typedef,
`nl_item` as `c_int` (C's `typedef int nl_item;`).

**Which arm**: the file has no `#if`, and it is not on
`docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the
whole file is live text.

**Consumers**: this tree's own `nt/inc/langinfo.h` includes it for `nl_item` (as C's
`#include <nl_types.h>` does), so `include/langinfo.rs` pulls in this mirror.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead, and `src/w32proc.c` and `lib-src/ntlib.c` are among the
units whose `#include <nl_types.h>` resolves here.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/nl_types.h
femacs/nt/inc/include/nl_types.rs` -> `OK`.
