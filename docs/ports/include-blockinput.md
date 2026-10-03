# emacs/src/blockinput.h (72 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] block_input  (line 53)
- [x] input_blocked_p  (line 65)

## Macros defined — 1
- [x] EMACS_BLOCKINPUT_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The two `INLINE` bodies are `#[inline(always)]` functions**: `block_input`'s
  `interrupt_input_blocked++` and `input_blocked_p`'s `> 0` are header bodies C gives every
  translation unit (P-12/P-37's shape for `INLINE`), so the port keeps them here and lets the
  unit that owns the object (keyboard.rs, when it lands) define it.
- **`INLINE_HEADER_BEGIN`/`INLINE_HEADER_END`** are attribute/spelling macros with no Rust
  counterpart and are recorded in `docs/ports/excluded-macros.txt`.
