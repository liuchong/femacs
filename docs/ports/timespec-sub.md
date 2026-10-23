# emacs/lib/timespec-sub.c (55 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] timespec_sub  (line 31)

## Macros defined — 0

## Types and their fields — 1
- [x] timespec: time_t rs, int rns

## File-scope objects — 0

## Decisions

- The same shape as lib/timespec-add.rs with `ckd_sub`/`overflowing_sub`: the store
  and the flag are two Rust statements per C call (PORTING.md 2.6/P-15), and the
  `ckd_sub` family's call arguments are compared by the checker.
- C's `int nsdiff = a.tv_nsec - b.tv_nsec;` truncates a `long` difference; `borrow` is C's
  `bool` kept as the `c_int` the second `ckd_sub` adds to `rs`.
- `TYPE_MINIMUM`/`TYPE_MAXIMUM` are the unit-local `<intprops.h>` macros (P-26), and
  `timespec_sub`'s definition shadows the timespec.h mirror's declaration (0.1-10) with
  `#[unsafe(no_mangle)]` (P-52) — C's non-`static` definition exports `timespec_sub` (the
  file is not in the frozen build's object list, so the symbol comes from the C source).
- Line-count drift (C 55 vs the port's 71) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
