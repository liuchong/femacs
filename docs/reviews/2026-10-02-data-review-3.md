# Adversarial review: `src/data.c` segment III (data.c 716-930) and the P-78 gate fix

Scope: the twelve symbol accessors and setters (`boundp`, `fboundp`, `makunbound`,
`fmakunbound`, `symbol-function`, `symbol-plist`, `symbol-name`, `bare-symbol`,
`symbol-with-pos-pos`, `remove-pos-from-symbol`, `position-symbol`, `fset`), plus the
gate change made while porting them: `DEFUN` subr bodies had never been token-compared
(P-78), so `tools/cinventory.py` gained `defun_spans`/`blank_literals` and
`tools/extract-abi.py` now compares subr bodies.

Method: reviewer A attacked the twelve ports against C; reviewer B attacked the *tool
change* (its own focus) with `/tmp` C/Rust fragments, direct calls into
`cinventory`/`extract-abi`, and mutation copies.  Both read-only, no builds.

## Reviewer A: no defects in the twelve

Verified, with C line references: `boundp`'s `start:`/`goto start` becoming
`loop { match }` (the `continue` target, the `blv->fwd` early `return Qt`, the four
`redirect` arms plus `_ => emacs_abort ()`, and `BASE_EQ` in the tail); `makunbound`'s
manual bitfield write (it built clang **and** gcc probes of the bitfield layout and
confirmed `gcmarkbit` bit 0, `redirect` bits 1-2, `trapped_write` 3-4, `interned` 5-6,
`declared_special` bit 7 match `lisp.rs`'s masks, so the write touches only bits 1-2);
`fset`'s non-circularity `while` (increment at the end, `EQ` not `BASE_EQ`),
`eassert! (valid_lisp_object_p (definition) != 0)` (C returns `int`), and the
`#[cfg(feature = "native_comp")]` block (dead in the frozen configuration, every symbol
it names exists, and `dead_arms` flips both the constant and the feature); the
symbol-with-pos four; the field paths; and all twelve docstrings byte for byte against a
re-implementation of `make-docfile.c`'s reader (its check is stricter than the tool's
digest, which folds whitespace).

Its two observations: (A) `data.rs` is compiled by no automated gate while its ledger is
incomplete - receipts for this segment are recorded below; (B) the token gate cannot see
a swap of paired constants (`SYMBOL_VARALIAS` <-> `SYMBOL_PLAINVAL`), which it measured;
recorded in P-78's residuals and owed to the ERT/differential layer.

## Reviewer B: six tool findings, all addressed

| # | Finding | Disposition |
| --- | --- | --- |
| D1 | `_matching` was not literal-aware: `printf ("}")` truncated a body, `char c = '{'` swallowed the rest of the file (both silent). | Fixed: `cinventory.blank_literals` (length- and line-preserving) feeds all matching; the four cases now yield exact spans.  Corpus scan by the reviewer found 0 such DEFUNs today, so this was latent, not active. |
| D2 | `DEFUN_HEAD`'s `"[^"]*"` silently dropped a DEFUN whose lisp name holds an escaped quote or a parenthesis. | Fixed: `cinventory._defun_heads` scans the head character by character (comments are blanked, escapes handled); both synthetic cases now produce spans. |
| D3 | The two casetab exemptions remove the only *semantic* token from those bodies, so `false` <-> `true` is invisible there. | Recorded in both notes rows and in P-78's residuals; the real closure is a behavioural probe (the ERT layer), not a token rule. |
| D4 | The eight declaration specifiers added to `local_types` are already in `MACRO_TOKEN_SKIP` - dead code (measured: 0 bodies change). | Removed, with a comment saying where they are handled. |
| D5 | `if "*" in allowed: continue` appeared twice in `compare()`. | The duplicate is gone. |
| D6 | P-78's coverage numbers were blind counts: 150 spans, but only **82** bodies are actually compared (52 data.c + 25 floatfns + 5 casetab); data.c also has 4 DEFUNs the condition table kills. | Corrected in P-78, together with the residuals (`function_spans`/`defuns` remain literal-blind; `skip_short` hides <= 2-character identifiers - 40 of the 52 ported subrs contain one). |

Reviewer B also confirmed the parts that matter for the fix: the subr body map is keyed by
`c_name` (no silent skip), a dropped `XSETCAR`/`SET_SYMBOL_VAL` statement in a subr body is
now reported, the `#[...]` stripping cannot eat string contents (strings are blanked
first), and the ledger (81 ticked / 101 open) matches the implementation exactly.

## Evidence

- `cargo xtask build-tu src/data.c` -> built; `rustc --cfg enable_checking` on the
  materialized unit -> 0 errors, 128,784-byte object (the receipts reviewer A asked for).
- `extract-abi.py --check data.c` -> 288 items, all "missing" except the partial file's
  line-count drift (C 4400 vs Rust 1099): 120 missing `defsubr`, 64 subrs, 26 functions,
  12 constants, 1 macro.
- `--all` -> 17 done / 10 WIP / 0 failing (casetab/floatfns stayed green through the new
  subr-body rule: the three first reports were 1 tool bug and 2 documented literals).
- `xtask gate-fast` -> 0; `xtask gate-probes` -> 0 (lisp-abi 89 declarations, dead arms 5).
