# emacs/lib/lchmod.c (97 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] lchmod  (line 40)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **O_PATH is not defined** by this target's <fcntl.h> (measured), so C 36-68's arm - the
  open/fstat-proc/issymlinkat dance - is dead text kept under `#[cfg (feature = "o_path")]`
  (P-51); its `INT_BUFSIZE_BOUND` token has no mirror and is in
  docs/ports/macro-body-notes.tsv.  The live text is C 70-96, whose trailing-slash check and
  `issymlink` test the port spells with the `strlen`/`lstat`/`S_ISDIR`/`issymlink` entries of
  the mirrors (the two `issymlink` headers are new mirrors this round).
- `EOPNOTSUPP` (102) was measured and added to the errno mirror; note it is *not* the same
  code as `ENOTSUP` (45) on this target.
- No call-relationship differences (D7).
