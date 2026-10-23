# emacs/lwlib/lwlib.h (129 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_H  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `lwlib/lwlib.h`, complete for this target: the `LWLIB_ID` typedef, every member of
`enum menu_separator`, the `lw_callback` type and the 23 entry points.  `lwlib.h` 43's
`#include "lwlib-widget.h"` is mirrored as `include! ("lwlib-widget.rs")`.

The file is a header, so the "not compiled by the frozen configuration" rule of docs/status.md
3.31 (which `tools/extract-abi.py` applies to `.c` files only) does not shrink its checklist.
The frozen configuration builds no window system at all - `USE_X_TOOLKIT`, `USE_XAW`,
`USE_LUCID`, `USE_MOTIF`, `USE_GTK`, `HAVE_X11` and `HAVE_X_WINDOWS` are all `undef` in
docs/baseline/config/darwin-arm64.macros.tsv - so nothing in the build includes this header,
and no probe compiles it: `xtask layout-check` reaches only `src/include` and `lib/include`
(docs/status.md 3.102/3.103).  The text gate is the whole acceptance.

### Deviations

1. `Widget`, `Boolean`, `XEvent` and `Window` are X11 names (from `<X11/Intrinsic.h>` and
   `<X11/Xlib.h>`); the port has no mirror for either header, so they keep C's spelling and are
   left for the unit that eventually uses this header to supply (PORTING.md P-26/P-72 (3)).
   The nine lwlib mirrors and the four oldXMenu ones all follow this one convention.
2. `lw_callback` (C 89) is a nullable C function pointer, so the port's type is
   `Option<unsafe extern "C" fn (...)>` (P-27).
3. The 23 prototypes are declarations inside one `unsafe extern "C"` block, and since
   2026-10-05 that block sits in `lwlib_decls` with `pub use lwlib_decls::*;` (PORTING.md
   0.1-10 / P-39): `lwlib/lwlib.c`'s port *defines* all 23, and a definition cannot share a
   scope with the foreign declaration of the same name (`E0428` - measured; `docs/ports/lwlib.md`
   `## Evidence` ④ also records the `pub use` half, which the `lwlib/xlwmenu.c` flip is what
   required).  A header mirror defines no symbol (PORTING.md 0.1-2), so there is nothing for the
   layout probes to link.
4. The ledger's "Macros defined" section prints `tools/cinventory.py`'s `macros` map alone, so
   the thirteen `SEPARATOR_*` members of `enum menu_separator` (C 64-86) are not checklist rows
   even though the `--check` constants rule compares them; all thirteen are mirrored, in C's
   declaration order, as `const SEPARATOR_*: menu_separator` (PORTING.md 1.3).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib.h
lwlib/include/lwlib.rs` -> `OK` (1 ledger item(s), the include guard).
