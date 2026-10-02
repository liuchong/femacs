# emacs/lib/filevercmp.h (93 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] FILEVERCMP_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/filevercmp.h` @ `fd7afced403`; port `femacs/lib/include/filevercmp.rs`.  Both
prototypes are declared (in `pub mod decls` + glob import, so the defining unit can shadow
them: the P-19/P-21/P-49 shape), `_GL_CONFIG_H_INCLUDED` is asserted as the md5 mirror does,
and `_GL_ATTRIBUTE_PURE` is a syntax-only attribute recorded in
`docs/ports/excluded-macros.txt`.  `--check` → `OK`.
