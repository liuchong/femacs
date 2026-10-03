# emacs/lib/close-stream.h (30 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The whole header is three pieces of text: `<stdio.h>`, the `extern "C"` guard (which has no
  Rust counterpart) and the one prototype.  The prototype is in a `pub mod` + glob import so
  lib/close-stream.c's definition shadows it (PORTING.md 0.1-10).
- Every box above is ticked: the header has no conditional text.
