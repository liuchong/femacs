# emacs/nt/inc/arpa/inet.h (2 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/arpa/inet.h, empty for this target: the reference is one comment line and
declares nothing, so there is no name to carry and the mirror is the note alone (the same
shape `src/include/getpagesize.rs` uses for a header whose live inventory is zero).

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead, and `src/process.c` is one of the units whose
`#include <arpa/inet.h>` resolves here.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/arpa/inet.h
nt/inc/arpa/include/inet.rs` -> `OK`.
