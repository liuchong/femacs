# emacs/lib/strftime.h (104 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): the header declares no macro, no type and no global, only
  `nstrftime`/`c_nstrftime` over `timezone_t`/`struct tm` (both from `time.in.rs`, which
  this mirror includes).  The header's long `%N`/conversion comment is the doc block of
  `nstrftime` in the mirror.
- `nstrftime`/`c_nstrftime` are defined by the separate unit `lib/nstrftime.c` (which
  `#include`s strftime.c with `my_strftime` renamed to `nstrftime`); `lib/strftime.c` itself
  defines `strftime`/`wcsftime` through the same `my_strftime` macro.
