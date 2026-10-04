# emacs/nt/inc/stdbool.h (16 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] _Bool  (line 10, body 4306fbbc0e35)
- [x] _NT_STDBOOL_H_  (line 2, body da39a3ee5e6b)
- [x] bool  (line 11, body 3bd5f76b0997)
- [x] false  (line 12, body b6589fc6ab0d)
- [x] true  (line 13, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/stdbool.h, complete for this target: the include guard and `_Bool`, the MSVC
boolean type, as `pub type _Bool = c_schar;`.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor
resolves no `#if` in it and both the `__GNUC__` arm (the `# include_next <stdbool.h>` a MinGW build
takes, since that compiler is GCC) and the `!__GNUC__` MSVC arm are live text.  The MSVC arm is
the only one that declares anything, and the mirror carries it.

**Names with no item**: `bool`, `true` and `false` (C 11-13) are Rust keywords rather than items -
`const true …`, `type bool …` and `macro_rules! true` are all rejected by the parser - and Rust's
own `bool`/`false`/`true` are the counterparts, so nothing is declared for them; the three rows
this round appends to `docs/ports/excluded-macros.txt` record it (the `alignas` shape: the
counterpart is a language feature, not an item).  `_Bool` has its own row there too, because the
port's spelling of it is the `pub type _Bool = c_schar;` above, and the gate's item scan reads
macros and constants, not types (the `file_offset`/`Pixel` shape).

**Includes**: C 8's `#include_next <stdbool.h>` resolves to MinGW's own header, so the frozen
target's stdbool is not pulled in; that header has no port mirror of its own.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/stdbool.h
femacs/nt/inc/include/stdbool.rs` -> `OK` (with the expected note that C's four MSVC-arm names have
recorded counterparts).
