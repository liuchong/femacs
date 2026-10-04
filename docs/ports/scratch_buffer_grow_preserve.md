# emacs/lib/malloc/scratch_buffer_grow_preserve.c (67 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_scratch_buffer_grow_preserve  (line 28)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The exported symbol is C's renamed one**: C 28 defines
  `__libc_scratch_buffer_grow_preserve`, and C 23's wrapper renames it to
  `_gl_scratch_buffer_grow_preserve` (emacs/lib/scratch_buffer.h 106), so the reference
  object exports `T __gl_scratch_buffer_grow_preserve`
  (`nm -g build/emacs-ref/lib/malloc/scratch_buffer_grow_preserve.o`).  The port keeps C's
  declarator name and lands the symbol with
  `#[unsafe(export_name = "_gl_scratch_buffer_grow_preserve")]` (PORTING.md 10.13).
- **`__set_errno`/`__glibc_likely`/`__glibc_unlikely`** come from `<libc-config.h>` (C 19-21's
  `#ifndef _LIBC` arm, live) and `<sys/cdefs.h>`, as in `scratch_buffer_grow.rs`; C 67's
  `libc_hidden_def (…)` is empty in this build.
- **Declarations in the unit**: `<scratch_buffer.h>` (C 23, the wrapper's gl.h text),
  `<errno.h>` (C 24), `<string.h>` (C 25, `memcpy`) from the mirrors; `malloc` (C 37) and
  `realloc` (C 46) are declared here - the generated `<stdlib.h>` renames neither
  (`nm` on the reference object prints `U _malloc` and `U _realloc`), while the `free` at
  C 56 is the mirror's `rpl_free` (`nm`: `U _rpl_free`).
- **The stack/heap test** C 33 writes as `buffer->data == buffer->__space.__c` is the same
  comparison `scratch_buffer_free` makes, spelled in Rust as the mirror does (the array
  decays to a pointer in C; the port casts the place expression), and the `memcpy` at C 40 is
  `<string.h>`'s, unrenamed.
- **Line count**: C 67 vs the port's 69 (3.0%), inside the default 5% tolerance.
- **Call-relationship differences (D7)**: none.

## Evidence

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sb cargo run -q -p xtask -- build-tu lib/malloc/scratch_buffer_grow_preserve.c
built lib/malloc/scratch_buffer_grow_preserve.rs -> ../build/femacs/scratch_buffer_grow_preserve.o

$ python3 tools/extract-abi.py --check emacs/lib/malloc/scratch_buffer_grow_preserve.c femacs/lib/malloc/scratch_buffer_grow_preserve.rs
OK emacs/lib/malloc/scratch_buffer_grow_preserve.c -> femacs/lib/malloc/scratch_buffer_grow_preserve.rs

$ nm -g build/femacs/scratch_buffer_grow_preserve.o
                 U ___error
                 U _malloc
                 U _memcpy
                 U _realloc
                 U _rpl_free
0000000000000020 T __gl_scratch_buffer_grow_preserve
$ nm -g build/emacs-ref/lib/malloc/scratch_buffer_grow_preserve.o
                 U ___error
                 U _malloc
                 U _memcpy
                 U _realloc
                 U _rpl_free
0000000000000000 T __gl_scratch_buffer_grow_preserve

$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name sb_check \
    -C panic=abort -C codegen-units=1 -C opt-level=0 \
    ../build/femacs/units/scratch_buffer_grow_preserve/_root.rs -o /tmp/sb.rmeta
(exit 0)
```
