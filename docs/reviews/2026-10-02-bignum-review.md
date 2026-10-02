# Adversarial review — `port(bignum.c)` (frozen commit `dceb14f`, round 6)

Two adversarial reviewers worked in independent context windows over `dceb14f`: reviewer A
(semantics) on a `git archive` copy under `/tmp/bignum-rev/` and reviewer B (ABI/gates/
evidence) under `/tmp/rev-b/`.  Both built real differentials — A linked the *frozen*
`emacs/src/bignum.c` and the *frozen* `femacs/src/bignum.rs` against the reference's own
`lib/mini-gmp.c` plus one shared glue object (278 assertions, byte-identical output),
B generated C harnesses out of the mirrors and type-checked them against the reference
headers.  Neither modified the repository (`git -C emacs status --short` empty before and
after both).

## Verdicts

- **Reviewer A: the statement and data layer holds; the three size-limited wrappers do
  not.**  Every group except the multipliers is byte-identical: `mpz_to_intmax`/
  `mpz_to_uintmax` (23 values, including the `-2^63 → INTMAX_MIN` edge and the
  `UINTMAX_MAX <= ULONG_MAX` short-circuit), the two slow setters, `make_integer_mpz`,
  `get_random_bignum` (4 limits × 3 seeds, exercising the `edgy`/`break` retry),
  `mpz_get_d_rounded`, the string paths and the `check_*` family.  A also re-ran the port
  with `-C overflow-checks=on` and found no arithmetic panic, and confirmed the line-count
  compaction drops nothing.
- **Reviewer B: every headline number reproduces; the gate story had one P0 and several
  holes.**  All 603→633 / 33 / 26 / 211/7 / 11-4-0 / gate numbers checked out, the six
  declarations the unit calls are ABI-exact against the reference headers (with five
  rejected positive controls), the `mpz[5]` shadowing works in a real crate-root TU, and
  `nm -g` matches C's 26 exported symbols exactly.
- **Both reviewers found the same P0 independently** (B-1 / A-1), which is the strongest
  evidence in this round.

## Findings and dispositions

| # | Finding | Disposition |
| --- | --- | --- |
| B-1 = A-1 (P0) | `NLIMBS_LIMIT` computed to **0**, not 2147483647: C's inner `min (PTRDIFF_MAX, SIZE_MAX)` is evaluated in `size_t`, so `SIZE_MAX as isize` is -1 and the bound collapsed — every size-limited wrapper (`emacs_mpz_mul`/`mul_2exp`/`pow_ui`) signalled `overflow_error` for ordinary operands, live through floatfns' `rounding_driver → rescale_for_division`. | fixed in `ff9cfe0`: the inner level is `min_const_u (PTRDIFF_MAX as usize, SIZE_MAX)`, and both bounds carry `const _: () = assert! (… == 2147483647)` (A verified the fix makes its D/E groups byte-identical to C; B verified the port's constant now matches the C compiler's). |
| A-2 (P1) | `emacs_mpz_pow_ui` narrowed `exp` to `c_int` *before* the overflow check, so exponents ≥ 2^31 escaped C's guard and were handed to `mpz_pow_ui` (a hang/OOM where C raises a Lisp range error). | fixed: the product is computed in C's common type (`(nbase as c_ulong).overflowing_mul (exp)`) and rejected when it does not fit `int`; A had verified exactly this formulation byte-identical for its F group.  *No gate could see this*: the family check counts occurrences, so the ledger records it as differential-only evidence. |
| A-3 = B-7 (P2) | `make_bignum_bits`' `integer_width < bits` is a signed compare in the port and unsigned in C (`intmax_t` vs `size_t`), so a negative `integer-width` (a settable DEFVAR_INT) behaved differently. | fixed: `(integer_width as usize) < bits`. |
| A-4 (P2) | `check_uinteger_max`'s `make_uint (hi as EMACS_INT)` narrowed a `uintmax_t` bound C keeps unsigned (`UINTMAX_MAX` became -1 → `make_fixnum (-1)` instead of a bignum; latent, no in-tree caller passes hi ≥ 2^63 today). | fixed: `make_uint` now takes `uintmax_t` (as lisp.h 3046 does) and decides in the unsigned width; `make_int` likewise takes `intmax_t` (lisp.h 3041), with the call sites cast. |
| B-2 (P2) | The `usize → ptrdiff_t` fix was incomplete (six declarations, not two: `intern_1`, `intern_c_string_1`, `fast_c_string_match_internal`, `egetenv_internal` too) and **no gate covered declaration types at all.** | fixed the four; new `tools/gen-lisp-abi-probe.py` + `spikes/lisp-abi-probe.sh` type-check `other_units`'s declarations against the reference `lisp.h` (51 checked, 11 unmapped, 15 pre-existing deviations baselined with clang's message as the reason — P-54), wired into `gate-probes`; reverting `intern_1` to `usize` fails it (verified). |
| B-3 (P2) | The six constants added to the `limits`/`stdint`/`float` mirrors were outside every gate (mutating `DBL_DIG`/`ULONG_MAX` stayed green). | fixed: `spikes/syshdrs_probe.{c,rs}` now prints them (`19 → 26` lines, identical). |
| B-4 (P2) | `shadow-probe.sh` could not fail for the P-39 leak it documents (it wrapped the mirrors in a `mod`, so lisp.rs's top level was not the crate root). | fixed: the probe includes the mirrors at the crate root; moving a declaration to lisp.rs's top level now fails it (verified). |
| B-5, B-8, B-9, A-5 (P2, records) | The shadowed `mpz` declaration is in `include/bignum.rs`'s `bignum_externs` (not mini-gmp.rs); "159 declarations this unit calls into" is 159 declared / 24 used; the lisp.h line numbers in the `ptrdiff_t` comment were 4541/4544; the ledger's "its value is pinned by the arithmetic it feeds" was false at `dceb14f`. | all corrected; the false claim is replaced by the two `const _` assertions. |
| B-6 (P2) | `__mpz_struct: Copy` is layout- and drop-neutral but re-admits `mpz_t` value copies, which C rejects. | recorded in `docs/ports/include-mini-gmp.md`. |

## Post-fix verification

- `python3 tools/extract-abi.py --all` → 11 done / 4 in progress / 0 failing.
- `xtask gate-fast` → green (10 unit builds = 5 units × both configurations, 7 probe
  programs, 211 layout assertions, 7 pending).
- `xtask gate-probes` → green: `inline 6081`, `accessor 11`, `write 4`, `va 6`,
  `syshdrs 26`, `gmp-abi 159`, `lisp-abi 51` (11 unmapped, 15 baselined), 5 units with the
  non-frozen arms.
- Sensitivity: the `[mpz_t; 6]` bound fails layout-check; a reverted `intern_1` width fails
  `lisp-abi-probe`; a `xmalloc` declaration at lisp.rs's top level fails `shadow-probe`; the
  `NLIMBS_LIMIT` value is pinned by a compile-time assertion.

## What round 6 did not cover

1. **No Emacs-level evidence** (no `src/emacs` binary exists): the P0's user-visible
   consequence is inferred from the unit-level differential plus the in-tree call sites.
2. The frozen configuration constant-folds several C arms (`FASTER_BIGNUM`'s
   `FIXNUM_OVERFLOW_P (LONG_MIN/LONG_MAX)`, the `DBL_MANT_DIG <= FIXNUM_BITS - 1` test), so
   the bit-assembly loop in `make_integer_mpz`, the limb loops in `mpz_to_int*max` and the
   `mpz_get_d` shortcut are unreachable here and were only read.
3. The 15 baselined declaration-type deviations (P-54) are known and unfixed: they are
   ABI-neutral on this target but must be cleared before the units that call them land.
4. `base.abs ()` for `base == INT_MIN` (C UB, Rust debug panic) is unreachable from the
   four in-tree callers and untested; Miri and non-darwin-arm64 configurations were out of
   scope for both reviewers.
