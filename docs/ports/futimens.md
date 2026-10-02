# emacs/lib/futimens.c (37 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] futimens  (line 31)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **A forwarding unit.**  futimens.c is C's `futimens` forwarding to gnulib's `fdutimens
  (fd, NULL, times)`, which is where the platform bugs it mentions are worked around.
  `fdutimens` is declared in this unit's own unsafe extern block with `utimens.h` cited,
  because that header's body is lib/utimens.rs's (the doprnt.rs precedent); `struct timespec`
  comes from lib/include/timespec.rs.
- **The comment block is kept line-for-line** (C's five lines about ENOSYS and TIMESPEC), and
  the file has no local scaffolding, which is what keeps it inside the drift rule.
