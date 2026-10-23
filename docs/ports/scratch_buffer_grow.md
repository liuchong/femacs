# emacs/lib/malloc/scratch_buffer_grow.c (56 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __libc_scratch_buffer_grow  (line 27)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The symbol the unit defines is C's own, not the port's identifier.**  C 27 defines
  `__libc_scratch_buffer_grow`, but C 23's `<scratch_buffer.h>` (the wrapper,
  `emacs/lib/scratch_buffer.h` 104-107) renames that identifier to `_gl_scratch_buffer_grow`
  before the function's declarator is read, so the reference object exports the renamed
  symbol.  `nm -g build/emacs-ref/lib/malloc/scratch_buffer_grow.o` prints
  `T __gl_scratch_buffer_grow` (Mach-O's one-underscore prefix on `_gl_scratch_buffer_grow`),
  which is why the port keeps C's declaration name and lands the symbol with
  `#[unsafe(export_name = "_gl_scratch_buffer_grow")]` (PORTING.md 10.13, the `rpl_mempcpy`
  precedent; Rust forbids combining it with `#[unsafe(no_mangle)]`).
- **`__set_errno` is `<libc-config.h>`'s** (C 19-21's `#ifndef _LIBC` arm is live: `_LIBC` is
  undefined), `# define __set_errno(val) (errno = (val))` at that header's line 46; the port
  keeps it as the macro every lib unit that needs it spells (`free.rs`, `mktime.rs`,
  `canonicalize-lgpl.rs`).  C 56's `libc_hidden_def (__libc_scratch_buffer_grow)` is empty in
  this build (libc-config.h's `# define libc_hidden_def(name)`) and is named at its site.
- **`__glibc_likely`/`__glibc_unlikely`** are `<sys/cdefs.h>`'s branch hints (PORTING.md 2.6);
  the port keeps the names and the identity, as `lib/regcomp.rs` spells them.
- **Declarations in the unit.**  C 23's `<scratch_buffer.h>` and C 24's `<errno.h>` come from
  the mirrors; `malloc` (C 37) is declared here with its C prototype, because the generated
  `<stdlib.h>` does not rename it (`nm` on the reference object prints `U _malloc`) while it
  *does* rename `free` - that one is the mirror's `rpl_free` (see
  docs/ports/include-lib-malloc-scratch_buffer.md).
- **`NULL`** is `std::ptr::null_mut ()` and the null test `is_null ()` (PORTING.md 1.1/2.x);
  `true`/`false` stay Rust's `bool`, and C 30's `buffer->length * 2` keeps C's operand order.
- **Line count**: C 56 vs the port's 58 (3.6%), inside the default 5% tolerance, so no
  docs/ports/line-drift-tolerance.tsv row is needed.
- **Call-relationship differences (D7)**: none.

## Evidence

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sb cargo run -q -p xtask -- build-tu lib/malloc/scratch_buffer_grow.c
built lib/malloc/scratch_buffer_grow.rs -> ../build/femacs/scratch_buffer_grow.o

$ python3 tools/extract-abi.py --check emacs/lib/malloc/scratch_buffer_grow.c lib/malloc/scratch_buffer_grow.rs
OK emacs/lib/malloc/scratch_buffer_grow.c -> lib/malloc/scratch_buffer_grow.rs

# the port's exports and C references, against the reference object (Rust panic symbols elided)
$ nm -g build/femacs/scratch_buffer_grow.o          $ nm -g build/emacs-ref/lib/malloc/scratch_buffer_grow.o
                 U ___error                                         U ___error
                 U _malloc                                          U _malloc
                 U _rpl_free                                        U _rpl_free
0000000000000020 T __gl_scratch_buffer_grow       0000000000000000 T __gl_scratch_buffer_grow

$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name sb_check \
    -C panic=abort -C codegen-units=1 -C opt-level=0 \
    ../build/femacs/units/scratch_buffer_grow/_root.rs -o /tmp/sb.rmeta
(exit 0)
```
