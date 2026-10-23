# emacs/src/indent.h (60 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_INDENT_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] position: ptrdiff_t bufpos, ptrdiff_t bytepos, EMACS_INT hpos, EMACS_INT vpos, EMACS_INT prevhpos, int contin

## File-scope objects — 1
- [x] extern ptrdiff_t last_known_column_point  (line 46)

# ----------------------------------------------------------------------
## Decisions

- **This ledger exists because the file was reading its unit's** (`docs/ports/indent.md`, the
  fallback `ledger_for` documents).  `src/indent.h`'s mirror has been complete for a long time and
  `--check` is `OK`, but a fallback checklist cannot say so: it is the *unit's*, and status 3.218
  tightened both the `--all` state machine and `tools/gen-port-inventory.py` so that a file with no
  checklist of its own is judged against its full item set (and named as `wip` in the inventory)
  instead of inheriting the unit's verdict.  With this file, no reference file in the tree reads a
  fallback any more - measured, the fallback list is now empty.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/src/indent.h src/include/indent.rs
    OK emacs/src/indent.h -> src/include/indent.rs

    $ python3 tools/gen-port-inventory.py
    (no "no checklist of their own" line: the fallback class is empty)
