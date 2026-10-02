# emacs/src/tparam.h (49 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_TPARAM_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 3
- [x] extern char PC  (line 40)
- [x] extern char BC  (line 41)
- [x] extern char UP  (line 42)

## Decisions

- **tparam.h is complete** (55 lines: the guard, the termcap interface, `tparam`, three
  globals and the live `#ifdef TERMINFO` pair).
- **`TERMINFO` is defined in the frozen configuration** (docs/baseline/config/darwin-arm64.macros.tsv
  1812), so `tigetflag`/`tigetstr` are live and mirrored, not dead text.
- **`PC`, `BC` and `UP` live in `pub mod tparam_externs`**: tparam.c *defines* `BC` and `UP`
  (tparam.h only declares them), so the unit's `#[unsafe(no_mangle)]` definitions shadow the
  declarations instead of colliding (P-21/P-39).
- **The `<stdlib.h>` and `<attribute.h>` includes have no counterpart here** - the unit
  includes the library mirrors it needs, and the port's attributes are items (PORTING.md
  0.1-9).
