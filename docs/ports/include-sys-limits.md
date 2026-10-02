# emacs/lib/sys-limits.h (38 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_SYS_LIMITS_H  (line 19, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/sys-limits.h` @ `fd7afced403`; port `femacs/lib/include/sys-limits.rs`.  The whole
header is `enum { SYS_BUFSIZE_MAX = INT_MAX >> 20 << 20 };`, which the port writes as the same
arithmetic over `limits.in.rs`'s `INT_MAX` (the value is 2146435072).
