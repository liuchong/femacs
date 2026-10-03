# emacs/src/msdos.c (4350 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and every condition it contains is decided by the frozen
  table: `MSDOS` is undefined, so the whole body - the MS-DOS (djgpp) system layer: file names,
  process control, screen, keyboard, subprocesses - is dead text and nothing is demanded of the
  port.
- Policy (docs/status.md 3.25): a file whose *whole* body is inside a disabled `#if` is ported as
  an empty unit with the license header and a note naming the gate, the config line and the
  headers the body would need; the reference text is not carried.
- `emacs/src/msdos.c` is not valid UTF-8 (it has DOS-era bytes); the reference is read with
  `errors="replace"` so nothing is corrupted here.
- The tolerance row records the line relation (4,350 C lines against an empty unit).
