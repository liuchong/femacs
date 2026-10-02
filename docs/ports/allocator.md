# emacs/lib/allocator.c (22 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_USE_STDLIB_ALLOC  (line 18, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- This is the unit docs/status.md 3.4 listed as blocked on the allocator.h mirror; the mirror is
  in the tree now, together with the `_GL_USE_STDLIB_ALLOC` marker the C's line 19 defines.
- C's `struct allocator const stdlib_allocator = { malloc, realloc, free, NULL };` is one
  `#[unsafe(no_mangle)] pub static` with the three system entries of the stdlib mirror and
  `die: None` for C's `NULL`.
- The C file is 22 lines and the port is 40 (the `// femacs:` note, the license header, two
  `include!` lines and the export attribute); the tolerance row records it.
- No call-relationship differences (D7).
