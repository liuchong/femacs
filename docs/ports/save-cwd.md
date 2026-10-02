# emacs/lib/save-cwd.c (90 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] save_cwd  (line 58)
- [x] restore_cwd  (line 73)
- [x] free_cwd  (line 87)

## Macros defined — 1
- [x] fchdir  (line 54, body eb883130e83d)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Emacs's trimmed variant: no getcwd fallback, no `#if`, so the whole file is live except
  C 46-48's `#if !defined HAVE_FCHDIR && !defined fchdir / # define fchdir(fd) (-1)` (dead:
  HAVE_FCHDIR is 1), kept as a cfg'd macro_rules (P-51).
- `open (".", O_SEARCH | O_CLOEXEC)` needs two fcntl-mirror constants this round added
  (`O_SEARCH` = 0x40100000 and `O_CLOEXEC` = 0x1000000, both measured); `fchdir`/`close` come
  from the unistd mirror, and `struct saved_cwd` from the new save-cwd mirror.
- No call-relationship differences (D7).
