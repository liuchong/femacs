# emacs/lib/save-cwd.h (44 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] SAVE_CWD_H  (line 22, body 356a192b7913)

## Types and their fields — 1
- [x] saved_cwd: int desc, char* name

## File-scope objects — 0

## Decisions

`emacs/lib/save-cwd.h` @ `fd7afced403`; port `lib/include/save-cwd.rs`.  The header is
`struct saved_cwd` plus three prototypes; the definitions live in `lib/save-cwd.rs`, so
the prototypes sit in a whole-import `decls` module the definitions shadow (PORTING.md 0.1-10).
`name` is a `char *` the C only ever sets to NULL; the type keeps C's shape.
