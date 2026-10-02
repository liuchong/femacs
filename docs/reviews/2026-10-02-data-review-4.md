# Adversarial review: `src/data.c` segment IV (data.c 1029-1291, 1832-1846) and the fixes it forced

Scope: `setplist`, `subr-arity`, `subr-name`, `native-comp-function-p`,
`subr-native-lambda-list`, `subr-type`, `interactive-form`, `command-modes`,
`indirect-variable`, plus `set_symbol_trapped_write`/`restore_symbol_trapped_write`
(data.c 1832-1846, pulled in because `lisp.rs` had an invented helper under that name).

Method: reviewer A attacked the nine subrs against C (including a re-implementation of
make-docfile's reader), reviewer B attacked the dead-text decisions, the ledger and the
gate numbers, with mutation probes in `/tmp`.  Both read-only, no builds.

## Defects found and fixed

| # | Finding | Disposition |
| --- | --- | --- |
| 1 | **`Findirect_variable` wrote to a shadowed binding** (B-P1, high, a real bug): `let mut object = object;` sat *inside* the `if`, so `XSETSYMBOL!` wrote the shadow while `return object` read the parameter - following an alias chain, the function's only purpose, silently did nothing. | Fixed: the binding is hoisted to the function's scope (the `floatfns.rs`/`buffer.rs` precedent).  Recorded in P-78's residuals: this is the "every token survives" class, invisible to a token-set comparison. |
| 2 | **`set_symbol_trapped_write` was invented in `lisp.rs`** (A-F1): C's function of that name is a `static` in data.c (1832-1840) with a `Qtrapping_constant` early exit; the mirror's version had neither the linkage nor the check, and once data.c 1832 landed in the same module it would have been a duplicate definition (E0428). | Fixed: the helper is gone, `make_symbol_constant` is back to C's direct bitfield write (lisp.h 4009-4013), and the real pair is ported at its C position with the `Qtrapping_constant` exit. |
| 3 | **The `missing function` rule counted mirror *declarations* as definitions** (A-F2): five C functions were hidden from that rule (and from the stub check) because a mirror they include declares them (`indirect_function` is data.c's). | Fixed: the rule now requires a body (`r["bodies"]`); `--check` went from 26 to 29 missing functions, exactly the ones that were hidden, and `--all` stayed 17 done / 10 WIP / 0 failing. |
| 4 | **This segment's dead-text decisions were only in Rust comments** (B-P3): the two preserved `#[cfg(feature = "native_comp")]` blocks, the three omitted `#ifdef HAVE_NATIVE_COMP` DEFUNs, and the two dropped `#ifdef HAVE_MODULES` arms had no ledger entry. | Fixed: three bullets in `docs/ports/data.md`, including the evidence that the preserved text compiles (`xtask dead-arms` with the ledger presented as complete compiled `src_data.o` under the flipped configuration) and the residual that this only happens at batch boundaries until the ledger is complete. |
| 5 | **Ledger numbers had gone stale** (B-P5/P4/P6): "76 remaining subrs" (now 55), "Rust 898" (now 1362), the segment list missing three segments, "the only hole is 931-1313" (now 931-1028), "four dead defsubrs" (the user-ptrp bullet said one), "two exemptions" (now four rows). | All corrected. |
| 6 | `subr-arity`'s `let mut minargs/maxargs` drew an unused-`mut` warning. | Fixed: C's declaration shape without `mut`. |

Reviewer A found no defect in the nine subrs themselves; it verified the docstrings byte
for byte (9/9), the `#[cfg]`/`#[cfg(not)]` pairing against the frozen text, the
`HAVE_MODULES` omission (C's live chain is the same shape), `Mnth`/`make_int`,
`PVSIZE`/`CLOSURE_*` width promotion, and the mutable-binding question.  Reviewer B's
mutation matrix (11 mutants) was caught except where P-78 records the blind spots, and it
confirmed the ledger is 1:1 with the implementation and that the three native-comp boxes
stay unchecked.

## Evidence

- `cargo xtask build-tu src/data.c` -> built; `--cfg enable_checking` -> built.
- `xtask dead-arms` with the ledger temporarily complete -> `dead arms compile: src/data.rs`
  (the `native_comp` arm of this unit compiles; ledger restored byte-identically).
- `extract-abi.py --check data.c` -> 273 items, only "missing" plus the partial file's
  69.0% line-count drift (C 4400 vs Rust 1362).
- `--all` -> 17 done / 10 WIP / 0 failing; `gate-fast` -> 0; `gate-probes` -> 0.
