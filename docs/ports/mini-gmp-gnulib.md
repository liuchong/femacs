# emacs/lib/mini-gmp-gnulib.c (52 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and the frozen table decides every condition in it; nothing
  is demanded of the port because the reference compiles no definition from it.
- the file is a wrapper that only carries `_GL_GNUC_PREREQ`-guarded `#pragma` lines around gnulib's copy of mini-gmp and then includes nothing else; the frozen configuration leaves no function, macro, type or object in it, and mini-gmp itself is ported as femacs/lib/mini-gmp.rs.
- Policy: docs/status.md 3.25 (a file the frozen configuration strips to its includes is
  replicated as an empty unit); the tolerance row records the line relation.
