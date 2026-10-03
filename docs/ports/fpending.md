# emacs/lib/fpending.c (63 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __fpending  (line 34)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file.  `__sferror` is defined on this target (macros.tsv 7030) and `_IO_EOF_SEEN`,
  `_IO_ftrylockfile`, `__GNU_LIBRARY__`, `__EMX__`, `__minix`, `_IOERR`, `__UCLIBC__`,
  `__QNX__`, `__MINT__`, `EPLAN9` are not, so C 42-44's BSD arm is the live one and
  `stdio-impl.h` 197-199 gives `fp_` as plain `fp`.  The other nine arms are kept as
  `// `-prefixed reference text inside the function (P-51) so the checker still compares
  their field names.
- `__fpending` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52); the reference
  object confirms the symbol (`nm build/emacs-ref/lib/fpending.o` prints `T ___fpending`),
  and the port's own object prints the same.
- The `return fp_->_p - fp_->_bf._base;` pointer difference is the port's
  `((*fp)._p as isize - (*fp)._bf._base as isize) as usize` (C's `ptrdiff_t` → `size_t`).
- **Mirror edits**: `femacs/lib/include/stdio-impl.rs` is new (the measured `__sFILE`/`__sbuf`
  layout plus `__SERR`), `femacs/lib/include/stdio.rs` now names `FILE` as that struct instead
  of an opaque enum (the three field-reading units need it), and
  `femacs/lib/include/fpending.rs` is this header's mirror (its declaration is in a
  `pub mod` so this unit's definition shadows it, PORTING.md 0.1-10).
- Line-count drift (C 63 vs the port's 75) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
