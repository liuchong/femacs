# emacs/lib/careadlinkat.c (173 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] readlink_stk  (line 58, static)
- [x] careadlinkat  (line 161)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- No `#if` decides any statement: the file is live text, and `tools/check-cond-eval.py` measures
  0 disagreements on it.  The two conditional constructs are recorded here: C 36-38's
  `#ifndef SIZE_MAX` fallback is **live** (this file does not include <stdint.h>, so the macro
  really is undefined - the port keeps `((size_t) -1)` as `usize::MAX`) and C 55-57's
  `_GL_GNUC_PREREQ (10, 1)` noinline attribute is dropped (PORTING.md 0.3).
- C's `MIN (IDX_MAX, MIN (SSIZE_MAX, SIZE_MAX))` is computed in the operands' common type; the
  port spells the two comparisons stepwise with the conversions C makes implicitly.
- The four function-pointer calls keep C's shape through the `.unwrap ()` spelling the port uses
  for a member the C calls directly (json.rs/vm-limit.rs precedent); the two nullable members
  (`reallocate`, `die`) keep C's `if (alloc->reallocate)` tests as `is_some ()`.
- `struct allocator`/`stdlib_allocator` come from the new allocator mirror, `idx_t`/`IDX_MAX`
  from idx.rs, `MIN`/`MAX` from minmax.rs.
- No call-relationship differences (D7).
