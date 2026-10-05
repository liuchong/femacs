# emacs/lwlib/lwlib-Xlw.h (29 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_XLW_H  (line 2, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 2
- [x] extern widget_creation_entry xlw_creation_table  (line 6)
- [x] extern widget_creation_function xlw_create_dialog  (line 7)
## Decisions

Mirror of `lwlib/lwlib-Xlw.h`, complete for this target: the two globals, the six prototypes
and `lwlib-Xlw.h` 4's `#include "lwlib-int.h"` (`include! ("lwlib-int.rs")`).

The frozen configuration builds no window system (`USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11` are
`undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header and
no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `Boolean` and `XEvent` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. `extern widget_creation_entry xlw_creation_table [];` (C 6) is declared `[widget_creation_entry;
   0]`, the incomplete-array shape; `extern widget_creation_function xlw_create_dialog;` (C 7) is
   a static of the `Option<unsafe extern "C" fn ...>` type `lwlib-int.rs` declares (P-27).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xlw.h
femacs/lwlib/include/lwlib-Xlw.rs` -> `OK` (3 ledger item(s)).

### status 3.500 (the mirror change this file took)

`lwlib/lwlib-Xlw.c`'s port *defines* every function this header declares (and
`xlw_creation_table`), and Rust forbids a definition and a foreign declaration of the same name in
one scope (E0428).  The declarations therefore moved into
`pub mod lwlib_Xlw_decls { use super::*; … } use lwlib_Xlw_decls::*;` - the
`include/systhread.rs` shape (PORTING.md 0.1-10 / P-39) - and an explicit item now shadows the glob
import instead of clashing with it.  `check-mirror-dupes` stays at 17 duplicates / 17 ratchet rows /
0 failures, and `layout-check` passed; the unit's own readings are in `docs/status.md` 3.500.
