# Adversarial review: `src/data.c` segment II (data.c 1314-1646) — 2026-10-02

Scope: `femacs/src/data.rs`'s forwarding machinery (`kboard_for_bindings`,
`do_symval_forwarding`, `wrong_choice`, `wrong_range`, `check_choice`,
`check_fwd_predicate`, `store_symval_forwarding`, `swap_in_global_binding`,
`swap_in_symval_forwarding`, `find_symbol_value`, `Fsymbol_value`), the `lisp.rs`
declarations it needs, and the four `macro-body-notes.tsv` exemptions.

Method: two reviewers in independent contexts, given the paths and the instruction to
find *why it is wrong* (no implementer reasoning), read-only, no builds.  Both re-derived
`make-docfile`'s docstring rule themselves, and reviewer B ran mutation experiments in
`/tmp` (`--check` against mutated copies of `data.rs`) to measure what the gate can and
cannot see.

## Defects found and fixed

| # | Finding | Disposition |
| --- | --- | --- |
| 1 | **`integer_to_intmax`/`integer_to_uintmax` in `lisp.rs` had an invented `\|\| INTEGERP (num)`** (lisp.h 4098-4122 is `i != 0`).  Because `INTEGERP` is true for every bignum, the overflow branch of every caller became unreachable: this segment's `store_symval_forwarding` `Lisp_Fwd_Int` arm silently wrote 0 instead of signalling `overflow-error`, and the already-finished `bignum.rs` `check_integer_range` returned 0 silently.  (A-D1, high; pre-existing, in a file this segment depends on) | Fixed in `lisp.rs` (both functions, with the reason in a comment). |
| 2 | **`swap_in_symval_forwarding` called the asserted `set_blv_found`** where C writes the one-bit field directly (data.c 1597).  The helper's `eassert` (data.c 84) compares `found` with `defcell`/`valcell`, which the port has not updated yet at that point, so a checking build could abort on a legal swap.  (A-D2) | Fixed: the port now writes `BLV_FOUND` directly, with the reason recorded. |
| 3 | **`wrong_choice` was not `-> !`** although lisp.h 4164 declares it `AVOID`.  The gate never sees `noreturn` on functions (PORTING.md 66), so this was invisible.  (A-D3) | Fixed. |
| 4 | **The `no_mangle` rule was one-directional and its lookback fixed at 400 characters** (A/B-M2).  Mutation evidence: deleting `#[unsafe(no_mangle)]` from `find_symbol_value` was caught, but *adding* one to a C-`static` helper was silent, and a >400-character comment between attribute and definition produced a false report. | Fixed in `tools/extract-abi.py`: both directions, with a shared attribute-to-`fn` resolver that skips attributes/comments/whitespace; the mutation is now reported (`invented exported symbol`) and `--all` is unchanged (17 done / 10 WIP / 0 failing).  Recorded as PORTING.md P-77. |
| 5 | **`lisp.rs`'s `use subrs::*;` was a duplicate no-op** (B-M3): `xtask gen-globals` already ends `globals.rs` with that line, and `include!` puts it in the same scope. | Fixed: the line is gone, the comment now says where the glob comes from, and the ledger bullet was corrected. |
| 6 | **The `store_symval_forwarding` exemption omitted `case`** (B-D1), so its green light came from the port's `'case` label by accident. | Fixed: `switch,case,default,for_each_live_buffer,tail,buffer`. |
| 7 | **The five exported helpers were undeclared in the mirror** (B-D3) - consistent with the on-demand convention, but nothing tracked them. | Fixed properly: declared in `lisp.rs`'s `other_units` with lisp.h line references.  The ABI probe then caught two type errors the declaration exposed (`lispfwd` is `const struct Lisp_Fwd *`; `kboard_for_bindings` returns `KBOARD *`, unnameable in lisp.rs) - now `*const Lisp_Fwd` and a `/// C:`-annotated `*mut c_void`. |
| 8 | **`data.rs`'s `eassert!` bodies were compiled by nothing** (B-M1): the checking pass covers finished units only, and data.c is WIP. | Fixed as evidence, not as a rule change: the unit was compiled by hand with `--cfg enable_checking` (see below); recorded here and in the ledger. |

## Independent confirmations (no defect found)

Both reviewers verified, against C and the mirrors: the `Kboard_Obj` byte-offset
arithmetic; the `buffer_defaults` range test (`addr_of_mut!` + `.add (1)` + `.offset_from`
are C's pointer comparisons and `char *` subtraction); the hand-expanded
`FOR_EACH_LIVE_BUFFER` loop (identical to `FOR_EACH_ALIST_VALUE`, lisp.h 5908-5911,
including the shadowing of the `buf` parameter, which the arm never reads again); the
`FWDPRED_*` values (0..7, matching lisp.h 3099-3108) and the fall-through to
`emacs_abort`; `wrong_choice`'s post-increment order and `SAFE_ALLOCA_LISP`'s element
count; `!NILP (x) as c_int`'s precedence; the `redirect` bitfield (2 bits at bit 1,
no value lost); `symbol-value`'s metadata and docstring byte for byte; `nm data.o`
(`T _kboard_for_bindings`, `_do_symval_forwarding`, `_wrong_choice`,
`_swap_in_global_binding`, `_find_symbol_value`, `_Fsymbol_value`; the five `static`
helpers stay private); and that `use subrs::*;` cannot shadow a unit's own `DEFUN!`
(casetab.rs's `Fcase_table_p` resolves to its own object: `T _Fcase_table_p`, no `U _Fcase_table_p`).

## What the gate still cannot see (recorded, not fixed)

Reviewer B's mutation experiment also showed that the body comparison is a token-set
test: swapping `do_symval_forwarding`'s `Lisp_Fwd_Int` and `Lisp_Fwd_Bool` arms, changing
`>` to `>=` in the `buffer_defaults` range test, or `idx <= 0` to `idx < 0`, are all
silent.  Those three spots are exactly this segment's behavioural content, so they rest on
the two human readings recorded above (and on the ERT/differential plan of
docs/port-master-plan.md §4, which is not runnable until the ladder exists).

## Evidence after the fixes

- `cargo xtask build-tu src/data.c` -> built.
- `rustc --edition 2024 --crate-type lib --emit obj --cfg enable_checking ... _root.rs`
  -> built (`/tmp/data.checking.o`): the four new `eassert!` bodies compile.
- `extract-abi.py --check data.c` -> 312 items, all "missing" except the partial file's
  line-count drift; `--check lisp.h` -> OK; `--all` -> 17 done / 10 WIP / 0 failing;
  the new `no_mangle` rule reports nothing on the tree and catches its mutation.
- `xtask gate-fast` -> 0; `xtask gate-probes` -> 0 (inline 6081, accessor 11, write 4,
  va 6, syshdrs 30, gmp-abi 159, lisp-abi 89 declarations, dead arms 5 units).
