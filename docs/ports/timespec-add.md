# emacs/lib/timespec-add.c (54 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] timespec_add  (line 30)

## Macros defined — 0

## Types and their fields — 1
- [x] timespec: time_t rs, int rns

## File-scope objects — 0

## Decisions

- `ckd_add (&rs, a, b)` both stores the wrapped sum in `rs` and yields the overflow flag;
  Rust's `overflowing_add` is that pair, written out as the store plus the flag
  (PORTING.md 2.6/P-15; the `ckd_add` family is in docs/ports/token-aliases.tsv and the
  numbers inside each call are compared, so the two calls stay two).
- C's `int nssum = a.tv_nsec + b.tv_nsec;` adds two `long` values and truncates on
  assignment, so the port truncates after the addition; `carry`, `TIMESPEC_HZ * carry` and
  `make_timespec`'s `long ns` parameter are the `c_int`/`c_long` boundary C leaves implicit.
- `TYPE_MINIMUM`/`TYPE_MAXIMUM` are unit-local macros over `<intprops.h>`'s `_Generic` tables
  (P-26, the mktime.rs precedent).  `timespec_add` is declared in the timespec.h mirror's
  `pub mod timespec_decls` + glob import, so this definition shadows it (0.1-10), and it is a
  non-`static` definition → `#[unsafe(no_mangle)]` (P-52; the reference object prints
  `T _timespec_add`).
- Line-count drift (C 54 vs the port's 76) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
