# emacs/lib/malloc/scratch_buffer_set_array_size.c (64 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_scratch_buffer_set_array_size  (line 28)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The exported symbol is C's renamed one**: C 28 defines
  `__libc_scratch_buffer_set_array_size` and C 23's wrapper renames it to
  `_gl_scratch_buffer_set_array_size` (emacs/lib/scratch_buffer.h 107), so the reference
  object exports `T __gl_scratch_buffer_set_array_size`.  The port keeps C's declarator name
  and lands the symbol with `#[unsafe(export_name = "_gl_scratch_buffer_set_array_size")]`
  (PORTING.md 10.13).
- **`__set_errno`** is `<libc-config.h>`'s (C 19-21's `#ifndef _LIBC` arm, live); C 64's
  `libc_hidden_def (…)` is empty in this build.
- **`CHAR_BIT`** (C 34) is `<limits.h>`'s; the port takes it from the `limits.in.rs` mirror
  (`CHAR_BIT: c_int = 8`, measured on the frozen target) and writes the same expression:
  `(nelem | size) >> (std::mem::size_of::<usize> () * CHAR_BIT as usize / 2) != 0`, with
  C's `nelem != 0 && size != new_length / nelem` guard in C's order (the `&&` chain
  short-circuits in both languages, which is what keeps the division safe).
- **Declarations in the unit**: `<scratch_buffer.h>` (C 23, the wrapper's gl.h text),
  `<errno.h>` (C 24), `<limits.h>` (C 25) from the mirrors; `malloc` (C 51) is declared here
  (the generated `<stdlib.h>` does not rename it: `nm` on the reference object prints
  `U _malloc`), while the `free` behind `scratch_buffer_free` is the mirror's `rpl_free`
  (`U _rpl_free`).
- **`char *new_ptr`** (C 51) stays `*mut c_char`, and C 60's assignment of it to the `void *`
  field is the explicit `as *mut c_void` cast (PORTING.md 1.1).
- **Line count**: C 64 vs the port's 63 (1.6%), inside the default 5% tolerance.
- **Call-relationship differences (D7)**: none.

## Evidence

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sb cargo run -q -p xtask -- build-tu lib/malloc/scratch_buffer_set_array_size.c
built lib/malloc/scratch_buffer_set_array_size.rs -> ../build/femacs/scratch_buffer_set_array_size.o

$ python3 tools/extract-abi.py --check emacs/lib/malloc/scratch_buffer_set_array_size.c femacs/lib/malloc/scratch_buffer_set_array_size.rs
OK emacs/lib/malloc/scratch_buffer_set_array_size.c -> femacs/lib/malloc/scratch_buffer_set_array_size.rs

$ nm -g build/femacs/scratch_buffer_set_array_size.o      $ nm -g build/emacs-ref/lib/malloc/scratch_buffer_set_array_size.o
                 U ___error                                            U ___error
                 U _malloc                                             U _malloc
                 U _rpl_free                                           U _rpl_free
0000000000000018 T __gl_scratch_buffer_set_array_size  0000000000000000 T __gl_scratch_buffer_set_array_size

$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name sb_check \
    -C panic=abort -C codegen-units=1 -C opt-level=0 \
    ../build/femacs/units/scratch_buffer_set_array_size/_root.rs -o /tmp/sb.rmeta
(exit 0)
```
