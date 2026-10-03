# emacs/lib/dtoastr.c (19 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LENGTH  (line 18, body da4b9237bacc)

## Types and their fields — 0

## File-scope objects — 0


# ----------------------------------------------------------------------
## Decisions

`emacs/lib/dtoastr.c` @ `fd7afced403`; port `femacs/lib/dtoastr.rs` (42 lines); the gates are
`build-tu lib/dtoastr.c` -> `built`, `python3 tools/extract-abi.py --check emacs/lib/dtoastr.c
femacs/lib/dtoastr.rs` -> `OK`, and the symbol ratchet (the unit no longer needs its
`docs/baseline/symbol-pending.txt` entry).

- **The file is an inclusion shim** (PORTING.md 10.13②): C 18-19 are `#define LENGTH 2` and
  `#include "ftoastr.c"`, so its text is the `LENGTH == 2` instantiation of gnulib's
  `ftoastr.c` template for `double`.  The port's own text is C 18's `LENGTH` plus the
  instantiation.

- **The instantiation is landed** on the shared body `femacs/lib/ftoastr.rs`, which is
  `macro_rules! ftoastr_body` (PORTING.md 10.13②'s "prelude `macro_rules!` or parameterised
  unit", the `strftime.rs`/`nstrftime.rs` shape).  This shim is that macro's only
  instantiation: `include!("ftoastr.rs")` brings the prelude, and `ftoastr_body! (2, dtoastr,
  "dtoastr", c_double, c_double, DBL_DIG, DBL_MIN, _GL_DBL_PREC_BOUND, strtod)` is C 52-58's
  arm - `$length` 2, C's identifier `dtoastr`, the symbol `dtoastr`, and the double/FLOAT
  bindings.  `docs/ports/ftoastr.md` records the arm mapping and the standalone-arm record the
  prelude keeps for its own `--check`.

- **The symbol is emitted.**  `nm -g build/femacs/dtoastr.o` shows `T _dtoastr` (plus Rust
  monomorphization weaks, which `check-symbols.py` filters), matching
  `build/emacs-ref/lib/dtoastr.o`'s `T _dtoastr`; both import `_sprintf` and `_strtod`, which
  is the `!GNULIB_SNPRINTF` fallback the frozen build takes.

- **The inventory** is the one macro; the line-count row (`dtoastr 1.30`, `total` basis) still
  covers a 19-line file whose port is almost wholly fixed costs - the copied GPL notice, the
  provenance note and the `#![allow (…)]` block the including unit must carry (an inner
  attribute in the included `ftoastr.rs` would be E0753).
