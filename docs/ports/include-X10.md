# emacs/oldXMenu/X10.h (77 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] VertexCurved  (line 37, body eeed498aee81)
- [x] VertexDontDraw  (line 36, body 5f399b2ba212)
- [x] VertexEndClosed  (line 39, body d73afeb43f0b)
- [x] VertexRelative  (line 35, body f2b1649be8f5)
- [x] VertexStartClosed  (line 38, body 92df71781c5c)
- [x] _X10_H_  (line 24, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `oldXMenu/X10.h`, complete for this target: the five flag-bit constants, the
`Vertex`, `XAssoc` and `XAssocTable` records and the two entry points.  C 40's commented-out
`#define VertexDrawLastPoint` and the note under it are C's own text and are kept as comments.

The frozen configuration builds no window system (`USE_X_TOOLKIT`, `HAVE_X11` and
`HAVE_X_WINDOWS` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing
includes this header and no probe compiles it; the text gate is the whole acceptance
(docs/status.md 3.31/3.102).

### Deviations

1. `Display` and `XID` are X11 names (`<X11/Xlib.h>`) with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. The two anonymous `typedef struct { ... }` records (`Vertex`, `XAssocTable`) are named after
   their typedefs, the `stdio-impl.rs` `FILE = __sFILE` / `struct itimerval` shape (PORTING.md 1.3).
3. `XCreateAssocTable`/`XLookUpAssoc` are declarations only; the definitions are the units
   `XCrAssoc.c`/`XLookAssoc.c`, which the frozen configuration does not build (they are the
   empty shapes of docs/status.md 3.31).

Evidence: `python3 tools/extract-abi.py --check emacs/oldXMenu/X10.h
femacs/oldXMenu/include/X10.rs` -> `OK` (6 ledger item(s)).
