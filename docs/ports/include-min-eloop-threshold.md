# emacs/lib/min-eloop-threshold.h (42 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] MIN_ELOOP_THRESHOLD  (line 40, body af3e133428b9)
- [x] _MIN_ELOOP_THRESHOLD_H  (line 20, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/min-eloop-threshold.h` @ `fd7afced403`; port `femacs/lib/include/min-eloop-threshold.rs`.  Every checklist item is
ticked and `extract-abi.py --check` reports `OK` for the pair.  The frozen target's arm is
the one implemented; PORTING.md P-24 does not compare a macro that has several definitions,
so the non-frozen arm's spellings are recorded here instead of being carried as text.

- The header is one constant; `SYMLOOP_MAX` is named only in its comment.
