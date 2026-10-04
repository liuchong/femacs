# emacs/lib/malloc/dynarray_resize_clear.c (39 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_dynarray_resize_clear  (line 27)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray_resize_clear.c` @ `fd7afced403`; port
`femacs/lib/malloc/dynarray_resize_clear.rs`.  The frozen build leaves `_LIBC` undefined, so
only C 20's `<libc-config.h>` include is conditional; the body has no `#if`.

- C 31's `__libc_dynarray_resize (list, size, scratch, element_size)` is the shared mirror's
  declaration - the renamed `_gl_dynarray_resize` symbol that `lib/malloc/dynarray_resize.c`
  defines.
- C 34's `char *array = list->array;` and C 35-36's `array + (old_size * element_size)` are
  `let array: *mut c_char = (*list).array as *mut c_char;` and `array.add (…)`: C's `char *`
  arithmetic is byte arithmetic, which `add` on `*mut c_char` reproduces, and `memset`'s first
  parameter is cast back to `*mut c_void`.
- `__libc_dynarray_resize_clear` keeps C's source name and emits C's renamed symbol through
  `#[unsafe(export_name = "_gl_dynarray_resize_clear")]`.
- C 23's `#include <dynarray.h>` is the gnulib wrapper; C 24's `<string.h>` is the string
  mirror.

**Deviation.** C 1's opening-block title line is not carried (PORTING.md 0.1-19).  Line count:
C 39 vs the port's 31 (20.5%) - the licence block that is not carried dominates a file this
small; a `dynarray_resize_clear 0.21` row in `docs/ports/line-drift-tolerance.tsv` is needed
(`total` basis).

## Evidence

- `build-tu lib/malloc/dynarray_resize_clear.c` -> `built
  lib/malloc/dynarray_resize_clear.rs -> ../build/femacs/dynarray_resize_clear.o`.
- `--check` -> `FAIL` with exactly **one** problem: `- line count drifted 20.5%: C 39 vs
  Rust 31`; the one box stays unchecked until that row lands.
- manual `rustc … --emit=metadata … units/dynarray_resize_clear/_root.rs` -> exit 0.
- `nm -g build/femacs/dynarray_resize_clear.o` prints `T __gl_dynarray_resize_clear` and
  `U __gl_dynarray_resize`; the reference object prints the same pair.
