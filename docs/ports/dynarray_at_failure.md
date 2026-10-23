# emacs/lib/malloc/dynarray_at_failure.c (40 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_dynarray_at_failure  (line 28)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray_at_failure.c` @ `fd7afced403`; port
`lib/malloc/dynarray_at_failure.rs`.  The frozen build leaves `_LIBC` undefined, so
C 36-38's `#else` arm (`abort ()`) is the live text and C 30-35's `__snprintf`/`__libc_fatal`
diagnostic is dead.  The file is **not** in `docs/baseline/config/cond-eval-verified.txt`, so
the body check reads both arms:

- the dead arm is kept as the non-frozen side's text under the feature that names it
  (`#[cfg (feature = "libc")]`, PORTING.md P-51; the `md5-stream.rs`/`malloc.rs` shape), which
  is also what supplies its `buf`, `200`, `__snprintf`, `sizeof`, `__libc_fatal`, `index` and
  `size` tokens to the comparison;
- `abort` is a system entry no mirror carries, so it is declared at its use site
  (P-26/P-72(3), the `openat-die.rs` shape); C 25's `<stdio.h>` is only used by the dead arm,
  so no stdio mirror is included;
- `__libc_dynarray_at_failure` keeps C's source name and emits C's renamed symbol through
  `#[unsafe(export_name = "_gl_dynarray_at_failure")]` (PORTING.md 10.13(1)); the frozen
  object exports `__gl_dynarray_at_failure` (Mach-O's spelling), measured with `nm -g` on both
  the port's and the reference's object;
- C 24's `#include <dynarray.h>` is the gnulib wrapper, whose `#else` arm reaches the shared
  header - `lib/include/dynarray.rs`, included here.

**Deviation.** C 1's opening-block title line is not carried (PORTING.md 0.1-19).  Line count:
C 40 vs the port's 50 (25.0%) - the licence block that is not carried plus the fixed costs
(`#![allow]`, the include block, the use-site `abort` declaration) and the P-51 arm; a
`dynarray_at_failure 0.26` row in `docs/ports/line-drift-tolerance.tsv` is needed
(`total` basis).

## Evidence

- `build-tu lib/malloc/dynarray_at_failure.c` -> `built lib/malloc/dynarray_at_failure.rs ->
  ../build/femacs/dynarray_at_failure.o`.
- `--check` -> `FAIL` with exactly **one** problem: `- line count drifted 25.0%: C 40 vs
  Rust 50`; the one box stays unchecked until that row lands.
- manual `rustc … --emit=metadata … units/dynarray_at_failure/_root.rs` -> exit 0.
- `nm -g build/femacs/dynarray_at_failure.o` prints `T __gl_dynarray_at_failure`, the same
  symbol as `nm -g build/emacs-ref/lib/malloc/dynarray_at_failure.o`.
