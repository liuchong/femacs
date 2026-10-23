# emacs/lib/malloc/dynarray_finalize.c (66 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_dynarray_finalize  (line 28)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/malloc/dynarray_finalize.c` @ `fd7afced403`; port
`lib/malloc/dynarray_finalize.rs`.  The frozen build leaves `_LIBC` undefined, so only
C 20's `<libc-config.h>` include is conditional; the body has no `#if`.

- C 32's `__dynarray_error (list)` is the shared header's `static inline`
  (`lib/malloc/include/dynarray.rs`), which C 23's `<dynarray.h>` reaches through the
  gnulib wrapper's `#else` arm.
- C 45's `(struct dynarray_finalize_result) { NULL, 0 }` is the port's
  `dynarray_finalize_result { array: null_mut (), length: 0 }`; C 58-59's designated
  initializer becomes the field-named struct expression in the same order.
- `__libc_dynarray_finalize` keeps C's source name and emits C's renamed symbol through
  `#[unsafe(export_name = "_gl_dynarray_finalize")]`.
- C's comment on the function (C 1's opening block aside) is carried verbatim, which is also
  why the port's line count matches the reference's (66 vs 66).

## Evidence

- `build-tu lib/malloc/dynarray_finalize.c` -> `built lib/malloc/dynarray_finalize.rs ->
  ../build/femacs/dynarray_finalize.o`.
- `--check` -> `OK`; the box is ticked.
- manual `rustc … --emit=metadata … units/dynarray_finalize/_root.rs` -> exit 0.
- `nm -g` on both objects prints `T __gl_dynarray_finalize`.
