# Collector preparation and remembered-root phase attribution

P442 adds three opt-in observer intervals to distinguish work previously mixed
inside the pre-static-root gap: phase22 for collector preparation, phase23 for
the frozen non-cons registry traversal, and phase24 for dirty/raw-tainted cons
children. The latter two include recursive marking. No root, traversal order,
barrier, marking, collection policy, layout or default changes.

Phase22 precedes instant classification21; analysis joins by cycle identity.
Phases23/24 exist only for actual Young traversal. These are inclusive elapsed
intervals nested in input-blocked phase3, not exclusive CPU work, hardware
stop-the-world pauses or application response time. Remaining gaps stay visible.
Disabled recording in an observer-enabled binary is not assumed free.

## Build and interruption

Normal/checking real terminal collector-observer products were compiled from
7,360 frozen inputs, including existing WIP. Only `src/alloc.rs` and
`src/runtime_gc_collector.rs` differ from the retained P441 production inputs.
Four candidate profiles select major/young, native disabled and no test hooks.

Both compiler subprocesses exited zero: build driver14.574 seconds and actual
collector build685.203 seconds. The outer controller subsequently exited125
on `monitor_error`, `inspect_owned`/`sample`, errno1; its tree was fully reaped.
That original result remains incomplete and is not reported as controller pass.

A separate file-only finalization verified all16,902 protected file identities,
completed product receipts, source export and unchanged prior products/App.
After confirming the original owner had exited, it restored only that owner's
flags and verified a fresh preservation journal was released. This took19.066
seconds and did not rerun compilers or products. The finalized build receipt
links the original incomplete result and the separate finalization outcome.

## Actual integration and protections

Full affected integration passed in196.155 seconds:490 fresh existing-process
receipts (480 candidate and10 reference), eight fresh fallback and32 fresh
phase-contract processes. Together with eight verified reused P441 fallback
receipts, this is538 receipts, **530 new processes**, not538 new processes.
All48 ordered registration comparisons,16 reference-language ID comparisons,
153 language observations,17 shared and17 language registrations and four
fallback pairs passed. The unmodified raw existing aggregate remains true.

The phase fixture verifies actual major/young products in off/phases/survival/all
modes. Off/survival run a collection before requiring no phase records; Full
has no23/24, while Young has ordered sibling23/24 before static roots. Phase22
ends before classification21 inside phase3. Dropped, incomplete, clock, depth
and invalid records are rejected. Unequal pre-latch heap counts are retained;
these checks do not prove identical live-object sets.

After integration, four actual numeric-recorder component protections passed
in3.977 seconds including compilation. They cover the new phase sequence,
abort/reuse, invalid/missing timestamps, fixed4096 capacity/drop accounting and
disabled modes. These use synthetic times and calls; they are not VM, owner
thread, real-clock, observer-off binary or GC-root correctness acceptance.

## Bounded workload attribution

Twelve serial real-product processes ran the unchanged six workloads in both
interpreted/bytecode modes, major/young, across three rounds. All passed in5.663
seconds, retaining72 workload executions and756 observed cycles. The following
are Young-cycle medians in milliseconds, pooled across the three rounds:

| Mode / workload | Cycles | Whole cycle | Registry23 | Dirty cons24 | Classification to static roots |
| --- | ---: | ---: | ---: | ---: | ---: |
| Interpreted / gc-churn | 141 | 4.008 | 3.490 | 0.249 | 3.744 |
| Interpreted / gc-retained | 18 | 4.080 | 3.482 | 0.337 | 3.827 |
| Bytecode / gc-churn | 99 | 4.049 | 3.512 | 0.246 | 3.752 |
| Bytecode / gc-retained | 15 | 4.087 | 3.506 | 0.280 | 3.789 |

Within each group, summing the matched per-cycle registry durations and dividing
by summed whole/gap durations puts registry tracing at85.3–87.3% of whole-cycle
time and91.1–93.7% of the classification-to-static interval. These are elapsed
shares, including recursive marking and instrumentation, not exclusive CPU.
Do not add overlapping phases or divide separate medians to obtain those shares.

`classification_to_static` begins after instant classification21, so its residual
subtracts only23/24. The broader `pre_static_gap` begins at phase3 and includes22
plus uninstrumented preparation/visitor work; its residual subtracts22/23/24.
Preparation and classification residual medians measured zero at the available
clock resolution, which does not establish zero overhead or absent work.

The narrow finding is that frozen non-cons registry tracing, including recursive
child marking, dominates sampled Young intervals in gc-churn/gc-retained only.
It does not extend to the other four workloads or major/Full cycles. Of756
cycles,483 are Full and273 Young. Preparation22 has identical start/end
timestamps in503 cycles; Young-product Full preparation has a median near0.008
ms, so the zero medians above do not describe every preparation interval. This does not license
omitting roots or establish marking equivalence. Host load/core was0.181–0.192,
above the existing0.10 quiet policy. There is no reference speedup comparison,
instrumentation-overhead comparison, performance adoption or App promotion.

## Evidence and remaining work

The [bound results](2026-10-30-gc-root-phases-results.json) preserve input/driver
hashes, original-result links, exact counts, per-test timing and controller
outcomes. The original build controller remains failed; subsequent file-only
finalization and real functional acceptance are separate outcomes. The build
receipt's historical pending flag is not rewritten as if acceptance occurred
during compilation. Controller metrics were read by the primary from separate
local receipts, not independently certified by the peer's functional review.

Independent peer review found no confirmed defects in the functional receipts,
all756 original cycle relationships, protection extraction/results and source
bindings. It did not rerun products or certify the separate controller metrics.

G1/G2/G4 remain three accepted outcomes out of seven. Full G3/G5/G6/G7 acceptance,
eligible performance comparisons and safe root-tracing optimization remain open.
The trial App is unchanged. Consumed-state retirement is recorded separately in
CLN160; raw observations and current/prior products remain available.
