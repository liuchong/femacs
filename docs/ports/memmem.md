# emacs/lib/memmem.c (71 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] memmem  (line 35)

## Macros defined — 2
- [x] AVAILABLE  (line 28, body 9512869d44e5)
- [x] RETURN_TYPE  (line 27, body a6c67a59fdf9)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `_LIBC` is undefined, so C 20-22's `<config.h>` include is live; the file is in
  cond-eval-verified.txt, so its conditions are the frozen configuration's.
- C 27-29's three lines are the port's `macro_rules! RETURN_TYPE` / `AVAILABLE` **before** the
  `include!` of the header mirror: the header's functions are written in terms of them, and
  Rust's `macro_rules!` are text-scoped (PORTING.md 0.1-5/P-57) — the same order C requires.
  `lib/memmem.c` is the only consumer of `str-two-way.h` in this tree (`lib/strstr.c` does not
  exist), so there is no second instantiation and no 10.13② split is needed.
- `memmem` is a non-`static` definition and gnulib's generated `<string.h>` renames it under
  REPLACE_MEMMEM, so the port keeps C's name and emits the real symbol with
  `#[unsafe(export_name = "rpl_memmem")]` (PORTING.md 10.13①, the mempcpy precedent).  The
  reference object prints `T _rpl_memmem`; the port's prints the same, with `_memchr`/`_memcmp`
  undefined as in C.
- `__builtin_expect` is the branch-hint helper PORTING.md 2.6 asks for; the port's macro is the
  expression itself (Rust has no block-layout hint — the deviation this file records).
- The `(const unsigned char *)` casts are `as *const u8`; `haystack_len -= haystack -
  (const unsigned char *) haystack_start` is the port's pointer-difference expression; and C's
  `!haystack || …` is `haystack.is_null () || …` (same short-circuit).
- **Mirror written with this file**: `femacs/lib/include/str-two-way.rs` (the header's three
  `static` functions as `#[inline(always)] pub unsafe fn`, i.e. one copy per unit and no
  exported symbol — the timespec.rs/c-ctype.rs shape for header-inline definitions, and the
  same linkage C's `static` gives).  `--check lib/str-two-way.h` → **OK**.
- Line-count drift (C 71 vs the port's 97) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
