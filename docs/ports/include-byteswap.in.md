# emacs/lib/byteswap.in.h (73 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] bswap_16  (line 44)
- [x] bswap_32  (line 52)
- [x] bswap_64  (line 61)

## Macros defined — 2
- [x] _GL_BYTESWAP_H  (line 19, body 356a192b7913)
- [x] _GL_BYTESWAP_INLINE  (line 29, body a960df3ab827)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **PORTING.md 0.1**: the mirror is `lib/include/byteswap.in.rs`; the frozen
  target's generated header is build/emacs-ref/lib/byteswap.h and substitutes only
  the guard.  The three wrappers are `#[inline(always)]` (P-12) and emit no symbol:
  lib/byteswap.rs re-exports them (P-81).  `_GL_BYTESWAP_INLINE` is in
  docs/ports/excluded-macros.txt (kind=syntax).
