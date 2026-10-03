# emacs/lib/binary-io.c (38 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] BINARY_IO_INLINE  (line 17, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/binary-io.c` @ `fd7afced403`; port `femacs/lib/binary-io.rs` (62 lines vs C's 38);
the gates are `build-tu lib/binary-io.c` -> `built`,
`python3 tools/extract-abi.py --check emacs/lib/binary-io.c femacs/lib/binary-io.rs` -> `OK`,
and `nm -g build/femacs/binary-io.o`, which prints the same two definitions as
`nm -g build/emacs-ref/lib/binary-io.o` (`_gl_set_fd_mode`, `set_binary_mode` - Mach-O prints
them with the leading underscore).

- **The unit is a P-81 carrier.**  C 17's `#define BINARY_IO_INLINE _GL_EXTERN_INLINE` makes the
  two `BINARY_IO_INLINE` bodies in `binary-io.h` this object's external definitions, so the port
  keeps `pub const BINARY_IO_INLINE: () = _GL_EXTERN_INLINE;` and re-exports both functions from
  a `binary_io_unit` module with `#[unsafe(export_name = ...)]` (the lib/byteswap.rs shape).
- **The header's `#ifndef BINARY_IO_INLINE` fallback** (`_GL_INLINE`) is dead in this unit
  because the unit defines the name first; it is not carried in the mirror and is recorded in
  `docs/ports/excluded-macros.txt` (the `_GL_BYTESWAP_INLINE` row's class), which is also what
  keeps `--check` for `binary-io.h` happy while the unit owns the constant.
- **Dead text (recorded here, not transcribed).**  C 21-38's
  `#if defined __DJGPP__ || defined __EMX__` region - the `<unistd.h>` include and the
  `isatty`/`O_TEXT`/`_gl_set_fd_mode` body of `set_binary_mode`.  The evaluator decides it false
  (`__DJGPP__` and `__EMX__` are not defined by the frozen compiler), and the inventory agrees:
  this file's function list is empty.  docs/status.md 3.5's open question ("the evaluator keeps
  both branches live") is therefore answered by the current tool - it does not.
- **Line-count basis.**  The `binary-io` row in `docs/ports/line-drift-tolerance.tsv` uses the
  default `total` basis with 0.70: 18 of C's 38 lines are the dead arm, while the port carries
  the provenance note, the `#![allow]` block and the include block -
  fixed costs that against the 20 live lines alone would read as 210%.
