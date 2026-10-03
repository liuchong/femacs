# emacs/lib/c-strncasecmp.c (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] c_strncasecmp  (line 27)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The same shape as femacs/lib/c-strcasecmp.rs, and blind for the same reason: C 42's
  `UCHAR_MAX <= INT_MAX` is live and its `else` arm is carried so the checker sees `_GL_CMP`.
- `c_tolower` and the `_C_CTYPE_UPPER` switch→match mapping are the c-ctype.rs mirror's (its
  ledger rows live in docs/ports/macro-body-notes.tsv).
- C 40's `--n == 0` is the port's `n = n.wrapping_sub (1); n == 0` (C's unsigned wrap,
  PORTING.md 2.6); the `p1++`/`p2++` of the `for` head move to the loop's tail (2.2).
- Line-count drift (C 51 vs the port's 61) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
