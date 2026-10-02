# emacs/src/macros.h (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_MACROS_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 2
- [x] extern EMACS_INT executing_kbd_macro_iterations  (line 29)
- [x] extern Lisp_Object executing_kbd_macro  (line 31)

## Decisions

- **The header is complete.**  macros.h is 51 lines: the guard, two file-scope objects and
  four prototypes; all of them are in the mirror.
- **The two objects are declared in `pub mod macros_externs`.**  They are *defined* by
  macros.c, so a flat `extern "C"` block would collide with the unit's definitions; the
  module is the shape `include/bignum.rs` uses for `mpz` (P-21/P-39), and `use
  macros_externs::*;` makes the names visible the way the rest of the header's declarations
  are.
- **`#include "lisp.h"` (macros.h 24) has no `include!` here.**  A `src/` mirror never
  includes lisp.h; the unit that owns it includes both in C's order (PORTING.md 0.1-9).
