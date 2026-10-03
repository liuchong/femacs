# emacs/src/treesit.c (5634 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] treesit-available-p  (Ftreesit_available_p, 0/0, doc de003bcfb47b, line 5295)

## Functions defined — 1
- [x] syms_of_treesit  (line 5312)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/treesit.c` @ `fd7afced403`; port `femacs/src/treesit.rs` (69 lines); the
gates are `build-tu src/treesit.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/treesit.c femacs/src/treesit.rs` -> `OK`.

- **Which text is live.**  `HAVE_TREE_SITTER` is `undef` (macros.tsv 1625), so 53 of
  5,634 lines are live: C 22-27 (`config.h`, `lisp.h`, `buffer.h`, `coding.h`, `treesit.h`),
  `Ftreesit_available_p` with its `#if HAVE_TREE_SITTER` arm dead (C 5303's `return Qnil;`) and
  `syms_of_treesit`'s one `defsubr`.
- **Dead text (recorded here, not transcribed).**  C 29-5293: the `<tree_sitter/api.h>` and
  `<tree_sitter/parser.h>` includes, the `Lisp_TS_Parser`/`Lisp_TS_Node`/`Lisp_TS_Query` and
  cursor records with their accessors, the predicate and query engines,
  `treesit_load_language`, the error signalers and the ninety-odd DEFUNs from
  `treesit-parser-p` to `treesit-subtree-stat`.  C 5314-5621: `syms_of_treesit`'s `DEFSYM`s, its
  `Vtreesit_*` variables and the `defsubr`s.  C 5623-5631's `#ifdef WINDOWSNT` block is dead too.
- **Include list.**  C 24-27's `buffer.h`, `coding.h` and `treesit.h` mirrors are not pulled in:
  every name the live text could take from them sits inside the dead region (the atimer.rs
  precedent for an include the live text never names).
- **Line-count basis.**  The `treesit` row in `docs/ports/line-drift-tolerance.tsv` is a `live`
  row (53 live lines vs the port's 69 - the fixed costs of the provenance note, the allow block
  and the include block).
