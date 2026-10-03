# emacs/lib/sig2str.c (364 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] str2signum  (line 277, static)
- [x] str2sig  (line 320)
- [x] sig2str  (line 331)

## Macros defined — 4
- [x] ISDIGIT  (line 271, body b8f34e2ae4f1)
- [x] NUMNAME  (line 41, body 49be74a77876)
- [x] SIGRTMAX  (line 38, body 23d5e8a24e49)
- [x] SIGRTMIN  (line 34, body b6589fc6ab0d)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

- The signal table is the 32 entries the frozen target's <signal.h> defines (the `#ifdef SIGxxx`
  guards are decided by the macro table, which carries the signal numbers), followed by C's
  `{ 0, "EXIT" }`; the 29 entries for signals this target has not got are dead by measurement.
- `SIGRTMIN`/`SIGRTMAX` are **not** defined here, so C 33-39's fallbacks are live (0 and -1) and
  both realtime branches are inert exactly as in C.  The two constants stay in the unit, because
  they are C's own fallbacks rather than system values.
- `NUMNAME` (C 41) concatenates an identifier and stringifies its argument, neither of which a
  Rust macro can do: it is in docs/ports/excluded-macros.txt and each entry is written out (the
  `name8` helper turns C's zero-padded `char[8]` into the same eight bytes).
- `streq` comes from the string mirror, which now carries gnulib's <string.h>-template macro
  (there is no separate reference header for it); `strtol`/`strncmp`/`strcpy` were added to the
  stdlib/string mirrors for the same reason, and `sprintf` is declared in-unit (the doprnt.rs
  shape, since that mirror deliberately does not carry it).
- `*signum = str2signum (signame);` and C's two `endp` locals keep C's shape; C's `long int n`
  is `c_long` and the comparisons cast the bound the way C's usual conversions do.
- No call-relationship differences (D7).
