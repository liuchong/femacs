# P436: remembered-owner batching evaluated, not adopted

The experimental collector was changed to submit the same frozen owner prefix
through one array-marker call. It retained every root, traversal order, barrier,
weak/finalizer rule, Full/Young policy and default feature setting. This candidate
is now withdrawn: qualified reliable benefit was not established. The result
neither proves batching ineffective nor completes the overall performance goal.

## Source and functional evidence

Each frozen cohort has 7,298 inputs. Only `src/runtime_gc_collector.rs` and its
`PORTING.md` explanation differ; all other 7,296 inputs match. Both actual
normal/checking collector-observer programs were built, with major/young runtime
controls: four programs, eight candidate profiles and the installed system
Emacs reference. Executables, dumps, resources and all original fixtures remained
bound to actual identities throughout execution.

The complete affected observer/collector selection passed 576 actual ERT cases
and 64 discoveries. The subsequent two focused protections passed 16 cases and
8 discoveries across the eight profiles. These exercise 9,000-level branching
vector graphs and two waves of newly attached non-cons/cons children through
actual automatic Young and explicit Full collection. They do not prove full
Lisp compatibility, exact internal visit order, actual stack-reallocation count
or recoverable active-GC OOM. The candidate may need one additional 16-byte
continuation; the existing active-GC fatal signal behavior remains.

## Complete measurement, unqualified comparison

Two warmups plus seven measured rounds produced 603 workload processes and
81 startup processes, all successful; 469 measured workload processes emitted
2,814 samples over six workloads. Actual interpreted/bytecode representations
were checked, with the installed reference also providing a native control.
Observer off/phases/survival/all results remain separate. No rounds or slow
samples were discarded. [The bounded data](2026-10-30-gc-owner-batch-results.json)
contains all 48 direct prior/batch pairs and their seven raw round observations,
402 group summaries, source-case hashes and the full detailed-result identity.
The ignored raw result is 212,750,988 bytes, SHA-256
`36633ba04a1536bc992dcbe32e591f2dc31a098fa6335da711625a6d8f0c4a62`.

All 402 groups exceed the unchanged 0.1 load-per-core quiet threshold; their
maximum per-group loads range from 0.20279947916666666 to 0.23396809895833334.
All 48 direct comparisons are therefore ineligible for adoption, including
ones with a positive statistical interval. Busy-host samples are diagnostics.

For normal/young/observer-off bytecode GC churn, wall medians are 199.522 ms
prior and 192.848 ms batch; the seven-pair median-delta interval spans
-0.651 to 13.696 ms. With all observation enabled, the same workload's sampled
Young preparation-before-static-roots interval has medians 3.878 and 3.801 ms
(231 Young/84 Full samples each). This interval includes preparation and tracing;
it is not exclusive function CPU or a hardware stop-the-world latency bound.
Its small difference cannot locate the dominant cost. The mechanism still
visits every owner; that source fact alone does not quantify its CPU share.

## Disposition and remaining work

Independent review agrees with not adopting the candidate under the original
policy. Production is restored byte for byte to the measured prior collector
SHA-256 `0f18ba7d761d3fcf4e99bacd8bd188289e816cdca933d4c22b83d37393a1b74e`.
No default/trial App is replaced. The useful new regression fixture remains;
no unchanged test or build is repeated solely for withdrawal. The candidate is
recoverable through [CLN150](../cleanup/reports/2026-10-30-150-owner-batch-withdrawal.md),
while [CLN148](../cleanup/reports/2026-10-30-148-consumed-owner-batch-copies.md)
and [CLN149](../cleanup/reports/2026-10-30-149-consumed-owner-batch-case-state.md)
record exact consumed-state cleanup. The four experimental programs, matching
source/object inputs and raw evidence remain for final independent receipt and
source review; they are not installed products or permanent cache exemptions.

G1/G2/G4 remain the three accepted modernization items. Full G3 compatibility,
remaining concrete failures, G5/G6/G7 research acceptance, qualified performance
improvement and final current macOS delivery remain open. No threshold, error,
compatibility assertion or scope was relaxed; no remote CI was used.

The retained focused cases load after `tools/fixtures/runtime-gc.el` and select
`^rtg-owner-batch-`; they are not silently added to the original ten-case group.
For a receipt-matched collector program/dump, use the existing resource guard
and isolated environment with `FEMACS_GC_COLLECTOR=major` or `young`, then:

```sh
"$program" --dump-file="$dump" -Q --batch \
  --load tools/fixtures/runtime-gc.el \
  --load tools/fixtures/runtime-gc-owner-batch-protection.el \
  --eval '(ert-run-tests-batch-and-exit "^rtg-owner-batch-")'
```
