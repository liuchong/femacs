# Adversarial review — `port(floatfns.c, bignum.h)` (frozen commit `373da02`, round 5)

Two adversarial reviewers worked in independent context windows over the frozen commit
`373da02`.  Both were told the artifact is wrong and asked to prove it; neither edited the
repository (reviewer A worked from `git archive 373da02` under `/tmp/revA/`, reviewer B from
a copy under `/tmp/ferev/`).  Reviewer A's session overlapped with the applier fixing
reviewer B's findings, so it pinned every experiment to the frozen blobs (`md5` of
`femacs/src/floatfns.rs` = `23a1018917e32e06f2fa11443a2e4bce`) — all its findings are stated
against `373da02`, and every one of them is disposed of below.

## Verdicts

- **Reviewer A (fidelity/semantics): the ported logic is not falsified.**  Three
  independent C-vs-Rust differentials — 1,331 + 6,033 + 10,565 calls — ran the *unmodified*
  `emacs/src/floatfns.c` against the *unmodified* frozen `floatfns.rs` (only a `main`
  appended), both linked against the reference's own `lib/mini-gmp.c`: every one of the 25
  subrs, `fmod_float`, `rounding_driver`, `rounddiv_q`, `rescale_for_division`,
  `double_integer_scale`, all four `*2` helpers, every Lisp-visible error path, and the
  bignum-zero-divisor abort came out byte-identical.  A real `make-docfile` run confirmed
  all 25 docstrings/min/max and the `defsubr` order byte for byte.  The reviewer's own
  mutants (three) are caught, one by a pure value difference, so the probes are not vacuous.
- **Reviewer B (ABI/gates/toolchain): the ABI claims hold; three gate holes are real.**
  All 22 `math.rs` libm declarations and all 27 `bignum.rs` declarations are type-identical
  to clang's headers (generated `static RET (*p_NAME)(ARGS) = NAME;` harnesses with negative
  controls), a 426-line runtime differential over NaN/−NaN/±inf/±0/subnormal is identical,
  a mixed probe against real `mini-gmp.o` agrees, the measured constants and `config.h` line
  numbers are right, and the default build really does select the frozen arms (`nm` shows
  `_rint`/`_trunc`/`_log2` imported).
- **What both found is the same shape**: the artifact is right, the *evidence layer* was
  weaker than the commit claimed, and one linkage rule was violated.

## Findings and dispositions

| # | Finding | Disposition |
| --- | --- | --- |
| A-F1 (P1) | `extract_float`, `fmod_float`, `double_integer_scale` are non-`static` C definitions (lisp.h declares them at 690/4296/4300, and `data.c`/`timefns.c`/`indent.c`/`dispnew.c`/`dbusbind.c` call them) but the port defines them as plain `pub unsafe fn`, so the object has no `_extract_float`/`_fmod_float`/`_double_integer_scale` symbol and the next unit cannot link (proved with `nm` plus a `clang -r` partial link). | fixed: all three are `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (the dead-arm `trunc` too, and `syms_of_floatfns`, which the new gate then caught), and `lisp.rs` declares the three where lisp.h does (P-39's shape).  Verified: `nm -g build/femacs/floatfns.o` now shows `_extract_float`, `_fmod_float`, `_double_integer_scale`, `_syms_of_floatfns`, `_Fabs`.  New gate rule + P-52. |
| B-B1 (P1) | `extern mpz_t mpz[5]`'s bound was asserted by *no* gate: `[mpz_t; 6]` passed `gate-fast`, `extract-abi`, `build-tu` and `spikes/run.sh`, and P-49's claim that the layout gate pinned it was false (the key was pending because its root is an object, not a type). | fixed: `xtask layout-asserts` resolves a key whose root is a file-scope object through the mirror's own declaration, so `sizeof mpz` asserts `size_of::<[mpz_t; 5]>() == 80`.  Verified: the `[mpz_t; 6]` mutation now fails the layout gate (E0080); assertions 200 → 211, pending stays 7. |
| B-B2 (P2) | The new `ckd_add`/`ckd_mul`/`ckd_sub` families compared counts only: `mpz_set_uintmax`'s `v.overflowing_add (0)` → `(1)` passed (the body still mentions a `0` in `FASTER_BIGNUM != 0`), and deleting `SAFE_ALLOCA_LISP!`'s overflow guard passed because that macro was wholly exempted. | fixed: the family rule now also compares the *numbers inside the call* (subscripts ignored, so `sizeof s1->keys[0]` in sort.c is not a "0" argument), and `SAFE_ALLOCA_LISP` lost its `*` exemption (it has its own row for `false`, which is C's `while (false)`).  Verified: both mutations now FAIL, the original regression still FAILs, and the control is OK. |
| A-F2 = B-B3 (P2) | The `no_signbit` dead arm is C's `#ifndef signbit` fallback spelled with Rust's bitwise `!` on `f64` — it cannot compile (E0600) — and it had no `const _` assertion, contradicting P-51's blanket claim (`HAVE_SIGNBIT` did not exist). | fixed: the body is C's `($x) == 0.0`, `HAVE_SIGNBIT` joined the frozen configuration (measured with `clang -E` on `#ifdef signbit`, recorded as *not* a config.h macro), and the arm has its `const _` assertion.  Verified: `xtask dead-arms` (new) flips the non-frozen constants and compiles every finished unit with the non-frozen features on — 4 units compile, including this arm. |
| B-B4 (P2) | `femacs/lib/include/math.rs` (new) and `float.in.rs` had no gate at all: a wrong `HUGE_VAL` or an inverted `signbit!` changed nothing anywhere. | fixed: `spikes/syshdrs-probe.sh` builds the same constants and macro verdicts from clang's headers and from the mirrors and diffs them (`19 lines identical`), and it runs inside `gate-probes`. |
| B-B5 (P2) | The ratchet `docs/baseline/layout/counts.tsv` had its `pending` ceiling raised 7 → 8 — cancelling exactly the key B1 needed — and a missing or unparsable ratchet silently disabled the check. | fixed: pending is back to 7 (asserted 211), and a missing/malformed ratchet is now a hard failure (both rows required, parse errors reported). |
| A-F3 = B-B6 (P2, records) | Wrong numbers in the frozen records: P-49 said "15 `mpz_*`" (16), the commit said 666 lines (668), "six inline accessors" (seven), the baseline "208 → 218 keys" (207 → 218), and P-49 claimed `sizeof mpz` was gated (it was not). | fixed in `PORTING.md`, `docs/ports/floatfns.md`, `docs/ports/include-bignum.md`; the `sizeof mpz` claim is now true (B1). |

## Post-fix verification (this round's gates)

- `python3 tools/extract-abi.py --all` → `9 done, 4 in progress, 0 failing`, now including
  the new export rule and the tightened family rule.
- `xtask gate-fast` → green: `cargo check`, 8 translation-unit builds (4 units × both
  configurations), 7 probe programs, 211 layout assertions, 7 pending, layout-check passed.
- `xtask gate-probes` → green: `inline-probe 6081`, `accessor-probe 11`, `write-probe 4`,
  `va-probe 6`, `syshdrs-probe 19` lines identical, and `dead-arms` compiling 4 units with
  the non-frozen arms selected.
- Mutation matrix: `[mpz_t; 6]` fails layout-check; `overflowing_add (0)` → `(1)` and the
  deleted `SAFE_ALLOCA_LISP` guard fail `--check`; a missing `#[unsafe(no_mangle)]` fails
  `--check`; the C symbols are present in `nm`.

## What round 5 did not cover

1. **No Emacs-level evidence.** `build/emacs-ref` has configure output but no `src/emacs`
   binary, so nothing ran through Lisp (`(abs -nan)`, `(round 2.5 2)`,
   `(documentation 'round)`, GC, dump).  Reviewer A's glue for the not-yet-ported units was
   copied or marked, so `Fexpt`'s integer-exponentiation *result* and the exact
   `Fcons`/`make_float` objects are not covered; that is P-30 territory.
2. **No whole-program link exists**, so A-F1 was proven by symbol tables and a `clang -r`
   partial link rather than a full link (it is latent until `timefns.c`/`data.c` land).
3. Reviewer B did not audit the other 200+ layout keys, `handler`/`window`, Miri, or the
   non-darwin-arm64 configurations; reviewer A exercised only the frozen arms.
4. `rounding_driver`'s argument-evaluation order (C unspecified vs Rust left-to-right) is
   argued from the two temporaries being distinct, not measured.
