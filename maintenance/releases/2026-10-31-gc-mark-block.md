# P445: retain a validated cons-parent proof during marking

The opt-in collector reuses its most recent successful parent `heap_cell`
validation only on the owner, in the same nonzero active token and Marking
phase, before any relevant storage retirement. A hit checks the dump predicate,
exact alignment and the saved allocated prefix; a current head also clamps to
its smaller current limit. Head changes never widen a hit. A miss uses the
unchanged helper and may establish a new, freshly validated proof.

Reset, eligible prepare/sweep/commit/abort and storage retirement invalidate
the proof. Pointer/storage hooks now clear it before their registry, cons-type
and Sweeping exclusions. A foreign-owner latch disables later reuse without
accessing owner-only state; this does not support concurrent foreign Lisp
mutation. A location proof is not a free-list liveness check.

Actual marking, roots/order, child/car-before-cdr decisions, dirty writes,
collection policy and public layouts are unchanged. This bounded source and
functional delivery is **not a speedup or trial App replacement**. G1/G2/G4
remain 3/7, with G3/G5/G6/G7 open. See the [bound receipt](2026-10-31-gc-mark-block-results.json).

## Real products and affected integration

Separate normal/checking terminal collector+observer products build in
720.676 seconds. Native and test hooks are disabled. Only the collector differs
from the P444 production cohort. The 7,447 frozen inputs include unrelated WIP,
so this is not a clean HEAD build. The historical build pending flag remains.

Integration passes in 192.553 seconds: **530 new processes and eight verified
P444 fallback receipts**, 538 total. Existing processes are 480 candidate and
ten reference, all 490 passing in the original raw aggregate. Forty-eight ordered
registration comparisons, sixteen reference-language lists, 153 language
observations, four fallback pairs and 32 phase-contract processes pass.
Public graphs, module errors, post-latch zero registries and deltas agree;
pre-latch heap-length equality is not a predicate. Inputs/products and the App
are unchanged under the drivers' checks; App before/after identities are not
embedded in the functional/attribution JSON for independent recomputation.

## Protections and descriptive observations

The existing extraction tool retains four checks and adds eight: **12/12** pass
in 0.648 seconds, after real integration. They cover exact lookup omission,
token/phase/owner gates, saved head limits, original fallback, lifecycle/guard
boundaries, retirement before early returns and foreign-latch bypass.
State, numeric code, hooks, block and macros are extracted from production.
Registry/index/probe capacities are 8/16/8; tree, tags, dump/owner/environment,
live totals and unwind registration are substitutes. Same-address reuse is
simulated in tree metadata. These are not allocator, VM, threading or ABI proof.
Original unsafe-function compiler warnings remain; compilation has no errors.

Twenty-four prior/candidate normal-terminal processes execute six fixed cases
in interpreted/bytecode modes over three rotated rounds: 144 executions,
1,512 cycles (966 Full, 546 Young), 272 interval aggregates and 48 workload
groups, in 11.624 seconds. Some cases have no GC cycle. Load/core 0.6255-0.6509
fails the quiet policy. Intervals include recursive marking and are inclusive
wall time, not exclusive CPU or hardware pauses. Performance and compatibility
acceptance remain false; fewer source lookups alone establish no gain.

## Review and retention

The independent partner reviewed source, raw integration, generated protection
and descriptive evidence without rerunning products. Controller times and
peaks remain primary-side receipts, explicitly not independently reviewed.
No original test, assertion, guard or ID changes. P444's original failed
extractor and attribution preflight remain unchanged.

CLN181 retires consumed cargo/objects, 554 new-process state groups, successful
generated protection source/binary, the consumed owner archive and three
CLN180 helpers. Original results/logs/observations, 7,447 frozen inputs, current
and prior products/resources and the trial App remain. Unique active drivers
remain for the next affected integration and qualified comparison, until those
consumers are replaced. See the [cleanup report](../cleanup/reports/2026-10-31-181-consumed-mark-block-state.md).
