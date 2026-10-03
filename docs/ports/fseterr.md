# emacs/lib/fseterr.c (80 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] fseterr  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file, and the same `__sferror` arm as lib/fpending.c is live: C 22-24's
  `fp_->_flags |= __SERR;` with `fp_` = `fp` and `__SERR` = 64 (measured; the value is in
  `lib/include/stdio-impl.rs`).  The other ten arms — including the `#elif 0` Rich-Felker
  fallback with its `fflush`/`fileno`/`dup`/`fputc`/`dup2`/`abort` sequence — are kept as
  reference text inside the function (P-51).
- `fseterr` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52); the reference object
  prints `T _fseterr` and so does the port's.
- `<errno.h>` is C 20's include; only the dead `#elif 0` arm uses it, and the mirror is
  included so the file's include closure matches C's.
- **Mirror edits**: `lib/include/fseterr.rs` (the `_FSETERR_H` guard, the config assertion and
  the declaration in a `pub mod`), and `lib/include/stdio-impl.rs`/`stdio.rs` as in
  docs/ports/fpending.md.
- Line-count drift (C 80 vs the port's 92) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
