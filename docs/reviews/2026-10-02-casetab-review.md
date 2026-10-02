# Adversarial review — `port(casetab.c)` (frozen commit `6211237`)

Two adversarial reviewers worked on independent context windows over the frozen commit
`6211237` (the first translation unit with `DEFUN`s: `femacs/src/casetab.rs`, plus the
`lisp.rs` changes it needed — the FAM accessor rewrite and the `other_units` module).  They
were told the artifact is wrong and asked to prove it.  Neither edited the tree; both wrote
scratch work under `/tmp`.  This file records what they found, what was fixed, and what
remains open.

## Reviewer A — fidelity and semantics

Expression layer: **nothing falsified**.  The reviewer built a behavioural differential
(a minimal model of chartab/alloc/print on both sides, `casetab.c` against the frozen
`casetab.rs`, ten scenarios covering `set_identity`/`shuffle`/`set_canon`,
`init_casetab_once`, both `set_case_table` branches, the two `DEFUN` wrappers, `Fcase_table_p`
with twelve inputs, `syms_of_casetab`'s four `staticpro` calls *by value* and five
`defsubr` records, and the `check_case_table` failure path): **153 output lines identical**,
and **20 of 23 semantic mutants caught**.  Two insights worth keeping:

- the review had to add a scenario with a *fresh* table whose three `extras` are nil, because
  `init_casetab_once`'s own `set_case_table (down, 1)` call never runs the `map_char_table`
  callbacks (by then all three extras are already filled) — so `init_casetab_once` alone
  covers none of that code;
- the `shuffle` fixture must reach the "target table already permuted" branch, otherwise the
  three-step swap is not exercised.

| # | Finding | Disposition |
| --- | --- | --- |
| A1 (P1) | All five docstrings carried the wrong **value**: two trailing spaces before `*/`, and `\"extra\"` kept a backslash that make-docfile drops.  The reviewer implemented `read_c_string_or_comment` and checked it byte-for-byte against the machine's real Emacs `etc/DOC` (5/5).  Scale: 1,838 of 1,849 DEFUN docs have trailing whitespace, 139 carry a `\`+non-`n/t` escape. | docstrings rewritten to make-docfile's value; `cinventory.doc_comment_value` is now the one rule both sides use, so `--check` compares the value (control: a changed word fails, a trailing space is still invisible — P-36) |
| A2 (P1) | The port's `Q*` indices differ from C's (`iQcase_table` 491 vs 339), which is recorded, but nobody recorded that `lread.c` names the i-th builtin through make-docfile's `defsym_name[i]` table: with the port's order every builtin would get another symbol's name.  `defsym_name` had 0 hits in the repository. | recorded as **P-41** with the reviewer's counts and the required action (regenerate the table in the port's order when `lread.rs` lands; `countof (lispsym)` also differs) |
| A3 (P2) | `set_sub_char_table_contents` is INLINE in lisp.h 4078, but the mirror declared it as an external symbol with no definition (the reviewer intersected all 403 INLINE definitions of the four mirrored headers with the mirror's 130 `extern` declarations: exactly one case). | now an `#[inline(always)]` function |
| A4 (P2) | The gate had zero statement-level coverage for this unit: 15 of 16 semantic mutants passed the frozen `--check`. | the new constant/macro-token/function-token/stub rules (P-38) catch the dropped-call and wrong-setter class; the occurrence-level residue is recorded |
| A5 (P2) | The commit message quoted `inline-probe 6081`, which covers `character.h` only. | evidence wording corrected in the ledger |

## Reviewer B — gates and Rust soundness

"All gates green" and "the code is wrong" held simultaneously: 16 mutants passed every gate.

| # | Finding | Disposition |
| --- | --- | --- |
| B1 (P1) | `FAM_ELEM!` was **undefined behaviour**: `as_mut_ptr ()` on a `[T; 0]` field is a zero-size retag whose tag exists nowhere, so Miri rejects `AREF`/`ASET` even at index 0 (Miri command and message in the report). | `fam_base (addr_of_mut! (…))` (provenance preserved); `spikes/miri-probe.sh` is the new Miri guard — it reports "clean" with the fix, and the reviewer's reproduction had exited 1 |
| B2 (P0) | **Twelve macros could not expand at all**: `AUTO_CONS`/`AUTO_LIST1-4`/`AUTO_STRING(_WITH_LEN)` ended in `let …` without `;`; `DEFVAR_LISP/_NOPRO/_INT/_BOOL/_KBOARD` named a non-existent field (`type_` vs the mirror's `r#type`) and `DEFVAR_INT` passed `*mut isize` for C's `intmax_t *`.  The reference has ~1,070 call sites.  The ledger ticked all twelve. | all fixed; `spikes/macro-probe.sh` expands one representative call per macro (metadata-only) and is a real guard: undoing either fix makes it fail |
| B3 (P0) | `--check` compares neither function bodies nor expressions; the 13-mutation matrix for `casetab.rs` (including `bset_case_canon_table` → `bset_case_eqv_table`) passed. | function-body token containment (P-38) now catches the dropped-call/wrong-name class: MA, MD and MJ of the reviewer's own matrix fail, as do the macro/constant/stub classes |
| B4 (P1) | `cfg (enable_checking)` was never set anywhere, so every `eassert!` was deleted from every build — the C side's assertions had no counterpart in any gate. | `gate-fast` compiles each finished unit a second time with `--cfg enable_checking`; the reviewer confirmed both existing units build that way |
| B5 (P2) | The shadow probe did not prove that a local definition *wins* over the glob import (it only proved "no E0428"). | the probe now defines a 0-argument `xmalloc` and coerces it to `unsafe extern "C" fn () -> *mut c_void`, which only compiles when the local item resolves |
| B6 (P2) | 22 macros reference `other_units` names but only `eassert`/`eassume` are ever expanded — no build evidence for those paths. | `spikes/macro-probe.sh` covers the call shapes; the remaining ones are exercised as their units land |

## Where the artifact stands after this round

- Fixed in code: A1 (docstrings, plus the gate rule), A3, B1, B2, B3's rules, B4, B5.
- Recorded as open: P-41 (builtin symbol names), P-30's method note (the reviewer's
  differential and the "extras already filled" insight), and the token rule's residual
  (occurrence-level changes such as swapping two `Faset` calls, `Qt`/`Qnil`, or one of two
  identical literals).
- Verified after the fixes: `--check` OK for the six ported files; `--all` 6 done / 0
  failing; `gate-fast` green (two units, twice each — normal and `enable_checking`);
  inline/write/accessor/fam/shadow/macro probes all pass; `miri-probe` clean.
