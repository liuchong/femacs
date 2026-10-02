# emacs/lib/minmax.h (65 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [ ] MAX  (line 57, body 54a3705d26d6)
- [ ] MIN  (line 62, body 7c03c63086ce)
- [ ] _MINMAX_H  (line 19, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/minmax.h` @ `fd7afced403`; port `femacs/lib/include/minmax.rs`.  `MAX`/`MIN` are value
macros whose arguments C evaluates twice; the port's `macro_rules!` keep that (the callers pass
plain variables).  The `#ifndef MAX`/`#ifndef MIN` guards are dead on this target and the
`HAVE_MINMAX_IN_LIMITS_H`/`HAVE_MINMAX_IN_SYS_PARAM_H` includes are only needed for building the
system definitions, which the port does not.
