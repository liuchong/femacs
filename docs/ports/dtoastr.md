# emacs/lib/dtoastr.c (19 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LENGTH  (line 18, body da4b9237bacc)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/dtoastr.c` @ `fd7afced403`; port `femacs/lib/dtoastr.rs`; the gates are
`build-tu lib/dtoastr.c` -> `built`, `python3 tools/extract-abi.py --check emacs/lib/dtoastr.c
femacs/lib/dtoastr.rs` -> `OK`, and the symbol ratchet (this unit is listed as pending).

- **The file is an inclusion shim** (PORTING.md 10.13②): C 18-19 are `#define LENGTH 2` and
  `#include "ftoastr.c"`, so its text is the instantiation of gnulib's `ftoastr.c` template for
  `double`.  The port's own text is C 18's `LENGTH` (the template's width selector) plus the
  provenance note.
- **The instantiation is pending on the shared body.**  `femacs/lib/ftoastr.rs` does not exist
  yet: `ftoastr.c` is the template `dtoastr.c` (`LENGTH 2`), `ldtoastr.c` (`LENGTH 3`, not
  compiled on this target) and `nstrftime.c` share, and 10.13② asks for the shared body to land
  first (as a prelude `macro_rules!` or a parameterised unit) and the instantiations to follow.
  The reference object exports `dtoastr`, so the unit is listed in
  `docs/baseline/symbol-pending.txt` with that reason rather than given an invented body here -
  the same "record the dependency, do not fake the definition" rule the ledger applies to dead
  arms.
- **The inventory** is the one macro; the line-count row (`dtoastr 1.30`, `total` basis) exists
  because a 19-line file cannot absorb the fixed costs of a port's header.
