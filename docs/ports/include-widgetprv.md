# emacs/src/widgetprv.h (70 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [ ] _EmacsFrameP_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 4
- [ ] <anon@27>: struct frame* frame, char* geometry, Boolean iconic, int minibuffer, Boolean unsplittable, int internal_border_width, int interline, Pixel foreground_pixel, Pixel cursor_color, Boolean bar_cursor, Boolean visual_bell, int bell_volume
- [ ] <anon@61>: int dummy
- [x] _EmacsFrameClassRec: CoreClassPart core_class, EmacsFrameClassPart emacs_frame_class
- [x] _EmacsFrameRec: CorePart core, EmacsFramePart emacs_frame

## File-scope objects — 0

## Decisions

- **The file is X-only, and this target has no X.**  `USE_X_TOOLKIT`, `USE_LUCID` and `HAVE_X11` are
  `undef` in the frozen macro table, so no unit compiles `src/widget.c`/`src/widget.h` and no unit
  `include!`s this mirror (`tools/check-cond-eval.py src/widgetprv.h` -> `1 condition, 0 unknown,
  0 disagreements`, the guard).  The `lwlib/include/xlwmenuP.rs` 1-10 note reads the same widget pair
  from the lwlib side.
- **`EmacsFrameClassPart` (C 61-63) is carried; the other three records are deferred, and that is a
  measurement rather than a preference.**  `EmacsFramePart` (C 27-54) needs Xlib's `Boolean` and
  `Pixel`, `EmacsFrameRec` (C 56-59) embeds Xt's `CorePart` **by value** and `EmacsFrameClassRec`
  (C 65-68) a `CoreClassPart` **by value**; all four names come from `<X11/Intrinsic.h>`,
  `<X11/CoreP.h>` or `<X11/Xlib.h>`, and this machine has no X11 headers to measure them with:
  ```
  $ ls /opt/X11/include/X11/CoreP.h …/MacOSX.sdk/usr/include/X11
  ls: /opt/X11/include/X11/CoreP.h: No such file or directory
  ls: …/MacOSX.sdk/usr/include/X11: No such file or directory
  ```
  Spelling `Boolean`/`Pixel` as `c_int`/`c_ulong` would be an unmeasured claim (P-55 wants C's text
  beside a *measured* substitution); the lwlib mirrors' alternative - keep C's spelling and let the
  including unit supply the names - is not open here, because `xtask layout-check` compiles every
  `src/include` mirror every round and an undeclared `CorePart` would redden that gate for every
  other lane.  So the three records are **recorded, not declared**: their C text is verbatim in the
  mirror, their boxes stay unchecked, and the extractor prints the three `missing type deferred`
  lines instead of failing.  The row that would let them be spelled, and what it would then require,
  is in `## Evidence`.
- **`EmacsFrameClassPart` is the one record C's text makes usable here**: its single member is `int
  dummy`, so the port declares it (the `<anon@61>` container under its typedef name, the lwlib
  reading) and that is what keeps this mirror from being a pure record.
- **`--check` asks for less than fidelity does**: the four records are checked for field *names*
  only, and the two `#include`s (C 24-25) have no port counterpart, so nothing else here is textual
  fidelity work.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/widgetprv.h femacs/src/include/widgetprv.rs`
  -> `OK emacs/src/widgetprv.h -> femacs/src/include/widgetprv.rs` (no `- ` problem lines) with the
  three stderr lines `missing type deferred: src/widgetprv.h <anon@27>/EmacsFramePart`,
  `… EmacsFrameClassRec/_EmacsFrameClassRec` and `… EmacsFrameRec/_EmacsFrameRec` (the ledger has
  unchecked boxes) - which is what the unchecked boxes are for.
- `python3 tools/check-cond-eval.py src/widgetprv.h` -> `1 condition, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/widgetprv.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 4 types (`<anon@27>` = the 12-member `EmacsFramePart`, `<anon@61>` = the
  1-member `EmacsFrameClassPart`, `_EmacsFrameRec` and `_EmacsFrameClassRec`), 0 file-scope objects.
- **The X11 names the three deferred records need**, for whoever writes them: `struct frame` (the
  port's `include/frame.rs`), `Boolean`, `Pixel` (Xlib), `CorePart` (Xt's instance part) and
  `CoreClassPart` (Xt's class part) - the last three have no mirror anywhere in the tree, and the
  lwlib mirrors list the same names as "left for the unit that eventually uses this header"
  (`lwlib/include/xlwmenuP.rs` 12-19).
- **The one row this file would need to carry the three records**: a
  `docs/baseline/layout-excluded-mirrors.txt` line for `widgetprv.rs` (the list's stated purpose is
  exactly "header mirrors the frozen target never compiles", and this one is X-only), after which the
  three records can be written with C's spelling the way the lwlib mirrors are.  That decision is the
  integrator's; this lane did not redden the shared gate to force it.
- **Compile evidence**: `xtask layout-check` compiles this mirror (it is among the mirrors it
  reports) - see the batch report's reading; the mirror's own content is the guard, one record of
  primitives and comments, so the probe finding nothing to check is itself the reading.

## Decisions (stale-box sweep: **not stale**, 2026-10-04)

The 3 box(es) this sweep tentatively ticked are **real pending work, not stale bookkeeping**,
and the tick was reverted.  Measured: while the ledger has unchecked boxes,
`python3 tools/extract-abi.py --check emacs/src/widgetprv.h femacs/src/include/widgetprv.rs` prints `0` `- ` lines **because the tool
defers this file's missing items to stderr** (`missing type deferred: src/widgetprv.h <name> (the ledger has
unchecked boxes)`); with the boxes ticked the deferral lifts and the same command prints

    - missing type EmacsFramePart/ClassRec/Rec

i.e. the type is genuinely absent from femacs/src/include/widgetprv.rs.  So: src/widgetprv.h's three records are the X11 widget interface (`EmacsFramePart`, `EmacsFrameRec`/`_EmacsFrameRec`, `EmacsFrameClassRec`/`_EmacsFrameClassRec`, plus the anonymous `<anon@27>` the tool cannot name); the frozen target has no window system, so they are dead-platform text (P-51/3.9 shape).
