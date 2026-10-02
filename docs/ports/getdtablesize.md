# emacs/lib/getdtablesize.c (121 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] getdtablesize  (line 107)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0


## Decisions

- The file is on the verified list, so the `_WIN32` arm (C 20-85: the `_setmaxstdio` probe,
  `_setmaxstdio_nothrow`, the `dtablesize` cache and a second `getdtablesize`) and the
  `__CYGWIN__` arm (C 102-106) are dead by measurement; C 88-120 is what the frozen
  configuration compiles, and that is one function plus the two `#ifndef RLIM_SAVED_*`
  fallbacks (this target's <sys/resource.h> defines neither, so C's `RLIM_INFINITY` values are
  live and the port keeps them as constants).
- `RLIM_INFINITY` (9223372036854775807, measured) joins the sys_resource mirror; the
  `RLIM_SAVED_*` values stay in the unit, because they are C's own fallback rather than a system
  constant.  The `__CYGWIN__` `#define rlim_cur rlim_max` is kept as a cfg'd macro (P-51).
- The comparison `lim.rlim_cur <= INT_MAX` is C's `int` converted to `rlim_t`; the port writes
  `INT_MAX as rlim_t` so the two sides are one type.
- No call-relationship differences (D7): one `getrlimit` call and two `return`s, as in C.
