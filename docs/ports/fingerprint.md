# emacs/lib/fingerprint.c (66 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and the frozen table decides every condition in it; nothing
  is demanded of the port because the reference compiles no definition from it.
- the file is the loader for Emacs's build fingerprint; the frozen configuration keeps only its two includes, so no function, macro, type or object survives (the symbol itself is emitted by lib-src/make-fingerprint.c, which is a separate unit).
- Policy: docs/status.md 3.25 (a file the frozen configuration strips to its includes is
  replicated as an empty unit); the tolerance row records the line relation.
