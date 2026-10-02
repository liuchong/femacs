# emacs/lib/binary-io.h (89 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] _gl_set_fd_mode  (line 52)
- [x] set_binary_mode  (line 73)

## Macros defined — 5
- [x] BINARY_IO_INLINE  (line 35, body a960df3ab827)
- [x] SET_BINARY  (line 80, body cd1d2f38f5ec)
- [x] _BINARY_H  (line 18, body da39a3ee5e6b)
- [x] _gl_set_fd_mode  (line 43, body 85e459cccfbc)
- [x] fileno  (line 45, body f3da3d29faba)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/binary-io.h` @ `fd7afced403`; port `femacs/lib/include/binary-io.rs`.  `O_BINARY` is
0 on this target, so the header's live arms are the two `_GL_INLINE` functions (`_gl_set_fd_mode`
returning O_BINARY, and `set_binary_mode` calling it) plus the `SET_BINARY` macro; the
`# if O_BINARY` arm with its `setmode`/`_setmode`/`fileno` renames is kept as cfg'd text (P-51)
and `BINARY_IO_INLINE`/`_GL_INLINE` are markers, as the other `*_INLINE` mirrors record theirs.
