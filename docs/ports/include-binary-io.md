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

`emacs/lib/binary-io.h` @ `fd7afced403`; port `lib/include/binary-io.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/lib/binary-io.h lib/include/binary-io.rs`
-> `OK`.

- **`O_BINARY` is 0 on this target**, so the header's whole `#if O_BINARY` arm (the `setmode`/
  `_setmode`/`fileno` renames for the DJGPP/EMX/Cygwin and MSVC cases) is dead; the live text is
  the `#else` arm's two `_GL_INLINE` bodies, `_gl_set_fd_mode` and `set_binary_mode`, which the
  mirror keeps as `#[inline(always)] pub fn` (P-12) and `lib/binary-io.rs` re-exports (P-81).
- **`BINARY_IO_INLINE`** (C 32-34) is the one row this mirror does not carry: the carrier unit
  defines the name to `_GL_EXTERN_INLINE` before including the header, so the `#ifndef` fallback
  is dead there, and the name is the unit's own constant.  The macro is listed in
  `docs/ports/excluded-macros.txt` (the `_GL_BYTESWAP_INLINE` row's class), which is what makes
  the mirror's `--check` pass while the unit owns it.
- `SET_BINARY` (C 80) is a `macro_rules!` that discards its result, the shape C's
  `((void) set_binary_mode (fd, O_BINARY))` has.
- The dead `#if O_BINARY` arm's `_gl_set_fd_mode`, `fileno` and `_setmode` spellings are kept as
  `#[cfg (feature = "o_binary")]` text (P-51): the arm is reachable in another configuration and
  its text is what the ledger's dropped-constant check reads.
