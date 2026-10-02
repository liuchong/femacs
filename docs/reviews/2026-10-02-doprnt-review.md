# Adversarial review — `port(doprnt.c)` (frozen commit `ec20c48`, round 4)

Two adversarial reviewers worked in independent context windows over the frozen commit
`ec20c48` (`femacs/src/doprnt.rs`, 609 lines; `femacs/src/include/lisp.rs`'s variadic
declarations; `femacs/lib/include/float.in.rs`; `tools/extract-abi.py`'s
`strip_char_literals`; the `docs/ports/*` ledgers of the same commit).  They were told the
artifact is wrong and asked to prove it.  Neither edited the repository: reviewer A worked
in `/tmp/review-da/`, reviewer D in `/tmp/review-db/`, and both left the worktree clean at
`ec20c48`.

Both reviewers agree on the outcome: **the artifact is not falsified** — reviewer A put
`doprnt.c` and `doprnt.rs` in one model (real `lisp.h`/`character.h` plus stubs for
`strwidth`/`char_string`/`error`/`xmalloc`) and the main phase agreed byte for byte on 101
lines with 12 error phases identical, and reviewer D could not falsify any ABI claim: the
four cross-language `va_list` directions, `error`/`verror` variadics and 14
`sprintf ("%*.*X")` shapes were byte-identical between C and Rust.  Everything that
survived is in the gates, the probes and the records — which is what this file disposes of.

Scope.  Round 4 covered the commit's own claims (P-04, variadics, `<float.h>`, the
character-literal scanner) and the gates that certify them.  Known-open items
(P-05…P-41) were not to be re-reported.

## Findings and dispositions

| # | Finding | Disposition |
| --- | --- | --- |
| D-F1 (P1) | `ec20c48` broke three probes (`inline`/`accessor`/`write`) and no gate noticed: the scripts `cd` to the repository root and run a bare `rustc`, which has no `rust-toolchain.toml` there, so the *stable* toolchain resolved and `lisp.rs`'s `VaList` parameter became `E0658`. | fixed: the three scripts compile the Rust side from `femacs/` (like `va-probe.sh` always did); `gate-fast` now compiles every probe program's Rust side; `xtask gate-probes` runs the four C differentials and fails (not skips) without `build/emacs-ref`.  PORTING.md P-47. |
| D-F2 (P1) | The `memcpy`/`mempcpy` token note was an *exemption*, and the token check is one-way containment: deleting all three of `doprnt`'s output copies kept `--check` **and** `build-tu` green. | fixed by replacing the exemption with a token *family* (`docs/ports/token-aliases.tsv`) plus an occurrence check (Rust count >= C count).  Mutation matrix below: M1/M2/M3/M8 now all FAIL.  PORTING.md P-44. |
| D-F3 (P2) | Expression-valued constants were not compared at all: `DBL_MAX_10_EXP + 50` -> `+ 500` passed every gate. | fixed: non-literal constant values are compared by their tokens (`constant … does not mention …`); the configuration-duplicate guard now runs first.  D9 now FAILs. |
| D-F4 (P2) | The type each `va_arg` reads had no guard: `%u`'s `c_uint` -> `c_int` compiled and passed everything (the value flows into a variadic `sprintf`). | fixed: `docs/ports/va-arg-types.tsv` + a multiset comparison of the reads; an unrecorded type is an error.  MV1 now FAILs.  PORTING.md P-45. |
| D-F5 (P2) | Four places recorded "on this target `va_list` is a struct, so a by-value pass copies it".  Measured: this target's `va_list` is `char *` (`sizeof == 8`, `__builtin_types_compatible_p (va_list, char *) == 1`).  The *conclusion* (clone at the call site) is right and reviewer D's cross-language probe confirms it; the model was wrong. | fixed in `docs/spikes/2026-10-02-variadics.md`, `spikes/va_probe.{c,rs}`, `docs/ports/doprnt.md`.  The x86-64 warning (array-typed `va_list` shares state across a by-value pass) is kept and is the one that matters. |
| D-F6 (P2) + A-F1 (P2) | `rust-toolchain.toml`'s comment and the spike document's table both claimed `nightly-2026-09-14` reports "C-variadic functions are unstable" and that stable reports `E0554`.  Measured by both reviewers: stable 1.98.1 reports `E0658`; `nightly-2026-09-14` **compiles** (and its binary matches C on all 6 probe lines).  Also `spikes/miri-probe.sh` used rolling `+nightly` (older than the pin) and exited 0 when miri was missing. | fixed: both records corrected (the pin stays `nightly-2026-10-01`, on the honest ground that the round's gate evidence was measured with it); `miri-probe.sh` now reads the pin from `rust-toolchain.toml`, requires miri for it, and exits 1 with the install command otherwise.  miri + rust-src installed for the pin; `miri-probe: clean (nightly-2026-10-01)`. |
| D-F7 (P2) | Two of `va-probe`'s six lines read an `int` where the next argument is a `double` (UB in C); both sides read the same deterministic garbage, so those lines could not discriminate a correct cursor. | fixed: `spikes/va_probe.{c,rs}` rewritten so every read reads a real argument; the six lines are now all evidence (see the spike document). |
| D-F8 (P2) | `lisp.h:4294`'s `static_assert (FLT_RADIX == 2 || FLT_RADIX == 16)` was not mirrored. | fixed: `static_assert! (FLT_RADIX == 2 || FLT_RADIX == 16)` in `lisp.rs` at the C location. |
| D-F9, D-F10 (P2) | `IEEE_FLOATING_POINT`'s operands were reordered (`FLT_MAX_EXP` before `FLT_MIN_EXP`); `lisp.rs` included `<stdint.h>` before `<float.h>` where C includes `<float.h>` first. | fixed both (values were equal; the ordering is now C's). |
| D-F11 (P2) | `strip_char_literals`'s docstring said the old regex hid 7 KB of `doprnt`'s body "from the token check"; measured, the 7 KB were the *Rust* side and the effect was a false *failure*, not a blind spot. | docstring corrected; the quantified numbers (old scanner ate 50.4% of the Rust text, new one 0.7%) are in PORTING.md P-38's residual, together with the detector's known false-swallow (`&'x abc'` loses `abc`) and the measurement that it changes no token on the seven ported pairs. |
| D-F12 (P2) | One repository, three answers to `unsafe_op_in_unsafe_fn` (`doprnt.rs` allowed it, `sort.rs`/`casetab.rs` did not: 438 vs 6 warnings in one revision). | fixed: one allow list for every mirror.  PORTING.md P-43. |
| A-F3 (P2) | `spikes/shim/stdckdint.h` **shadowed** the real system `<stdckdint.h>` (`HAVE_STDCKDINT_H == 1` in the frozen configuration) with no-op `ckd_add`/`ckd_sub`/`ckd_mul`, so any C reference built the way the probes build theirs silently loses the overflow checks — reviewer A measured `%99999999999999999999d` wrapping into 63 spaces instead of raising "Format width or precision too large".  `src/lisp.h:27` includes the header and 48 `src/*.c` files use `ckd_*`. | fixed: the two shadowing stand-ins (`stdckdint.h`, `stdcountof.h` — both present here) are deleted, the shim keeps only `stdbit.h` (the one header that really is missing), every probe passes it with `-idirafter` so a system/gnulib header always wins, and `spikes/shim/README.md` records the rule.  PORTING.md P-46. |
| A-F2 (P2) | The spike document's counts were a tree-level number presented as a file property (`49` `va_list` uses in `doprnt.c`; it is 7 there and 49 in `src/`), and `va_end` was counted as 2 instead of 3. | fixed in the spike document. |
| A-F4 (P2) | `docs/ports/doprnt.md` presented `--check -> OK` as *the* gate and listed it as the first evidence, although that gate has zero coverage of replacement-shaped mutations (reviewer A's matrix: `tail = 2 -> 1`, inverted `if`, changed string literal, reordered declarations all `OK`). | fixed: the ledger now states which gates run and what each tier can and cannot see, with the mutation matrix. |
| A-F5 (P2) | `eassert (0 <= tem)` is compiled out on *both* sides (`ENABLE_CHECKING` is undefined in the frozen configuration); the "with checking" build only proves it compiles, not that it is evaluated. | recorded in `docs/ports/doprnt.md` as the honest boundary of that evidence. |
| A-F6 (nit) | Three file-scope constants the port invents (`SIZE_BOUND_EXTRA` lifted out of a function-local enum, `TBUF_LEN` for C's `char tembuf[SIZE_BOUND_EXTRA + 50]`, `PRIdMAX`) were not named in the ledger. | named in the ledger; P-48 tracks the general fix (a "port-invented constant <-> C expression" table with a token check), needed before batch B adds more of them. |

## The gate changes this round, measured

`python3 tools/extract-abi.py --all` is green again after the three new rules
(`7 done, 4 in progress, 0 failing`), and the rules were tested by mutation, not asserted:

| Mutation (on `femacs/src/doprnt.rs`) | Before this round | Now |
| --- | --- | --- |
| M1 `rpl_mempcpy` -> `strcpy` | OK | **FAIL** (family `memcpy`) |
| M2 delete one output copy | OK | **FAIL** (family count 3 < 4) |
| M3 delete two output copies | OK | **FAIL** |
| M8 delete all three output copies | OK | **FAIL** |
| MV1 `%u`: `next_arg::<c_uint>` -> `c_int` | OK | **FAIL** (va_arg multiset) |
| D9 `SIZE_BOUND_EXTRA = DBL_MAX_10_EXP + 50` -> `+ 500` | OK | **FAIL** (constant tokens) |
| D5 `TBUF_LEN = (SIZE_BOUND_EXTRA + 50)` -> `+ 500` | OK | OK — **still open, P-48** (`TBUF_LEN` is port-invented and `50` is a recorded exemption) |
| control (unchanged file) | OK | OK |

Gate runs on this round's checked tree: `gate-fast` green (inventory, `cargo check`, six
translation-unit builds including `--cfg enable_checking`, seven probe programs compiled,
`fam`/`shadow`/`macro`, 200 layout assertions, layout-check passed);
`gate-probes` green (`inline-probe: 6081 lines identical`, `accessor-probe: 11`,
`write-probe: 4`, `va-probe: 6`); `spikes/miri-probe.sh` clean on the pinned toolchain.

## What round 4 did *not* cover

1. Only `aarch64-apple-darwin`: this target's `va_list` is `char *` and arm64 passes every
   variadic argument on the stack, so the x86-64 register-save-area case (the one P-04
   itself calls target-dependent) is untested; same for 32-bit targets.
2. No Lisp/ERT evidence: `gate-full` is not implemented, and `doprnt`'s formatting
   algorithm (wide `%c`, `strwidth`, `text-quoting-style`, truncation) was only compared
   inside the reviewers' C model, not against a real Emacs.
3. Reviewer A's differential lives in a model (one-column `strwidth`, a simplified
   `xpalloc`), so it is C-vs-Rust equivalence in that model, not equivalence with Emacs.
4. `exprinf`'s `#[cfg(feature = "x_windows")]` branch is never compiled or run under the
   frozen configuration.
5. The mutation matrix is textual: only some mutants were compiled (`build-tu`), and no
   run had ASan/valgrind, so an out-of-bounds write that happens not to change the output
   (reviewer A's `MI`) would still be invisible.
6. Not reviewed: the rest of `lisp.rs`/`globals.rs`, `LIFETIMES.tsv`, the layout baseline,
   `PORTING.md`'s other items (round 3 covered those), and the other ten ported pairs
   (only the `--all` summary was used).

Evidence: reviewer A kept its model, corpus and mutation matrix under `/tmp/review-da/`
(its own README lists every reproduction command); reviewer D kept its probes, raw outputs
and per-item reproduction recipes under `/tmp/review-db/evidence/`.  Both `/tmp` trees are
scratch: the durable record is this file plus the gate commands above.
