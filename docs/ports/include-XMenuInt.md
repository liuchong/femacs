# emacs/oldXMenu/XMenuInt.h (90 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] Pixel  (line 65, body 48fd659982bc)
- [x] _FAILURE  (line 57, body 7984b0a0e139)
- [x] _SUCCESS  (line 56, body 356a192b7913)
- [x] _XMenuInternal_h_  (line 36, body da39a3ee5e6b)
- [x] _X_FAILURE  (line 54, body 7984b0a0e139)
- [x] abs  (line 52, body be8b7b337521)
- [x] max  (line 51, body 8db5c1c0a39f)
- [x] min  (line 50, body 0981b6c9dab0)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `oldXMenu/XMenuInt.h`, complete for this target: `min`/`max`/`abs`, the three
`_SUCCESS`/`_FAILURE` codes, the `Pixel` alias, the `_XMEventHandler` variable and the
seventeen prototypes.  C 47-48's `#include "X10.h"` / `"XMenu.h"` are `include! ("X10.rs")` /
`include! ("XMenu.rs")`; C 38's `<config.h>`, C 40's `<attribute.h>`, C 45's `<stdio.h>` and
C 46's `<X11/Xlib.h>` are noted where they stand (this header uses nothing from the first
three, and the port has no Xlib mirror).

The frozen configuration builds no window system (`USE_X_TOOLKIT`, `HAVE_X11` and
`HAVE_X_WINDOWS` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing
includes this header and no probe compiles it; the text gate is the whole acceptance
(docs/status.md 3.31/3.102).

### Deviations

1. `Display`, `XID` and `XEvent` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. C 50-52's three macros are `macro_rules!` (PORTING.md 2.1).  `min`/`max` are C's
   double-evaluating helpers and the port's arms evaluate both arguments too (P-16); `abs`
   keeps the same shape.
3. `#define Pixel unsigned long` (C 65, under `#ifndef Pixel`) is a type spelling, so the port
   writes `pub type Pixel = c_ulong;` and every use site keeps the name; the row is recorded in
   `docs/ports/excluded-macros.txt` (the `MINI_GMP_LIMB_TYPE` / `file_offset` shape).
4. `extern int (*_XMEventHandler)(XEvent*);` (C 62) is a nullable C function pointer in a
   variable, so the port's static is `Option<unsafe extern "C" fn (...)>` (P-27).
5. C 47-48's two includes form a diamond (`XMenuInt.h` includes `X10.h` and `XMenu.h`, and
   `XMenu.h` includes `X10.h` again).  C's include guards collapse it; for Rust the equivalent
   is `xtask`'s include-graph expansion, which de-duplicates by file (PORTING.md 0.1-4), so the
   two `include!` lines are correct as written even though a plain `rustc` of this mirror alone
   reports `E0428` for X10's names.
6. C 1-19's opening declaration block and C 24-33's title comment are the file's own text; the
   opening block is not part of the mirror (PORTING.md 0.1-19).

Evidence: `python3 tools/extract-abi.py --check emacs/oldXMenu/XMenuInt.h
oldXMenu/include/XMenuInt.rs` -> `OK` (8 ledger item(s); `Pixel` is the one recorded
exclusion).
