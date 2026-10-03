# emacs/src/blockinput.h (72 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 3 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The two `INLINE` bodies are `#[inline(always)]` functions**: `block_input`'s
  `interrupt_input_blocked++` and `input_blocked_p`'s `> 0` are header bodies C gives every
  translation unit (P-12/P-37's shape for `INLINE`), so the port keeps them here and lets the
  unit that owns the object (`keyboard.rs`, when it lands) define it.
- **`INLINE_HEADER_BEGIN`/`INLINE_HEADER_END`** are attribute/spelling macros with no Rust
  counterpart and are recorded in `docs/ports/excluded-macros.txt`.
