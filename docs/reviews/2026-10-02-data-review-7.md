# Adversarial review: `src/data.c` segment VII (data.c 2178-2412)

Scope: `make-variable-buffer-local` (2178), `make-local-variable` (2245) and
`kill-local-variable` (2350) - the three subrs that create and remove buffer-local
variables and the first users of `make_blv` - plus their three new rows in
`docs/ports/macro-body-notes.tsv` and the `UNINIT` reading they depend on.

Method: reviewer A attacked the code (ordered token/call alignment with independent
scripts, per-branch mapping, argument orders, byte-level docstring comparison against a
re-implementation of make-docfile's rule, and an independent derivation of the `UNINIT`
expansion chain); reviewer B attacked the gate/ledger surface by building 30+ mutants of
the three bodies in `/tmp` and running the gate's own `--check` on each, plus the ledger
arithmetic and the two generated artifacts. Both were read-only and neither compiled
anything.

## Reviewer A: no defect in any of the eight attack points

Verified: the `UNINIT` expansion is `= {0,}` (conf_post.h 366-371 with `GCC_LINT`
defined - `configure.ac` 1894 defines it unconditionally inside the warnings block,
`build/emacs-ref/src/config.h` line 209, and the frozen table records `UNINIT = {0,}`
itself), so the port's `Lisp_Val_Fwd { value: Qnil () }` / `false` are the same bytes, not
an invented write (`Qnil ()` is word zero: `globals.rs` builds it from `iQnil = 0`, and
`NIL_IS_ZERO`/`lisp_h_Qnil` agree); all three bodies match C statement by statement
including every argument order (`Fcons`/`bset_local_var_alist`/`Fdelq`/
`set_per_buffer_value`/`notify_variable_watchers`' four arguments), the `XBUFFER_OFFSET`
`u16 -> int` widening and the `idx`-before-use order; the parameter rebinding lands on the
returned value at function-body scope in exactly the two functions C rebinds it in (and
`make-variable-buffer-local`, which C does not rebind, is not shadowed); the port's first
C-variadic call is byte-correct (33-byte format string + NUL, same argument order) and
`AUTO_STRING!`'s 47-byte literal too; the `start:`/`goto start` loops re-read the redirect;
`kill-local-variable`'s uninitialized `blv` is provably assigned on the only path that
reaches it (every other arm returns, continues or diverges through `emacs_abort () -> !`);
the packed-bit writes select the same bits and keep C's order (redirect before
`SET_SYMBOL_BLV`, whose `eassume` requires it); and all three docstrings are byte-equal
(845/710/143 bytes, including `\\=` -> `\=` and `\(` -> `(`).

| # | Finding (info only) | Disposition |
| --- | --- | --- |
| A-I1 | the `Fmake_local_variable` row said "C's other `0`s (`idx > 0`, `idx < 0`)" but `idx < 0` is comment text, which the tokenizer blanks | reworded (only `idx > 0` is a token) |
| A-I2 | the `Lisp_Symbol`/`Lisp_Buffer_Local_Value` bitfield layouts rest on the round-3 clang+gcc probe and ABI reasoning, unlike `buffer_text`/`itree_node`, which have rerunnable `spikes/write_probe.*` artifacts | recorded in the ledger as the cheapest follow-up of the three gate gaps this round found |
| A-I3 | `eassert! (idx != 0)` strings a different message than C's `eassert (idx)` under `enable_checking` | accepted: `ENABLE_CHECKING` is undefined in this configuration, so both are no-ops; the `!= 0` form is the port's existing convention |

## Reviewer B: three stale artifacts/text errors (fixed) and one residual class (recorded)

B confirmed the three notes rows are *exactly* minimal (`C - Rust - allowance` and
`allowance - C` both empty for all three), that the ledger's arithmetic is right in all
five places it could be checked (71 of 116 live subrs, 37/60 functions, 45 remaining
subrs, 202 difference lines, `--all` = 17/10/0), that `docs/port-inventory.md` needs no
regeneration, and that `tools/extract-abi.py` is untouched.

| # | Finding | Disposition |
| --- | --- | --- |
| B-F8 (P2) | `docs/port-map.md` was stale - `src/data.c` said 1,637 target lines (should be 2,122) and `src/lisp.h` 3,870 (should be 3,871) - and CI's "Generated artifacts are fresh" step regenerates that file with `git diff --exit-code`, so it would have failed on push.  The data.rs half was left by the previous commit | fixed: `tools/gen-port-map.py` re-run (exactly those two lines) |
| B-F10 (P3) | three text errors: `idx < 0` (comment, not a token), `lisp.rs 1970` for `BLV_LOCAL_IF_SET` (it is 1971 now - the round-6 edit of the `lisp.rs` 1069 comment added a line, shifting everything below it, and the `set_internal` row's `1970-1971` too), and `local-variable-if-set-p` at `data.c 2467` (the `DEFUN` is at 2465) | all fixed |
| B-F11 (P3) | the ledger said "the preprocessor eats the macro name, so ... `uninit` in its notes row" - backwards: the gate compares the *unexpanded* text, which is exactly why the token is compared and needs the row | reworded |
| B-F9 (P3) | the ledger's generated header says 120 `DEFUN` boxes while the generator now emits 116 (the condition table drops the four dead ones) | recorded in the ledger: the 120 is deliberate (P-76) and a regeneration would lose the four dead lines |
| B-F1/F2/F3/F4/F5/F6/F7 (P1/P2) | the exemptions still hide a class, not an instance: the initializer *values* the `uninit` exemption stands for (`Qnil ()`/`false` -> `Qunbound ()`/`Qt ()`, or deleting the initializer), `forwarded`'s polarity in four places, deleting a whole packed-bit write (its `SYMBOL_LOCALIZED` is satisfied by the `match` arm head and its `redirect` by the dispatch read - so family-ising `redirect` would *not* catch that, contrary to B's own suggestion), swapping the two `SYMBOL_*_MASK/SHIFT` pairs (names with no C counterpart), the family's lack of polarity checking (`\| BLV_LOCAL_IF_SET` -> `& ! BLV_LOCAL_IF_SET`), `continue` -> `break` in the alias arm, `Fdelq`'s first argument, `if idx > 0` -> `idx < 0`, `SET_PER_BUFFER_VALUE_P`'s third argument (the `make-local-variable` half was unrecorded), a lost `!`, `&&` -> `\|\|`, and `CALLN!` -> `calln!` | recorded, with B's mutation list, in the ledger's Decisions and in P-78's residuals; the two mechanisms that would close most of it (`docs/ports/init-values.tsv` for initializer text, `docs/ports/bitfields.tsv` for the C-bitfield/constant pairs) are named there rather than invented mid-segment |

## Gates run for this round

`cargo build-tu src/data.c` (green; 15 warnings, all of the pre-existing classes);
`extract-abi.py --check` (all three new bodies clean - the only remaining line is the
51.8% line-count drift, C 4400 vs Rust 2122); `--all` (27 pairs: 17 done / 10 WIP / 0
failing); `gate-fast` exit 0 with the frozen configuration rebuilt from scratch this round
(`build/emacs-ref/src/config.h`: 3,911 lines, 342 `#define`, `GCC_LINT 1` at 209) and
`layout-check passed`; `gate-probes` exit 0; and the `enable_checking` build of all six
materialized roots (bignum 47,000 / casetab 47,688 / data 252,624 / doprnt 37,808 /
floatfns 52,024 / sort 132,616 bytes) - which is also this segment's only compile
credential, since both reviewers were barred from building.  `git -C emacs status --short`
is empty.

## Note for the next round

Editing a *mirror* file shifts every line reference below the edit, and the ledgers are
full of `lisp.rs NNNN` pointers: round 6's two-line comment in `lisp.rs` moved
`BLV_LOCAL_IF_SET` from 1970 to 1971 and left two stale citations behind (B-F10).  When a
mirror's line numbers move, grep the ledgers and `macro-body-notes.tsv` for the old number
in the same commit.
