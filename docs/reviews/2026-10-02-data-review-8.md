# Adversarial review: `src/data.c` segment VIII (data.c 2414-2564)

Scope: `local-variable-p` (2414), `local-variable-if-set-p` (2465), `variable-binding-locus`
(2500, the file's only `FALLTHROUGH`), the plain function `indirect_function` (2550) and
`indirect-function` (2560) - the last of data.c's buffer-local family, plus three new
`docs/ports/macro-body-notes.tsv` rows and the ledger/`LIFETIMES`-adjacent edits they need.

Method: reviewer A attacked the code (an exhaustive table over the four `redirect` values
and both forwarding kinds, per-statement and argument-order alignment, byte-level docstring
comparison against an independent re-implementation of make-docfile's rule); reviewer B
attacked the gate/ledger surface with ~25 mutants run through the gate's own `--check` in
`/tmp`, the ledger arithmetic, both generated artifacts and the CI workflow. Neither
compiled anything (per the protocol).

## Reviewer A: no defect in any of the seven attack points

A's control-flow table for `variable-binding-locus` shows the port reaches C's shared tail
from exactly the two arms C reaches it from (LOCALIZED, and FORWARDED-with-BUFFER_OBJFWD),
with `PLAINVAL`/kboard/non-buffer-forwarding returning and the `_` arm diverging through
`emacs_abort () -> !`; the only `continue` sits in the `SYMBOL_VARALIAS` arm; and the tail's
`redirect == SYMBOL_LOCALIZED` test is kept verbatim, which is what distinguishes the
fallthrough case - its `&&` short-circuit is what keeps `SYMBOL_BLV`'s `eassume!` from
firing on the FORWARDED path.  It also verified: the two `break`-less loops (every arm
terminates, so no accidental loop), `decode_buffer` evaluated before `CHECK_SYMBOL`,
`XSETBUFFER`/`BASE_EQ` order, the `blv_found` and `assq_no_quit` ternary polarities, `buf`
(not `current_buffer`) on both sides of the `idx == -1 ||` short-circuit, the hoisted
`let mut variable = variable;` writes landing on the binding `assq_no_quit` and
`Flocal_variable_p` read, the `BLV_LOCAL_IF_SET` bit, `indirect_function`'s loop and
`#[unsafe(no_mangle)]` (P-39/P-77 shadowing), `indirect-function` ignoring `noerror` as C
does, and all four docstrings byte for byte (129/269/209/196 bytes, including the two
spaces in "just return it.  Otherwise").

| # | Finding (LOW, documentation only) | Disposition |
| --- | --- | --- |
| A-F1 | the new `unused_parens` bullet said the port "keeps C's parenthesisation" and that this segment adds two warnings: C writes `if (sym->u.redirect == SYMBOL_LOCALIZED)` with no inner pair, the parens come from the mirror's bitfield re-derivation, the quoted snippet did not match `data.rs:2260`, and the count was unverifiable without a compiler | rewritten: measured (`build-tu` reports four such warnings, at `data.rs` 1749/1993/2097 and one inside an included mirror), attributed to the mirror, and stated that this segment adds none |

A also flagged, outside this diff, that `docs/ports/token-aliases.tsv` still cited
`lisp.rs 1970` for `BLV_LOCAL_IF_SET` (round 6 moved it to 1971) - fixed here.

## Reviewer B: the gate still cannot see this class (recorded), three stale artifacts (fixed), one pre-existing CI blocker (recorded)

B confirmed the three rows are exactly the difference set in both directions and correctly
placed, that the ledger's arithmetic is right in all of it (75/116 live subrs, 38/60
functions, 197 difference lines = 41+22+1+12+120+1, no other category, no tick without a
definition and vice versa), that `--all` is 17/10/0 with only data.c moving (197 -> 200 when
the rows are deleted, so they are load-bearing for exactly those three bodies and inert
everywhere else), that `docs/port-inventory.md` is fresh, and that the `local_if_set` family
from round 6 does fire here (deleting the bit read or retargeting it to `BLV_FOUND` is
reported).

| # | Finding | Disposition |
| --- | --- | --- |
| B-F1 (High) | the whole `FALLTHROUGH` shape is unconstrained: six mutants stay green (empty arm -> `return Qnil ()`/`return Qt ()`, deleting or `continue`-ing the tail-reaching `break`, moving the tail into the arm, adding `continue;` to the FORWARDED arm) | recorded in the ledger and P-78; B's fix (a positive requirement: C's `FALLTHROUGH` implies the port has a `break`) is named as the next gate work |
| B-F2 (High) | the packed-bit dispatch is unconstrained in all three bodies (mask/shift swap, dropped `>> SHIFT`, flipped tail test, swapped arm labels, `KBOARD_OBJFWDP`/`BUFFER_OBJFWDP` swap - all green) | same; the ledger already names the `docs/ports/bitfields.tsv` mechanism |
| B-F3 (High) | loop control is unconstrained (`continue` -> `break`, `continue` deleted, `break` added to `indirect_function`, `while` rewritten as `loop`, `&&` -> `\|\|`: `&&` is not a token) | recorded; `goto start` -> `continue` is another positive requirement |
| B-F4 (High) | `buf` vs `current_buffer` is invisible at all four sites - `decode_buffer (buffer)` survives only because the C token `buffer` is satisfied by the Rust *type* `*mut buffer` | recorded; a per-token count floor would catch all four |
| B-F5 (High) | polarity/pairing: `!= 0` -> `== 0` on `BLV_LOCAL_IF_SET`, and the `Qt`/`Qnil` ternaries | recorded as probe territory (P-78's residual class) |
| B-F6 (Med) | the two new `let mut variable = variable;` rebinds are invisible (delete, retarget, or re-shadow inside the arm - the round-4 defect class) | recorded (P-78 residual 3/4) |
| B-F7 (Med, CI) | `docs/port-map.md`'s `src/data.c` target cell was stale (2,122 vs 2,286) | fixed: regenerated in this commit |
| B-F8 (Med, CI, pre-existing) | the CI "Generated artifacts are fresh" step is **already red at HEAD**: `tools/gen-lifetimes.py` crashes with `KeyError: 'c_type'`, and `tools/gen-baseline.py`'s output differs from the committed `docs/baseline/*.tsv`; both reproduce from a HEAD-archived copy of `tools/`, so neither is this round's doing | recorded in the ledger and P-78 (with `docs/port-master-plan.md` 403's "CI has no `emacs/` checkout" finding); repairing the generators and the derived baseline is a separate change |
| B-F9 / B-F10 (Low) | the ledger said "71 ticked + 49 unticked" (now 75 + 45) and gave the segment's range as 2414-**2566** (2566 is the *next* section's comment; the code ends at 2564) | both fixed |
| B-F11 / B-F12 (Info) | the three rows are minimal and load-bearing; the `local_if_set` family works as designed | no change |

B also produced the round's "right by accident" list so it is recorded rather than counted
as verification: `buffer` (satisfied by a type name), `buf` (four occurrences), `variable`
and `object` (the hoisted rebinds add occurrences), `lispfwd` (a cast, and exempt as a
`local_types` member), and the pairwise-swappable enum constants (`SYMBOL_FORWARDED`,
`SYMBOL_LOCALIZED`, `SYMBOL_BLV`, `Qnil`).

## Gates run for this round

`cargo build-tu src/data.c` (green; 17 warnings, all of the pre-existing classes);
`extract-abi.py --check` (the five new bodies clean - only the 48.0% line-count drift, C 4400
vs Rust 2286); `--all` (27 pairs: 17 done / 10 WIP / 0 failing); `gate-fast` exit 0 (frozen
configuration rebuilt from scratch this round: 3911 lines, 342 `#define`, `GCC_LINT 1` at
209; inventory no differences; `layout-check passed`); `gate-probes` exit 0; the
`enable_checking` build of all six materialized roots (data 278,312 bytes);
`docs/port-inventory.md` fresh; `docs/port-map.md` regenerated and committed;
`git -C emacs status --short` empty.

## Note for the next round

The repo is now carrying two recorded, pre-existing CI blockers (B-F8) plus the
already-recorded missing `emacs/` checkout.  They are independent of the porting loop (they
touch `tools/` and `docs/baseline/`), and the cascade is real: regenerating
`docs/baseline/*.tsv` changes `docs/port-inventory.md`'s per-file counts and the numbers the
master plan quotes, so it wants its own commit with its own review rather than a ride-along
on a porting round.
