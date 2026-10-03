# emacs/lib/stdc_count_ones.c (23 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_STDC_COUNT_ONES_INLINE  (line 17, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **PORTING.md 10.12 applies**: this file is a gnulib `_GL_*_INLINE` wrapper - its C content
  is `#define <qualifier> _GL_EXTERN_INLINE` and its includes, i.e. the
  C99 inline model's "this unit emits the external definition" file.  The port's functions
  are single Rust items in the header's mirror, so what survives is the macro and its
  relationship to `_GL_EXTERN_INLINE` (the marker in lib/include/gnulib_config.rs).
- **No `// femacs:` header**: C's own file is 20-23 lines, and the drift rule's +/-5% leaves
  no room for one; this ledger is where the provenance is recorded.
