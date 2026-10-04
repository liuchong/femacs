# emacs/lib/issymlink.h (65 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] issymlink  (line 49)

## Macros defined — 2
- [x] _GL_ISSYMLINK_INLINE  (line 32, body a960df3ab827)
- [x] _ISSYMLINK_H  (line 18, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/issymlink.h` @ `fd7afced403`; port `femacs/lib/include/issymlink.rs`.  The one
function is `_GL_ISSYMLINK_INLINE`-qualified, so it is an ordinary Rust item here and the
wrapper `femacs/lib/issymlink.rs` keeps the qualifier relationship - which is why the
`_GL_ISSYMLINK_INLINE` box stays unchecked here (the timespec/stat-time shape).
