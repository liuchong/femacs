# emacs/lib/openat-die.c (7 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and the frozen table decides every condition in it; nothing
  is demanded of the port because the reference compiles no definition from it.
- the file's live text is its three includes and nothing else - no function, macro, type or object survives the frozen configuration's preprocessing (the `openat_save_fail`/`openat_restore_fail` definitions this gnulib module is named for are reached through `openat.h`, which this build leaves to the system header).
- Policy: docs/status.md 3.25 (a file the frozen configuration strips to its includes is
  replicated as an empty unit); the tolerance row records the line relation.
