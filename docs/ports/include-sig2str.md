# emacs/lib/sig2str.h (32 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] SIGNUM_BOUND  (line 29, body 82f8fed24694)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/sig2str.h` @ `fd7afced403`; port `femacs/lib/include/sig2str.rs`.  The reference header holds the
guard and one constant: C 24-35 chooses `SIGNUM_BOUND` from `_sys_nsig`,
`_SIG_MAXSIG`, `NSIG` or the literal 64.  This target defines `NSIG` (through `__DARWIN_NSIG`,
32) and neither of the first two, so the `NSIG - 1` arm is the live one (measured) and the port
keeps it; the NSIG value itself joins the signal mirror.
