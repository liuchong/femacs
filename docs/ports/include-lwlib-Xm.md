# emacs/lwlib/lwlib-Xm.h (58 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_XM_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern widget_creation_entry xm_creation_table  (line 24)
## Decisions

Mirror of `lwlib/lwlib-Xm.h`, complete for this target: the `xm_creation_table` global, the
eleven prototypes and `lwlib-Xm.h` 22's `#include "lwlib-int.h"` (`include! ("lwlib-int.rs")`).

The frozen configuration builds no window system (`USE_MOTIF`/`USE_X_TOOLKIT` and `HAVE_X11`
are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header
and no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `Boolean` and `XEvent` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. `extern widget_creation_entry xm_creation_table [];` (C 24) is declared `[widget_creation_entry;
   0]`, the incomplete-array shape.
3. `lwlib-Xm.h` 1-17's head note is a description of the back end, not a declaration; the file's
   opening block is not part of the mirror (PORTING.md 0.1-19).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xm.h
lwlib/include/lwlib-Xm.rs` -> `OK` (2 ledger item(s)).
