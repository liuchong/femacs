# emacs/lib/idx.h (152 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] IDX_MAX  (line 31, body e2d341dab357)
- [x] _IDX_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/idx.h` @ `fd7afced403`; port `femacs/lib/include/idx.rs`.  Every checklist item is
ticked and `extract-abi.py --check` reports `OK` for the pair.  The frozen target's arm is
the one implemented; PORTING.md P-24 does not compare a macro that has several definitions,
so the non-frozen arm's spellings are recorded here instead of being carried as text.

- `__PTRDIFF_TYPE__`/`__PTRDIFF_MAX__` are the compiler's builtins; the mirror spells their
  frozen-target values once and `idx_t`/`IDX_MAX` in terms of them.
