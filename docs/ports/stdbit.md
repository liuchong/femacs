# emacs/lib/stdbit.c (23 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_STDBIT_INLINE  (line 20, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions
1. **C 1-18's opening block is not ported** (PORTING.md 0.1-19).  What survives of the C text is
   the `_GL_STDBIT_INLINE = _GL_EXTERN_INLINE` definition and the two includes (PORTING.md 10.6);
   the unit's first comment line names only the file.  (3.100 removed the phrase that had recited
   the block there.)
