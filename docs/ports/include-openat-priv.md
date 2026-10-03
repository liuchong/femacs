# emacs/lib/openat-priv.h (74 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [ ] EXPECTED_ERRNO  (line 58, body bd274aa5f35c)
- [ ] OPENAT_BUFFER_SIZE  (line 49, body ff3cc59dd0a1)
- [ ] SAFER_ALLOCA  (line 42, body 270b2ec8d62c)
- [ ] SAFER_ALLOCA_MAX  (line 40, body 640ebb1c0282)
- [ ] _GL_HEADER_OPENAT_PRIV  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

WIP mirror of lib/openat-priv.h: the `SAFER_ALLOCA_MAX`/`SAFER_ALLOCA`/`OPENAT_BUFFER_SIZE`
family (the frozen `PATH_MAX` = 1024 arm), `EXPECTED_ERRNO`, and the two prototypes in a
glob-imported module.  lib/intprops.h (this header's own include) still has no mirror - its
`INT_STRLEN_BOUND` lands in the unit that uses it, lib/openat-proc.rs.
