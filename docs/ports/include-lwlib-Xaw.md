# emacs/lwlib/lwlib-Xaw.h (29 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_XAW_H  (line 2, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern widget_creation_entry xaw_creation_table  (line 6)
## Decisions

Mirror of `lwlib/lwlib-Xaw.h`, complete for this target: the `xaw_creation_table` global, the
seven prototypes and `lwlib-Xaw.h` 4's `#include "lwlib-int.h"` (mirrored as
`include! ("lwlib-int.rs")`).

The frozen configuration builds no window system (`USE_XAW`/`USE_X_TOOLKIT` and `HAVE_X11` are
`undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header and
no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `Boolean` and `XEvent` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. `extern widget_creation_entry xaw_creation_table [];` (C 6) is an incomplete array; the port
   declares it `[widget_creation_entry; 0]`, the lisp.h `my_edata`/`my_endbss` shape.
3. `widget_creation_entry`/`widget_instance`/`widget_value` come from the `lwlib-int.rs` and
   `lwlib-widget.rs` mirrors the include line pulls in; this mirror names no type of its own.

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xaw.h
lwlib/include/lwlib-Xaw.rs` -> `OK` (2 ledger item(s)).
