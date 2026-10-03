# emacs/src/xsmfns.c (581 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and every condition it contains is decided by the frozen
  table: `HAVE_X_SM` is undefined, so the body is dead text and nothing is demanded of the port.
- Policy (docs/status.md 3.25): a file whose *whole* body is inside a disabled `#if` is ported
  as an empty unit with the license header and this note - the same treatment P-51 gives an arm
  whose body needs headers the target has not got, applied at file granularity.  The reference
  text is not carried, because it needs <X11/SM/SMlib.h>.
- The line-count relation is 0 against 582 C lines; the tolerance row records why.
