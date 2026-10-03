# emacs/lib/strtoll.c (33 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] QUAD  (line 19, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

`emacs/lib/strtoll.c` @ `fd7afced403`; port `femacs/lib/strtoll.rs` (33 lines); the gates are
`build-tu lib/strtoll.c` -> `built` and `python3 tools/extract-abi.py --check
emacs/lib/strtoll.c femacs/lib/strtoll.rs` -> `OK`.  The frozen build compiles this file (no
`build/emacs-ref/lib/strtoll.o` exists), so `tools/check-symbols.py` does not compare it;
`nm -g build/femacs/strtoll.o` shows `T _strtoll` plus the imports `_isspace`, `_isalpha`,
`_toupper` and `___error` - the body of the `QUAD == 1` instantiation.

- **The file is an inclusion shim** (PORTING.md 10.13②): C 19-21 are `#define QUAD 1` and
  `#include <strtol.c>`, so its whole text is the `QUAD == 1` instantiation of gnulib's
  `strtol.c` template.  The port's own text is C 19's `QUAD` plus the instantiation.

- **The instantiation is landed** on the shared body `femacs/lib/strtol.rs`, which is
  `macro_rules! strtol_body`.  This shim `include!`s the prelude and instantiates it once:
  `strtol_body! (strtoll, "strtoll", c_longlong, c_ulonglong, c_longlong::MIN,
  c_longlong::MAX, c_ulonglong::MAX)` - C's `QUAD` arm (the `long long` types, the
  `LLONG_MIN`/`LLONG_MAX`/`ULLONG_MAX` limits) and the `strtoll` symbol C 109-112's
  `#define strtol strtoll` produces.  `docs/ports/strtol.md` records the mapping.

- **The `_LIBC` block is dead.**  C 23-33's `#ifdef _LIBC` (`SHLIB_COMPAT`'s
  `__strtoq_internal` alias and the `strtoq` weak alias) is not part of the gnulib text this
  port carries; it is recorded here and not transcribed.  The frozen `#if` evaluator resolves
  all 3 of the file's conditions (`tools/check-cond-eval.py lib/strtoll.c` -> 3 conditions, 0
  unknown, 0 disagreements).

- **No line-drift row is needed**: C 33 vs the port's 33 (0.0%).

- **The `#![allow (…)]` block lives here**, not in `femacs/lib/strtol.rs`: an attribute inside
  an included file is E0753 (the `strftime.rs`/`ftoastr.rs` decision).
