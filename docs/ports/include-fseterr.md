# emacs/lib/fseterr.h (55 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _FSETERR_H  (line 18, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `HAVE___FSETERR` is undefined (macros.tsv 1696), so the live arm is C 36-45's
  `extern void fseterr (FILE *fp);`; C 29-35's musl arm (the `stdio_ext.h` rename and its
  inner `__HAIKU__` declaration) is the one unchecked box above — it needs a header this
  target has not got, and no ported unit can reach it.
- The declaration is in a `pub mod` + glob import so lib/fseterr.c's definition shadows it.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/fseterr.h femacs/lib/include/fseterr.rs
OK emacs/lib/fseterr.h -> femacs/lib/include/fseterr.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/fseterr.c
built lib/fseterr.rs -> ../build/femacs/fseterr.o
```

The ledger's single box is the include guard; the musl arm its older Decisions text described
(C 29-35, needing `<stdio_ext.h>`) produces no row in the current extractor output, which is why
the box it left is the guard's - ticked here, with the arm's absence from the checklist noted.
