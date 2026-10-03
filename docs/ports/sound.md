# emacs/src/sound.c (1554 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is on the verified list and every condition it contains is decided by the frozen
  table: `HAVE_SOUND` is undefined, so the body is dead text and nothing is demanded of the port.
- Policy (docs/status.md 3.25): a file whose *whole* body is inside a disabled `#if` is ported
  as an empty unit with the `// femacs:` note - the same treatment P-51 gives an arm
  whose body needs headers the target has not got, applied at file granularity.  The reference
  text is not carried, because it needs <sys/soundcard.h>.
- The line-count relation is 0 against 1556 C lines; the tolerance row records why.
