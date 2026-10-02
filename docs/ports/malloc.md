# emacs/lib/malloc.c (56 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_malloc  (line 32)

## Macros defined — 1
- [x] _GL_USE_STDLIB_ALLOC  (line 21, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The three `#if !HAVE_*` arms are kept as gated text (P-51).**  The frozen configuration has
  `HAVE_MALLOC_0_NONNULL`, `HAVE_MALLOC_PTRDIFF` and `HAVE_MALLOC_POSIX` all 1
  (docs/baseline/config/darwin-arm64.macros.tsv 855, 859, 858), so C's live text is the three
  lines around `malloc (n)`; the arms are written under features that name the non-frozen side
  (`no_malloc_0_nonnull`, `no_malloc_ptrdiff`, `no_malloc_posix`) so `--check` sees their tokens
  - including the `ckd_add`/`overflowing_add` family occurrence - and the ledger keeps the text.
  They are not compiled by `xtask dead-arms`, because the frozen configuration has no key to
  flip for them (FLIPS only names config.rs constants).
- **`_GL_USE_STDLIB_ALLOC` is the constant the ledger compares** (C 21's
  `#define _GL_USE_STDLIB_ALLOC 1`, before `<config.h>`); it is what makes C's `<stdlib.h>`
  declare the system `malloc` unrenamed, which is why the replacement may call it.  The port
  declares that system entry itself.
- **`ckd_add (&signed_n, n, 0)` is the `overflowing_add` pair** (P-15), as elsewhere.
- **The line-count drift (C 56 vs the port's 71, 26.8%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none, beyond `malloc` being the system allocator the
  macro selected.
