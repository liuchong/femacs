# emacs/lib/tempname.h (72 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] GL_TEMPNAME_H  (line 21, body da39a3ee5e6b)
- [x] GT_DIR  (line 31, body 356a192b7913)
- [x] GT_FILE  (line 30, body b6589fc6ab0d)
- [x] GT_NOCREATE  (line 32, body da4b9237bacc)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of lib/tempname.h, complete: the `GL_TEMPNAME_H` guard marker (the idx.rs shape), the
three `GT_*` kinds of C 30-32 (the live `#else` arm; `__GT_FILE` is undefined when the header is
included by lib/tempname.c, and C 34-37's `#error` compares the two spellings' values), and the
four prototypes in `pub mod tempname_decls` + glob import (PORTING.md 0.1-10).
`python3 tools/extract-abi.py --check emacs/lib/tempname.h lib/include/tempname.rs` -> `OK`.
