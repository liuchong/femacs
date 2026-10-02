# emacs/lib/strnlen.c (34 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] strnlen  (line 26)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **A unit, not a wrapper**: strnlen.c has no `_GL_*_INLINE` macro; it is gnulib's own
  `strnlen`, which the frozen configuration's system <string.h> already provides (the file
  has no `#if`, so the port keeps C's text as it stands - PORTING.md 0.1-1).
- **C's `<string.h>` include has no counterpart**: the body deliberately calls nothing from
  it (that is why C's comment warns against `memchr`).
- **The `for (…; …; i++) continue;` shape becomes a `while`**, which is the notes row's
  subject; the increment stays in the loop body, as C's for-increment does.
