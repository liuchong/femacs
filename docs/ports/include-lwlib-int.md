# emacs/lwlib/lwlib-int.h (75 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_INTERNAL_H  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `lwlib/lwlib-int.h`, complete for this target: the `widget_instance`, `widget_info`,
`widget_creation_function` and `widget_creation_entry` definitions, the `widget_xft_data`
stand-in, the three prototypes and `lwlib-int.h` 24's `#include "lwlib.h"`
(`include! ("lwlib.rs")`).

The frozen configuration builds no window system (`USE_CAIRO`, `HAVE_XFT` and `USE_X_TOOLKIT`
are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header
and no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `Boolean` and `XtPointer` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. `struct widget_xft_data;` (C 26) is an incomplete type C only points at, so the mirror gives
   it the zero-size `pub struct widget_xft_data;` stand-in (P-70).  The one field that names it
   (C 34) sits in the dead `USE_CAIRO`/`HAVE_XFT` arm.
3. C 33-35 (`#if defined USE_CAIRO || defined HAVE_XFT`) is dead in the frozen configuration and
   is carried as `#[cfg (feature = "use_cairo")]` text (P-51), which keeps `xft_data`'s token.
4. `typedef struct _widget_instance { ... } widget_instance;` (and the other three) is mirrored
   as the tag struct plus `pub type NAME = _tag;`, the `stdio-impl.rs` `FILE = __sFILE` shape,
   so the tag each field writes (`*mut _widget_instance`) keeps C's spelling.
5. `widget_creation_function` (C 55) is a nullable C function pointer -> `Option<unsafe extern
   "C" fn (...)>` (P-27).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-int.h
femacs/lwlib/include/lwlib-int.rs` -> `OK` (1 ledger item(s), the include guard).
