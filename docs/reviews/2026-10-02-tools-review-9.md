# Adversarial review: the CI "Generated artifacts are fresh" repair (round 9, tooling)

Scope: the follow-up to round 8's recorded CI finding, plus what the two reviewers found while
attacking it.  `tools/cinventory.py` (the `<aggregate>` record's key name and `_defun_heads`),
`tools/gen-lifetimes.py` (carrying the decision columns across a regeneration), one latent
NameError in `tools/extract-abi.py`, the regenerated `docs/baseline/{inventory,subrs,totals}.tsv`,
`LIFETIMES.tsv` and `docs/port-inventory.md`, and the numbers quoted in
`docs/port-master-plan.md`, `docs/goal.md`, `PORTING.md` and `docs/ports/data.md`.

Method: reviewer A attacked the tool code (`cinventory`/`gen-lifetimes` semantics, consumers of
the renamed key, the new rows, the carry-over pairing) with seeded /tmp ledgers; reviewer B
attacked the artifacts (idempotence of all four generators, baseline self-consistency, the
changed columns, every quoted number, and what else CI would run).  Both read-only, no builds.

## What the round fixed (found by round 8's reviewer B, then diagnosed here)

1. `cinventory.struct_fields`'s second `<aggregate>` branch built its record with the key `type`
   where its sibling branch uses `c_type`: 54 records in the headers `gen-lifetimes.py` reads
   (81 tree-wide).  `gen-lifetimes.py` died on the first such header, and so did
   `extract-abi.py`'s per-file doc mode - the path that generates `docs/ports/*.md` (reviewer A
   verified `HEAD` dies at `extract-abi.py:1465` and the fix renders `<aggregate> slice` etc.).
   `emit-mirror.py` was **not** affected (A measured byte-identical output on all 12 headers):
   the round-8 note that said it was is corrected in this round's docs.
2. `gen-lifetimes.py` rewrote the table without carrying the five decision columns, so the first
   `owner`/`freed_by` a porter fills would have been destroyed by any regeneration.  It now
   carries them, keyed by (the row's non-decision columns, its occurrence with that key).
3. `docs/baseline/*.tsv` were stale relative to the current extractor (last written at
   `7ad447f`), so CI's `git diff --exit-code` could not pass: constants 1,834 -> 3,898 and
   struct fields 5,441 -> 5,764, plus 148 rows whose only change is the subr doc digest.
   Regenerated, with the quoted numbers updated in the four docs above.

## Reviewer A: one real defect in the new code, three smaller ones, and a correction of the record

| # | Finding | Disposition |
| --- | --- | --- |
| A-F1 (Medium) | the carry-over paired a decision with the *first* row matching its key, so for duplicated rows (39 groups; e.g. one anonymous struct under three `#if` variants of `lib/ieee754.in.h` are three identical rows) a decision lands on the wrong copy - the porter's legitimate edit then makes the regeneration non-idempotent and CI fails with a diff that cannot explain itself (A reproduced it with a seeded ledger) | fixed: keys are (columns, occurrence) over **all** rows, and its own seed test now lands on the same copy |
| A-F2 (Low) | the comment claimed an orphaned decision "fails the run" but `main` returned 0 - the local regenerate-and-commit path could drop a decision with a green CI | fixed: orphan and malformed rows make the generator exit 1 (verified: exit 1 + the full key on stderr) |
| A-F3 (Low) | rows whose cell count is not 17 were skipped silently, so a schema drift or a stray tab would drop decisions with no orphan entry | fixed: counted, reported and fatal |
| A-F4 (Info) | the round's own figures were wrong in three ways: 54 is the header domain (81 tree-wide); the 5,441 -> 5,764 delta is **not** the aggregate rows (HEAD's own extractor already yields 5,764 - the baseline was simply stale; the real decomposition is +324/-1 records, of which 84 are `<aggregate>`); and `emit-mirror.py` was not a victim | corrected in `docs/ports/data.md`, `PORTING.md` and `docs/port-master-plan.md`, with the measurements attributed |
| A-I1 (Info) | `extract-abi.py`'s missing-verified-list path called an **undefined** `fail`, so the loud message was a NameError; `gen-lifetimes.py` counted a row as decided on `owner` alone although its own header says `owner` **and** `freed_by` | both fixed (the first now prints and returns 2, verified) |

A also confirmed: no consumer expects the old `type` key; the new `<aggregate>` rows are genuine
fields with distinct keys (checked against `coding.h`, `dispextern.h`, `lisp.h`, `intervals.h`,
`sfnt.h`, `ieee754.in.h`); `raw` is set consistently with its sibling branch and nothing parses
it; and the 5,567 -> 5,825 row delta is a stale-file refresh (+57 aggregates, +342 ordinary rows,
-136 rows the old scanner had invented).

## Reviewer B: one HIGH gate hole, one already-fixed claim, two wording items

| # | Finding | Disposition |
| --- | --- | --- |
| B-1 (High) | `cinventory._defun_heads` skipped only `", \t"` after the Lisp name, so every `DEFUN ("name",\n       Fname, ...)` - Emacs's own style - fell through `re.match`: only **1,708 of 1,849** DEFUN bodies were reachable by the body comparison, i.e. exactly the blind spot P-78 exists to close (treesit.c 11/58, xwidget.c 25/36, process.c 9/63, w32fns.c 11/60, xfaces.c 8/33). Pre-existing at HEAD (`93b04f1`), not caused by this change | fixed: `\r\n` are skipped now; `treesit.c` 11 -> 58, `data.c` stays 120, tree-wide 1,849 = the textual count; `--all` is unchanged (17 done / 10 WIP / 0 failing) because the newly reachable bodies are in files with no port yet |
| B-2 (Medium) | the "fails on an orphan" claim was false (A-F2) | fixed in the same edit |
| B-3 (Low) | the delta attribution in the ledger and `PORTING.md` | corrected (A-F4) |
| B-4 (Low) | `docs/port-master-plan.md`'s new parenthetical said "the last two columns" for 1,834/5,441 (1,834 is the 6th column, and the last column, globals, did not change) | rewritten with the columns named and the real decomposition |

B's positive measurements, which are the round's main artifact evidence: all four generators are
idempotent in two rounds (byte-identical, tree untouched); `totals.tsv` equals the column sums of
`inventory.tsv` on every column, and the subr table's 1,841 rows equal the live-DEFUN count; the
262 changed lines of `inventory.tsv` differ **only** in the constants/fields columns (131 rows,
no counterexample); the subr-table diff is one column (digest) with an unchanged row set; the
per-directory table, `docs/goal.md`, `PORTING.md`'s counting table and the regenerated
`docs/port-inventory.md` all agree with the new baseline; `extract-abi.py --all` exits 0 with
17 done / 10 WIP / 563 todo (matching the inventory doc); and the **only** CI blocker left is the
one already recorded in the master plan: CI has no `emacs/` checkout (`/emacs` is gitignored, so
the fast tier's first step fails).

## Gates run for this round

`gate-fast` exit 0 (`inventory: no differences`, ten finished TUs compiled twice, `layout-check
passed`) and `gate-probes` exit 0, with the frozen configuration rebuilt from scratch for this
round; `extract-abi.py --all` 17 done / 10 WIP / 0 failing; `--check` on `data.c` and on the
three finished units unchanged/clean; `xtask gen-globals` regenerates `globals.rs` byte-identically
(so the baseline's digest refresh does not reach the mirror); the three CI generators plus
`gen-port-inventory.py` leave the tree byte-identical when re-run; `git -C emacs status --short`
empty.

## Note for the next round

`PORTING.md` now carries P-79 (the user's safe-first rule): `unsafe` only where C's construct has
no safe Rust spelling, an `unsafe` scope no wider than the expression it guards, every new
`unsafe` traceable to the C line it mirrors, and no mass rewrite of the existing shape (the
allow-list `unsafe_op_in_unsafe_fn` in each unit header stays, with narrowing done on the next
touch of a file).  Porting resumes with data.c's arithmetic core and then `syms_of_data`.
