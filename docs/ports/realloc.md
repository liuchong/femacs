# emacs/lib/realloc.c (101 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_realloc  (line 38)

## Macros defined — 2
- [x] _GL_REALLOC_INLINE  (line 24, body a9f37094e6e8)
- [x] _GL_USE_STDLIB_ALLOC  (line 22, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The dead arms are kept as gated text (P-51).**  `NEED_SANITIZED_REALLOC` is undefined and
  `HAVE_REALLOC_0_NONNULL`, `HAVE_REALLOC_POSIX`, `HAVE_MALLOC_PTRDIFF` are all 1
  (docs/baseline/config/darwin-arm64.macros.tsv 1404, 919, 920, 859), so the live body is C's
  `n1 = n`, the (now empty) `if (n == 0)` block, `realloc (p, n1)` and the return; the three
  arms and the `<errno.h>`/`<stdckdint.h>` uses they carry are written under
  `need_sanitized_realloc`, `no_realloc_0_nonnull`, `no_malloc_ptrdiff` and `no_realloc_posix`.
- **`#ifndef _GL_INLINE_RPL_REALLOC` (C 27) is not in the frozen table**, so it does not decide
  anything here: the whole file is the live text the ledger lists (there is no `#else`).
- **C 21-23's two `#define`s are constants**: `_GL_USE_STDLIB_ALLOC` (the system allocator
  selection, as in malloc.c) and `_GL_REALLOC_INLINE` = `_GL_EXTERN_INLINE` (the 10.12 shape,
  the `timespec.rs` precedent).  The system `malloc`/`realloc`/`abort` entries are declared in
  the unit.
- **The line-count drift (C 101 vs the port's 88, 12.9%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none, beyond the system allocator pair above.
