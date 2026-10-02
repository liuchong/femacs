# emacs/lib/getline.c (27 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] getline  (line 24)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **A unit, not a wrapper.**  getline.c has no `#define _GL_*_INLINE`; its whole body is
  `getline` forwarding to the system's `getdelim`.  The frozen configuration has
  HAVE_GETLINE = 1, so gnulib's build would not compile this file at all; the port keeps C's
  text as it stands (PORTING.md 0.1-1).
- **No local type aliases and no blank padding.**  `FILE` comes from
  lib/include/stdio.rs and the two sizes are the port's `usize`/`isize` (C's
  `size_t`/`ssize_t`), which is what keeps this file inside the drift rule - the C file is 27
  lines and its own license is 17 of them, so there is no room for a `// femacs:` header or
  for local scaffolding.  The provenance is this ledger (PORTING.md 10.12's rule).
