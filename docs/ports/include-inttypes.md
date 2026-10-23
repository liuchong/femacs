# emacs/nt/inc/inttypes.h (36 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] _REPL_INTTYPES_H  (line 21, body da39a3ee5e6b)
- [x] strtoimax  (line 32, body 4d93dd47aa52)
- [x] strtoumax  (line 31, body e100796a7819)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/inttypes.h, complete for this target: the include guard and the `strtoumax` /
`strtoimax` renames, which are the only two names the file adds to whatever `<inttypes.h>` the
tool chain already has.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so the
extractor resolves no `#if` in it and both the `__MINGW32__` arm (the `#include_next`) and the
MSVC arm (the renames) are live text; the mirror carries the renames, which is the only part
that declares anything.  C defines each of the two names twice (`_WIN64` and the 32-bit arm), so
the extractor leaves their bodies uncompared and the mirror spells the 32-bit pair.

**Includes**: C 26's `#include "stdint.h"` is the quoted form, so it resolves to this tree's own
`nt/inc/stdint.h`; the mirror pulls in `include/stdint.rs` for the same reason.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/inttypes.h
nt/inc/include/inttypes.rs` -> `OK` (with the expected note that C's two duplicated macros
have their bodies left uncompared).
