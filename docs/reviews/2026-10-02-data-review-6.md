# Adversarial review: `src/data.c` segment VI (data.c 1660-2176)

Scope: `set` (1660) and the move of `set_internal`'s machinery into C's definition order;
`default_value` (1962), `default-boundp` (2007), `default-value` (2020),
`set_default_internal` (2034), `set-default` (2131), the file's first `union Lisp_Val_Fwd`
(2142) and `make_blv` (2149); plus the three new `docs/ports/macro-body-notes.tsv` rows.

Method: reviewer A attacked the code itself (ordered token alignment with `difflib`, operand
and argument orders, branch/return order, byte-exact docstrings against `make-docfile`'s
rule, C-order monotonicity of all 106 top-level items, a line-multiset comparison against
`git show HEAD:femacs/src/data.rs`); reviewer B attacked the gate/ledger surface with mutation
probes in `/tmp` (the notes row, the union's field comparison, `--check`'s accounting).
Both were read-only and neither built anything.

## Reviewer A: no high or medium semantic defect

Verified in the pass: the move is lossless (old-minus-new is empty, the 98 shared segments
are byte-identical modulo indentation, `only-new` is exactly the eight items this round
added); the top-level order is now 106/106 monotone in C's definition order, where HEAD had
two inversions (`restore_symbol_trapped_write` before `set_internal`,
`notify_variable_watchers` before `find_symbol_value`); `default_value`'s `loop` re-evaluates
the scrutinee each round and keeps C's branch/return order in the `SYMBOL_LOCALIZED` arm;
`set_default_internal`'s precedence (`(bits & MASK) >> SHIFT != SYMBOL_PLAINVAL && bindflag
!= SET_INTERNAL_THREAD_SWITCH`), its hand-expanded `FOR_EACH_LIVE_BUFFER` (C's own
`FOR_EACH_ALIST_VALUE` carries no `BUFFER_LIVE_P` filter either), the `idx`-before-
`set_per_buffer_default` order and the `KBOARD` arm's byte arithmetic all match; all 68
ported subrs' arity/intspec/docstring digests are byte-equal; `Lisp_Buffer_Local_Value` is
40/8 with `fwd`/`where`/`defcell`/`valcell` at the baseline's offsets; `make_blv`'s call
order (`set_blv_where` -> `local_if_set` clear -> `set_blv_defcell` -> `set_blv_valcell` ->
`set_blv_found`) is C's, which matters because `blv_found`'s `eassert` depends on it.

| # | Finding | Disposition |
| --- | --- | --- |
| A-D1 (low) | the ledger's `Lisp_Val_Fwd` box was unchecked although the union is ported (the only "implemented but not ticked" item) | ticked; ledger now 68 subrs + 37 functions + 1 union |
| A-D2 (low) | `set_internal`'s eight-line C comment (1671-1677) was still a one-line `/* ... */` placeholder - a text-fidelity defect under PORTING.md 0.3, in a region this round moved | restored verbatim |
| A-D3 (info) | C's `blv->local_if_set = 0` and the mirror's `bits & !BLV_LOCAL_IF_SET` both read-modify-write a word `xmalloc` left uninitialized; `set_blv_found` reads it too | kept as C's own statement order and observable state (no initialization C does not have); the two bits are the only ones any code reads; probe P1 recorded for the segment that brings `make-variable-buffer-local` (see B-D1 for the gate side) |
| A-D4 (info) | `// data.c 1832-1840.` for `set_symbol_trapped_write`: C's `static void` is on 1833 | corrected |
| A-D5 (info) | lisp.h's `set_internal` (4206), `notify_variable_watchers` (4165) and `default_value` (621) prototypes are not in the mirror's `other_units` | recorded in the ledger as the first calling unit's prerequisite (this unit needs none: a definition is visible in its own module), with `set_default_internal` (lisp.rs 3672) as the shape to copy |

## Reviewer B: two real gate/ledger items, one rejected

| # | Finding | Disposition |
| --- | --- | --- |
| B-D1 (medium) | `make_blv`'s `blv->local_if_set = 0` had **no** mechanical check left: the statement contributes only the field name and a `0` literal, both exempted, and the port's substitute spelling `BLV_LOCAL_IF_SET` is a token the C body never has.  Deleting the whole statement passed `--check` (only the insensitive line-count line moved) and `build-tu`. | Fixed by mechanism, not by a note: `docs/ports/token-aliases.tsv` gains the family `local_if_set`/`blv_local_if_set`, whose *occurrence* check requires the port to spell it at least as often as the C body does (the mechanism that closed review D's F2 for `memcpy`).  `set_internal`'s exemption drops `local_if_set`; `make_blv`'s row keeps only the unspeakable `0`.  Evidence: mutation M1 (delete the statement) and M2 (retarget it to `BLV_FOUND`) now both report `family local_if_set: the C body uses it 1 time(s), the port 0`. |
| B-D2 (medium, same row) | the two `u.s.bits` mask choices were only human-checked | accepted as P-78 residual (4); the `local_if_set` half is now pinned, `redirect`/`trapped_write`/`found` remain per-body exemptions, and generalising the family to them is recorded in P-78 as the next gate improvement |
| B-D3 (low-medium) | `Lisp_Val_Fwd` implemented but unchecked (A-D1's item) | ticked |
| B-D4 (low) | the ledger's "Ported so far" numbers were a round stale (52/26, Rust 1637, 62.8%) and did not name 1962-2176 | updated to 48/23, Rust 1885, 57.2%, with the new range and `set`'s move named |
| B-D5 (low) | the union has no `c-layout` key and no recorded decision | structurally impossible (`layout-check` compiles only `include/*.rs`, and P-72 keeps a data.c-local type in `data.rs`); the port asserts `size_of`/`align_of` == 8 next to the declaration (the `bignum.rs`/`doprnt.rs` precedent) and the ledger records the decision |
| B-D6 (low) | `--check` reported every unported subr twice ("missing subr" plus "`DEFUN!` ... has no definition") | fixed in `tools/extract-abi.py`: the second loop skips names the first already reported.  This file's `--check` went 253 -> 205 differences, and the remainder is now a one-to-one list (120 `defsubr` + 48 subrs + 23 functions + 12 constants + 1 macro + 1 line drift) |
| B-D7 (info) | `store_symval_forwarding`'s row was said to allow a `case` its C body lacks | **rejected**: measured with the gate's own tokenizer, `allowed - C == {}` and `case` is in the C body (a `switch` over `XFWDTYPE` with six `case` labels, data.c 1472-1540).  No change |
| B-D8 (info) | `lisp.rs`'s pointer at 1069 said `src/lisp.h 2000-2040` for the three symbol enums, which are at 763-784 (the block's own `SYMBOL_TRAPPED_WRITE` comment already said 777-783) | corrected, with the three enum names named; the same wrong pointer is what the `redirect`/`trapped_write` notes rows cite, so it is an auditability fix |

## Gates run for this round

`cargo build-tu src/data.c` (green, warnings only), `extract-abi.py --check emacs/src/data.c
femacs/src/data.rs` (205 differences, all of them "not ported yet" plus the line-count drift -
no docstring, body-token, field or macro-body difference), `--all` (27 pairs: 17 done / 10 WIP /
0 failing), `gate-fast` (exit 0: condition evaluator verified, inventory no differences, five
finished units built twice each, `layout-check passed`), `gate-probes` (exit 0, five units
compiled with the non-frozen arms), and the enable_checking build of every materialized root
(bignum 47,000 / casetab 47,688 / data 211,408 / doprnt 37,808 / floatfns 52,024 / sort 132,616
bytes).  The frozen configuration was rebuilt from scratch for this round
(`build/emacs-ref/src/config.h`: 3,911 lines, 342 `#define` - the health line) and
`git -C emacs status --short` is empty, so the reference tree is untouched.

## Toolchain note (new, worth remembering)

The credential command must run with `femacs/` as its working directory, or with an explicit
`+nightly-2026-10-01`.  Run from the repository root, rustup resolves the *default* stable
1.98.1 and the build dies with `E0658: use of unstable library feature c_variadic` on
`core::ffi::VaList` in the lisp.h mirror - which is exactly P-04's reason for the pin, not a
defect in the unit under test.  Under the pinned nightly the same command is green for all six
roots.
