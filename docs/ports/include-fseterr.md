# emacs/lib/fseterr.h (55 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [ ] _FSETERR_H  (line 18, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `HAVE___FSETERR` is undefined (macros.tsv 1696), so the live arm is C 36-45's
  `extern void fseterr (FILE *fp);`; C 29-35's musl arm (the `stdio_ext.h` rename and its
  inner `__HAIKU__` declaration) is the one unchecked box above — it needs a header this
  target has not got, and no ported unit can reach it.
- The declaration is in a `pub mod` + glob import so lib/fseterr.c's definition shadows it.
