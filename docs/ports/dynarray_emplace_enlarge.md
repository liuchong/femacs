# emacs/lib/malloc/dynarray_emplace_enlarge.c (77 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_dynarray_emplace_enlarge  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray_emplace_enlarge.c` @ `fd7afced403`; port
`femacs/lib/malloc/dynarray_emplace_enlarge.rs`.  The frozen build leaves `_LIBC` undefined, so
only C 20's `<libc-config.h>` include is conditional; the body has no `#if`.

- `__set_errno` is `<libc-config.h>`'s `#define __set_errno(val) (errno = (val))`, kept as the
  macro the other units spell it with (`tempname.rs`, `strtol.rs`).
- C 25's `<stdckdint.h>` is the `ckd_mul` alias family (PORTING.md 2.6, P-15): C's
  `if (ckd_mul (&new_size, new_allocated, element_size)) return false;` is the
  `overflowing_mul` pair, whose destination C writes only when the multiplication did not
  overflow - the port assigns the pair to `(new_size, overflow)` and returns before using it.
- C's `size_t new_allocated;` is assigned on every path before its first read, so the port's
  `let mut new_allocated: usize;` keeps C's shape and needs no PORTING.md 0.1-13 zero-init
  deviation.
- `__libc_dynarray_emplace_enlarge` keeps C's source name and emits C's renamed symbol through
  `#[unsafe(export_name = "_gl_dynarray_emplace_enlarge")]`; both objects export
  `__gl_dynarray_emplace_enlarge`.
- C 23's `#include <dynarray.h>` is the gnulib wrapper (its `#else` arm reaches the shared
  header mirror).

## Evidence

- `build-tu lib/malloc/dynarray_emplace_enlarge.c` -> `built
  lib/malloc/dynarray_emplace_enlarge.rs -> ../build/femacs/dynarray_emplace_enlarge.o`.
- `--check` -> `OK` (no problem lines); the box is ticked.
- manual `rustc … --emit=metadata … units/dynarray_emplace_enlarge/_root.rs` -> exit 0.
- `nm -g` on both objects prints `T __gl_dynarray_emplace_enlarge`.
