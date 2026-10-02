# emacs/src/terminfo.c (55 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] tparam  (line 44)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 3
- [x] char UP  (line 30)
- [x] char BC  (line 30)
- [x] char PC  (line 30)

## Decisions

- **`#ifndef TERMINFO_DEFINES_BC` (C 29-31) is the live arm.**  TERMINFO_DEFINES_BC is set by the
  terminfo headers Emacs is configured against, not by the frozen configuration table, so the
  evaluator asks for the arm and the port defines the three objects here: `UP` and `BC` as
  `*mut c_char` and `PC` as `c_char` (C's `char *UP, *BC, PC;` declares the third differently).
- **These are the definitions, not declarations.**  `tparam.h` 41-42 declares `extern char *BC;
  extern char *UP;`, and the port had defined both in femacs/src/tparam.rs when tparam.c was
  ported (before this file was); porting terminfo.c corrected that: tparam.rs now carries the
  two `extern` declarations and this unit owns the objects, which is where C defines them.
- **`xstrdup` (lisp.h 5551) was added to the lisp.h mirror** for `tparam`'s return, and `tparm`
  is declared in the unit (a variadic libc entry, the `error` precedent).
- **The line-count drift (C 55 vs the port's 76, 38.2%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none beyond `tparm`/`xstrdup` above.
