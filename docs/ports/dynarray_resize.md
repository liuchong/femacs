# emacs/lib/malloc/dynarray_resize.c (68 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_dynarray_resize  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray_resize.c` @ `fd7afced403`; port
`lib/malloc/dynarray_resize.rs`.  The frozen build leaves `_LIBC` undefined, so only
C 20's `<libc-config.h>` include is conditional; the body has no `#if`.

- `__set_errno` is `<libc-config.h>`'s macro (the `tempname.rs`/`strtol.rs` spelling), used by
  C 48's overflow arm.
- C 45's `ckd_mul (&new_size_bytes, size, element_size)` is the `overflowing_mul` pair
  (PORTING.md 2.6/P-15); on overflow the port runs C's own `__set_errno (ENOMEM); return
  false;`, so the errno path is not dropped.
- `__libc_dynarray_resize` keeps C's source name and emits C's renamed symbol through
  `#[unsafe(export_name = "_gl_dynarray_resize")]`; it is the symbol
  `dynarray_resize_clear.rs` and `regexec.rs` call.
- C 23's `#include <dynarray.h>` is the gnulib wrapper (its `#else` arm reaches the shared
  header mirror).

## Evidence

- `build-tu lib/malloc/dynarray_resize.c` -> `built lib/malloc/dynarray_resize.rs ->
  ../build/femacs/dynarray_resize.o`.
- `--check` -> `OK`; the box is ticked.
- manual `rustc … --emit=metadata … units/dynarray_resize/_root.rs` -> exit 0.
- `nm -g` on both objects prints `T __gl_dynarray_resize`; the port object's undefineds
  (`malloc`, `realloc`, `memcpy`) match the reference's.
