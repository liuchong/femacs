# emacs/lib/fsync.c (86 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file has no items to tick: `HAVE_...`-style conditions do not appear in it - its two arms
  are selected by `_WIN32`/`__CYGWIN__` alone, which the frozen table decides (this is
  darwin-arm64), so the Windows arm is dead and the live arm is C 84-85's `#error`.
- The reference therefore refuses to compile here, and the frozen build's object list does not
  contain `fsync.o`; the port is an empty unit that says so (policy: docs/status.md 3.25).
- `lib/fsync.c`'s port is *not* the system `fsync`: no symbol is defined, exactly as in the
  reference.
