# emacs/lib/fpending.h (45 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- One declaration, live because `HAVE_DECL___FPENDING` is undefined (macros.tsv 1288); it is
  in a `pub mod` + glob import so lib/fpending.c's definition shadows it (PORTING.md 0.1-10).
- `HAVE_STDIO_EXT_H` is undefined, so C 32-34's `<stdio_ext.h>` include is dead; the
  `_GL_ATTRIBUTE_PURE` attribute on the declaration is recorded in excluded-macros terms
  (attributes have no Rust counterpart, PORTING.md 0.3).
- `- [_]` none: every box above is ticked, since the live arm is all this header has.
