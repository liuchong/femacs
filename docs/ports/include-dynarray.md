# emacs/lib/dynarray.h (284 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] _GL_DYNARRAY_H  (line 20, body da39a3ee5e6b)
- [x] _GL_LIKELY  (line 262, body 3dd405fb64c2)
- [x] _GL_UNLIKELY  (line 263, body f06ed8d0852f)
- [x] __libc_dynarray_at_failure  (line 252, body 409fd32f24b9)
- [x] __libc_dynarray_emplace_enlarge  (line 253, body 3214d380295d)
- [x] __libc_dynarray_finalize  (line 254, body 1df174cea904)
- [x] __libc_dynarray_resize  (line 256, body ec55c88fd5a3)
- [x] __libc_dynarray_resize_clear  (line 255, body 58d3b3a437c5)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/dynarray.h` @ `fd7afced403`; port `lib/include/dynarray.rs`.  This is the
gnulib wrapper.  Its two readings and what the frozen build does with them:

- `#if defined DYNARRAY_STRUCT || defined DYNARRAY_ELEMENT || defined DYNARRAY_PREFIX`
  (C 258-276): `_GL_LIKELY`/`_GL_UNLIKELY`, then `#include <malloc/dynarray.gl.h>` and
  `#include <malloc/dynarray-skeleton.gl.h>`.  **No file in this build takes it**:
  `lib/regexec.c` includes `malloc/dynarray-skeleton.c` directly (C 1441) and reaches the
  wrapper through `regex_internal.h` 35 with none of the three names defined
  (`build/emacs-ref/lib/deps/regex.d` lists `emacs/lib/dynarray.h` then
  `emacs/lib/malloc/dynarray.h`, never a `.gl.h`).  The arm's two generated includes are
  therefore represented by `lib/malloc/dynarray-skeleton.rs` (the template as a
  `macro_rules!`) and its one instantiation in `lib/regexec.rs`; this mirror keeps the
  `_GL_LIKELY`/`_GL_UNLIKELY` text that precedes them.
- `#else` (C 279-281): `#include <malloc/dynarray.h>`, the reading every
  `lib/malloc/dynarray_*.c` takes and the one this mirror's `include!` spells.

- **Both readings reach the same header text** (see `include-lib-malloc-dynarray.md`), so the
  shared mirror is included once and no include guard is needed (xtask's include dedupe is
  the guard, PORTING.md 0.1-4).
- **The five rename macros are the record of C's identifier renaming.**  The port implements
  the rename where a symbol is actually emitted - `#[link_name = "_gl_dynarray_…"]` on the
  shared mirror's declarations and `#[unsafe(export_name = "_gl_dynarray_…")]` on the units'
  definitions (PORTING.md 10.13(1)); the macros themselves are never invoked, exactly as
  `include/scratch_buffer.rs`'s three rename macros are not.
- `_GL_LIKELY`/`_GL_UNLIKELY` keep C's `__builtin_expect` through a port-local
  `__builtin_expect` macro (PORTING.md 2.6: a branch hint, not a layout hint), and the three
  macros precede their users, which PORTING.md 0.1-18 requires of a mirror.
- The reference's opening block is not carried (PORTING.md 0.1-19).

## Evidence

- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-dyn cargo run -q -p xtask -- build-tu
  lib/dynarray.h` -> `built lib/include/dynarray.rs -> ../build/femacs/include_dynarray.o`.
- `python3 tools/extract-abi.py --check emacs/lib/dynarray.h lib/include/dynarray.rs`
  -> `OK` (8 macros, 0 functions, 0 types).
- manual `rustc --edition 2024 --crate-type=lib --emit=metadata ... units/dynarray/_root.rs`
  (this mirror's materialized root) -> exit 0.
- Compiled through the five `lib/malloc/dynarray_*.c` units and `lib/regexec.rs` as well.
