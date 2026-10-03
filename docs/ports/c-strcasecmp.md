# emacs/lib/c-strcasecmp.c (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] c_strcasecmp  (line 27)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file (not in cond-eval-verified.txt): both arms of C 42's `if (UCHAR_MAX <= INT_MAX)`
  are carried as text.  The condition is live on this target (both constants are in
  `lib/include/limits.in.rs`), and the dead `else` arm keeps `_GL_CMP` visible to the checker
  (`lib/include/gnulib_config.rs`, P-51) — it compiles too, since `_GL_CMP`'s body is
  well-typed for two `u8` arguments.
- `c_tolower` comes from the c-ctype.h mirror; the mirror gained `c_isupper`/`c_tolower` in
  the same shape as the three predicates already there (C's `switch (c)` over
  `_C_CTYPE_UPPER` becomes the port's `match` with that range pattern; the
  switch/case/default tokens are in docs/ports/macro-body-notes.tsv, the c_isalpha class).
- C 35's `for (;; p1++, p2++)` moves its increment expression to the loop's tail, where C runs
  it (PORTING.md 2.2).  The `(const unsigned char *)` casts are `as *const u8`, and the C
  implicit `unsigned char c1 = c_tolower (…)` is the port's explicit
  `c_tolower (*p1 as c_int) as u8`.
- Line-count drift (C 51 vs the port's 62) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
