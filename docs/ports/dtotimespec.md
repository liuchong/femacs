# emacs/lib/dtotimespec.c (53 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] dtotimespec  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- C's two `TYPE_MINIMUM (time_t)`/`TYPE_MAXIMUM (time_t)` uses come from `<intprops.h>`, which
  has no mirror (P-26); the port writes them as unit-local macros over the type's own limits
  (the nanosleep.rs/mktime.rs precedent), which for this target's `time_t = i64` is exact.
- C's implicit conversions are written out: `time_t s = sec;` truncates toward zero, so the
  port says `sec as time_t`; `ns += ns < frac;` promotes `ns` to `double`, so the port says
  `((ns as c_double) < frac) as c_long`; `TIMESPEC_HZ` is an `int` used against `long`
  values, so each use is `as c_long`; and `make_timespec`'s `ns` parameter is `c_long`.
- `dtotimespec` is declared in the timespec.h mirror's `pub mod timespec_decls` + glob
  import, so this unit's definition shadows it (PORTING.md 0.1-10).  Non-`static` definition →
  `#[unsafe(no_mangle)]` (P-52); the reference object confirms `T _dtotimespec`.
- Line-count drift (C 53 vs the port's 65) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
