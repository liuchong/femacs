# emacs/src/treesit.h (257 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_TREESIT_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header's whole body is dead text, and it is still a unit.**  `HAVE_TREE_SITTER` is
  `undef` in the frozen configuration (`docs/baseline/config/darwin-arm64.macros.tsv` 1656) and
  `tools/check-cond-eval.py src/treesit.h` reports 2 conditions, 0 unknown, 0 disagreements, so
  C 25-253's `#ifdef HAVE_TREE_SITTER` region is recorded rather than transcribed (P-08): the
  three pseudo-vector records (`struct Lisp_TS_Parser` with its `TSParser`/`TSTree`/`TSInput`
  members and the `bool` flags, `struct Lisp_TS_Node` with `TSNode`, `struct Lisp_TS_Query` with
  `TSQuery`/`TSQueryCursor`), the nine inline accessors/checkers, `struct ts_linecol`'s four
  uses, `TREESIT_EMPTY_LINECOL` and the ten `extern` functions.  `docs/ports/treesit.md` records
  the same region from the unit's side; the mirror and the unit's ledger must agree on the
  boundary, which is what the two `--check` runs measure.
- **What stays is one `extern` declaration, and it is live.**  C 255's
  `extern void syms_of_treesit (void);` sits outside the guard because `src/emacs.c` 147-149
  includes this header *unguarded* ("We don't guard this with HAVE_TREE_SITTER because treesit.o
  is always compiled (to provide treesit-available-p)") and C 2347 calls it unconditionally for
  the same reason.  The mirror therefore declares exactly that name, and `emacs.rs` includes the
  mirror in C order in place of the use-site copy it carried in its "headers with no mirror at
  all" block (which now holds only `syms_of_xwidget`).
- **The include guard carries its name** the way the other mirrors do (`pub const
  EMACS_TREESIT_H: bool = true;`); the guard itself has no Rust counterpart (PORTING.md 0.1-4,
  the include graph's dedup does the same job).
- **`src/pdumper.c` 47 also includes this header unguarded**, so `pdumper.rs` includes the
  mirror too, in C order.  The two names it takes from the header (`struct Lisp_TS_Query`,
  `XTS_COMPILED_QUERY`) are inside the dead arm and stay text, so three comments there that
  said "`treesit.h` has no mirror" were corrected in the same commit; the same phrase in
  `casefiddle.rs` 563 (whose dead arm needs `treesit_linecol_maybe`/`treesit_record_change`) was
  corrected with it.  The other units that name the header (`buffer.c` 51-52, `print.c` 50-51,
  `alloc.c` 59-60, `fns.c` 40-41, `editfns.c` 63-64, `casefiddle.c` 33-34, `insdel.c` 34-35) do
  it inside `#ifdef HAVE_TREE_SITTER` and take nothing from it here, so their include lists are
  unchanged.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/treesit.h src/include/treesit.rs`
  → `OK emacs/src/treesit.h -> src/include/treesit.rs` (0 problem lines).
- `python3 tools/check-cond-eval.py src/treesit.h`
  → `2 conditions, 0 unknown to the evaluator, 0 disagreements`, and `src/treesit.h` is in
  `docs/baseline/config/cond-eval-verified.txt` (line 392), so the frozen table decides it.
- Consumer builds (the only two unguarded includers): `build-tu src/emacs.c` →
  `built src/emacs.rs -> ../build/femacs/emacs.o`; `build-tu src/pdumper.c` →
  `built src/pdumper.rs -> ../build/femacs/pdumper.o`.
- Gates on the same worktree: `check-globals.py` 283 units / 0; `check-declarations.py`
  13 sites (0 forbidden, 13 allowed literals) / 0 failures; `check-mirror-dupes.py` 0 failures;
  `check-verbatim.py` 6 trees, 4607 files, 0 failures.
- `python3 tools/extract-abi.py --all` after the landing: see docs/status.md 3.137.
