# emacs/lib/time-internal.h (54 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] local_tz  (line 54, body 7daeff373755)

## Types and their fields — 1
- [x] tm_zone: struct tm_zone* next, char tz_is_set, char abbrs

## File-scope objects — 0

## Decisions

- Header mirror (PORTING.md 0.1): `struct tm_zone` (with `FLEXIBLE_ARRAY_MEMBER` spelled
  `[c_char; 0]`, the lisp.rs shape), the two time_rz entry points and the `local_tz` cookie
  macro.  `timezone_t` and `struct tm` come from the time.h mirror, which C's
  `#include <time.h>` chain supplies, so this mirror includes `time.in.rs`.
- The include graph is de-duplicated by `xtask` (PORTING.md 0.1-4): a unit that includes
  both this mirror and `strftime.rs` (which includes `time.in.rs` as well) gets one copy.
