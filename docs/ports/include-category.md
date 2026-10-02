# emacs/src/category.h (124 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] CATEGORY_MEMBER  (line 82)
- [x] CHAR_HAS_CATEGORY  (line 89)

## Macros defined — 12
- [x] CATEGORYP  (line 62, body 9aff4b25cedf)
- [x] CATEGORY_DOCSTRING  (line 100, body b63c74aac76c)
- [x] CATEGORY_SET  (line 77, body e743c9afb193)
- [x] CATEGORY_SET_P  (line 67, body bf8b6cfe9272)
- [x] CATEGORY_TABLE_VERSION  (line 109, body b6553a1dc335)
- [x] CHECK_CATEGORY  (line 64, body 0d1bc69105d3)
- [x] CHECK_CATEGORY_SET  (line 73, body 5f91913b0ea4)
- [x] EMACS_CATEGORY_H  (line 26, body da39a3ee5e6b)
- [x] MAKE_CATEGORY_SET  (line 71, body bccb87f14235)
- [x] SET_CATEGORY_DOCSTRING  (line 104, body 8514a11179df)
- [x] Vstandard_category_table  (line 97, body 30fbd8ed0d63)
- [x] WORD_BOUNDARY_P  (line 116, body f003e9640a9b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **category.h is complete**, and the `Vstandard_category_table` macro is the one item that
  cannot be a Rust item: `#define Vstandard_category_table BVAR (&buffer_defaults,
  category_table)` pastes the field name, so the port writes the `BVAR!` expansion at each
  use site; `docs/ports/excluded-macros.txt` records the name (the `KVAR` class).
- **Three macros became functions** (`CATEGORYP`, `CATEGORY_SET_P`, `MAKE_CATEGORY_SET`) and
  one became a `macro_rules!` (`SET_CATEGORY_DOCSTRING`, because C's `ASET (…)` is an
  assignment); `char_category_set` is declared in lisp.rs 5354 as in C, not here.
- **`word_boundary_p` lives in `pub mod category_externs`** so category.c's definition
  shadows the declaration instead of colliding with it (P-21/P-39).
